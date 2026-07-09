use std::path::PathBuf;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_codex-token-monitor"))
}

fn fixture_home() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/codex-home")
}

fn empty_codex_home() -> PathBuf {
    let path =
        std::env::temp_dir().join(format!("codex-token-monitor-empty-{}", std::process::id()));
    std::fs::remove_dir_all(&path).ok();
    std::fs::create_dir_all(&path).expect("empty codex home");
    path
}

fn codex_home_with_session(name: &str, content: &str) -> PathBuf {
    let path =
        std::env::temp_dir().join(format!("codex-token-monitor-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&path).ok();
    let sessions = path.join("sessions");
    std::fs::create_dir_all(&sessions).expect("sessions dir");
    std::fs::write(sessions.join("large.jsonl"), content).expect("session jsonl");
    path
}

#[test]
fn summary_runs_with_codex_home_override() {
    let output = bin()
        .args(["summary", "--codex-home"])
        .arg(fixture_home())
        .output()
        .expect("run summary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Codex Token Summary"));
    assert!(stdout.contains("All time"));
}

#[test]
fn summary_runs_without_explicit_subcommand() {
    let output = bin()
        .arg("--codex-home")
        .arg(fixture_home())
        .output()
        .expect("run default summary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Codex Token Summary"));
    assert!(stdout.contains("All time"));
}

#[test]
fn summary_output_has_range_rows_and_token_columns() {
    let output = bin()
        .args(["summary", "--codex-home"])
        .arg(fixture_home())
        .output()
        .expect("run summary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    for expected in [
        "Today",
        "This week",
        "This month",
        "All time",
        "Input",
        "Cached",
        "Output",
        "Reasoning",
        "Total",
        "360",
    ] {
        assert!(stdout.contains(expected), "missing {expected} in {stdout}");
    }
}

#[test]
fn summary_hides_warnings_by_default() {
    let output = bin()
        .args(["summary", "--codex-home"])
        .arg(fixture_home())
        .output()
        .expect("run summary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("Warnings:"), "{stdout}");
    assert!(!stdout.contains("missing total_token_usage"), "{stdout}");
}

#[test]
fn summary_shows_warnings_when_requested() {
    let output = bin()
        .args(["--warnings", "summary", "--codex-home"])
        .arg(fixture_home())
        .output()
        .expect("run summary with warnings");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Warnings:"), "{stdout}");
    assert!(stdout.contains("missing total_token_usage"), "{stdout}");
}

#[test]
fn summary_formats_large_counts_with_units() {
    let codex_home = codex_home_with_session(
        "large-units",
        r#"{"timestamp":"2026-07-07T08:00:00Z","type":"token_count","payload":{"type":"token_count","session_id":"large","info":{"total_token_usage":{"input_tokens":1200000,"cached_input_tokens":500000,"output_tokens":1200,"reasoning_output_tokens":0,"total_tokens":1701200}}}}
"#,
    );
    let output = bin()
        .args(["summary", "--codex-home"])
        .arg(&codex_home)
        .output()
        .expect("run summary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1.2M"), "{stdout}");
    assert!(stdout.contains("500K"), "{stdout}");
    assert!(stdout.contains("1.7M"), "{stdout}");

    std::fs::remove_dir_all(codex_home).ok();
}

#[test]
fn no_session_logs_exits_nonzero() {
    let codex_home = empty_codex_home();
    let output = bin()
        .args(["summary", "--codex-home"])
        .arg(&codex_home)
        .output()
        .expect("run summary with no logs");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No Codex session logs found"));
    assert!(stderr.contains("sessions"));
    assert!(stderr.contains("archived_sessions"));

    std::fs::remove_dir_all(codex_home).ok();
}
