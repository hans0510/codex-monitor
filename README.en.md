# Codex Token Monitor

[中文](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | English

Codex Token Monitor is a local Codex token usage monitor for the current machine. It reads `token_count` events from local Codex JSONL session logs, summarizes usage for the current OS user, and provides an Ubuntu-friendly CLI plus a Windows desktop companion.

It is not an OpenAI account-wide usage dashboard. It only reports logs available on this machine, so usage from other machines or other OS users on the same account is not included.

## Features

- **Local usage summary**: shows token totals for today, this week, this month, and all time.
- **Token fields**: reports input, cached input, output, reasoning output, and total tokens.
- **No double-counting**: Codex `total_token_usage` values are cumulative within a session; this project calculates per-session deltas instead of summing cumulative events directly.
- **Latest session**: the CLI prints the latest session ID and total tokens.
- **Local privacy boundary**: scans only `.jsonl` files under `sessions` and `archived_sessions`; it does not read `auth.json`.
- **No account API calls**: does not fetch account usage from OpenAI and does not upload local logs.
- **Compact formatting**: large numbers are displayed with `K` and `M`.
- **Windows Capybara Lulu**: opens the desktop companion named 水豚噜噜 (Capybara Lulu); clicking it expands or collapses the stats panel.

## Supported Platforms

| Platform | Supported surface |
|---|---|
| Ubuntu / Linux | CLI summary over local Codex logs |
| Windows | CLI, Tauri desktop companion, stats panel |

The desktop surface is currently intended for Windows. Use the CLI on Ubuntu.

## Data Source

Codex home resolution order:

1. CLI flag `--codex-home <PATH>`
2. Environment variable `CODEX_HOME`
3. Default directory: `~/.codex` on Linux/macOS, `%USERPROFILE%\.codex` on Windows

Scanned directories:

- `<codex-home>/sessions`
- `<codex-home>/archived_sessions`

Not read:

- `<codex-home>/auth.json`
- Credential files
- Non-session log directories

## Installation

### 1. Install Rust

Install the stable Rust toolchain:

```bash
rustup toolchain install stable
rustup default stable
```

Verify the tools:

```bash
rustc --version
cargo --version
```

### 2. Clone the Repository

```bash
git clone https://github.com/hans0510/codex-monitor.git
cd codex-monitor
```

### 3. Install the CLI into Cargo bin

```bash
cargo install --path crates/codex-token-cli
```

Then run:

```bash
codex-token-monitor summary
```

If the command is not found, add Cargo's bin directory to `PATH`:

- Linux/macOS: `$HOME/.cargo/bin`
- Windows: `%USERPROFILE%\.cargo\bin`

### 4. Run from Source without Installing

```bash
cargo run -p codex-token-cli -- summary
```

### 5. Build a Release Binary

```bash
cargo build --release -p codex-token-cli
```

Build output:

- Linux/macOS: `target/release/codex-token-monitor`
- Windows: `target\release\codex-token-monitor.exe`

## CLI Usage

### Read the default Codex home

```bash
codex-token-monitor summary
```

Source-run form:

```bash
cargo run -p codex-token-cli -- summary
```

### Specify a Codex home

```bash
codex-token-monitor summary --codex-home /path/to/.codex
```

PowerShell example:

```powershell
codex-token-monitor summary --codex-home "$env:USERPROFILE\.codex"
```

### Use the committed fixture

```bash
cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home
```

The `summary` subcommand is optional:

```bash
cargo run -p codex-token-cli -- --codex-home fixtures/codex-home
```

### Output

The CLI prints:

- Codex home path
- Number of scanned session files
- Number of detected sessions
- Today, This week, This month, and All time rows
- Input, Cached, Output, Reasoning, and Total token columns
- Latest session ID and total tokens
- Up to 5 parser warnings

Example shape:

```text
Codex Token Summary
Codex home: /home/you/.codex
Session files: 12
Sessions: 8

Range            Input     Cached     Output  Reasoning      Total
Today             1.2K       300          800          0       2.3K
This week         8.4K       1.1K        5.2K        120      14.8K
This month       20.1K       3.6K       12.4K        240      36.3K
All time        120.4K      40.2K       80.1K        1.1K    241.8K

Latest session: session-id (2.3K total)
```

## Windows Desktop Usage

Run the desktop companion:

```powershell
cargo run -p codex-token-desktop
```

Desktop behavior:

- Opens the Capybara Lulu desktop companion window.
- Click the companion to show or hide the stats panel.
- The stats panel refreshes every 2 seconds.
- The panel uses the same Rust local parser as the CLI.
- Drag the companion or panel drag handle to move the window.
- Use the size slider to resize the companion; the value is stored in browser local storage.

Run the desktop with fixture data:

```powershell
$env:CODEX_HOME = "C:\codes\codex-Monitor\fixtures\codex-home"
cargo run -p codex-token-desktop
```

`tauri.conf.json` currently has `bundle.active` set to `false`, so the repository documents local running and binary builds, not a packaged Windows installer yet.

## Development

### Run Tests

```bash
cargo test
```

### Format

```bash
cargo fmt
```

### Common Checks

```bash
cargo check
cargo test
```

## Repository Layout

```text
crates/codex-token-core      Shared scanning, parsing, and aggregation logic
crates/codex-token-cli       Command-line entry point
crates/codex-token-desktop   Tauri Windows desktop entry point
fixtures/codex-home          Synthetic test logs, no real credentials
```

## Accuracy Notes

Codex `total_token_usage` values are cumulative within a session. Directly adding every cumulative event would overcount usage. Codex Token Monitor:

- Groups token events by session ID.
- Sorts events by timestamp and file position.
- Uses deltas between adjacent cumulative values for period summaries.
- Uses the latest cumulative value per session for all-time totals.
- Tolerates missing fields where possible and reports diagnostics as warnings.

## Privacy Notes

This project is intentionally local and minimal:

- It does not read Codex auth files.
- It does not call OpenAI APIs.
- It does not upload logs.
- It does not count usage from other machines on the same OpenAI account.
- The fixture `auth.json` is a synthetic decoy used to verify that credential files are ignored.

## Limitations

- The CLI currently provides a one-shot summary, not a watch subcommand; rerun it to refresh.
- The desktop app auto-refreshes but still depends only on local logs.
- If Codex changes its JSONL event format, the parser will try to remain tolerant and emit warnings, but code updates may be needed.
- Tauri installer bundling is not enabled yet.

## Troubleshooting

### No logs found

Check the Codex home path:

```bash
codex-token-monitor summary --codex-home /path/to/.codex
```

Confirm these directories exist:

```text
sessions/
archived_sessions/
```

### Command not found after install

Check whether Cargo bin is in `PATH`:

```bash
echo $PATH
```

PowerShell:

```powershell
$env:Path
```

### Desktop has no data

Run the CLI with the same `CODEX_HOME` first:

```powershell
$env:CODEX_HOME = "C:\path\to\.codex"
cargo run -p codex-token-cli -- summary
```
