use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::ops::AddAssign;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Datelike, Duration, Local, LocalResult, NaiveDate, TimeZone};
use serde_json::Value;

pub const APP_NAME: &str = "codex-token-monitor";

const SESSION_DIRS: [&str; 2] = ["sessions", "archived_sessions"];

#[derive(Debug)]
pub enum ScanError {
    ReadDir { path: PathBuf, source: io::Error },
    Entry { path: PathBuf, source: io::Error },
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadDir { path, source } => {
                write!(f, "failed to read directory {}: {source}", path.display())
            }
            Self::Entry { path, source } => {
                write!(f, "failed to read entry under {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for ScanError {}

#[derive(Debug)]
pub enum UsageError {
    Scan(ScanError),
    ReadFile { path: PathBuf, source: io::Error },
}

impl fmt::Display for UsageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scan(error) => write!(f, "{error}"),
            Self::ReadFile { path, source } => {
                write!(f, "failed to read file {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for UsageError {}

impl From<ScanError> for UsageError {
    fn from(error: ScanError) -> Self {
        Self::Scan(error)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub cached_input_tokens: u64,
    pub output_tokens: u64,
    pub reasoning_output_tokens: u64,
    pub total_tokens: u64,
}

impl TokenUsage {
    pub fn saturating_delta(self, previous: Self) -> Self {
        Self {
            input_tokens: self.input_tokens.saturating_sub(previous.input_tokens),
            cached_input_tokens: self
                .cached_input_tokens
                .saturating_sub(previous.cached_input_tokens),
            output_tokens: self.output_tokens.saturating_sub(previous.output_tokens),
            reasoning_output_tokens: self
                .reasoning_output_tokens
                .saturating_sub(previous.reasoning_output_tokens),
            total_tokens: self.total_tokens.saturating_sub(previous.total_tokens),
        }
    }
}

impl AddAssign for TokenUsage {
    fn add_assign(&mut self, rhs: Self) {
        self.input_tokens += rhs.input_tokens;
        self.cached_input_tokens += rhs.cached_input_tokens;
        self.output_tokens += rhs.output_tokens;
        self.reasoning_output_tokens += rhs.reasoning_output_tokens;
        self.total_tokens += rhs.total_tokens;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenEvent {
    pub session_id: String,
    pub timestamp: DateTime<Local>,
    pub usage: TokenUsage,
    pub source_path: PathBuf,
    pub line_number: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSummary {
    pub session_id: String,
    pub total: TokenUsage,
    pub event_count: usize,
    pub last_event_at: DateTime<Local>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UsageSummary {
    pub today: TokenUsage,
    pub this_week: TokenUsage,
    pub this_month: TokenUsage,
    pub all_time: TokenUsage,
}

#[derive(Debug, Default)]
pub struct UsageReport {
    pub summary: UsageSummary,
    pub sessions: Vec<SessionSummary>,
    pub diagnostics: Vec<Diagnostic>,
    pub session_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub path: PathBuf,
    pub line_number: usize,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct ParseReport {
    pub events: Vec<TokenEvent>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn discover_codex_home(override_path: Option<&Path>) -> Option<PathBuf> {
    if let Some(path) = override_path {
        return Some(path.to_path_buf());
    }

    if let Some(path) = env::var_os("CODEX_HOME").filter(|value| !value.is_empty()) {
        return Some(PathBuf::from(path));
    }

    home_dir().map(|home| home.join(".codex"))
}

pub fn discover_session_files(codex_home: &Path) -> Result<Vec<PathBuf>, ScanError> {
    let mut files = Vec::new();

    for dir_name in SESSION_DIRS {
        let dir = codex_home.join(dir_name);
        if dir.is_dir() {
            collect_jsonl_files(&dir, &mut files)?;
        }
    }

    files.sort();
    Ok(files)
}

pub fn parse_session_file(path: &Path) -> Result<ParseReport, UsageError> {
    let content = fs::read_to_string(path).map_err(|source| UsageError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let mut report = ParseReport::default();

    for (line_index, line) in content.lines().enumerate() {
        let line_number = line_index + 1;
        if line.trim().is_empty() {
            continue;
        }

        let value = match serde_json::from_str::<Value>(line) {
            Ok(value) => value,
            Err(error) => {
                report.diagnostics.push(Diagnostic {
                    path: path.to_path_buf(),
                    line_number,
                    message: format!("malformed JSONL: {error}"),
                });
                continue;
            }
        };

        if !is_token_count_event(&value) {
            continue;
        }

        let Some(timestamp) = parse_timestamp(&value) else {
            report.diagnostics.push(Diagnostic {
                path: path.to_path_buf(),
                line_number,
                message: "missing or invalid timestamp".to_string(),
            });
            continue;
        };

        let Some(usage_value) = value.pointer("/payload/info/total_token_usage") else {
            report.diagnostics.push(Diagnostic {
                path: path.to_path_buf(),
                line_number,
                message: "missing total_token_usage".to_string(),
            });
            continue;
        };

        let session_id = session_id(&value).unwrap_or_else(|| fallback_session_id(path));

        report.events.push(TokenEvent {
            session_id,
            timestamp,
            usage: parse_token_usage(usage_value),
            source_path: path.to_path_buf(),
            line_number,
        });
    }

    Ok(report)
}

pub fn aggregate_usage(codex_home: &Path, now: DateTime<Local>) -> Result<UsageReport, UsageError> {
    let session_files = discover_session_files(codex_home)?;
    let mut diagnostics = Vec::new();
    let mut events_by_session: BTreeMap<String, Vec<TokenEvent>> = BTreeMap::new();

    for path in &session_files {
        let report = parse_session_file(path)?;
        diagnostics.extend(report.diagnostics);

        for event in report.events {
            events_by_session
                .entry(event.session_id.clone())
                .or_default()
                .push(event);
        }
    }

    let ranges = RangeStarts::new(now);
    let mut summary = UsageSummary::default();
    let mut sessions = Vec::new();

    for (session_id, mut events) in events_by_session {
        events.sort_by(|left, right| {
            left.timestamp
                .cmp(&right.timestamp)
                .then_with(|| left.source_path.cmp(&right.source_path))
                .then_with(|| left.line_number.cmp(&right.line_number))
        });

        let mut previous = None;
        for event in &events {
            let delta = previous.map_or(event.usage, |usage| event.usage.saturating_delta(usage));

            if event.timestamp >= ranges.today {
                summary.today += delta;
            }
            if event.timestamp >= ranges.this_week {
                summary.this_week += delta;
            }
            if event.timestamp >= ranges.this_month {
                summary.this_month += delta;
            }

            previous = Some(event.usage);
        }

        if let Some(last) = events.last() {
            summary.all_time += last.usage;
            sessions.push(SessionSummary {
                session_id,
                total: last.usage,
                event_count: events.len(),
                last_event_at: last.timestamp,
            });
        }
    }

    sessions.sort_by(|left, right| left.session_id.cmp(&right.session_id));

    Ok(UsageReport {
        summary,
        sessions,
        diagnostics,
        session_files,
    })
}

pub fn aggregate_usage_now(codex_home: &Path) -> Result<UsageReport, UsageError> {
    aggregate_usage(codex_home, Local::now())
}

pub fn format_token_count(value: u64) -> String {
    if value >= 1_000_000 {
        format_scaled_token_count(value, 1_000_000, "M")
    } else if value >= 1_000 {
        format_scaled_token_count(value, 1_000, "K")
    } else {
        value.to_string()
    }
}

fn collect_jsonl_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), ScanError> {
    let entries = fs::read_dir(dir).map_err(|source| ScanError::ReadDir {
        path: dir.to_path_buf(),
        source,
    })?;

    for entry in entries {
        let entry = entry.map_err(|source| ScanError::Entry {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|source| ScanError::Entry {
            path: path.clone(),
            source,
        })?;

        if file_type.is_dir() {
            collect_jsonl_files(&path, files)?;
        } else if file_type.is_file() && path.extension().is_some_and(|ext| ext == "jsonl") {
            files.push(path);
        }
    }

    Ok(())
}

fn is_token_count_event(value: &Value) -> bool {
    value.get("type").and_then(Value::as_str) == Some("token_count")
        || value.pointer("/payload/type").and_then(Value::as_str) == Some("token_count")
}

fn parse_timestamp(value: &Value) -> Option<DateTime<Local>> {
    let timestamp = value
        .get("timestamp")
        .or_else(|| value.pointer("/payload/timestamp"))
        .and_then(Value::as_str)?;

    DateTime::parse_from_rfc3339(timestamp)
        .ok()
        .map(|datetime| datetime.with_timezone(&Local))
}

fn session_id(value: &Value) -> Option<String> {
    [
        "/payload/session_id",
        "/payload/id",
        "/payload/session/id",
        "/session_id",
    ]
    .iter()
    .find_map(|pointer| {
        value
            .pointer(pointer)
            .and_then(Value::as_str)
            .filter(|id| !id.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn fallback_session_id(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown-session")
        .to_string()
}

fn parse_token_usage(value: &Value) -> TokenUsage {
    let usage = TokenUsage {
        input_tokens: usage_field(value, "input_tokens"),
        cached_input_tokens: usage_field(value, "cached_input_tokens"),
        output_tokens: usage_field(value, "output_tokens"),
        reasoning_output_tokens: usage_field(value, "reasoning_output_tokens"),
        total_tokens: usage_field(value, "total_tokens"),
    };

    if usage.total_tokens == 0 {
        TokenUsage {
            total_tokens: usage.input_tokens
                + usage.cached_input_tokens
                + usage.output_tokens
                + usage.reasoning_output_tokens,
            ..usage
        }
    } else {
        usage
    }
}

fn usage_field(value: &Value, field: &str) -> u64 {
    value.get(field).and_then(Value::as_u64).unwrap_or(0)
}

fn format_scaled_token_count(value: u64, unit: u64, suffix: &str) -> String {
    let scaled = value as f64 / unit as f64;
    let rendered = format!("{scaled:.1}");
    let trimmed = rendered.trim_end_matches(".0");
    format!("{trimmed}{suffix}")
}

struct RangeStarts {
    today: DateTime<Local>,
    this_week: DateTime<Local>,
    this_month: DateTime<Local>,
}

impl RangeStarts {
    fn new(now: DateTime<Local>) -> Self {
        let today_date = now.date_naive();
        let week_date = today_date - Duration::days(now.weekday().num_days_from_monday() as i64);
        let month_date =
            NaiveDate::from_ymd_opt(now.year(), now.month(), 1).expect("valid month start");

        Self {
            today: local_midnight(today_date),
            this_week: local_midnight(week_date),
            this_month: local_midnight(month_date),
        }
    }
}

fn local_midnight(date: NaiveDate) -> DateTime<Local> {
    let naive = date.and_hms_opt(0, 0, 0).expect("valid midnight");
    match Local.from_local_datetime(&naive) {
        LocalResult::Single(datetime) => datetime,
        LocalResult::Ambiguous(earliest, _) => earliest,
        LocalResult::None => Local.from_utc_datetime(&naive),
    }
}

#[cfg(windows)]
fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE").map(PathBuf::from)
}

#[cfg(not(windows))]
fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_home() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/codex-home")
            .canonicalize()
            .expect("fixture home exists")
    }

    #[test]
    fn discovery_finds_active_and_archived_session_logs() {
        let files = discover_session_files(&fixture_home()).expect("session files");
        let rendered = files
            .iter()
            .map(|path| path.to_string_lossy())
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(files.len(), 2);
        assert!(rendered.contains("session-a.jsonl"));
        assert!(rendered.contains("session-old.jsonl"));
    }

    #[test]
    fn discovery_ignores_auth_json() {
        let files = discover_session_files(&fixture_home()).expect("session files");

        assert!(files.iter().all(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name != "auth.json")
        }));
    }

    #[test]
    fn explicit_home_override_wins() {
        let override_path = Path::new("custom-codex-home");

        assert_eq!(
            discover_codex_home(Some(override_path)),
            Some(PathBuf::from("custom-codex-home"))
        );
    }

    #[test]
    fn token_count_format_uses_k_and_m_units() {
        assert_eq!(format_token_count(999), "999");
        assert_eq!(format_token_count(1_000), "1K");
        assert_eq!(format_token_count(1_550), "1.6K");
        assert_eq!(format_token_count(15_400), "15.4K");
        assert_eq!(format_token_count(150_000), "150K");
        assert_eq!(format_token_count(1_000_000), "1M");
        assert_eq!(format_token_count(1_240_000), "1.2M");
    }
}
