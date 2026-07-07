# Requirements: Codex Token Monitor

**Defined:** 2026-07-07
**Core Value:** Show accurate local Codex token usage for this machine, including both cumulative totals and the currently active session.

## v1 Requirements

### Local Data Source

- [ ] **DATA-01**: User can run the tool without configuring a path when Codex data exists in the current user's default Codex directory.
- [ ] **DATA-02**: Tool reads only local Codex session log locations and does not read credential files such as `auth.json`.
- [ ] **DATA-03**: Tool parses Codex JSONL `token_count` events into structured token usage records.
- [ ] **DATA-04**: Tool includes active and archived local sessions in all-time statistics while deduplicating sessions by session id.
- [ ] **DATA-05**: Tool reports clear diagnostics when expected token fields are missing or partially unsupported.

### Token Statistics

- [ ] **STAT-01**: User can see accurate per-session totals without overcounting cumulative `total_token_usage` events.
- [ ] **STAT-02**: User can see token totals for today, this week, this month, and all time.
- [ ] **STAT-03**: User can see input, cached input, output, reasoning output, and total token counts when those fields exist.
- [ ] **STAT-04**: User can see current active Codex session usage update automatically as the session log grows.
- [ ] **STAT-05**: User can see rate-limit percentage context when Codex logs provide it.

### Ubuntu CLI

- [ ] **CLI-01**: Ubuntu user can run a command that prints local Codex token summaries in a readable terminal format.
- [ ] **CLI-02**: Ubuntu user can run a watch mode that refreshes the current active session statistics.
- [ ] **CLI-03**: CLI exits with a useful message when no Codex session logs are found.

### Windows Desktop

- [ ] **WIN-01**: Windows user can run a resident tray app for Codex token monitoring.
- [ ] **WIN-02**: Windows user can open a stats panel from the tray and view cumulative and active-session token statistics.
- [ ] **WIN-03**: Windows tray menu includes refresh, show stats, show or hide companion, and quit actions.
- [ ] **WIN-04**: Windows user can click the desktop capybara companion to open the stats panel.
- [ ] **WIN-05**: Windows user can hide or close the companion without stopping the tray app.

### Visual and Safety Boundaries

- [ ] **SAFE-01**: Windows companion uses an original capybara-inspired design and does not directly copy a protected character.
- [ ] **SAFE-02**: Windows companion does not permanently steal focus or block normal desktop work during normal use.
- [ ] **SAFE-03**: Tool performs no network requests for usage data in v1.

## v2 Requirements

### Export and History

- **EXP-01**: User can export token summaries as JSON.
- **EXP-02**: User can export token summaries as CSV.
- **EXP-03**: User can view historical charts.
- **EXP-04**: User can group usage by project or workspace.

### Notifications

- **NOTF-01**: User can configure local usage threshold notifications.
- **NOTF-02**: Windows tray tooltip can show current session token count.
- **NOTF-03**: Companion can show additional states such as idle, refreshing, and warning.

### Platform Expansion

- **PLAT-01**: Ubuntu user can optionally run a GUI version if requested later.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Cloud account or organization usage aggregation | The requested product is local personal usage only and must not include other account users. |
| OpenAI billing or cost estimation | Token counts and Codex plan/rate-limit behavior do not map cleanly to a simple cost estimate. |
| Mobile apps | The requested platforms are Ubuntu and Windows. |
| Shared server dashboard | Unnecessary for a personal desktop utility. |
| Direct replica of "Lulu" character art | Use an original capybara-inspired mascot to avoid copying a protected design. |
| Parsing prompts to estimate tokens | Less accurate than logged `token_count` events and tokenizer dependent. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| DATA-01 | TBD | Pending |
| DATA-02 | TBD | Pending |
| DATA-03 | TBD | Pending |
| DATA-04 | TBD | Pending |
| DATA-05 | TBD | Pending |
| STAT-01 | TBD | Pending |
| STAT-02 | TBD | Pending |
| STAT-03 | TBD | Pending |
| STAT-04 | TBD | Pending |
| STAT-05 | TBD | Pending |
| CLI-01 | TBD | Pending |
| CLI-02 | TBD | Pending |
| CLI-03 | TBD | Pending |
| WIN-01 | TBD | Pending |
| WIN-02 | TBD | Pending |
| WIN-03 | TBD | Pending |
| WIN-04 | TBD | Pending |
| WIN-05 | TBD | Pending |
| SAFE-01 | TBD | Pending |
| SAFE-02 | TBD | Pending |
| SAFE-03 | TBD | Pending |

**Coverage:**
- v1 requirements: 21 total
- Mapped to phases: 0
- Unmapped: 21

---
*Requirements defined: 2026-07-07*
*Last updated: 2026-07-07 after initial definition*
