---
phase: 01-core-parser-and-cli
plan: 01
subsystem: infra
tags: [rust, cargo, fixtures, discovery]
requires: []
provides:
  - Rust workspace with reusable core crate and CLI crate
  - Synthetic Codex JSONL fixtures
  - Allowlisted session log discovery for sessions and archived_sessions
affects: [core-parser, cli, windows-tauri]
tech-stack:
  added: [rust, cargo, chrono, clap, serde, serde_json]
  patterns:
    - Shared Rust core with thin adapters
    - Local Codex session allowlist scanning
key-files:
  created:
    - Cargo.toml
    - Cargo.lock
    - .gitignore
    - crates/codex-token-core/Cargo.toml
    - crates/codex-token-core/src/lib.rs
    - crates/codex-token-cli/Cargo.toml
    - crates/codex-token-cli/src/main.rs
    - fixtures/codex-home/sessions/2026/07/07/session-a.jsonl
    - fixtures/codex-home/archived_sessions/2026/07/06/session-old.jsonl
    - fixtures/codex-home/auth.json
  modified: []
key-decisions:
  - "Use standard-library recursive scanning instead of adding walkdir for two allowlisted roots."
  - "Commit Cargo.lock because this repo builds a binary CLI."
patterns-established:
  - "Discovery allowlist: only sessions and archived_sessions are scanned under a Codex home."
  - "Synthetic fixtures: tests use committed hand-written JSONL, not copied local logs."
requirements-completed: [DATA-01, DATA-02, DATA-04, SAFE-03]
duration: 13 min
completed: 2026-07-07
---

# Phase 01 Plan 01: Scaffold and Discovery Summary

**Rust workspace with synthetic Codex fixtures and allowlisted local session discovery**

## Performance

- **Duration:** 13 min
- **Started:** 2026-07-07T15:30:00+08:00
- **Completed:** 2026-07-07T15:42:53+08:00
- **Tasks:** 3
- **Files modified:** 10

## Accomplishments

- Created the Rust workspace, reusable core crate, and CLI crate.
- Added synthetic Codex JSONL fixtures plus a dummy `auth.json` sentinel.
- Implemented deterministic discovery for `.jsonl` files under `sessions` and `archived_sessions` only.

## Task Commits

1. **Task 1: Create Rust workspace and crate boundaries** - `6545d67` (chore)
2. **Task 2: Add synthetic Codex home fixtures** - `0bae23c` (test)
3. **Task 3: Implement allowlisted session discovery** - `096c41b` (feat)

## Files Created/Modified

- `Cargo.toml` - Workspace members and shared dependencies.
- `Cargo.lock` - Locked Rust dependencies for reproducible CLI builds.
- `.gitignore` - Ignores Rust build output under `target/`.
- `crates/codex-token-core/src/lib.rs` - Codex home discovery and allowlisted session JSONL scanning.
- `crates/codex-token-cli/src/main.rs` - Minimal CLI crate entry point.
- `fixtures/codex-home/...` - Synthetic active, archived, and credential-name decoy fixtures.

## Decisions Made

- Used standard-library directory traversal to keep discovery small and avoid an extra dependency.
- Added `.gitignore` during execution because `cargo test` produced `target/`, which should never be committed.
- Installed a user-local Rust toolchain and rustfmt because the machine did not have `cargo` available on PATH.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added build-output ignore and committed lockfile**
- **Found during:** Task 3 (Implement allowlisted session discovery)
- **Issue:** Running Cargo created `target/` and `Cargo.lock`; without `.gitignore`, build output would remain as untracked noise.
- **Fix:** Added `.gitignore` with `/target/` and committed `Cargo.lock` for the binary workspace.
- **Files modified:** `.gitignore`, `Cargo.lock`
- **Verification:** `git status --short` no longer lists `target/`; `cargo metadata --no-deps --format-version 1` passes.
- **Committed in:** `096c41b`

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The change keeps generated build output out of git and preserves reproducible dependency resolution. No scope expansion.

## Issues Encountered

- Rust was not installed on the machine. Installed Rustup minimal profile and rustfmt locally under the user profile, without modifying PATH.

## Verification

- `cargo metadata --no-deps --format-version 1` - passed.
- `cargo test -p codex-token-core discovery` - passed, 2 tests.
- `rg "DO_NOT_READ" fixtures/codex-home/auth.json` - passed.

## Self-Check: PASSED

All acceptance criteria in `01-01-PLAN.md` passed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for `01-02`: the core crate has safe file discovery and fixtures for parser and aggregation tests.

---
*Phase: 01-core-parser-and-cli*
*Completed: 2026-07-07*
