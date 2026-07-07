# Phase 1: Core Parser and CLI - Context

**Gathered:** 2026-07-07
**Status:** Ready for planning
**Mode:** Smart discuss, user delegated decisions to the agent

<domain>
## Phase Boundary

This phase delivers the smallest useful local Codex token monitor: a shared Rust parser/statistics core plus an Ubuntu-friendly CLI summary command. It must prove local-only log scanning, safe parsing, correct cumulative token handling, and readable summary output. It does not include current-session watch mode, Windows tray UI, the desktop companion, charts, exports, or cloud/account usage.

</domain>

<decisions>
## Implementation Decisions

### CLI Output
- Default CLI output should be compact and readable: a small summary table for time ranges plus a short current/latest-session block if available.
- Show `input`, `cached input`, `output`, `reasoning output`, and `total` when present, but keep the layout simple enough for a terminal.
- Do not add charts, colors, interactive prompts, or rich TUI behavior in Phase 1.
- Add machine-readable JSON only if it is nearly free while implementing DTOs; otherwise defer to v2/export work.

### Time Range Semantics
- Use the local machine timezone for "today", "this week", and "this month".
- Treat weeks as starting on Monday.
- Parse event timestamps as absolute timestamps from the logs, then convert to local time for range grouping.
- If a timestamp is missing or invalid, skip that event with a diagnostic rather than guessing.

### Log Discovery and Overrides
- Default discovery order: `CODEX_HOME` environment variable when set, then the current user's `~/.codex`.
- Only scan whitelisted paths: `sessions`, `archived_sessions`, and optional `session_index.jsonl` metadata if needed.
- Do not recursively scan the whole Codex directory. Do not read `auth.json`, config files, caches, sqlite databases, or prompt/message content for counting.
- Provide a lightweight `--codex-home <path>` CLI option for tests, non-default installs, and debugging.

### Error Handling and Diagnostics
- Prefer partial results with clear warnings over strict failure.
- Bad JSONL lines, missing optional token fields, missing rate-limit metadata, and inaccessible individual files should produce diagnostics and continue.
- Missing Codex home or no session logs should exit cleanly with a useful message and nonzero status.
- Keep diagnostics concise; detailed debug logging is out of scope for Phase 1.

### Test Fixtures
- Use hand-written synthetic JSONL fixtures by default so no private prompts, paths, account data, or credentials enter the repo.
- Fixtures must include multiple `token_count` events in one session to prove cumulative totals are not overcounted.
- Fixtures must include at least one malformed/irrelevant line and one missing-field case to verify diagnostics.
- Do not copy raw local Codex logs into the repository.

### the agent's Discretion
- Keep the implementation intentionally small. Prefer straightforward Rust structs and functions over abstraction layers.
- Use standard crates already identified in research when useful, but avoid adding a dependency unless it removes real code or risk.
- Choose exact command names and table formatting during planning, as long as the success criteria remain satisfied.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- No implementation code exists yet.
- Planning assets exist under `.planning/`, especially `PROJECT.md`, `REQUIREMENTS.md`, `ROADMAP.md`, `research/SUMMARY.md`, and ADR-0001.

### Established Patterns
- This is a greenfield Rust/Tauri project.
- Project decisions favor a shared Rust core with thin platform adapters.
- GSD workflow artifacts are committed and should remain in sync.

### Integration Points
- Phase 1 should create the initial repo structure for later phases: a shared core crate and CLI crate.
- Later phases will reuse the core API for active session refresh and Windows Tauri UI.

</code_context>

<specifics>
## Specific Ideas

- User explicitly prefers a lightweight solution because this should be an easy feature.
- Phase 1 should optimize for correctness and simplicity, not a polished dashboard.
- Local-only scope is non-negotiable.
- `total_token_usage` is cumulative within a session and must not be summed directly across events.

</specifics>

<deferred>
## Deferred Ideas

- Current-session watch mode belongs to Phase 2.
- Windows tray app and stats panel belong to Phase 3.
- Original capybara desktop companion belongs to Phase 4.
- JSON/CSV export, charts, per-workspace grouping, and threshold notifications remain v2.

</deferred>
