# Quick Task 260708-fdg Summary

## Result

Updated all four README files to name the Windows desktop companion as Water Capybara Lulu / Capybara Lulu instead of describing it as an unnamed original capybara-style companion.

## Changed Files

- `README.md`: uses `水豚噜噜` in the feature list and Windows desktop usage section.
- `README.en.md`: uses `水豚噜噜 (Capybara Lulu)` on first mention and `Capybara Lulu` in the usage section.
- `README.ko.md`: uses `水豚噜噜 (Capybara Lulu)` on first mention and `Capybara Lulu` in the usage section.
- `README.ja.md`: uses `水豚噜噜 (Capybara Lulu)` on first mention and `Capybara Lulu` in the usage section.

## Verification

- `rg` confirmed the old generic wording no longer appears in the four README files.
- `rg` confirmed `水豚噜噜` / `Capybara Lulu` appears in the expected locations.
- `git diff --check` passed for the README files and quick plan.
- Pre-existing untracked `dist/` release artifacts were left untouched.

## Commits

- `31bfbb3` - `docs: name the Lulu capybara companion`
