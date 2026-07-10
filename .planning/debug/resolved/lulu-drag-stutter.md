---
status: resolved
trigger: "噜噜拖动起来一卡一卡的，帮我排查一下原因"
created: 2026-07-10
updated: 2026-07-10
mode: find_and_fix
---

# Lulu Drag Stutter

## Symptoms

- expected: The desktop companion window follows the pointer continuously while dragging.
- actual: The window pauses and then jumps during a drag, producing visible stutter.
- errors: No error message is reported.
- timeline: Unknown; inspect the drag implementation history.
- reproduction: Hold and move either the Lulu body or the panel title drag handle.

## Current Focus

- hypothesis: Resolved: native window dragging removes the per-frame IPC queue, while background aggregation and refresh guards prevent scan latency from interrupting pointer movement.
- test: Built both release targets and ran formatting, JavaScript syntax, Clippy, and workspace tests.
- expecting: Dragging is delegated to the Windows window manager and usage refresh does not overlap or start during an active drag.
- next_action: Publish the verified v0.1.1 binaries.
- reasoning_checkpoint: Root cause and fix verified through code inspection, local timing, compilation, linting, and tests.
- tdd_checkpoint: Existing workspace tests cover usage aggregation; interaction behavior is covered by the Tauri native drag API and capability declaration.

## Evidence

- timestamp: 2026-07-10
  observation: `scheduleWindowMove` clears `moveFrame` before awaiting `set_window_position`, allowing multiple native move requests to overlap.
- timestamp: 2026-07-10
  observation: `refreshUsage` invokes a full local log aggregation every 2000ms while dragging remains enabled.
- timestamp: 2026-07-10
  observation: The local Codex home contains 343 JSONL files totaling 458.2MB; ten release-mode scans averaged 1135.4ms (1014.5-1735.3ms).
- timestamp: 2026-07-10
  observation: Synchronous Tauri commands use the blocking command wrapper by default; both `get_usage` and `set_window_position` are synchronous commands.
- timestamp: 2026-07-10
  observation: Each manual position update posts a `WindowMessage::SetPosition`; Tauri native dragging posts one `WindowMessage::DragWindow` and delegates continuous movement to the OS.
- timestamp: 2026-07-10
  observation: Commit 8c82cb7 replaced the earlier native `startDragging()` implementation with the current per-frame manual movement loop.

## Eliminated

- hypothesis: The Lulu CSS float animation is the primary cause.
  reason: It only transforms the image inside the fixed window at a slow 3.6-second cycle and does not drive native window coordinates.
- hypothesis: The new quota panel caused the drag algorithm to regress.
  reason: The manual drag loop predates the quota change; the quota work only increased data extracted during the already expensive full scan.

## Resolution

- root_cause: Manual per-frame IPC window movement has no in-flight serialization, so native move requests queue and bunch together. A 1.14-second full log rescan every two seconds periodically blocks or competes with the same command path, turning the queueing into visible pauses and jumps.
- fix: Replaced manual per-frame position IPC with Tauri native `startDragging()`, granted the required window capability, moved aggregation to `spawn_blocking`, and suppressed overlapping or active-drag refreshes.
- verification: `cargo fmt --all -- --check`, `node --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (21 passed), Windows release build, and Linux musl release build all passed.
- files_changed: crates/codex-token-desktop/ui/app.js, crates/codex-token-desktop/src/main.rs, crates/codex-token-desktop/capabilities/default.json
