# ADR-0001: Use Local Codex Logs with a Shared Rust Core and Tauri Windows Shell

## Status
Accepted

## Context

The tool must count only this machine's Codex usage for the current OS user. It must support Ubuntu CLI output and a Windows tray app with a clickable desktop capybara companion. Local Codex JSONL session logs contain `token_count` events with token usage data, while cloud/account-level usage could include other users on the same account.

## Decision

Use local Codex session JSONL logs as the v1 source of truth. Implement parsing and aggregation in a shared Rust core. Expose that core through a Rust CLI for Ubuntu and a Tauri v2 Windows desktop shell for tray, stats panel, and companion UI.

## Consequences

### Positive
- Counts stay local to the current machine and user.
- Parser and aggregation logic are shared across Ubuntu and Windows.
- Rust tests can lock in correctness for cumulative token event handling.
- Tauri provides a small Windows desktop shell with tray and window APIs.

### Negative
- Codex local log format may change, so the parser must be defensive.
- Tauri companion window behavior still needs Windows validation.
- Rust plus Tauri adds more setup than a quick script.

### Neutral
- Ubuntu v1 remains CLI-only by design.
- No network usage data source is used in v1.

## Alternatives Considered

**Electron + Node.js**
- Rejected for v1 because it gives a heavier desktop shell and would not naturally share a compiled CLI core.

**Python + PySide**
- Rejected for v1 because packaging and resident desktop behavior are less clean for this small cross-platform utility.

**Cloud/account usage API**
- Rejected because it can include other people on the account and violates the local-only scope.

## References

- `.planning/PROJECT.md`
- `.planning/research/SUMMARY.md`
- Tauri v2 system tray and window documentation via Context7
