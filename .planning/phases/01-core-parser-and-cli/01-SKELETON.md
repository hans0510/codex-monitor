# Walking Skeleton - Codex Token Monitor

**Phase:** 1
**Generated:** 2026-07-07

## Capability Proven End-to-End

A user can run one local CLI command that scans synthetic or default Codex session JSONL files, aggregates token usage without overcounting cumulative events, and prints today, week, month, and all-time totals.

## Architectural Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Core language | Rust workspace | Shared parser can be reused by Ubuntu CLI and later Windows Tauri shell. |
| Data layer | Local JSONL files under `sessions` and `archived_sessions` | The product is local-only and must not use account-wide cloud usage. |
| Auth | None | The tool reads local usage logs and must not read credentials. |
| UI | Terminal CLI in Phase 1 | Ubuntu only needs command-line statistics in this phase. |
| Deployment target | Local `cargo run` / packaged binary later | Lightweight personal utility; packaging can follow after behavior is proven. |
| Directory layout | `crates/codex-token-core` plus `crates/codex-token-cli` | Keeps counting logic reusable for the later Windows app. |

## Stack Touched in Phase 1

- [ ] Project scaffold: Rust workspace, core crate, CLI crate, tests.
- [ ] Command routing: `codex-token-monitor summary` and default summary behavior.
- [ ] Data read: local Codex JSONL session files only.
- [ ] Terminal UI: compact readable summary table.
- [ ] Local run command: `cargo run -p codex-token-cli -- summary`.

## Out of Scope (Deferred to Later Slices)

- Watch mode and active-session refresh.
- Windows tray app and desktop companion.
- Charts, exports, notifications, and cloud/account aggregation.
- Reading `auth.json`, config files, prompt text, or network APIs.

## Subsequent Slice Plan

- Phase 2: Add current-session refresh over the same parser.
- Phase 3: Reuse the Rust core inside the Windows tray and stats panel.
- Phase 4: Add the original clickable capybara companion.
