<!-- GSD:project-start source:PROJECT.md -->
## Project

**Codex Token Monitor**

Codex Token Monitor is a desktop and command-line utility for tracking Codex token usage on the current machine. It reads local Codex session logs for the current OS user only, so usage from other people on the same OpenAI account is not included. Ubuntu gets a CLI-first view, while Windows gets a tray app plus a clickable desktop capybara companion that opens the stats panel.

**Core Value:** Show accurate local Codex token usage for this machine, including both cumulative totals and the currently active session.

### Constraints

- **Privacy**: Read local Codex logs only and avoid collecting unrelated account or credential data.
- **Compatibility**: Support Windows and Ubuntu. Use one shared parsing core when practical, with platform-specific entry points only where needed.
- **Accuracy**: Prefer `token_count` events in Codex JSONL logs over estimating tokens from message text.
- **Resilience**: Codex log formats may change, so parsing should tolerate missing fields and report partial data clearly.
- **Simplicity**: Start with useful stats and automatic refresh before adding advanced charts or historical analytics.
- **Visual design**: The Windows companion must be an original capybara-inspired mascot, not a direct copy of "Lulu".
<!-- GSD:project-end -->

<!-- GSD:stack-start source:research/STACK.md -->
## Technology Stack

## Recommended Stack
### Core Technologies
| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Rust | stable toolchain | Shared parser, statistics engine, CLI, Tauri backend | One compiled core can serve Ubuntu CLI and Windows desktop without duplicating parsing logic. |
| Tauri | v2 | Windows tray app and desktop companion shell | Tauri v2 has documented system tray APIs and Rust backend integration, fitting a small resident desktop utility. |
| HTML/CSS/TypeScript | current browser baseline | Windows stats panel and companion window UI | Tauri uses web UI while keeping native capabilities in Rust. |
| Serde / serde_json | current crates.io | JSONL event parsing | Codex logs are JSONL; structured parsing is safer than text matching. |
| clap | current crates.io | Ubuntu CLI arguments | Standard Rust CLI parser with clear subcommands and flags. |
### Supporting Libraries
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| notify | current crates.io | File watch for active session refresh | Watch the latest session JSONL file; fall back to polling if native events are unreliable. |
| chrono or time | current crates.io | Date range grouping | Needed for today/week/month/all-time rollups. |
| tauri tray APIs | Tauri v2 | Tray icon, menu, click events | Windows resident tray process. |
| tauri window APIs | Tauri v2 | Borderless companion and stats windows | Desktop capybara companion and stats panel. |
### Development Tools
| Tool | Purpose | Notes |
|------|---------|-------|
| cargo test | Parser and statistics tests | Use fixture JSONL logs to prevent overcounting regressions. |
| cargo fmt / clippy | Rust style and linting | Keep code simple and idiomatic. |
| npm / pnpm | Tauri frontend tooling | Only needed for the Windows desktop shell. |
| PowerShell and bash smoke scripts | Platform checks | Validate Windows paths and Ubuntu `~/.codex` paths. |
## Installation
# Rust CLI/core
# Windows desktop shell, once scaffolded
## Alternatives Considered
| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| Rust + Tauri | Electron + Node.js | Use Electron if the team prioritizes JS-only implementation over binary size and Rust sharing. |
| Rust + Tauri | Python + PySide/PyInstaller | Use Python if rapid scripting matters more than clean packaging and resident memory footprint. |
| Local JSONL parser | OpenAI account/API usage endpoint | Use an API only if local logs stop containing token counts; it would not satisfy local-only personal tracking as cleanly. |
## What NOT to Use
| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Summing every `total_token_usage` event | The field is cumulative within a session, so direct summing overcounts. | Use per-session deltas or latest cumulative values. |
| Reading `auth.json` or credentials | The app does not need tokens and should not touch sensitive account files. | Read only session JSONL logs and optional session indexes. |
| Building separate parsers for Windows and Ubuntu | Duplicate logic creates inconsistent counts. | Shared Rust parser with platform-specific entry points. |
| Direct clone of a protected "Lulu" character | Creates avoidable IP risk. | Original capybara-inspired mascot. |
## Stack Patterns by Variant
- Use the Rust CLI only.
- Because the user only asked for command-line statistics on Ubuntu.
- Use Tauri v2 with Rust commands calling the shared statistics core.
- Because tray events, click-to-open windows, and a small resident desktop UI are required.
## Version Compatibility
| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| Tauri v2 | Rust stable and web frontend | Tauri v2 docs use `TrayIconBuilder`, `WebviewWindowBuilder`, and current window APIs. |
| Rust core crate | CLI and Tauri backend | Keep parser independent from Tauri so Ubuntu does not need GUI dependencies. |
## Sources
- `/websites/v2_tauri_app` - Context7 Tauri v2 docs for system tray and window APIs.
- Tauri v2 system tray docs - verified `TrayIconBuilder`, menu events, and tray click handling.
- Tauri v2 window docs - verified borderless window configuration and `setAlwaysOnTop()`.
- Local Codex session JSONL sample - verified `token_count` payload shape and cumulative usage fields.
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

Conventions not yet established. Will populate as patterns emerge during development.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

Architecture not yet mapped. Follow existing patterns found in the codebase.
<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->
## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, `.github/skills/`, or `.codex/skills/` with a `SKILL.md` index file.
<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
