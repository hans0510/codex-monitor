# Codex Token Monitor

[中文](README.md) | [한국어](README.ko.md) | 日本語 | [English](README.en.md)

Codex Token Monitor は、このマシン上のローカル Codex token 使用量を確認するためのツールです。ローカル Codex JSONL セッションログの `token_count` イベントを読み取り、現在の OS ユーザーの使用量を集計します。Ubuntu 向けの CLI と Windows デスクトップコンパニオンを提供します。

これは OpenAI アカウント全体の使用量ダッシュボードではありません。このマシンで読めるローカルログだけを集計するため、同じアカウントでも他のマシンや他の OS ユーザーで発生した使用量は含まれません。

## 機能

- **ローカル使用量集計**：今日、今週、今月、全期間の token 使用量を表示します。
- **token フィールド表示**：input、cached input、output、reasoning output、total token を表示します。
- **二重計上の回避**：Codex の `total_token_usage` はセッション内の累積値なので、累積イベントをそのまま合算せず、セッションごとの差分を計算します。
- **最新セッション表示**：CLI は最新セッション ID と合計 token を出力します。
- **ローカルのプライバシー境界**：`sessions` と `archived_sessions` 配下の `.jsonl` ファイルだけをスキャンし、`auth.json` は読みません。
- **アカウント API 呼び出しなし**：OpenAI アカウント使用量を取得せず、ローカルログもアップロードしません。
- **短い数値表記**：大きな数値は `K`、`M` で表示します。
- **Windows Capybara Lulu**：水豚噜噜 (Capybara Lulu) デスクトップコンパニオンを表示し、クリックで統計パネルを展開または折りたたみます。

## 対応プラットフォーム

| プラットフォーム | 対応内容 |
|---|---|
| Ubuntu / Linux | ローカル Codex ログの CLI 集計 |
| Windows | CLI、Tauri デスクトップコンパニオン、統計パネル |

デスクトップ画面は現在 Windows 向けです。Ubuntu では CLI の利用を優先してください。

## データソース

Codex home の解決順序：

1. CLI オプション `--codex-home <PATH>`
2. 環境変数 `CODEX_HOME`
3. 既定ディレクトリ：Linux/macOS は `~/.codex`、Windows は `%USERPROFILE%\.codex`

スキャンするディレクトリ：

- `<codex-home>/sessions`
- `<codex-home>/archived_sessions`

読み取らないもの：

- `<codex-home>/auth.json`
- 認証情報ファイル
- セッションログ以外のディレクトリ

## インストール

### 1. Rust をインストール

Rust stable toolchain をインストールします。

```bash
rustup toolchain install stable
rustup default stable
```

ツールを確認します。

```bash
rustc --version
cargo --version
```

### 2. リポジトリを clone

```bash
git clone https://github.com/hans0510/codex-monitor.git
cd codex-monitor
```

### 3. CLI を Cargo bin にインストール

```bash
cargo install --path crates/codex-token-cli
```

インストール後：

```bash
codex-token-monitor summary
```

コマンドが見つからない場合は Cargo の bin ディレクトリを `PATH` に追加してください。

- Linux/macOS：`$HOME/.cargo/bin`
- Windows：`%USERPROFILE%\.cargo\bin`

### 4. インストールせずにソースから実行

```bash
cargo run -p codex-token-cli -- summary
```

### 5. release バイナリをビルド

```bash
cargo build --release -p codex-token-cli
```

ビルド結果：

- Linux/macOS：`target/release/codex-token-monitor`
- Windows：`target\release\codex-token-monitor.exe`

## CLI の使い方

### 既定の Codex home を読む

```bash
codex-token-monitor summary
```

ソースから実行する場合：

```bash
cargo run -p codex-token-cli -- summary
```

### Codex home を指定

```bash
codex-token-monitor summary --codex-home /path/to/.codex
```

PowerShell 例：

```powershell
codex-token-monitor summary --codex-home "$env:USERPROFILE\.codex"
```

### リポジトリ内の fixture を使う

```bash
cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home
```

`summary` サブコマンドは省略できます。

```bash
cargo run -p codex-token-cli -- --codex-home fixtures/codex-home
```

### 出力内容

CLI は以下を出力します。

- Codex home パス
- スキャンした session ファイル数
- 検出した session 数
- Today、This week、This month、All time の各行
- Input、Cached、Output、Reasoning、Total token の各列
- 最新 session ID と合計 token
- 最大 5 件の parser warning

出力例：

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

## Windows デスクトップの使い方

デスクトップコンパニオンを起動：

```powershell
cargo run -p codex-token-desktop
```

デスクトップの動作：

- Capybara Lulu デスクトップコンパニオンウィンドウを開きます。
- コンパニオンをクリックすると統計パネルの表示と非表示を切り替えます。
- 統計パネルは 2 秒ごとに更新されます。
- CLI と同じ Rust のローカル parser を使います。
- コンパニオンまたはパネルのドラッグ領域をドラッグしてウィンドウを移動できます。
- サイズスライダーでコンパニオンの大きさを調整でき、値は browser local storage に保存されます。

fixture データで起動：

```powershell
$env:CODEX_HOME = "C:\codes\codex-Monitor\fixtures\codex-home"
cargo run -p codex-token-desktop
```

現在 `tauri.conf.json` の `bundle.active` は `false` です。そのため、このリポジトリは Windows installer ではなく、ローカル実行とバイナリビルド手順を説明しています。

## 開発

### テスト実行

```bash
cargo test
```

### フォーマット

```bash
cargo fmt
```

### よく使うチェック

```bash
cargo check
cargo test
```

## リポジトリ構成

```text
crates/codex-token-core      共通の scan、parse、aggregate ロジック
crates/codex-token-cli       CLI エントリポイント
crates/codex-token-desktop   Tauri Windows デスクトップエントリポイント
fixtures/codex-home          合成テストログ。実際の認証情報は含まない
```

## 精度について

Codex の `total_token_usage` はセッション内の累積値です。すべての累積イベントを直接足すと使用量が過大になります。Codex Token Monitor は以下の方法で計算します。

- token event を session ID ごとにグループ化します。
- timestamp とファイル位置でイベントを並べます。
- 期間別集計では隣接する累積値の差分を使います。
- all-time 合計では各 session の最新累積値を使います。
- 可能な範囲で欠損フィールドを許容し、診断情報を warning として出力します。

## プライバシーについて

このプロジェクトはローカルかつ最小権限で動作することを目的にしています。

- Codex 認証ファイルを読みません。
- OpenAI API を呼びません。
- ログをアップロードしません。
- 同じ OpenAI アカウントの他マシンの使用量は集計しません。
- fixture の `auth.json` は、認証情報ファイルを無視することを検証するための合成 decoy です。

## 制限

- CLI は現在 watch サブコマンドではなく、一回限りの summary 出力です。更新するには再実行してください。
- デスクトップアプリは自動更新しますが、ローカルログだけに依存します。
- Codex JSONL イベント形式が変わった場合、parser は可能な限り warning を出して動作しますが、コード更新が必要になることがあります。
- Tauri installer bundle はまだ有効化されていません。

## トラブルシューティング

### ログが見つからない

Codex home パスを確認してください。

```bash
codex-token-monitor summary --codex-home /path/to/.codex
```

以下のディレクトリがあるか確認してください。

```text
sessions/
archived_sessions/
```

### インストール後にコマンドが見つからない

Cargo bin が `PATH` に入っているか確認してください。

```bash
echo $PATH
```

PowerShell：

```powershell
$env:Path
```

### デスクトップにデータが出ない

先に同じ `CODEX_HOME` で CLI を実行してください。

```powershell
$env:CODEX_HOME = "C:\path\to\.codex"
cargo run -p codex-token-cli -- summary
```
