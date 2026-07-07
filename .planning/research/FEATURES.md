# Feature Research

**Domain:** Local Codex token usage monitor
**Researched:** 2026-07-07
**Confidence:** MEDIUM

## Feature Landscape

### Table Stakes (Users Expect These)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Local Codex log discovery | The app must work without manual path entry for common installs. | MEDIUM | Default to `$CODEX_HOME`, then `~/.codex`; allow override for tests. |
| Accurate token aggregation | Core value depends on trustworthy counts. | MEDIUM | Do not sum cumulative totals directly. |
| Current active session refresh | User explicitly requested current-session real-time visibility. | MEDIUM | Watch newest active JSONL; poll fallback. |
| Time range summaries | Users need quick usage views, not raw event dumps. | LOW | Today, week, month, all time. |
| CLI output for Ubuntu | Explicit platform requirement. | LOW | Human-readable table plus optional JSON output. |
| Windows tray app | Explicit platform requirement. | MEDIUM | Tray menu: show stats, show/hide companion, refresh, quit. |
| Clickable desktop companion | Explicit Windows UX requirement. | MEDIUM | Original capybara opens stats panel. |

### Differentiators (Competitive Advantage)

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Original capybara companion | Makes usage monitoring visible and friendly. | MEDIUM | Keep it lightweight and non-intrusive. |
| Rate-limit context display | Helps interpret usage beyond raw token counts. | LOW | Show only if present in logs. |
| Parsing diagnostics | Explains partial/incomplete counts when logs are missing fields. | LOW | Useful because Codex log format is not a public stability contract. |
| JSON CLI output | Enables shell scripts and future integrations. | LOW | Add `--json` once summary output exists. |

### Anti-Features (Commonly Requested, Often Problematic)

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Cloud account aggregation | Looks more complete. | Violates local-only scope and may include other users. | Local logs only. |
| Billing/cost estimation | Token counts invite cost conversion. | Codex plans/rate limits are not simple per-token billing; risk of misleading output. | Show tokens and rate-limit percentages only. |
| Always-on-top stats dashboard | Easy to monitor constantly. | Can become distracting and heavy. | Small companion; click opens panel. |
| Real-time parsing of message text | Works even without `token_count`. | Inaccurate and model-tokenizer dependent. | Prefer `token_count`; report unsupported if absent. |

## Feature Dependencies

```text
Log discovery
  -> JSONL parser
      -> session-level usage model
          -> range summaries
          -> active session refresh
              -> Ubuntu CLI
              -> Windows tray/stats panel
                  -> desktop companion
```

### Dependency Notes

- **JSONL parser before every UI:** All visible features depend on the same statistics model.
- **Active session refresh after static summaries:** Static parsing validates correctness before file watching adds complexity.
- **Windows companion after stats panel:** The companion can be a shell around an already working stats view.

## MVP Definition

### Launch With (v1)

- [ ] Local Codex session discovery and JSONL parsing.
- [ ] Correct per-session and date-range token aggregation.
- [ ] Current active session view with automatic refresh.
- [ ] Ubuntu CLI summary.
- [ ] Windows tray app with stats panel.
- [ ] Clickable original capybara companion.

### Add After Validation (v1.x)

- [ ] Export JSON/CSV summaries.
- [ ] User-configurable refresh interval.
- [ ] Tray tooltip showing current session token count.
- [ ] Better companion states such as idle, refreshing, and warning.

### Future Consideration (v2+)

- [ ] Historical charts.
- [ ] Per-project or per-workspace grouping.
- [ ] Notifications for local usage thresholds.
- [ ] Linux GUI variant if the user later wants more than CLI.

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| JSONL parser and aggregation | HIGH | MEDIUM | P1 |
| Active session refresh | HIGH | MEDIUM | P1 |
| Ubuntu CLI | HIGH | LOW | P1 |
| Windows tray stats panel | HIGH | MEDIUM | P1 |
| Desktop capybara companion | HIGH | MEDIUM | P1 |
| Rate-limit display | MEDIUM | LOW | P2 |
| Export formats | MEDIUM | LOW | P2 |
| Charts | LOW | MEDIUM | P3 |

## Sources

- Local Codex session logs under `~/.codex/sessions` - token event schema sample.
- User requirements from `PROJECT.md`.
- Tauri v2 documentation - tray and window support for Windows desktop shell.

---
*Feature research for: Codex Token Monitor*
*Researched: 2026-07-07*
