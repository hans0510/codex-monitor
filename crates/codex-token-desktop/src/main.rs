use std::path::PathBuf;

use codex_token_core::{
    aggregate_usage_now, discover_codex_home, format_token_count, RateLimitSnapshot,
    RateLimitWindow, TokenUsage, UsageReport,
};
use serde::Serialize;

const FIVE_HOUR_WINDOW_MINUTES: u64 = 300;
const WEEK_WINDOW_MINUTES: u64 = 10_080;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DesktopUsage {
    codex_home: Option<String>,
    session_files: usize,
    sessions: usize,
    ranges: Vec<UsageRange>,
    quotas: QuotasView,
    diagnostics: Vec<String>,
    status: UsageStatus,
    message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UsageRange {
    label: &'static str,
    usage: TokenUsageView,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TokenUsageView {
    input: String,
    cached: String,
    output: String,
    reasoning: String,
    total: String,
    total_raw: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct QuotasView {
    five_hour: Option<QuotaView>,
    weekly: Option<QuotaView>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct QuotaView {
    remaining_percent: f64,
    resets_at: i64,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
enum UsageStatus {
    Ready,
    NoLogs,
    Error,
}

#[tauri::command]
async fn get_usage() -> DesktopUsage {
    tauri::async_runtime::spawn_blocking(collect_usage)
        .await
        .unwrap_or_else(|error| DesktopUsage {
            codex_home: None,
            session_files: 0,
            sessions: 0,
            ranges: empty_ranges(),
            quotas: empty_quotas(),
            diagnostics: Vec::new(),
            status: UsageStatus::Error,
            message: format!("统计任务失败：{error}"),
        })
}

fn collect_usage() -> DesktopUsage {
    let Some(codex_home) = discover_codex_home(None) else {
        return DesktopUsage {
            codex_home: None,
            session_files: 0,
            sessions: 0,
            ranges: empty_ranges(),
            quotas: empty_quotas(),
            diagnostics: Vec::new(),
            status: UsageStatus::Error,
            message: "无法定位 Codex home".to_string(),
        };
    };

    match aggregate_usage_now(&codex_home) {
        Ok(report) if report.session_files.is_empty() => no_logs_usage(codex_home),
        Ok(report) => ready_usage(codex_home, report),
        Err(error) => DesktopUsage {
            codex_home: Some(codex_home.display().to_string()),
            session_files: 0,
            sessions: 0,
            ranges: empty_ranges(),
            quotas: empty_quotas(),
            diagnostics: Vec::new(),
            status: UsageStatus::Error,
            message: error.to_string(),
        },
    }
}

fn ready_usage(codex_home: PathBuf, report: UsageReport) -> DesktopUsage {
    let quotas = quota_views(report.latest_rate_limits.as_ref());

    DesktopUsage {
        codex_home: Some(codex_home.display().to_string()),
        session_files: report.session_files.len(),
        sessions: report.sessions.len(),
        ranges: vec![
            usage_range("Today", report.summary.today),
            usage_range("Week", report.summary.this_week),
            usage_range("Month", report.summary.this_month),
            usage_range("All", report.summary.all_time),
        ],
        quotas,
        diagnostics: report
            .diagnostics
            .iter()
            .take(4)
            .map(|diagnostic| {
                format!(
                    "{}:{} {}",
                    diagnostic.path.display(),
                    diagnostic.line_number,
                    diagnostic.message
                )
            })
            .collect(),
        status: UsageStatus::Ready,
        message: "实时记录中".to_string(),
    }
}

fn no_logs_usage(codex_home: PathBuf) -> DesktopUsage {
    DesktopUsage {
        codex_home: Some(codex_home.display().to_string()),
        session_files: 0,
        sessions: 0,
        ranges: empty_ranges(),
        quotas: empty_quotas(),
        diagnostics: Vec::new(),
        status: UsageStatus::NoLogs,
        message: "还没有发现 Codex session 日志".to_string(),
    }
}

fn empty_ranges() -> Vec<UsageRange> {
    vec![
        usage_range("Today", TokenUsage::default()),
        usage_range("Week", TokenUsage::default()),
        usage_range("Month", TokenUsage::default()),
        usage_range("All", TokenUsage::default()),
    ]
}

fn usage_range(label: &'static str, usage: TokenUsage) -> UsageRange {
    UsageRange {
        label,
        usage: token_usage_view(usage),
    }
}

fn token_usage_view(usage: TokenUsage) -> TokenUsageView {
    TokenUsageView {
        input: format_token_count(usage.input_tokens),
        cached: format_token_count(usage.cached_input_tokens),
        output: format_token_count(usage.output_tokens),
        reasoning: format_token_count(usage.reasoning_output_tokens),
        total: format_token_count(usage.total_tokens),
        total_raw: usage.total_tokens,
    }
}

fn empty_quotas() -> QuotasView {
    QuotasView {
        five_hour: None,
        weekly: None,
    }
}

fn quota_views(snapshot: Option<&RateLimitSnapshot>) -> QuotasView {
    QuotasView {
        five_hour: snapshot
            .and_then(|snapshot| quota_window(snapshot, FIVE_HOUR_WINDOW_MINUTES))
            .map(quota_view),
        weekly: snapshot
            .and_then(|snapshot| quota_window(snapshot, WEEK_WINDOW_MINUTES))
            .map(quota_view),
    }
}

fn quota_window(snapshot: &RateLimitSnapshot, window_minutes: u64) -> Option<&RateLimitWindow> {
    [snapshot.primary.as_ref(), snapshot.secondary.as_ref()]
        .into_iter()
        .flatten()
        .find(|window| window.window_minutes == window_minutes)
}

fn quota_view(window: &RateLimitWindow) -> QuotaView {
    QuotaView {
        remaining_percent: (100.0 - window.used_percent).clamp(0.0, 100.0),
        resets_at: window.resets_at,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_usage])
        .run(tauri::generate_context!())
        .expect("failed to run Codex Token Monitor desktop app");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn desktop_usage_payload_uses_formatted_totals() {
        let root =
            std::env::temp_dir().join(format!("codex-token-desktop-large-{}", std::process::id()));
        fs::remove_dir_all(&root).ok();
        fs::create_dir_all(root.join("sessions")).expect("sessions dir");
        fs::write(
            root.join("sessions/large.jsonl"),
            r#"{"timestamp":"2026-07-07T08:00:00Z","type":"token_count","payload":{"type":"token_count","session_id":"large","info":{"total_token_usage":{"input_tokens":1200000,"cached_input_tokens":500000,"output_tokens":1200,"reasoning_output_tokens":0,"total_tokens":1701200}},"rate_limits":{"primary":{"used_percent":12.0,"window_minutes":300,"resets_at":1783411200},"secondary":{"used_percent":34.0,"window_minutes":10080,"resets_at":1784016000}}}}
"#,
        )
        .expect("write session");

        let previous = std::env::var_os("CODEX_HOME");
        std::env::set_var("CODEX_HOME", &root);

        let usage = collect_usage();
        let all = usage
            .ranges
            .iter()
            .find(|range| range.label == "All")
            .expect("all range");

        assert_eq!(usage.status, UsageStatus::Ready);
        assert_eq!(all.usage.input, "1.2M");
        assert_eq!(all.usage.cached, "500K");
        assert_eq!(all.usage.output, "1.2K");
        assert_eq!(all.usage.total, "1.7M");
        let five_hour = usage.quotas.five_hour.expect("five-hour quota");
        let weekly = usage.quotas.weekly.expect("weekly quota");
        assert_eq!(five_hour.remaining_percent, 88.0);
        assert_eq!(five_hour.resets_at, 1_783_411_200);
        assert_eq!(weekly.remaining_percent, 66.0);
        assert_eq!(weekly.resets_at, 1_784_016_000);

        if let Some(value) = previous {
            std::env::set_var("CODEX_HOME", value);
        } else {
            std::env::remove_var("CODEX_HOME");
        }
        fs::remove_dir_all(root).ok();
    }
}
