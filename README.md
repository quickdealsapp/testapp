# Notes

A cross-platform desktop notes app written in Rust with [Dioxus](https://dioxuslabs.com/).
Notes are written in Markdown, rendered to polished HTML for reading, and persisted as JSON in
your OS data directory.

## Features

- **Two-pane layout** — sidebar with the note list, content area for reading and editing.
- **Markdown editing** with a live side-by-side preview, rendered by `pulldown-cmark`.
- **Create / Edit / Archive / Delete** — toolbar actions are disabled until a note is selected,
  and deleting asks for confirmation.
- **Active / Archived filters** so archived notes stay out of the way without being lost.
- **Local persistence** — every mutation is written to disk immediately, so notes survive restarts.
- **Modern dark UI** with the Inter typeface and styled Markdown (headings, code, tables, quotes).

## Prerequisites

- Rust toolchain (the pinned version in `rust-toolchain.toml` is installed automatically by rustup).
- A system webview:
  - **Linux**: WebKitGTK and friends, e.g. on Debian/Ubuntu:
    ```bash
    sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev librsvg2-dev
    ```
  - **macOS**: WebKit ships with the OS (Xcode command line tools required).
  - **Windows**: WebView2 runtime (preinstalled on Windows 11, otherwise install from Microsoft).

## Build & run

```bash
cargo run            # debug build
cargo run --release  # optimized build
cargo test           # unit tests
cargo fmt --all --check && cargo clippy --all-targets -- -D warnings
```

## Releases

Prebuilt binaries for Linux, macOS (Intel + Apple Silicon) and Windows are attached to every
[GitHub release](https://github.com/quickdealsapp/testapp/releases). Download the archive for your
platform, extract it, and run the `testapp` executable (the Linux webview packages above are still
required at runtime).

Releases are cut by pushing a tag, which runs `.github/workflows/release.yml`:

```bash
git tag v0.1.0 && git push origin v0.1.0
```

## Where notes are stored

All notes live in a single JSON file inside the platform data directory:

| Platform | Path |
| --- | --- |
| Linux | `~/.local/share/testapp/notes.json` |
| macOS | `~/Library/Application Support/testapp/notes.json` |
| Windows | `%APPDATA%\testapp\notes.json` |

## Project layout

| Path | Purpose |
| --- | --- |
| `src/main.rs` | Window configuration and Dioxus desktop launch |
| `src/app.rs` | UI components, state and actions (RSX) |
| `src/note.rs` | `Note` data model |
| `src/storage.rs` | JSON load/save in the OS data directory |
| `src/markdown.rs` | Markdown → HTML rendering |
| `assets/main.css` | Stylesheet embedded at compile time |
