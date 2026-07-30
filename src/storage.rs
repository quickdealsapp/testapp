use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::note::Note;

const APP_DIR: &str = "testapp";
const NOTES_FILE: &str = "notes.json";

/// Path of the JSON file holding every note, e.g. `~/.local/share/testapp/notes.json`.
pub fn notes_path() -> PathBuf {
    let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join(APP_DIR).join(NOTES_FILE)
}

/// Load every note from disk. A missing file yields an empty list.
pub fn load() -> Result<Vec<Note>, StorageError> {
    load_from(&notes_path())
}

/// Persist every note to disk, creating the data directory when needed.
pub fn save(notes: &[Note]) -> Result<(), StorageError> {
    save_to(&notes_path(), notes)
}

pub fn load_from(path: &Path) -> Result<Vec<Note>, StorageError> {
    match fs::read_to_string(path) {
        Ok(contents) if contents.trim().is_empty() => Ok(Vec::new()),
        Ok(contents) => Ok(serde_json::from_str(&contents)?),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(StorageError::Io(error)),
    }
}

pub fn save_to(path: &Path, notes: &[Note]) -> Result<(), StorageError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let contents = serde_json::to_string_pretty(notes)?;
    fs::write(path, contents)?;
    Ok(())
}

#[derive(Debug)]
pub enum StorageError {
    Io(io::Error),
    Serde(serde_json::Error),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::Io(error) => write!(f, "could not access notes file: {error}"),
            StorageError::Serde(error) => write!(f, "could not parse notes file: {error}"),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<io::Error> for StorageError {
    fn from(error: io::Error) -> Self {
        StorageError::Io(error)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(error: serde_json::Error) -> Self {
        StorageError::Serde(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_notes_through_disk() {
        let dir = std::env::temp_dir().join(format!("testapp-test-{}", uuid::Uuid::new_v4()));
        let path = dir.join("nested").join(NOTES_FILE);

        assert!(load_from(&path).unwrap().is_empty());

        let mut note = Note::new();
        note.title = "Hello".to_string();
        note.body = "# Hi".to_string();
        save_to(&path, std::slice::from_ref(&note)).unwrap();

        let loaded = load_from(&path).unwrap();
        assert_eq!(loaded, vec![note]);

        fs::remove_dir_all(&dir).unwrap();
    }
}
