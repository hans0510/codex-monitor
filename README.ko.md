# Codex Token Monitor

[中文](README.md) | 한국어 | [日本語](README.ja.md) | [English](README.en.md)

Codex Token Monitor는 현재 컴퓨터의 로컬 Codex 토큰 사용량을 확인하는 도구입니다. 로컬 Codex JSONL 세션 로그의 `token_count` 이벤트를 읽어 현재 OS 사용자의 사용량을 요약하며, Ubuntu 친화적인 CLI와 Windows 데스크톱 컴패니언을 제공합니다.

이 도구는 OpenAI 계정 전체 사용량 대시보드가 아닙니다. 이 컴퓨터에서 접근 가능한 로컬 로그만 집계하므로, 같은 계정이라도 다른 컴퓨터나 다른 OS 사용자에서 발생한 사용량은 포함되지 않습니다.

## 기능

- **로컬 사용량 요약**: 오늘, 이번 주, 이번 달, 전체 기간의 토큰 사용량을 표시합니다.
- **토큰 필드 표시**: input, cached input, output, reasoning output, total token을 표시합니다.
- **중복 계산 방지**: Codex의 `total_token_usage` 값은 세션 내 누적값이므로, 누적 이벤트를 그대로 더하지 않고 세션별 증가분을 계산합니다.
- **최신 세션 표시**: CLI가 가장 최근 세션 ID와 총 토큰 수를 출력합니다.
- **로컬 개인정보 경계**: `sessions`와 `archived_sessions` 아래의 `.jsonl` 파일만 스캔하며 `auth.json`은 읽지 않습니다.
- **계정 API 호출 없음**: OpenAI 계정 사용량을 조회하지 않고 로컬 로그를 업로드하지 않습니다.
- **간단한 숫자 형식**: 큰 숫자는 `K`, `M` 단위로 표시합니다.
- **Windows 데스크톱 컴패니언**: 독자적인 카피바라 스타일 컴패니언을 표시하고, 클릭하면 통계 패널을 펼치거나 접습니다.

## 지원 플랫폼

| 플랫폼 | 지원 내용 |
|---|---|
| Ubuntu / Linux | 로컬 Codex 로그 기반 CLI 요약 |
| Windows | CLI, Tauri 데스크톱 컴패니언, 통계 패널 |

현재 데스크톱 화면은 Windows 사용을 기준으로 합니다. Ubuntu에서는 CLI 사용을 권장합니다.

## 데이터 소스

Codex home 결정 순서:

1. CLI 옵션 `--codex-home <PATH>`
2. 환경 변수 `CODEX_HOME`
3. 기본 디렉터리: Linux/macOS는 `~/.codex`, Windows는 `%USERPROFILE%\.codex`

스캔하는 디렉터리:

- `<codex-home>/sessions`
- `<codex-home>/archived_sessions`

읽지 않는 항목:

- `<codex-home>/auth.json`
- 자격 증명 파일
- 세션 로그가 아닌 디렉터리

## 설치

### 1. Rust 설치

Rust stable 툴체인을 설치합니다.

```bash
rustup toolchain install stable
rustup default stable
```

도구를 확인합니다.

```bash
rustc --version
cargo --version
```

### 2. 저장소 클론

```bash
git clone https://github.com/hans0510/codex-monitor.git
cd codex-monitor
```

### 3. CLI를 Cargo bin에 설치

```bash
cargo install --path crates/codex-token-cli
```

설치 후 실행:

```bash
codex-token-monitor summary
```

명령을 찾을 수 없다면 Cargo bin 디렉터리를 `PATH`에 추가하세요.

- Linux/macOS: `$HOME/.cargo/bin`
- Windows: `%USERPROFILE%\.cargo\bin`

### 4. 설치 없이 소스에서 실행

```bash
cargo run -p codex-token-cli -- summary
```

### 5. Release 바이너리 빌드

```bash
cargo build --release -p codex-token-cli
```

빌드 결과:

- Linux/macOS: `target/release/codex-token-monitor`
- Windows: `target\release\codex-token-monitor.exe`

## CLI 사용법

### 기본 Codex home 읽기

```bash
codex-token-monitor summary
```

소스 실행 방식:

```bash
cargo run -p codex-token-cli -- summary
```

### Codex home 지정

```bash
codex-token-monitor summary --codex-home /path/to/.codex
```

PowerShell 예시:

```powershell
codex-token-monitor summary --codex-home "$env:USERPROFILE\.codex"
```

### 저장소의 fixture 사용

```bash
cargo run -p codex-token-cli -- summary --codex-home fixtures/codex-home
```

`summary` 하위 명령은 생략할 수 있습니다.

```bash
cargo run -p codex-token-cli -- --codex-home fixtures/codex-home
```

### 출력 내용

CLI 출력 항목:

- Codex home 경로
- 스캔한 session 파일 수
- 감지한 session 수
- Today, This week, This month, All time 행
- Input, Cached, Output, Reasoning, Total token 열
- 최신 session ID와 총 토큰 수
- 최대 5개의 파서 경고

출력 형태 예시:

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

## Windows 데스크톱 사용법

데스크톱 컴패니언 실행:

```powershell
cargo run -p codex-token-desktop
```

데스크톱 동작:

- 독자적인 카피바라 스타일 컴패니언 창을 엽니다.
- 컴패니언을 클릭하면 통계 패널을 표시하거나 숨깁니다.
- 통계 패널은 2초마다 새로고침됩니다.
- CLI와 동일한 Rust 로컬 파서를 사용합니다.
- 컴패니언이나 패널의 드래그 영역을 끌어 창을 이동할 수 있습니다.
- 크기 슬라이더로 컴패니언 크기를 조절할 수 있으며 값은 브라우저 local storage에 저장됩니다.

fixture 데이터로 실행:

```powershell
$env:CODEX_HOME = "C:\codes\codex-Monitor\fixtures\codex-home"
cargo run -p codex-token-desktop
```

현재 `tauri.conf.json`의 `bundle.active`는 `false`입니다. 따라서 이 저장소는 아직 Windows 설치 패키지가 아니라 로컬 실행과 바이너리 빌드 흐름을 문서화합니다.

## 개발

### 테스트 실행

```bash
cargo test
```

### 포맷

```bash
cargo fmt
```

### 일반 점검

```bash
cargo check
cargo test
```

## 저장소 구조

```text
crates/codex-token-core      공통 스캔, 파싱, 집계 로직
crates/codex-token-cli       명령줄 진입점
crates/codex-token-desktop   Tauri Windows 데스크톱 진입점
fixtures/codex-home          합성 테스트 로그, 실제 자격 증명 없음
```

## 정확성 설명

Codex의 `total_token_usage` 값은 세션 내 누적값입니다. 모든 누적 이벤트를 그대로 더하면 사용량이 과대 계산됩니다. Codex Token Monitor는 다음 방식으로 계산합니다.

- 토큰 이벤트를 session ID별로 그룹화합니다.
- timestamp와 파일 위치 기준으로 이벤트를 정렬합니다.
- 기간별 요약에는 인접한 누적값의 차이를 사용합니다.
- all-time 합계에는 각 session의 최신 누적값을 사용합니다.
- 가능한 경우 필드 누락을 허용하고 진단 정보를 warning으로 출력합니다.

## 개인정보 설명

이 프로젝트는 로컬과 최소 권한을 기준으로 설계되었습니다.

- Codex 인증 파일을 읽지 않습니다.
- OpenAI API를 호출하지 않습니다.
- 로그를 업로드하지 않습니다.
- 같은 OpenAI 계정의 다른 컴퓨터 사용량은 집계하지 않습니다.
- fixture의 `auth.json`은 자격 증명 파일을 무시하는지 검증하기 위한 합성 미끼 파일입니다.

## 제한 사항

- CLI는 현재 watch 하위 명령이 아니라 일회성 summary 출력입니다. 새로고침하려면 다시 실행하세요.
- 데스크톱 앱은 자동 새로고침되지만 여전히 로컬 로그만 사용합니다.
- Codex JSONL 이벤트 형식이 바뀌면 파서는 가능한 한 경고를 출력하며 동작하지만 코드 수정이 필요할 수 있습니다.
- Tauri 설치 패키지 번들은 아직 활성화되어 있지 않습니다.

## 문제 해결

### 로그를 찾을 수 없음

Codex home 경로를 확인하세요.

```bash
codex-token-monitor summary --codex-home /path/to/.codex
```

아래 디렉터리가 있는지 확인하세요.

```text
sessions/
archived_sessions/
```

### 설치 후 명령을 찾을 수 없음

Cargo bin이 `PATH`에 있는지 확인하세요.

```bash
echo $PATH
```

PowerShell:

```powershell
$env:Path
```

### 데스크톱에 데이터가 없음

먼저 같은 `CODEX_HOME`으로 CLI를 실행해 보세요.

```powershell
$env:CODEX_HOME = "C:\path\to\.codex"
cargo run -p codex-token-cli -- summary
```
