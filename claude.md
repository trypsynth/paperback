# Paperback - Claude Code Project Guide

## Project Overview

Paperback is an accessible, lightweight, fast ebook and document reader for desktop. Written entirely in Rust using wxdragon bindings for the GUI.

## Architecture

The project is a **pure Rust application** built with `cargo build`:

- Entry point: `src/main.rs`
- Uses wxdragon for wxWidgets bindings
- All UI, dialogs, config, document handling, and IPC are in Rust

### Directory Structure

```
paperback/
├── src/                    # Rust source
│   ├── main.rs             # Entry point
│   ├── ipc.rs              # IPC for single-instance support
│   ├── ui/                 # UI components
│   │   ├── app.rs          # PaperbackApp main struct
│   │   ├── main_window.rs  # Main window
│   │   ├── dialogs.rs      # All dialogs
│   │   ├── document_manager.rs
│   │   ├── menu_ids.rs
│   │   └── utils.rs
│   ├── parser/             # Document format parsers
│   │   ├── epub.rs, pdf.rs, docx.rs, chm.rs, fb2.rs
│   │   ├── odt.rs, pptx.rs, odp.rs, html.rs
│   │   ├── markdown.rs, text.rs
│   │   └── utils.rs
│   ├── config.rs           # Configuration management (uses wxConfig)
│   ├── session.rs          # Document session state
│   ├── document.rs         # Document data structures
│   ├── reader_core.rs      # Navigation logic
│   ├── update.rs           # Update checking
│   ├── live_region.rs      # Accessibility
│   ├── translation_manager.rs
│   └── utils/              # Text, encoding, zip utilities
└── Cargo.toml              # Rust build config
```

## How to Build

**IMPORTANT: Always use `--release` flag for builds.**

```bash
cargo build --release
```

### Running

```bash
cargo run --release
```

## Key Patterns

### Dialog Implementation Pattern

```rust
pub fn show_example_dialog(parent: &Frame, ...) -> Option<Result> {
    let dialog = Dialog::builder(parent, &t("Title")).build();
    // ... build UI with BoxSizer, Button, etc.
    dialog.set_sizer_and_fit(content_sizer, true);
    dialog.centre();
    if dialog.show_modal() == wxdragon::id::ID_OK {
        Some(result)
    } else {
        None
    }
}
```

### Translation Pattern

```rust
use wxdragon::translations::translate as t;
let label = t("Some text");  // Looks up translation
```

### Configuration Access

```rust
let config = config.lock().unwrap();
let value = config.get_app_bool("setting_name", default_value);
```

## Testing

The project doesn't have automated tests. Manual testing with various document formats is recommended.

## Supported Formats

- EPUB (.epub)
- PDF (.pdf)
- DOCX (.docx)
- ODT (.odt)
- PPTX (.pptx)
- ODP (.odp)
- CHM (.chm)
- FB2 (.fb2)
- HTML (.html, .htm)
- Markdown (.md)
- Plain text (.txt)

## Accessibility

The application is designed for accessibility with screen readers:
- Live regions for announcements (`src/live_region.rs`)
- Keyboard navigation throughout
- Accessible controls and labels

## Notes

- Translations use `.po` files - ensure translation keys match
