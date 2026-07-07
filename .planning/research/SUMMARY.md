# Project Research Summary

**Project:** Codex Token Monitor
**Domain:** Local Codex token usage monitor
**Researched:** 2026-07-07
**Confidence:** MEDIUM

## Executive Summary

This project is best built as a local-first desktop/CLI utility, not an account analytics dashboard. The reliable source for the requested scope is the current OS user's local Codex session logs, specifically JSONL `token_count` events under `~/.codex/sessions` and archived session files. This keeps usage personal to the machine and avoids counting other people on the same account.

The recommended implementation is one shared Rust statistics core with two thin entry points: an Ubuntu CLI and a Windows Tauri v2 desktop shell. Tauri is a fit for the Windows tray and companion requirements, while Rust keeps the parser and aggregation logic testable and reusable.

The main technical risk is incorrect counting. Local samples show `total_token_usage` is cumulative within a session, so summing those events directly will overcount. The roadmap should put parser fixtures and aggregation tests first, then layer active refresh and Windows UI on top.

## Key Findings

### Recommended Stack

Use Rust for parser/statistics/CLI, Tauri v2 for Windows tray and desktop UI, and a small TypeScript/CSS frontend for the stats panel and original capybara companion.

**Core technologies:**
- Rust: shared parser and aggregation core.
- Tauri v2: Windows tray, stats panel, and companion window.
- Serde: structured JSONL parsing.
- clap: Ubuntu CLI interface.
- notify plus polling fallback: active session refresh.

### Expected Features

**Must have (table stakes):**
- Local Codex log discovery.
- Correct token aggregation without cumulative overcounting.
- Today/week/month/all-time summaries.
- Current active session refresh.
- Ubuntu CLI.
- Windows tray stats panel.
- Clickable original capybara companion.

**Should have (competitive):**
- Rate-limit context display when available.
- Parsing diagnostics for missing/changed fields.
- JSON output for CLI automation.

**Defer (v2+):**
- Historical charts.
- Per-workspace grouping.
- Threshold notifications.
- Linux GUI variant.

### Architecture Approach

Use a modular monolith: `core` owns path discovery, session scanning, event parsing, dedupe, aggregation, and active watching; `cli` renders terminal output; `windows-desktop` wraps the core with Tauri tray/menu/window UI. The UI must receive already-aggregated DTOs and must not duplicate counting logic.

**Major components:**
1. Path discovery - current user's Codex data path.
2. Session scanner - active and archived JSONL enumeration with dedupe.
3. Token parser - defensive `token_count` extraction.
4. Aggregator - session totals, date ranges, active session metrics.
5. CLI adapter - Ubuntu output.
6. Windows adapter - Tauri tray, stats panel, companion.

### Critical Pitfalls

1. **Overcounting cumulative token events** - use latest cumulative values or deltas, never sum every `total_token_usage`.
2. **Reading sensitive Codex files** - whitelist session paths and avoid credentials.
3. **Mixing account and local usage** - do not call cloud usage APIs in v1.
4. **Unreliable file watchers** - add polling fallback.
5. **Annoying companion window** - make it hideable, small, and separate from stats panel.

## Implications for Roadmap

### Phase 1: Core Parser and CLI
**Rationale:** Counting correctness is the core value and must be proven before UI.
**Delivers:** Shared Rust core, fixtures, aggregation tests, Ubuntu CLI summary.
**Addresses:** Local-only statistics, cumulative summaries.
**Avoids:** Overcounting and sensitive file access.

### Phase 2: Active Session Refresh
**Rationale:** Real-time current-session view depends on a correct static parser.
**Delivers:** Latest-session detection, file watcher/polling fallback, CLI watch mode, refresh DTO.
**Uses:** Core watcher and aggregation model.
**Implements:** Active session flow.

### Phase 3: Windows Tray and Stats Panel
**Rationale:** Desktop shell should sit on top of proven core APIs.
**Delivers:** Tauri tray process, stats window, menu actions, manual Windows smoke path.
**Uses:** Tauri v2 tray and window APIs.

### Phase 4: Capybara Companion and Polish
**Rationale:** Mascot is important for Windows UX but should not precede accurate stats.
**Delivers:** Original clickable desktop capybara, hide/show controls, focused UX polish.
**Avoids:** Focus stealing and character-copy risk.

### Phase Ordering Rationale

- Parser correctness is the dependency for every other feature.
- Active refresh should extend the parser rather than introduce separate logic.
- Windows UI can be built after the core exposes stable summary APIs.
- The companion is last because it is mostly UX and should not mask weak statistics.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2:** File watcher behavior on Windows and Ubuntu.
- **Phase 3:** Exact Tauri v2 window flags for skip-taskbar, focus, transparency, and packaging.
- **Phase 4:** Best implementation for a non-intrusive desktop companion window.

Phases with standard patterns:
- **Phase 1:** Rust JSONL parsing and CLI testing are straightforward.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | MEDIUM | Tauri tray/window basics verified; exact companion behavior needs Windows implementation testing. |
| Features | HIGH | Directly derived from user requirements and local log feasibility. |
| Architecture | HIGH | One shared core with two adapters is the simplest reliable shape. |
| Pitfalls | MEDIUM | Counting pitfalls verified locally; UI pitfalls require manual Windows validation. |

**Overall confidence:** MEDIUM

### Gaps to Address

- **Codex log format stability:** Handle with defensive parser and fixtures from sanitized samples.
- **Tauri companion details:** Validate in Phase 3/4 with actual Windows smoke tests.
- **Archived session dedupe:** Implement session-id dedupe and tests before all-time totals are trusted.

## Sources

### Primary (HIGH confidence)
- `/websites/v2_tauri_app` - Tauri v2 system tray and window API documentation via Context7.
- Local Codex JSONL samples - verified `session_meta` and `token_count` payload shapes.

### Secondary (MEDIUM confidence)
- Architecture Designer references - modular monolith, ADR format, and NFR checklist.

---
*Research completed: 2026-07-07*
*Ready for roadmap: yes*
