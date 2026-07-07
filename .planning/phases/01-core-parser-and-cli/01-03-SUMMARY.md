---
phase: 01-core-parser-and-cli
plan: 03
subsystem: cli
tags: [rust, clap, cli, summary]
requires:
  - phase: 01-core-parser-and-cli
    provides: JSONL parser, cumulative-safe aggregation, and diagnostics
provides:
  - Ubuntu-friendly summary CLI command
  - Compact token usage table for today, week, month, and all-time totals
  - Empty-log error handling with searched paths
affects: [cli, core-parser]
tech-stack:
  added: []
  patterns:
    - Thin CLI over shared Rust core
    - Nonzero CLI error for missing local Codex session logs
key-files:
  created:
    - crates/codex-token-cli/tests/summary.rs
  modified:
    - README.md
    - crates/codex-token-core/src/lib.rs
    - crates/codex-token-cli/src/main.rs
    - crates/codex-token-core/tests/parser.rs
    - fixtures/codex-home/sessions/2026/07/07/session-a.jsonl
key-decisions:
  - "Default invocation runs the same summary output as the explicit summary subcommand."
  - "CLI depends on the core aggregate_usage_now wrapper instead of adding its own chrono dependency."
patterns-established:
  - "CLI tests execute the compiled binary against sanitized fixtures."
  - "Warnings are shown after the table without blocking valid token totals."
requirements-completed: [DATA-01, STAT-02, STAT-03, CLI-01, CLI-03, SAFE-03]
duration: 4 min
completed: 2026-07-07
---

# Phase 01 Plan 03: CLI Summary Summary

**Ubuntu-friendly CLI summary for local Codex token usage**

## Performance

- **Duration:** 4 min
- **Started:** 2026-07-07T15:48:00+08:00
- **Completed:** 2026-07-07T15:51:31+08:00
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments

- Added a `summary` CLI command and made the default invocation render the same summary.
- Rendered today, this week, this month, and all-time token totals in a compact terminal table.
- Added a nonzero no-log path with searched directories in the error message.
- Documented CLI usage and local-only safety behavior in `README.md`.

## Task Commits

1. **Task 1: Wire CLI command arguments to core aggregation** - `68eb24e` (feat)
2. **Task 2: Render compact terminal summary output** - `583d5a0` (feat)
3. **Task 3: Handle no-log and partial-parse CLI states** - `e612e2e` (feat)

## Files Created/Modified

- `crates/codex-token-cli/src/main.rs` - Clap command parsing, default summary dispatch, table output, warning output, and no-log errors.
- `crates/codex-token-cli/tests/summary.rs` - CLI integration tests using the sanitized Codex home fixture.
- `crates/codex-token-core/src/lib.rs` - Added `aggregate_usage_now` for thin CLI integration.
- `crates/codex-token-core/tests/parser.rs` - Kept optional-field coverage independent of the CLI fixture output.
- `README.md` - Added build, test, usage, and safety notes.

## Decisions Made

- Kept the CLI intentionally small: no watch mode, no config file, and no Windows UI in this phase.
- Used `--codex-home` for deterministic tests and debugging while defaulting to `CODEX_HOME` or the user's local `.codex`.
- Returned a useful error when no session logs are present instead of showing an all-zero table that could look like successful counting.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Advisory] Added a core time wrapper for the CLI**
- **Found during:** Task 1 (Wire CLI command arguments to core aggregation)
- **Issue:** Calling the lower-level aggregation API directly from the CLI would require the CLI crate to manage `chrono::DateTime`.
- **Fix:** Added `aggregate_usage_now` to the core crate so the CLI stays a thin adapter.
- **Files modified:** `crates/codex-token-core/src/lib.rs`, `crates/codex-token-cli/src/main.rs`
- **Verification:** `cargo test` passed.
- **Committed in:** `68eb24e`

**2. [Rule 3 - Advisory] Moved optional-field coverage out of the main CLI fixture**
- **Found during:** Task 2 (Render compact terminal summary output)
- **Issue:** A fixture line for missing optional `reasoning_tokens` changed the fixture totals used by CLI output tests.
- **Fix:** Moved that optional-field case into a temp-file parser test so summary fixture totals remain clear.
- **Files modified:** `crates/codex-token-core/tests/parser.rs`, `fixtures/codex-home/sessions/2026/07/07/session-a.jsonl`
- **Verification:** `cargo test` passed.
- **Committed in:** `583d5a0`

---

**Total deviations:** 2 auto-fixed advisory items
**Impact on plan:** No scope expansion. The CLI remains smaller and the fixture output is easier to validate.

## Issues Encountered

None.

## Verification

- `cargo test` - passed, 16 tests.
- `cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home` - passed.
- `cargo run -p codex-token-cli -- --codex-home fixtures/codex-home` - passed.

## Self-Check: PASSED

All acceptance criteria in `01-03-PLAN.md` passed.

## User Setup Required

Install Rust or use the user-local Rustup installation already created on this machine.

## Next Phase Readiness

Ready for phase-level review and verification. Phase 2 can build watch/refresh behavior on the same core aggregation output.

---
*Phase: 01-core-parser-and-cli*
*Completed: 2026-07-07*
