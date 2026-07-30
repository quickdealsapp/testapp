use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single markdown note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub archived: bool,
}

impl Note {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            body: String::new(),
            created_at: now,
            updated_at: now,
            archived: false,
        }
    }

    /// Title to show in lists, falling back to a placeholder for empty titles.
    pub fn display_title(&self) -> String {
        let title = self.title.trim();
        if !title.is_empty() {
            return title.to_string();
        }
        let first_line = self
            .body
            .lines()
            .map(|line| line.trim_start_matches('#').trim())
            .find(|line| !line.is_empty());
        match first_line {
            Some(line) => truncate(line, 60),
            None => "Untitled note".to_string(),
        }
    }

    /// Short plain-text excerpt of the body for the sidebar.
    pub fn snippet(&self) -> String {
        let text = self
            .body
            .lines()
            .map(|line| line.trim_start_matches(['#', '>', '-', '*', ' ']).trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        if text.is_empty() {
            "No content yet".to_string()
        } else {
            truncate(&text, 90)
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}

fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let mut out: String = text.chars().take(max).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_title_falls_back_to_body_then_placeholder() {
        let mut note = Note::new();
        assert_eq!(note.display_title(), "Untitled note");

        note.body = "# Shopping list\nmilk".to_string();
        assert_eq!(note.display_title(), "Shopping list");

        note.title = "  Groceries  ".to_string();
        assert_eq!(note.display_title(), "Groceries");
    }

    #[test]
    fn snippet_strips_markdown_markers_and_truncates() {
        let mut note = Note::new();
        assert_eq!(note.snippet(), "No content yet");

        note.body = "# Title\n- item one\n> quoted".to_string();
        assert_eq!(note.snippet(), "Title item one quoted");

        note.body = "a".repeat(120);
        let snippet = note.snippet();
        assert_eq!(snippet.chars().count(), 91);
        assert!(snippet.ends_with('…'));
    }
}
