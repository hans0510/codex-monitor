---
status: complete
created: 2026-07-10
---

# Publish v0.1.2 Token Lineage Fix

## Objective

Publish the verified subagent token-overcount fix as Windows and Linux v0.1.2 releases.

## Tasks

1. Bump all project package and desktop versions to 0.1.2.
2. Run formatting, tests, Clippy, and Windows/Linux release builds.
3. Package binaries with checksums and verify archive contents.
4. Commit source and release metadata, push master and v0.1.2 tags.
5. Create GitHub releases and verify their public assets.

## Verification

- All workspace tests pass.
- Clippy passes with warnings denied.
- Windows and Linux archives contain the expected executable and README files.
- Both GitHub releases are public and point to the final master commit.
