use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const APP_NAME: &str = "codex-token-monitor";

const SESSION_DIRS: [&str; 2] = ["sessions", "archived_sessions"];

#[derive(Debug)]
pub enum ScanError {
    ReadDir { path: PathBuf, source: io::Error },
    Entry { path: PathBuf, source: io::Error },
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadDir { path, source } => {
                write!(f, "failed to read directory {}: {source}", path.display())
            }
            Self::Entry { path, source } => {
                write!(f, "failed to read entry under {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for ScanError {}

pub fn discover_codex_home(override_path: Option<&Path>) -> Option<PathBuf> {
    if let Some(path) = override_path {
        return Some(path.to_path_buf());
    }

    if let Some(path) = env::var_os("CODEX_HOME").filter(|value| !value.is_empty()) {
        return Some(PathBuf::from(path));
    }

    home_dir().map(|home| home.join(".codex"))
}

pub fn discover_session_files(codex_home: &Path) -> Result<Vec<PathBuf>, ScanError> {
    let mut files = Vec::new();

    for dir_name in SESSION_DIRS {
        let dir = codex_home.join(dir_name);
        if dir.is_dir() {
            collect_jsonl_files(&dir, &mut files)?;
        }
    }

    files.sort();
    Ok(files)
}

fn collect_jsonl_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), ScanError> {
    let entries = fs::read_dir(dir).map_err(|source| ScanError::ReadDir {
        path: dir.to_path_buf(),
        source,
    })?;

    for entry in entries {
        let entry = entry.map_err(|source| ScanError::Entry {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|source| ScanError::Entry {
            path: path.clone(),
            source,
        })?;

        if file_type.is_dir() {
            collect_jsonl_files(&path, files)?;
        } else if file_type.is_file() && path.extension().is_some_and(|ext| ext == "jsonl") {
            files.push(path);
        }
    }

    Ok(())
}

#[cfg(windows)]
fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE").map(PathBuf::from)
}

#[cfg(not(windows))]
fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_home() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/codex-home")
            .canonicalize()
            .expect("fixture home exists")
    }

    #[test]
    fn discovery_finds_active_and_archived_session_logs() {
        let files = discover_session_files(&fixture_home()).expect("session files");
        let rendered = files
            .iter()
            .map(|path| path.to_string_lossy())
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(files.len(), 2);
        assert!(rendered.contains("session-a.jsonl"));
        assert!(rendered.contains("session-old.jsonl"));
    }

    #[test]
    fn discovery_ignores_auth_json() {
        let files = discover_session_files(&fixture_home()).expect("session files");

        assert!(files.iter().all(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name != "auth.json")
        }));
    }

    #[test]
    fn explicit_home_override_wins() {
        let override_path = Path::new("custom-codex-home");

        assert_eq!(
            discover_codex_home(Some(override_path)),
            Some(PathBuf::from("custom-codex-home"))
        );
    }
}
