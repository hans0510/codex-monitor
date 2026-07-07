# Roadmap: Codex Token Monitor

## Overview

Build the product vertically from the trustworthy counting core outward. Phase 1 proves local-only parsing and Ubuntu CLI summaries. Phase 2 adds active-session refresh. Phase 3 wraps the proven core in a Windows tray app and stats panel. Phase 4 adds the original clickable capybara companion and desktop polish.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

- [ ] **Phase 1: Core Parser and CLI** - Accurate local token summaries from Codex JSONL logs.
- [ ] **Phase 2: Active Session Refresh** - Current Codex session updates automatically.
- [ ] **Phase 3: Windows Tray and Stats Panel** - Resident tray app shows the same statistics on Windows.
- [ ] **Phase 4: Capybara Companion and Polish** - Original clickable desktop companion opens stats without getting in the way.

## Phase Details

### Phase 1: Core Parser and CLI
**Goal:** User can run an Ubuntu-friendly CLI that accurately reports local Codex token summaries from session logs.
**Mode:** mvp
**Depends on:** Nothing (first phase)
**Requirements:** DATA-01, DATA-02, DATA-03, DATA-04, DATA-05, STAT-01, STAT-02, STAT-03, CLI-01, CLI-03, SAFE-03
**Success Criteria** (what must be TRUE):
  1. User can run a CLI command and see today, this week, this month, and all-time token totals from local Codex logs.
  2. Parser tests prove cumulative `total_token_usage` events are not overcounted.
  3. Scanner reads only session log locations and excludes credential files.
  4. CLI exits with a useful message when no session logs are found.
**Plans:** 3 plans

Plans:
- [ ] 01-01: Scaffold Rust workspace, sanitized fixtures, and session log discovery.
- [ ] 01-02: Implement JSONL token parser, dedupe, aggregation, and diagnostics.
- [ ] 01-03: Implement Ubuntu CLI summary output and no-log handling.

### Phase 2: Active Session Refresh
**Goal:** User can monitor the current active Codex session as token usage changes.
**Mode:** mvp
**Depends on:** Phase 1
**Requirements:** STAT-04, STAT-05, CLI-02
**Success Criteria** (what must be TRUE):
  1. User can run CLI watch mode and see the active session update as the JSONL log grows.
  2. Active session detection picks the most recent active local session without counting archived duplicates.
  3. Refresh still works through polling if filesystem watcher events are missed.
  4. Rate-limit percentages are displayed when present and omitted cleanly when absent.
**Plans:** 2 plans

Plans:
- [ ] 02-01: Implement latest-session detection, watcher, polling fallback, and refresh throttling.
- [ ] 02-02: Add CLI watch mode and active-session/rate-limit output.

### Phase 3: Windows Tray and Stats Panel
**Goal:** Windows user can run a resident tray app and open a stats panel showing cumulative and active-session usage.
**Mode:** mvp
**UI hint:** yes
**Depends on:** Phase 2
**Requirements:** WIN-01, WIN-02, WIN-03
**Success Criteria** (what must be TRUE):
  1. Windows user can start the tray app and keep it resident.
  2. Tray menu can open the stats panel, refresh statistics, toggle the companion placeholder, and quit.
  3. Stats panel shows the same cumulative and active-session values exposed by the shared core.
  4. Manual Windows smoke test verifies tray actions work without requiring Ubuntu GUI support.
**Plans:** 3 plans

Plans:
- [ ] 03-01: Scaffold Tauri v2 Windows desktop app wired to the shared Rust core.
- [ ] 03-02: Implement tray menu actions and stats panel data binding.
- [ ] 03-03: Add Windows smoke checks for tray lifecycle and stats rendering.

### Phase 4: Capybara Companion and Polish
**Goal:** Windows user can click an original capybara desktop companion to open stats, and can hide it when it is not wanted.
**Mode:** mvp
**UI hint:** yes
**Depends on:** Phase 3
**Requirements:** WIN-04, WIN-05, SAFE-01, SAFE-02
**Success Criteria** (what must be TRUE):
  1. Windows desktop shows an original capybara-inspired companion, not a direct copy of a protected character.
  2. User can click the companion to open or focus the stats panel.
  3. User can hide and show the companion from the tray without stopping the tray app.
  4. Companion does not permanently steal focus or block normal desktop work during manual testing.
**Plans:** 2 plans

Plans:
- [ ] 04-01: Design and implement original capybara companion window and click behavior.
- [ ] 04-02: Polish companion positioning, hide/show behavior, focus handling, and final smoke tests.

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Core Parser and CLI | 0/3 | Not started | - |
| 2. Active Session Refresh | 0/2 | Not started | - |
| 3. Windows Tray and Stats Panel | 0/3 | Not started | - |
| 4. Capybara Companion and Polish | 0/2 | Not started | - |
