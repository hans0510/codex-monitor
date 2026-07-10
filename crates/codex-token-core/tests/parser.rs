use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local, LocalResult, TimeZone};
use codex_token_core::{aggregate_usage, parse_session_file};

fn fixture_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/codex-home/sessions/2026/07/07/session-a.jsonl")
        .canonicalize()
        .expect("fixture exists")
}

fn fixture_home() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/codex-home")
        .canonicalize()
        .expect("fixture home exists")
}

fn local_datetime(year: i32, month: u32, day: u32, hour: u32) -> DateTime<Local> {
    match Local.with_ymd_and_hms(year, month, day, hour, 0, 0) {
        LocalResult::Single(datetime) => datetime,
        LocalResult::Ambiguous(earliest, _) => earliest,
        LocalResult::None => panic!("invalid local datetime"),
    }
}

fn write_temp_jsonl(name: &str, content: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "codex-token-monitor-{name}-{}.jsonl",
        std::process::id()
    ));
    fs::write(&path, content).expect("write temp jsonl");
    path
}

fn write_temp_codex_home(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "codex-token-monitor-home-{name}-{}",
        std::process::id()
    ));
    fs::remove_dir_all(&root).ok();
    fs::create_dir_all(root.join("sessions")).expect("sessions dir");
    fs::create_dir_all(root.join("archived_sessions")).expect("archived dir");
    root
}

#[test]
fn parser_reads_fixture_token_count() {
    let report = parse_session_file(&fixture_file()).expect("parse fixture");

    assert_eq!(report.events.len(), 2);

    let first = &report.events[0];
    assert_eq!(first.session_id, "session-a");
    assert_eq!(first.usage.input_tokens, 100);
    assert_eq!(first.usage.cached_input_tokens, 20);
    assert_eq!(first.usage.output_tokens, 30);
    assert_eq!(first.usage.reasoning_output_tokens, 5);
    assert_eq!(first.usage.total_tokens, 155);
}

#[test]
fn parser_reads_rate_limits_without_token_totals() {
    let path = write_temp_jsonl(
        "rate-limits",
        r#"{"timestamp":"2026-07-07T08:00:00Z","type":"event_msg","payload":{"type":"token_count","rate_limits":{"primary":{"used_percent":12.0,"window_minutes":300,"resets_at":1783411200},"secondary":{"used_percent":34.5,"window_minutes":10080,"resets_at":1784016000}}}}
"#,
    );

    let report = parse_session_file(&path).expect("parse rate limits");
    let snapshot = report.rate_limits.first().expect("rate-limit snapshot");
    let primary = snapshot.primary.expect("primary window");
    let secondary = snapshot.secondary.expect("secondary window");

    assert_eq!(primary.used_percent, 12.0);
    assert_eq!(primary.window_minutes, 300);
    assert_eq!(primary.resets_at, 1_783_411_200);
    assert_eq!(secondary.used_percent, 34.5);
    assert_eq!(secondary.window_minutes, 10_080);
    assert_eq!(secondary.resets_at, 1_784_016_000);

    fs::remove_file(path).ok();
}

#[test]
fn parser_ignores_non_token_lines_without_warning() {
    let report = parse_session_file(&fixture_file()).expect("parse fixture");

    assert!(report
        .diagnostics
        .iter()
        .all(|diagnostic| !diagnostic.message.contains("session_meta")));
}

#[test]
fn parser_reports_malformed_json_and_continues() {
    let path = write_temp_jsonl(
        "malformed",
        r#"{"timestamp":"2026-07-07T08:00:00Z","type":"session_meta","payload":{"session_id":"bad"}}
not-json
{"timestamp":"2026-07-07T08:01:00Z","type":"token_count","payload":{"type":"token_count","session_id":"bad","info":{"total_token_usage":{"input_tokens":1,"cached_input_tokens":2,"output_tokens":3,"reasoning_output_tokens":4,"total_tokens":10}}}}
"#,
    );

    let report = parse_session_file(Path::new(&path)).expect("parse temp");

    assert_eq!(report.events.len(), 1);
    assert_eq!(report.diagnostics.len(), 1);
    assert!(report.diagnostics[0].message.contains("malformed JSONL"));

    fs::remove_file(path).ok();
}

#[test]
fn aggregation_does_not_overcount_cumulative_totals() {
    let report = aggregate_usage(&fixture_home(), local_datetime(2026, 7, 7, 23))
        .expect("aggregate fixture");

    let session = report
        .sessions
        .iter()
        .find(|session| session.session_id == "session-a")
        .expect("session-a summary");

    assert_eq!(session.total.total_tokens, 245);
    assert_eq!(report.summary.all_time.total_tokens, 360);
    assert_ne!(report.summary.all_time.total_tokens, 155 + 245 + 115);
}

#[test]
fn aggregation_covers_time_ranges() {
    let report = aggregate_usage(&fixture_home(), local_datetime(2026, 7, 7, 23))
        .expect("aggregate fixture");

    assert_eq!(report.summary.today.total_tokens, 245);
    assert_eq!(report.summary.this_week.total_tokens, 360);
    assert_eq!(report.summary.this_month.total_tokens, 360);
    assert_eq!(report.summary.all_time.total_tokens, 360);
}

#[test]
fn aggregation_keeps_the_latest_rate_limit_snapshot() {
    let root = write_temp_codex_home("latest-rate-limits");
    fs::write(
        root.join("sessions/limits.jsonl"),
        r#"{"timestamp":"2026-07-07T08:00:00Z","type":"event_msg","payload":{"type":"token_count","session_id":"limits","info":{"total_token_usage":{"input_tokens":10,"cached_input_tokens":0,"output_tokens":0,"reasoning_output_tokens":0,"total_tokens":10}},"rate_limits":{"primary":{"used_percent":10.0,"window_minutes":300,"resets_at":1783411200},"secondary":{"used_percent":20.0,"window_minutes":10080,"resets_at":1784016000}}}}
{"timestamp":"2026-07-07T09:00:00Z","type":"event_msg","payload":{"type":"token_count","session_id":"limits","info":{"total_token_usage":{"input_tokens":20,"cached_input_tokens":0,"output_tokens":0,"reasoning_output_tokens":0,"total_tokens":20}},"rate_limits":{"primary":{"used_percent":25.0,"window_minutes":300,"resets_at":1783414800},"secondary":{"used_percent":40.0,"window_minutes":10080,"resets_at":1784019600}}}}
"#,
    )
    .expect("write rate limits");

    let report =
        aggregate_usage(&root, local_datetime(2026, 7, 7, 23)).expect("aggregate rate limits");
    let snapshot = report.latest_rate_limits.expect("latest rate limits");

    assert_eq!(snapshot.primary.expect("primary").used_percent, 25.0);
    assert_eq!(snapshot.secondary.expect("secondary").used_percent, 40.0);

    fs::remove_dir_all(root).ok();
}

#[test]
fn aggregation_deduplicates_active_and_archived_by_session_id() {
    let root = write_temp_codex_home("dedupe");
    fs::write(
        root.join("archived_sessions/dup.jsonl"),
        r#"{"timestamp":"2026-07-06T09:00:00Z","type":"token_count","payload":{"type":"token_count","session_id":"dup-session","info":{"total_token_usage":{"input_tokens":10,"cached_input_tokens":0,"output_tokens":0,"reasoning_output_tokens":0,"total_tokens":10}}}}
"#,
    )
    .expect("write archived duplicate");
    fs::write(
        root.join("sessions/dup.jsonl"),
        r#"{"timestamp":"2026-07-07T09:00:00Z","type":"token_count","payload":{"type":"token_count","session_id":"dup-session","info":{"total_token_usage":{"input_tokens":20,"cached_input_tokens":0,"output_tokens":0,"reasoning_output_tokens":0,"total_tokens":20}}}}
"#,
    )
    .expect("write active duplicate");

    let report =
        aggregate_usage(&root, local_datetime(2026, 7, 7, 23)).expect("aggregate duplicate");

    assert_eq!(report.sessions.len(), 1);
    assert_eq!(report.sessions[0].session_id, "dup-session");
    assert_eq!(report.summary.all_time.total_tokens, 20);

    fs::remove_dir_all(root).ok();
}

#[test]
fn diagnostics_reports_missing_total_token_usage() {
    let report = parse_session_file(&fixture_file()).expect("parse fixture");

    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.message.contains("missing total_token_usage")));
}

#[test]
fn diagnostics_keep_valid_events_in_same_file() {
    let report = aggregate_usage(&fixture_home(), local_datetime(2026, 7, 7, 23))
        .expect("aggregate fixture");

    assert_eq!(report.summary.all_time.total_tokens, 360);
    assert!(report.diagnostics.len() >= 2);
}

#[test]
fn diagnostics_missing_optional_reasoning_does_not_panic() {
    let path = write_temp_jsonl(
        "missing-optional",
        r#"{"timestamp":"2026-07-07T08:01:00Z","type":"token_count","payload":{"type":"token_count","session_id":"missing-optional","info":{"total_token_usage":{"input_tokens":1,"cached_input_tokens":2,"output_tokens":3,"total_tokens":6}}}}
"#,
    );
    let report = parse_session_file(&path).expect("parse temp fixture");
    let event = report
        .events
        .first()
        .expect("event with missing optional reasoning");

    assert_eq!(event.usage.reasoning_output_tokens, 0);
    assert_eq!(event.usage.total_tokens, 6);

    fs::remove_file(path).ok();
}
