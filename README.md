# Codex Token Monitor

Local-only Codex token usage summary for the current machine.

## CLI

The CLI binary is `codex-token-monitor`.

Run against the default Codex home (`CODEX_HOME`, then `~/.codex`):

```bash
cargo run -p codex-token-cli -- summary
```

Run against the committed synthetic fixture:

```bash
cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home
```

You can also omit the subcommand; it defaults to `summary`:

```bash
cargo run -p codex-token-cli -- --codex-home fixtures/codex-home
```

The CLI reads only local `sessions` and `archived_sessions` JSONL files. It does not read `auth.json` or make network requests for usage data.

Token counts are displayed with compact units (`K`, `M`) when values are large.

## Windows Desktop

Run the desktop companion:

```powershell
cargo run -p codex-token-desktop
```

The Windows app opens a small original capybara-inspired desktop companion. Click it to show or hide the stats panel. The panel refreshes local Codex token totals every 2 seconds and uses the same local-only parser as the CLI.

For testing against a fixture:

```powershell
$env:CODEX_HOME = "C:\codes\codex-Monitor\fixtures\codex-home"
cargo run -p codex-token-desktop
```
