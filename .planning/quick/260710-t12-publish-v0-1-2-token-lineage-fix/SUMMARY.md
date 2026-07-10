---
status: complete
completed: 2026-07-10
commit: f928b64
---

# Publish v0.1.2 Token Lineage Fix Summary

Published the lineage-aware token aggregation fix as separate Windows and Linux v0.1.2 releases.

## Delivered

- Excluded inherited parent usage from full-prefix and partial-history subagent logs.
- Added two regression tests for observed Codex log variants.
- Bumped the shared core, CLI, desktop app, and Tauri package to 0.1.2.
- Published one executable archive per platform without redundant checksum assets.

## Verification

- All 23 workspace tests passed.
- Formatting, JavaScript syntax, diff checks, and Clippy with denied warnings passed.
- Windows desktop and Linux musl release builds passed.
- Public GitHub release assets and tag commit f928b64 were verified.
