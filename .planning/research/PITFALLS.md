# Pitfalls Research

**Domain:** Local Codex token usage monitor
**Researched:** 2026-07-07
**Confidence:** MEDIUM

## Critical Pitfalls

### Pitfall 1: Overcounting Cumulative Token Events

**What goes wrong:**
The app sums every `total_token_usage.total_tokens` value across a session, producing inflated totals.

**Why it happens:**
The field name looks like an event total, but local samples show it is cumulative within the session.

**How to avoid:**
For a session total, use the latest cumulative value. For date-range increments, sort events and sum positive deltas or use `last_token_usage` with duplicate protection.

**Warning signs:**
All-time totals are much larger than the latest session totals imply, or totals grow faster than Codex rate-limit percentages.

**Phase to address:**
Phase 1.

---

### Pitfall 2: Reading Sensitive Codex Files

**What goes wrong:**
The app scans all of `~/.codex`, accidentally reading `auth.json` or other sensitive files.

**Why it happens:**
Recursive directory scanning is easy and seems convenient.

**How to avoid:**
Whitelist `sessions`, `archived_sessions`, and optional `session_index.jsonl`. Never parse credential files.

**Warning signs:**
Parser tests or logs mention access tokens, refresh tokens, auth files, or unrelated config files.

**Phase to address:**
Phase 1.

---

### Pitfall 3: Mistaking Account Usage for Local Usage

**What goes wrong:**
The tool mixes cloud account/org usage with local machine usage, counting other people on the same account.

**Why it happens:**
Usage dashboards often start from API or billing-style concepts.

**How to avoid:**
Use local session logs as the source of truth. Label the app as "local machine usage".

**Warning signs:**
Implementation requires OpenAI credentials, network calls, or organization-level usage APIs.

**Phase to address:**
Phase 1.

---

### Pitfall 4: Desktop Companion Blocks Normal Work

**What goes wrong:**
The capybara window steals focus, covers useful UI, appears in Alt-Tab/taskbar, or cannot be dismissed.

**Why it happens:**
Always-on-top desktop windows are easy to make annoying.

**How to avoid:**
Make the companion small, draggable or positionable, hideable from the tray, and keep the stats panel separate from the mascot.

**Warning signs:**
Manual testing shows focus stealing, taskbar clutter, or no obvious hide/quit path.

**Phase to address:**
Phase 3.

---

### Pitfall 5: Assuming File Watchers Are Reliable Everywhere

**What goes wrong:**
The active session view stops updating because file watcher events are missed or coalesced.

**Why it happens:**
Filesystem watch behavior varies by OS and editor/app write pattern.

**How to avoid:**
Use watcher events as an optimization and add polling fallback with throttled refresh.

**Warning signs:**
CLI watch or desktop panel only updates after restart.

**Phase to address:**
Phase 2.

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Parse JSON with regex | Very fast prototype | Breaks on nested payloads and optional fields | Never for core parser. |
| Duplicate parser in JS UI | Faster UI demo | CLI and desktop counts diverge | Never. |
| Hard-code Windows path | Quick local demo | Ubuntu breaks and tests are brittle | Only in throwaway spike, not v1. |
| Ignore archived sessions | Simpler scan | All-time totals incomplete | Acceptable only if clearly labeled, but not recommended. |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Codex logs | Count every cumulative event | Use latest cumulative total or deltas. |
| Tauri tray | Hide all windows without quit path | Provide tray menu with show/hide/quit. |
| Tauri window | Put mascot and stats in one window | Separate companion from stats panel. |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Loading all JSONL into memory | Slow startup and high memory | Stream line-by-line | Large local histories. |
| Refreshing UI on every file event | Flicker and CPU churn | Debounce/throttle refresh | Active long Codex sessions. |
| Full rescan on every refresh | Watch mode gets slow | Cache session mtimes and parse increments | Hundreds of logs. |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Reading `auth.json` | Credential exposure | Whitelist session paths only. |
| Sending usage data to a server | Privacy violation | No network usage in v1. |
| Storing raw prompts in app cache | Sensitive content retention | Store only derived token statistics if caching is added. |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Cute mascot but weak stats | Fails core value | Build stats core before mascot polish. |
| Too much detail in CLI | Hard to scan | Default summary plus optional detail flags. |
| No explanation for partial data | User distrusts counts | Show warnings when fields/files are skipped. |

## "Looks Done But Isn't" Checklist

- [ ] **Parser:** Verify cumulative token events are not overcounted.
- [ ] **Local-only scope:** Verify no cloud/account usage is requested.
- [ ] **Active refresh:** Verify current session updates while a log file grows.
- [ ] **Windows tray:** Verify show, hide, refresh, and quit.
- [ ] **Companion:** Verify it is clickable, hideable, and original art.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Overcounting | MEDIUM | Replace aggregation with session deltas; update fixtures and tests. |
| Sensitive file access | HIGH | Remove broad scans, audit logs, add whitelist tests. |
| Watcher unreliability | LOW | Add polling fallback and refresh status. |
| Annoying desktop companion | MEDIUM | Add tray toggle and revise window flags. |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Overcounting cumulative events | Phase 1 | Fixture tests with multiple token_count events per session. |
| Sensitive file access | Phase 1 | Tests assert scanner only includes session paths. |
| Watcher unreliability | Phase 2 | Simulated file append updates active stats. |
| Desktop companion blocks work | Phase 3 | Manual Windows smoke test for focus, hide, quit. |

## Sources

- Local Codex JSONL samples from the current machine.
- Tauri v2 tray/window docs via Context7.
- Architecture NFR checklist for privacy, reliability, and maintainability prompts.

---
*Pitfalls research for: Codex Token Monitor*
*Researched: 2026-07-07*
