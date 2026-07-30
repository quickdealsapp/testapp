use dioxus::prelude::*;
use uuid::Uuid;

use crate::markdown;
use crate::note::Note;
use crate::storage;

const MAIN_CSS: &str = include_str!("../assets/main.css");
const DATE_FORMAT: &str = "%b %d, %Y · %H:%M";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    View,
    Edit,
}

/// Pre-rendered sidebar row, so the markup only interpolates plain strings.
#[derive(Clone, PartialEq)]
struct NoteRow {
    id: Uuid,
    title: String,
    snippet: String,
    date: String,
    class: String,
}

#[component]
pub fn App() -> Element {
    let status = use_signal(|| Option::<String>::None);
    let mut notes = use_signal(|| match storage::load() {
        Ok(notes) => notes,
        Err(error) => {
            eprintln!("failed to load notes: {error}");
            Vec::new()
        }
    });
    let mut selected = use_signal(|| Option::<Uuid>::None);
    let mut mode = use_signal(|| Mode::View);
    let mut show_archived = use_signal(|| false);
    let mut draft_title = use_signal(String::new);
    let mut draft_body = use_signal(String::new);
    let mut confirming_delete = use_signal(|| false);

    let selected_note = selected().and_then(|id| find(notes, id));
    let has_selection = selected_note.is_some();
    let editing = mode() == Mode::Edit;
    let archived_view = show_archived();

    let mut visible_notes: Vec<Note> = notes
        .read()
        .iter()
        .filter(|note| note.archived == archived_view)
        .cloned()
        .collect();
    visible_notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    let rows: Vec<NoteRow> = visible_notes
        .iter()
        .map(|note| NoteRow {
            id: note.id,
            title: note.display_title(),
            snippet: note.snippet(),
            date: note.updated_at.format(DATE_FORMAT).to_string(),
            class: if selected() == Some(note.id) {
                "note-item selected".to_string()
            } else {
                "note-item".to_string()
            },
        })
        .collect();

    let create_note = move |_| {
        let note = Note::new();
        let id = note.id;
        notes.write().push(note);
        persist(notes, status);
        selected.set(Some(id));
        draft_title.set(String::new());
        draft_body.set(String::new());
        show_archived.set(false);
        mode.set(Mode::Edit);
    };

    let start_editing = move |_| {
        if let Some(note) = selected().and_then(|id| find(notes, id)) {
            draft_title.set(note.title.clone());
            draft_body.set(note.body.clone());
            mode.set(Mode::Edit);
        }
    };

    let save_note = move |_| {
        if let Some(id) = selected() {
            {
                let mut notes = notes.write();
                if let Some(note) = notes.iter_mut().find(|note| note.id == id) {
                    note.title = draft_title();
                    note.body = draft_body();
                    note.touch();
                }
            }
            persist(notes, status);
        }
        mode.set(Mode::View);
    };

    let toggle_archive = move |_| {
        if let Some(id) = selected() {
            {
                let mut notes = notes.write();
                if let Some(note) = notes.iter_mut().find(|note| note.id == id) {
                    note.archived = !note.archived;
                    note.touch();
                }
            }
            persist(notes, status);
            selected.set(None);
            mode.set(Mode::View);
        }
    };

    let delete_note = move |_| {
        if let Some(id) = selected() {
            notes.write().retain(|note| note.id != id);
            persist(notes, status);
            selected.set(None);
            mode.set(Mode::View);
        }
        confirming_delete.set(false);
    };

    let archive_label = if selected_note.as_ref().is_some_and(|note| note.archived) {
        "Unarchive"
    } else {
        "Archive"
    };
    let active_filter_class = if archived_view {
        "filter"
    } else {
        "filter active"
    };
    let archived_filter_class = if archived_view {
        "filter active"
    } else {
        "filter"
    };
    let empty_message = if archived_view {
        "No archived notes."
    } else {
        "No notes yet — press + to create one."
    };

    let content = match selected_note {
        None => rsx! {
            div { class: "placeholder",
                h1 { "Nothing selected" }
                p { "Pick a note from the sidebar, or press + to write a new one in Markdown." }
            }
        },
        Some(_) if editing => {
            let preview = markdown::to_html(&draft_body());
            rsx! {
                div { class: "editor",
                    input {
                        class: "title-input",
                        placeholder: "Note title",
                        value: "{draft_title}",
                        oninput: move |event| draft_title.set(event.value()),
                    }
                    div { class: "editor-panes",
                        textarea {
                            class: "body-input",
                            placeholder: "# Write Markdown here…",
                            value: "{draft_body}",
                            oninput: move |event| draft_body.set(event.value()),
                        }
                        div { class: "live-preview markdown", dangerous_inner_html: "{preview}" }
                    }
                    div { class: "editor-actions",
                        button {
                            class: "toolbar-button",
                            onclick: move |_| mode.set(Mode::View),
                            "Cancel"
                        }
                        button { class: "toolbar-button primary", onclick: save_note, "Save" }
                    }
                }
            }
        }
        Some(note) => {
            let title = note.display_title();
            let updated = format!("Updated {}", note.updated_at.format(DATE_FORMAT));
            let body = markdown::to_html(&note.body);
            let archived_badge = note.archived;
            rsx! {
                article { class: "viewer",
                    header { class: "viewer-header",
                        h1 { "{title}" }
                        p { class: "viewer-meta",
                            "{updated}"
                            if archived_badge {
                                span { class: "badge", "Archived" }
                            }
                        }
                    }
                    div { class: "markdown", dangerous_inner_html: "{body}" }
                }
            }
        }
    };

    rsx! {
        style { dangerous_inner_html: "{MAIN_CSS}" }
        div { class: "app",
            aside { class: "sidebar",
                header { class: "sidebar-header",
                    div { class: "brand",
                        span { class: "brand-mark", "N" }
                        span { class: "brand-name", "Notes" }
                    }
                    button {
                        class: "icon-button primary",
                        title: "New note",
                        onclick: create_note,
                        "+"
                    }
                }
                div { class: "toolbar",
                    button {
                        class: "toolbar-button",
                        disabled: !has_selection,
                        onclick: start_editing,
                        "Edit"
                    }
                    button {
                        class: "toolbar-button",
                        disabled: !has_selection,
                        onclick: toggle_archive,
                        "{archive_label}"
                    }
                    button {
                        class: "toolbar-button danger",
                        disabled: !has_selection,
                        onclick: move |_| confirming_delete.set(true),
                        "Delete"
                    }
                }
                div { class: "filters",
                    button {
                        class: "{active_filter_class}",
                        onclick: move |_| select_view(false, show_archived, selected, mode),
                        "Active"
                    }
                    button {
                        class: "{archived_filter_class}",
                        onclick: move |_| select_view(true, show_archived, selected, mode),
                        "Archived"
                    }
                }
                div { class: "note-list",
                    if rows.is_empty() {
                        p { class: "empty-list", "{empty_message}" }
                    }
                    for row in rows.iter().cloned() {
                        button {
                            key: "{row.id}",
                            class: "{row.class}",
                            onclick: move |_| {
                                selected.set(Some(row.id));
                                mode.set(Mode::View);
                            },
                            span { class: "note-item-title", "{row.title}" }
                            span { class: "note-item-snippet", "{row.snippet}" }
                            span { class: "note-item-date", "{row.date}" }
                        }
                    }
                }
            }
            main { class: "content", {content} }
            if confirming_delete() {
                div { class: "modal-backdrop",
                    div { class: "modal",
                        h2 { "Delete note?" }
                        p { "This permanently removes the note from disk." }
                        div { class: "modal-actions",
                            button {
                                class: "toolbar-button",
                                onclick: move |_| confirming_delete.set(false),
                                "Cancel"
                            }
                            button {
                                class: "toolbar-button danger-solid",
                                onclick: delete_note,
                                "Delete"
                            }
                        }
                    }
                }
            }
            if let Some(message) = status() {
                div { class: "status-bar", "{message}" }
            }
        }
    }
}

fn select_view(
    archived: bool,
    mut show_archived: Signal<bool>,
    mut selected: Signal<Option<Uuid>>,
    mut mode: Signal<Mode>,
) {
    show_archived.set(archived);
    selected.set(None);
    mode.set(Mode::View);
}

fn find(notes: Signal<Vec<Note>>, id: Uuid) -> Option<Note> {
    notes.read().iter().find(|note| note.id == id).cloned()
}

fn persist(notes: Signal<Vec<Note>>, mut status: Signal<Option<String>>) {
    match storage::save(&notes.read()) {
        Ok(()) => status.set(None),
        Err(error) => status.set(Some(error.to_string())),
    }
}
