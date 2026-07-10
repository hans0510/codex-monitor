# Quick Task 260710-cac: Optimize Windows Lulu quota panel

## Scope

Lower Lulu's minimum width from 95px to 47.5px, remove the duplicate All tile from the range row, and replace the latest-session block with Codex's locally logged five-hour and weekly quota remaining plus each reset time. Keep CLI output unchanged.

## Tasks

1. Parse and aggregate the latest local Codex rate-limit snapshot in the shared core.
   - Verify: parser tests cover primary and secondary windows, remaining percentages, reset timestamps, and latest-snapshot selection.
2. Expose quota data in the Windows desktop payload and update the panel UI.
   - Verify: the range row renders Today, Week, and Month only; two quota blocks render remaining percentage and reset time; missing quota data has a clear fallback.
3. Lower the Lulu size limit and rebuild the Windows executable.
   - Verify: both slider and runtime clamp use 47.5px; Rust tests, formatting, JavaScript syntax check, and release build pass.
