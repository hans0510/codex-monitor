# Codex Token Monitor

中文 | [한국어](README.ko.md) | [日本語](README.ja.md) | [English](README.en.md)

Codex Token Monitor 是一个本机 Codex token 用量统计工具。它从当前机器上的 Codex JSONL 会话日志中读取 `token_count` 事件，汇总当前 OS 用户的本地用量，并提供 Ubuntu 友好的 CLI 和 Windows 桌面伴侣界面。

它不是 OpenAI 账号级用量面板。它只统计这台机器上能读取到的本地 Codex 日志，因此同一账号在其他机器或其他用户下产生的用量不会出现在这里。

## 功能

- **本机用量汇总**：统计今天、本周、本月和全部时间的 token 用量。
- **多字段展示**：展示 input、cached input、output、reasoning output 和 total token。
- **避免重复计数**：Codex 的 `total_token_usage` 是会话内累计值，本项目按会话计算增量，避免直接累加造成高估。
- **最新会话提示**：CLI 输出最近一次会话 ID 和该会话总 token。
- **本地隐私边界**：只扫描 `sessions` 和 `archived_sessions` 下的 `.jsonl` 文件，不读取 `auth.json`。
- **无账号 API 请求**：不会联网拉取 OpenAI 账号用量，也不会上传本地日志。
- **紧凑数字格式**：大数自动显示为 `K`、`M`。
- **Windows 桌面伴侣**：桌面端显示一个原创水豚风格伴侣，点击可展开或收起统计面板，面板自动刷新。

## 支持平台

| 平台 | 支持内容 |
|---|---|
| Ubuntu / Linux | CLI 汇总本地 Codex 日志 |
| Windows | CLI、Tauri 桌面伴侣、统计面板 |

当前桌面端面向 Windows 使用。Ubuntu 上优先使用 CLI。

## 数据来源

默认 Codex home 的解析顺序：

1. CLI 参数 `--codex-home <PATH>`
2. 环境变量 `CODEX_HOME`
3. 默认目录：Linux/macOS 为 `~/.codex`，Windows 为 `%USERPROFILE%\.codex`

扫描目录：

- `<codex-home>/sessions`
- `<codex-home>/archived_sessions`

不会读取：

- `<codex-home>/auth.json`
- 其他凭据文件
- 非 Codex 会话日志目录

## 安装

### 1. 安装 Rust

先安装 Rust stable 工具链：

```bash
rustup toolchain install stable
rustup default stable
```

确认工具可用：

```bash
rustc --version
cargo --version
```

### 2. 克隆仓库

```bash
git clone https://github.com/hans0510/codex-monitor.git
cd codex-monitor
```

### 3. 安装 CLI 到 Cargo bin

```bash
cargo install --path crates/codex-token-cli
```

安装后可直接运行：

```bash
codex-token-monitor summary
```

如果 Cargo bin 不在 PATH，请把 Cargo 的 bin 目录加入 PATH：

- Linux/macOS：`$HOME/.cargo/bin`
- Windows：`%USERPROFILE%\.cargo\bin`

### 4. 不安装，直接从源码运行

```bash
cargo run -p codex-token-cli -- summary
```

### 5. 构建 release 二进制

```bash
cargo build --release -p codex-token-cli
```

构建结果：

- Linux/macOS：`target/release/codex-token-monitor`
- Windows：`target\release\codex-token-monitor.exe`

## CLI 使用

### 读取默认 Codex home

```bash
codex-token-monitor summary
```

源码运行形式：

```bash
cargo run -p codex-token-cli -- summary
```

### 指定 Codex home

```bash
codex-token-monitor summary --codex-home /path/to/.codex
```

PowerShell 示例：

```powershell
codex-token-monitor summary --codex-home "$env:USERPROFILE\.codex"
```

### 使用仓库内测试 fixture

```bash
cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home
```

也可以省略 `summary` 子命令，默认执行汇总：

```bash
cargo run -p codex-token-cli -- --codex-home fixtures/codex-home
```

### 输出内容

CLI 会输出：

- Codex home 路径
- 扫描到的 session 文件数量
- 识别出的 session 数量
- Today、This week、This month、All time 四个时间范围
- Input、Cached、Output、Reasoning、Total 五类 token
- 最新 session 的 ID 和总 token
- 最多 5 条解析警告

示例结构：

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

## Windows 桌面端使用

启动桌面伴侣：

```powershell
cargo run -p codex-token-desktop
```

桌面端行为：

- 打开一个原创水豚风格伴侣窗口。
- 点击伴侣可显示或隐藏统计面板。
- 统计面板每 2 秒刷新一次。
- 面板使用与 CLI 相同的 Rust 本地解析核心。
- 可通过拖动伴侣或面板拖动区域移动窗口。
- 可通过大小滑块调整伴侣尺寸，设置会保存在浏览器本地存储中。

使用 fixture 启动桌面端：

```powershell
$env:CODEX_HOME = "C:\codes\codex-Monitor\fixtures\codex-home"
cargo run -p codex-token-desktop
```

当前 `tauri.conf.json` 中 `bundle.active` 为 `false`，因此仓库默认提供本地运行和二进制构建流程，还没有启用 Windows 安装包打包。

## 开发

### 运行测试

```bash
cargo test
```

### 格式化

```bash
cargo fmt
```

### 常用检查

```bash
cargo check
cargo test
```

## 项目结构

```text
crates/codex-token-core      共享解析、扫描、聚合逻辑
crates/codex-token-cli       命令行入口
crates/codex-token-desktop   Tauri Windows 桌面入口
fixtures/codex-home          合成测试日志，不包含真实凭据
```

## 准确性说明

Codex 日志中的 `total_token_usage` 是会话内累计值。如果把每一条累计事件直接相加，会严重高估用量。Codex Token Monitor 的策略是：

- 按 session ID 分组。
- 按时间和文件位置排序 token 事件。
- 对时间范围统计使用相邻累计值的差值。
- 对 all-time 统计使用每个 session 的最新累计值。
- 字段缺失时尽量容错，并把可诊断问题放到 warnings 中。

## 隐私说明

本项目的目标是本机、最小权限统计：

- 不读取 Codex 认证文件。
- 不请求 OpenAI API。
- 不上传日志。
- 不统计同一 OpenAI 账号在其他机器上的用量。
- fixture 中的 `auth.json` 是合成诱饵文件，用于测试扫描逻辑不会读取凭据。

## 限制

- CLI 当前是一次性 summary 输出，没有 watch 子命令；需要刷新时请重新运行命令。
- 桌面端会自动刷新，但仍只基于本地日志。
- 如果 Codex JSONL 日志结构变化，解析器会尽量容错并输出 warning，但可能需要更新代码适配新格式。
- 当前仓库没有启用 Tauri installer bundle。

## 故障排查

### 没有发现日志

确认 Codex home 是否正确：

```bash
codex-token-monitor summary --codex-home /path/to/.codex
```

确认目录下存在：

```text
sessions/
archived_sessions/
```

### 安装后找不到命令

确认 Cargo bin 在 PATH 中：

```bash
echo $PATH
```

Windows PowerShell：

```powershell
$env:Path
```

### 桌面端没有数据

先用同一个 `CODEX_HOME` 跑 CLI，确认本地日志可解析：

```powershell
$env:CODEX_HOME = "C:\path\to\.codex"
cargo run -p codex-token-cli -- summary
```
