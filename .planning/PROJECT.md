# Codex Token Monitor

## What This Is

Codex Token Monitor is a desktop and command-line utility for tracking Codex token usage on the current machine. It reads local Codex session logs for the current OS user only, so usage from other people on the same OpenAI account is not included. Ubuntu gets a CLI-first view, while Windows gets a tray app plus a clickable desktop capybara companion that opens the stats panel.

## Core Value

Show accurate local Codex token usage for this machine, including both cumulative totals and the currently active session.

## Requirements

### Validated

- [x] Phase 1: Shared Rust core parses local Codex JSONL `token_count` events from the current user's allowlisted session directories.
- [x] Phase 1: CLI prints today, this week, this month, and all-time token summaries with input, cached input, output, reasoning output, and total columns.
- [x] Phase 1: Local-only safety boundary is enforced: no network usage source, no credential-file scanning, synthetic fixtures only.
- [x] Phase 1: CLI exits nonzero with searched paths when no local Codex session logs are found.

### Active

- [ ] Show the currently active Codex session with automatic refresh as new token events arrive.
- [ ] Provide a Windows version with a tray icon, a desktop capybara companion, and a click-to-open stats panel.
- [ ] Keep the Windows capybara visual original and cute without copying a protected character design.

### Out of Scope

- Cloud account usage aggregation - the requested scope is local machine usage only.
- Other users on the same OpenAI account - local logs are the source of truth for personal usage.
- Mobile apps - the requested platforms are Ubuntu and Windows.
- A shared server dashboard - unnecessary for a personal desktop tool.
- Exact replication of "Lulu" or any protected character art - use an original capybara-inspired design instead.

## Context

The current machine has Codex session logs under the user's `.codex` directory. Recent session files include JSONL `token_count` events whose payload contains `total_token_usage`, `last_token_usage`, `model_context_window`, and rate-limit metadata. This makes a local parser feasible without calling OpenAI account APIs.

The user wants a personal monitor, not an account-wide billing or organization report. The implementation should therefore default to the current OS user's Codex data path and treat local logs as the only usage source.

Windows should feel like a small desktop companion: a tray process stays resident, and a cute original capybara on the desktop can be clicked to open token statistics. Ubuntu does not need a graphical app for v1; clear CLI output is enough.

## Constraints

- **Privacy**: Read local Codex logs only and avoid collecting unrelated account or credential data.
- **Compatibility**: Support Windows and Ubuntu. Use one shared parsing core when practical, with platform-specific entry points only where needed.
- **Accuracy**: Prefer `token_count` events in Codex JSONL logs over estimating tokens from message text.
- **Resilience**: Codex log formats may change, so parsing should tolerate missing fields and report partial data clearly.
- **Simplicity**: Start with useful stats and automatic refresh before adding advanced charts or historical analytics.
- **Visual design**: The Windows companion must be an original capybara-inspired mascot, not a direct copy of "Lulu".

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use local Codex logs as the usage source | This satisfies personal-only tracking and avoids account-wide data | Accepted |
| Build both cumulative and active-session views | The user explicitly selected both statistics scopes | Accepted |
| Make Ubuntu CLI-first | The user only needs command-line stats on Ubuntu | Accepted |
| Make Windows tray + desktop companion | The user selected tray plus clickable capybara desktop pet | Accepted |
| Use original capybara art | Avoids copying a protected character while preserving the intended feel | Accepted |
| Use shared Rust core with Tauri Windows shell | Keeps parser logic shared while supporting Windows tray/window UI | Accepted |
| Use Vertical MVP phase structure | Each phase should deliver a usable slice instead of isolated technical layers | Accepted |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? Move to Out of Scope with reason
2. Requirements validated? Move to Validated with phase reference
3. New requirements emerged? Add to Active
4. Decisions to log? Add to Key Decisions
5. "What This Is" still accurate? Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check - still the right priority?
3. Audit Out of Scope - reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-07-07 after Phase 1 verification*
