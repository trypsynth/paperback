# Paperback - Claude Code Project Guide

## Project Overview

Paperback is an accessible, lightweight, fast ebook and document reader for desktop. The project has been migrated from C++ (wxWidgets) to Rust (using wxdragon bindings). The migration is nearly complete, with only IPC/single-instance logic remaining in C++ (awaiting wxDragon IPC bindings).

## Architecture

### Current State

The project is a **pure Rust application** built with `cargo build`:

- Entry point: `src/main.rs`
- Uses wxdragon for wxWidgets bindings
- All UI, dialogs, config, and document handling are in Rust

The only remaining C++ code (`app/app.cpp`, `app/app.hpp`) contains IPC logic for single-instance checking. This will be ported once wxDragon provides IPC bindings.

### Directory Structure

```
paperback/
├── src/                    # Rust source (ACTIVE)
│   ├── main.rs             # Entry point
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
├── app/                    # C++ source (IPC only - awaiting wxDragon bindings)
│   ├── app.cpp             # IPC server/client, single instance checking
│   └── app.hpp             # IPC class declarations
└── Cargo.toml              # Rust build config
```

## Remaining C++ Code

Only `app/app.cpp` and `app/app.hpp` remain. They contain:

| Feature | Purpose | Notes |
|---------|---------|-------|
| `paperback_server` / `paperback_client` | IPC for single instance | Uses wxIPC |
| `wxSingleInstanceChecker` | Prevents multiple instances | |
| `open_file()` via IPC | Opens files in existing instance | |

This will be ported once wxDragon provides IPC bindings (`wxServer`, `wxClient`, `wxConnection`, `wxSingleInstanceChecker`).

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

### Dialog Implementation Pattern (Rust)

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

- All active development targets the Rust codebase
- The remaining C++ code is minimal (IPC only) and will be removed once wxDragon provides IPC bindings
- Translations use `.po` files - ensure translation keys match
