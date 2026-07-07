use std::fs;
use std::path::{Path, PathBuf};

use codex_token_core::parse_session_file;

fn fixture_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/codex-home/sessions/2026/07/07/session-a.jsonl")
        .canonicalize()
        .expect("fixture exists")
}

fn write_temp_jsonl(name: &str, content: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "codex-token-monitor-{name}-{}.jsonl",
        std::process::id()
    ));
    fs::write(&path, content).expect("write temp jsonl");
    path
}

#[test]
fn parser_reads_fixture_token_count() {
    let report = parse_session_file(&fixture_file()).expect("parse fixture");

    assert_eq!(report.events.len(), 2);
    assert!(report.diagnostics.is_empty());

    let first = &report.events[0];
    assert_eq!(first.session_id, "session-a");
    assert_eq!(first.usage.input_tokens, 100);
    assert_eq!(first.usage.cached_input_tokens, 20);
    assert_eq!(first.usage.output_tokens, 30);
    assert_eq!(first.usage.reasoning_output_tokens, 5);
    assert_eq!(first.usage.total_tokens, 155);
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
