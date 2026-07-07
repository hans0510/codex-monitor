---
phase: 01-core-parser-and-cli
verified: 2026-07-07T15:57:28+08:00
status: passed
score: 4/4 must-haves verified
requirements_satisfied: 11/11
human_verification: []
---

# Phase 1: Core Parser and CLI Verification Report

**Phase Goal:** User can run an Ubuntu-friendly CLI that accurately reports local Codex token summaries from session logs.
**Verified:** 2026-07-07T15:57:28+08:00
**Status:** passed

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | User can run a CLI command and see today, this week, this month, and all-time token totals from local Codex logs. | VERIFIED | `cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home` exits 0 and prints all four range rows. |
| 2 | Parser tests prove cumulative `total_token_usage` events are not overcounted. | VERIFIED | `aggregation_does_not_overcount_cumulative_totals` verifies all-time total 360, not 155 + 245 + 115. |
| 3 | Scanner reads only session log locations and excludes credential files. | VERIFIED | `discover_session_files` only walks `sessions` and `archived_sessions`; `discovery_ignores_auth_json` passes. |
| 4 | CLI exits with a useful message when no session logs are found. | VERIFIED | Empty-home CLI run exits 1 and prints `No Codex session logs found` plus searched directories. |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `Cargo.toml` | Rust workspace definition | VERIFIED | Workspace includes both core and CLI crates. |
| `crates/codex-token-core/src/lib.rs` | Discovery, parser, aggregation core | VERIFIED | Contains allowlisted discovery, JSONL parsing, `aggregate_usage`, and `aggregate_usage_now`. |
| `fixtures/codex-home/auth.json` | Credential-name decoy fixture | VERIFIED | Exists with `DO_NOT_READ`; discovery tests prove it is ignored. |
| `crates/codex-token-core/tests/parser.rs` | Parser and aggregation tests | VERIFIED | Covers malformed JSON, missing total usage, time ranges, dedupe, and cumulative overcounting. |
| `crates/codex-token-cli/src/main.rs` | CLI entry point | VERIFIED | Defines `summary`, `--codex-home`, table output, warnings, and no-log errors. |
| `crates/codex-token-cli/tests/summary.rs` | CLI integration tests | VERIFIED | Covers explicit summary, default summary, table columns, and no-log exit. |
| `README.md` | Basic local run instructions | VERIFIED | Documents `codex-token-monitor`, fixture run, default run, and local-only safety. |

**Artifacts:** 7/7 verified

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|----|--------|---------|
| `crates/codex-token-cli/Cargo.toml` | `crates/codex-token-core` | Path dependency | VERIFIED | GSD key-link check passed. |
| `discover_session_files` | `aggregate_usage` | Session JSONL paths feed parser | VERIFIED | Manual source check: `aggregate_usage` calls `discover_session_files` and then `parse_session_file` in `crates/codex-token-core/src/lib.rs:230`. |
| `crates/codex-token-cli/src/main.rs` | shared core aggregation | `aggregate_usage_now` call | VERIFIED | CLI imports `aggregate_usage_now` and calls it from summary mode. |

**Wiring:** 3/3 connections verified

## Requirements Coverage

| Requirement | Status | Evidence |
|-------------|--------|----------|
| DATA-01 | SATISFIED | `discover_codex_home` uses `CODEX_HOME`, then the current user's default `.codex`; CLI calls it. |
| DATA-02 | SATISFIED | Discovery only scans `sessions` and `archived_sessions`; `auth.json` fixture is ignored. |
| DATA-03 | SATISFIED | Parser reads JSONL `token_count` events into typed records. |
| DATA-04 | SATISFIED | Aggregation includes active and archived sessions and deduplicates by session id. |
| DATA-05 | SATISFIED | Malformed JSONL and missing `total_token_usage` produce diagnostics without dropping valid events. |
| STAT-01 | SATISFIED | Latest cumulative total per session is used for all-time totals. |
| STAT-02 | SATISFIED | CLI and tests cover today, this week, this month, and all-time totals. |
| STAT-03 | SATISFIED | Input, cached input, output, reasoning output, and total fields are preserved and printed. |
| CLI-01 | SATISFIED | `codex-token-monitor summary` prints a readable terminal table. |
| CLI-03 | SATISFIED | No-log path exits nonzero with searched paths. |
| SAFE-03 | SATISFIED | Phase 1 has no network code or usage API path. |

**Coverage:** 11/11 phase requirements satisfied

## Behavioral Verification

| Check | Result | Detail |
|-------|--------|--------|
| `cargo test` | PASS | 16 tests passed. |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS | No warnings after installing the clippy component. |
| `cargo fmt --check` | PASS | Formatting is clean. |
| `cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home` | PASS | Prints totals: Today 245, This week 360, This month 360, All time 360. |
| `cargo run -p codex-token-cli -- --codex-home fixtures/codex-home` | PASS | Default command path prints the same summary. |
| Empty Codex home CLI run | PASS | Exits 1 with `No Codex session logs found` and searched directories. |

## Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| - | - | - | - | None found. |

**Anti-patterns:** 0 found

## Test Quality Audit

| Test File | Active | Skipped | Circular | Assertion Level | Verdict |
|-----------|--------|---------|----------|-----------------|---------|
| `crates/codex-token-core/tests/parser.rs` | 9 | 0 | 0 | Value and behavioral | PASS |
| `crates/codex-token-cli/tests/summary.rs` | 4 | 0 | 0 | Status and output content | PASS |

Disabled-test and circular-fixture scans found no blocking issues. Matches for words such as `ignore` were test function names, not disabled tests.

## Human Verification Required

None. The Phase 1 CLI behavior is covered by automated command execution against sanitized fixtures.

## Gaps Summary

No open gaps found. Phase goal achieved.

During verification, the static artifact checker initially missed two 01-03 patterns because the CLI option and binary name were behaviorally present but not literal enough for the checker. Fixed and committed in `76289e6`:

- Added CLI help text containing `--codex-home`.
- Added README text naming the `codex-token-monitor` binary.

After the fix, all artifact checks pass.

## Verification Metadata

**Verification approach:** Goal-backward from Phase 1 roadmap success criteria.
**Must-haves source:** ROADMAP success criteria, cross-checked against PLAN frontmatter.
**Automated checks:** 6 passed, 0 failed.
**Human checks required:** 0.
**Total verification time:** 6 min.

---
*Verified: 2026-07-07T15:57:28+08:00*
*Verifier: the agent*
