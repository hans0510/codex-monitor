---
status: clean
phase: 01-core-parser-and-cli
phase_number: "01"
depth: standard
files_reviewed: 13
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
reviewed: 2026-07-07
---

# Phase 01 Code Review

## Scope

Reviewed source, tests, fixtures, and project configuration introduced or modified by phase 01:

- `Cargo.toml`
- `Cargo.lock`
- `.gitignore`
- `README.md`
- `crates/codex-token-core/Cargo.toml`
- `crates/codex-token-core/src/lib.rs`
- `crates/codex-token-core/tests/parser.rs`
- `crates/codex-token-cli/Cargo.toml`
- `crates/codex-token-cli/src/main.rs`
- `crates/codex-token-cli/tests/summary.rs`
- `fixtures/codex-home/sessions/2026/07/07/session-a.jsonl`
- `fixtures/codex-home/archived_sessions/2026/07/06/session-old.jsonl`
- `fixtures/codex-home/auth.json`

## Findings

No critical, warning, or info findings.

## Checks

- Directory scanning is constrained to `sessions` and `archived_sessions`; credential-like files are not traversed.
- JSONL parsing is tolerant of malformed lines and missing required token usage fields while preserving valid events.
- Aggregation uses the latest cumulative usage per session for all-time totals and positive deltas for range totals.
- CLI defaults to `summary`, supports `--codex-home`, prints diagnostics, and exits nonzero when no session logs exist.
- Tests cover parser behavior, cumulative overcount protection, duplicate active/archive sessions, CLI output, and no-log errors.

## Verification

- `cargo test` - passed.
- `cargo clippy --all-targets --all-features -- -D warnings` - passed.
- `cargo fmt --check` - passed.

## Residual Risk

Codex local session log shape is not a public stability contract. The parser intentionally uses defensive `serde_json::Value` extraction, but future log format changes may require a parser update.
