---
status: resolved
trigger: "当前项目在使用子代理时把父任务继承历史重复计入 Token 总量"
created: 2026-07-10
updated: 2026-07-10
mode: find_and_fix
---

# Subagent Token Overcount

## Symptoms

- expected: A subagent contributes only the token usage generated after it forks from its parent session.
- actual: The inherited parent token history is counted again as usage belonging to the subagent.
- errors: No runtime error is shown; range totals are inflated.
- timeline: Present since aggregation began treating each fallback file name as an independent session.
- reproduction: Create a subagent whose JSONL starts with copied parent `token_count` totals, then aggregate both files.

## Current Focus

- hypothesis: Resolved: lineage-aware aggregation removes inherited cumulative snapshots before calculating child-session deltas.
- test: Covered both full parent-prefix replay and replay beginning from a parent-history suffix.
- expecting: Child summaries contain only post-fork usage while parent continuation remains counted once.
- next_action: Publish the fix when requested.
- reasoning_checkpoint: Real-log output agrees with an independent lineage-aware Node audit while active logs continue to grow.
- tdd_checkpoint: Both regression tests failed before the implementation and pass after it.

## Evidence

- timestamp: 2026-07-10
  observation: The monitor reported about 908M for today, while lineage-aware auditing estimated about 252M.
- timestamp: 2026-07-10
  observation: A child session's first 449 cumulative usage snapshots matched the parent exactly even though their timestamps differed.
- timestamp: 2026-07-10
  observation: The child session source identifies `subagent.thread_spawn.parent_thread_id` in its first `session_meta` event.
- timestamp: 2026-07-10
  observation: Current aggregation falls back to the JSONL file stem as a session ID and treats the first cumulative event as a full delta from zero.

## Eliminated

- hypothesis: Direct cumulative-event summation causes the inflation.
  reason: Aggregation already computes per-session deltas; the defect is the incorrect session baseline for forked child logs.
- hypothesis: Cached input is accidentally added twice to `total_tokens`.
  reason: Raw Codex totals define cached input as a subset of input, and the stored `total_tokens` value is used directly.

## Resolution

- root_cause: Subagent logs inherit cumulative parent usage, sometimes as a full prefix and sometimes as a contiguous suffix, but fallback file names made aggregation treat every child counter as starting from zero.
- fix: Parse the first `session_meta` lineage, map child threads to parents, find the longest child prefix matching any contiguous parent usage segment, and use the last inherited snapshot as the child's delta baseline.
- verification: Two regression tests pass; all 23 workspace tests pass; Clippy passes with warnings denied; Windows desktop and Linux musl release builds pass; live output dropped from about 908M to about 278M and matched an independent audit within ongoing-log growth.
- files_changed: crates/codex-token-core/src/lib.rs, crates/codex-token-core/tests/parser.rs
