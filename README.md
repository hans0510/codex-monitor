# Codex Token Monitor

[中文（默认）](#中文默认) | [한국어](#한국어) | [日本語](#日本語) | [English](#english)

## 中文（默认）

Codex Token Monitor 是一个本机 Codex token 用量统计工具，提供命令行统计和 Windows 桌面伴侣界面。它只读取当前机器、当前用户本地的 Codex 会话日志，不读取 `auth.json`，也不会通过网络请求账号用量数据。

### 功能

- 汇总今天、本周、本月和全部时间的 token 用量。
- 展示 input、cached input、output、reasoning output 和 total token。
- 优先读取 Codex JSONL 日志里的 `token_count` 事件，避免把累计值重复相加。
- 只扫描 `sessions` 和 `archived_sessions` 目录，忽略凭据文件。
- token 数较大时使用 `K`、`M` 等紧凑单位。

### 命令行

命令行二进制名称是 `codex-token-monitor`。默认读取 `CODEX_HOME`，未设置时读取 `~/.codex`。

```bash
cargo run -p codex-token-cli -- summary
```

使用仓库内的合成 fixture 测试：

```bash
cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home
```

也可以省略子命令，默认执行 `summary`：

```bash
cargo run -p codex-token-cli -- --codex-home fixtures/codex-home
```

### Windows 桌面端

启动桌面伴侣：

```powershell
cargo run -p codex-token-desktop
```

桌面端会打开一个原创水豚风格的小型伴侣窗口。点击伴侣可以显示或隐藏统计面板，面板使用与命令行相同的本地解析核心。

使用 fixture 测试：

```powershell
$env:CODEX_HOME = "C:\codes\codex-Monitor\fixtures\codex-home"
cargo run -p codex-token-desktop
```

### 测试

```bash
cargo test
```

## 한국어

Codex Token Monitor는 현재 컴퓨터의 로컬 Codex 토큰 사용량을 확인하는 도구입니다. 명령줄 요약과 Windows 데스크톱 컴패니언을 제공합니다. 현재 OS 사용자에게 있는 로컬 Codex 세션 로그만 읽으며, `auth.json`을 읽지 않고 계정 사용량을 조회하기 위한 네트워크 요청도 보내지 않습니다.

### 기능

- 오늘, 이번 주, 이번 달, 전체 기간의 토큰 사용량을 요약합니다.
- input, cached input, output, reasoning output, total token을 표시합니다.
- Codex JSONL 로그의 `token_count` 이벤트를 우선 사용하여 누적 사용량 중복 합산을 피합니다.
- `sessions` 및 `archived_sessions` 디렉터리만 스캔하고 자격 증명 파일은 무시합니다.
- 큰 토큰 수는 `K`, `M` 단위로 간단히 표시합니다.

### CLI

CLI 바이너리 이름은 `codex-token-monitor`입니다. 기본값은 `CODEX_HOME`을 사용하고, 없으면 `~/.codex`를 사용합니다.

```bash
cargo run -p codex-token-cli -- summary
```

저장소에 포함된 합성 fixture로 실행:

```bash
cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home
```

하위 명령을 생략하면 기본적으로 `summary`가 실행됩니다.

```bash
cargo run -p codex-token-cli -- --codex-home fixtures/codex-home
```

### Windows 데스크톱

데스크톱 컴패니언 실행:

```powershell
cargo run -p codex-token-desktop
```

Windows 앱은 독자적인 카피바라 스타일의 작은 컴패니언 창을 엽니다. 컴패니언을 클릭하면 통계 패널을 표시하거나 숨길 수 있으며, 패널은 CLI와 동일한 로컬 전용 파서를 사용합니다.

fixture로 테스트:

```powershell
$env:CODEX_HOME = "C:\codes\codex-Monitor\fixtures\codex-home"
cargo run -p codex-token-desktop
```

### 테스트

```bash
cargo test
```

## 日本語

Codex Token Monitor は、このマシン上のローカル Codex token 使用量を確認するためのツールです。CLI の集計表示と Windows デスクトップコンパニオンを提供します。現在の OS ユーザーのローカル Codex セッションログだけを読み取り、`auth.json` は読み取らず、アカウント使用量を取得するためのネットワークリクエストも行いません。

### 機能

- 今日、今週、今月、全期間の token 使用量を集計します。
- input、cached input、output、reasoning output、total token を表示します。
- Codex JSONL ログの `token_count` イベントを優先し、累積値の二重加算を避けます。
- `sessions` と `archived_sessions` ディレクトリだけをスキャンし、認証情報ファイルは無視します。
- 大きな token 数は `K`、`M` などの短い単位で表示します。

### CLI

CLI バイナリ名は `codex-token-monitor` です。デフォルトでは `CODEX_HOME` を使用し、未設定の場合は `~/.codex` を使用します。

```bash
cargo run -p codex-token-cli -- summary
```

リポジトリ内の合成 fixture で実行:

```bash
cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home
```

サブコマンドを省略すると、既定で `summary` が実行されます。

```bash
cargo run -p codex-token-cli -- --codex-home fixtures/codex-home
```

### Windows デスクトップ

デスクトップコンパニオンを起動:

```powershell
cargo run -p codex-token-desktop
```

Windows アプリは、オリジナルのカピバラ風コンパニオンウィンドウを開きます。コンパニオンをクリックすると統計パネルの表示と非表示を切り替えられ、パネルは CLI と同じローカル専用パーサーを使用します。

fixture でテスト:

```powershell
$env:CODEX_HOME = "C:\codes\codex-Monitor\fixtures\codex-home"
cargo run -p codex-token-desktop
```

### テスト

```bash
cargo test
```

## English

Codex Token Monitor is a local Codex token usage utility for the current machine. It provides a command-line summary and a Windows desktop companion. It reads only local Codex session logs for the current OS user, does not read `auth.json`, and does not make network requests for account usage data.

### Features

- Summarizes token usage for today, this week, this month, and all time.
- Shows input, cached input, output, reasoning output, and total tokens.
- Prefers `token_count` events in Codex JSONL logs to avoid double-counting cumulative usage.
- Scans only `sessions` and `archived_sessions`, ignoring credential files.
- Uses compact units such as `K` and `M` for large token counts.

### CLI

The CLI binary is `codex-token-monitor`. It reads `CODEX_HOME` by default, then falls back to `~/.codex`.

```bash
cargo run -p codex-token-cli -- summary
```

Run against the committed synthetic fixture:

```bash
cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home
```

You can omit the subcommand; it defaults to `summary`:

```bash
cargo run -p codex-token-cli -- --codex-home fixtures/codex-home
```

### Windows Desktop

Run the desktop companion:

```powershell
cargo run -p codex-token-desktop
```

The Windows app opens a small original capybara-inspired companion window. Click it to show or hide the stats panel. The panel uses the same local-only parser as the CLI.

For testing against a fixture:

```powershell
$env:CODEX_HOME = "C:\codes\codex-Monitor\fixtures\codex-home"
cargo run -p codex-token-desktop
```

### Tests

```bash
cargo test
```
