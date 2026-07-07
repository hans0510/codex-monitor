use std::path::PathBuf;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_codex-token-monitor"))
}

fn fixture_home() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/codex-home")
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
