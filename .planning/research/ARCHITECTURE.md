# Architecture Research

**Domain:** Local desktop/CLI token monitor
**Researched:** 2026-07-07
**Confidence:** MEDIUM

## Standard Architecture

### System Overview

```text
            Windows                                Ubuntu
    +-----------------------+              +------------------+
    | Tauri tray process    |              | CLI binary       |
    | - tray menu           |              | - summary output |
    | - stats panel         |              | - watch mode     |
    | - capybara window     |              +---------+--------+
    +-----------+-----------+                        |
                | Rust commands                       |
                v                                     v
        +-----------------------------------------------+
        | shared Rust core                              |
        | - Codex path discovery                        |
        | - JSONL token_count parser                    |
        | - session dedupe and aggregation              |
        | - active session watcher/poller               |
        +---------------------+-------------------------+
                              |
                              v
        +-----------------------------------------------+
        | local Codex data                              |
        | ~/.codex/sessions and archived_sessions       |
        | optional session_index.jsonl                  |
        +-----------------------------------------------+
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| Path discovery | Locate current user's Codex data directory. | `$CODEX_HOME` override, then OS home `.codex`. |
| Log scanner | Enumerate active and archived session JSONL files. | Recursive scan with session-id dedupe. |
| Event parser | Parse `token_count` payloads into typed structs. | Serde structs with optional fields. |
| Usage aggregator | Convert event streams into session totals and date ranges. | Per-session ordered deltas or latest cumulative totals. |
| Active watcher | Refresh newest active session. | File watcher with polling fallback. |
| CLI adapter | Render summaries for Ubuntu. | `clap` commands calling shared core. |
| Windows desktop adapter | Render tray, stats window, and companion. | Tauri v2 Rust backend + web UI. |

## Recommended Project Structure

```text
crates/
  core/                 # parser, discovery, aggregation, tests
  cli/                  # Ubuntu-friendly CLI binary
apps/
  windows-desktop/      # Tauri v2 Windows tray and companion UI
fixtures/
  codex-jsonl/          # sanitized token_count samples
```

### Structure Rationale

- **`crates/core`:** Keeps correctness-sensitive logic independent from any UI framework.
- **`crates/cli`:** Lets Ubuntu use the tool without installing GUI dependencies.
- **`apps/windows-desktop`:** Isolates Tauri frontend and packaging concerns from core statistics.
- **`fixtures`:** Makes parser behavior testable without reading private local logs.

## Architectural Patterns

### Pattern 1: Modular Monolith

**What:** One repo and one shared core, with separate binaries/adapters.
**When to use:** Small personal desktop tool with multiple entry points.
**Trade-offs:** Simple development and testing; less plugin-like extensibility.

### Pattern 2: Ports and Adapters

**What:** Core exposes pure functions/services; CLI and desktop call them.
**When to use:** Same domain logic needs multiple interfaces.
**Trade-offs:** Slight upfront module boundaries; avoids UI-driven parser duplication.

### Pattern 3: Defensive Parsing

**What:** Treat log fields as optional and report unsupported/missing data.
**When to use:** Parsing local app logs without a public stable schema.
**Trade-offs:** More explicit error handling; much better resilience.

## Data Flow

### Summary Flow

```text
User opens CLI or stats panel
  -> adapter requests summary
  -> core scans session JSONL files
  -> parser extracts token_count events
  -> aggregator dedupes sessions and computes totals
  -> adapter renders table/cards
```

### Active Session Flow

```text
Watcher identifies newest active session file
  -> file append or polling tick
  -> parse new token_count events
  -> update active session model
  -> desktop UI or CLI watch output refreshes
```

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| Current local use | Full scan on startup is acceptable; cache in memory. |
| Hundreds of session files | Incremental scan by mtime and session id. |
| Very large logs | Stream JSONL line-by-line; never load all logs into memory. |

### Scaling Priorities

1. **First bottleneck:** Large JSONL files. Fix with streaming parser and mtime cache.
2. **Second bottleneck:** UI refresh frequency. Fix with throttled watcher events.

## Non-Functional Requirements

### Performance
- Startup summary should feel immediate on typical local logs.
- Large logs should be streamed, not fully loaded.

### Security and Privacy
- No network calls for usage data.
- Do not read credentials or tokens.
- Fixtures must be sanitized.

### Reliability
- Missing or changed fields should produce partial results and warnings, not crashes.
- File watcher failures should fall back to polling.

### Maintainability
- Parser tests must lock in counting semantics.
- UI code must not reimplement statistics.

## Anti-Patterns

### Anti-Pattern 1: UI Owns Counting Logic

**What people do:** Implement token aggregation directly in the desktop UI.
**Why it's wrong:** CLI and Windows counts drift.
**Do this instead:** Keep all parsing and aggregation in the shared core.

### Anti-Pattern 2: Treat Log Format as Stable Public API

**What people do:** Assume every event has every field forever.
**Why it's wrong:** Codex local logs can change.
**Do this instead:** Use optional fields and diagnostics.

### Anti-Pattern 3: Platform Forks

**What people do:** Build separate Windows and Ubuntu tools.
**Why it's wrong:** Doubles implementation effort and test burden.
**Do this instead:** One core, two thin adapters.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| None for v1 | Local file read only | This is intentional to keep usage personal and local. |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| CLI -> core | Direct Rust API | Same binary workspace. |
| Tauri backend -> core | Direct Rust API / Tauri commands | Frontend receives already-aggregated DTOs. |
| Core -> local Codex logs | Read-only filesystem access | Avoid credential files. |

## Sources

- Architecture Designer references: modular monolith and NFR checklist.
- Tauri v2 docs via Context7: tray and window APIs.
- Local Codex JSONL samples: session metadata and token_count schema.

---
*Architecture research for: Codex Token Monitor*
*Researched: 2026-07-07*
