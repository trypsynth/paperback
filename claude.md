# Paperback - Claude Code Project Guide

## Project Overview

Paperback is an accessible, lightweight, fast ebook and document reader for desktop. The project is actively being migrated from C++ (wxWidgets) to Rust (using wxdragon bindings).

## Architecture

### Current State

The project has **two build systems**, but only the Rust one is active:

1. **Pure Rust (Active)**: Built with `cargo build` from root `Cargo.toml`
   - Entry point: `src/main.rs`
   - Uses wxdragon for wxWidgets bindings
   - This is the actively developed version

2. **C++ with Rust lib (Legacy/Broken)**: CMakeLists.txt references a non-existent `lib/` folder
   - The `app/` folder contains legacy C++ code
   - CMake build no longer functions
   - C++ code is being incrementally removed as features are ported to Rust

### Directory Structure

```
paperback/
├── src/                    # Rust source (ACTIVE)
│   ├── main.rs             # Entry point
│   ├── ui/                 # UI components
│   │   ├── app.rs          # PaperbackApp main struct
│   │   ├── main_window.rs  # Main window (~1,666 lines)
│   │   ├── dialogs.rs      # Ported dialogs (~25,824 lines)
│   │   ├── document_manager.rs
│   │   ├── menu_ids.rs
│   │   └── utils.rs
│   ├── parser/             # Document format parsers
│   │   ├── epub.rs, pdf.rs, docx.rs, chm.rs, fb2.rs
│   │   ├── odt.rs, pptx.rs, odp.rs, html.rs
│   │   ├── markdown.rs, text.rs
│   │   └── utils.rs
│   ├── config.rs           # Configuration management
│   ├── session.rs          # Document session state
│   ├── document.rs         # Document data structures
│   ├── reader_core.rs      # Navigation logic
│   ├── update.rs           # Update checking
│   ├── live_region.rs      # Accessibility
│   ├── translation_manager.rs
│   └── utils/              # Text, encoding, zip utilities
├── app/                    # C++ source (LEGACY - being removed)
│   ├── dialogs.cpp/hpp     # Legacy dialogs (most ported)
│   ├── main_window.cpp/hpp # Legacy main window
│   ├── document_manager.cpp/hpp
│   ├── app.cpp/hpp         # Legacy app entry
│   └── ...
├── Cargo.toml              # Rust build config
└── CMakeLists.txt          # C++ build (non-functional)
```

## Recently Ported Features

These C++ features have been ported to Rust (their C++ implementations have been removed or stubbed):

| Feature | Rust Location | Commit |
|---------|---------------|--------|
| Table of Contents Dialog | `src/ui/dialogs.rs::show_toc_dialog` | 4037e04 |
| Update Dialog | `src/ui/dialogs.rs::show_update_dialog` | bf11ea3 |
| Navigation Logic | `src/reader_core.rs::reader_navigate` | 51c25bc |
| Open As Dialog | `src/ui/dialogs.rs::show_open_as_dialog` | - |
| Find Dialog | `src/ui/main_window.rs::FindDialogState` | - |
| Document Info Dialog | `src/ui/dialogs.rs::show_document_info_dialog` | - |
| All Documents Dialog | `src/ui/dialogs.rs::show_all_documents_dialog` | - |

## C++ Features Still Needing Port

### Dialogs (app/dialogs.cpp)

| Dialog | Purpose | Complexity | Notes |
|--------|---------|------------|-------|
| `bookmark_dialog` | Jump to/manage bookmarks | Medium | Has filter, edit note, delete |
| `elements_dialog` | View headings/links tree | Medium | Dual view (tree + list) |
| `go_to_line_dialog` | Navigate to line number | Low | Simple spin control |
| `go_to_page_dialog` | Navigate to page | Low | Simple spin control |
| `go_to_percent_dialog` | Navigate by percentage | Low | Slider + spin control |
| `note_entry_dialog` | Add/edit bookmark notes | Low | Multiline text entry |
| `options_dialog` | Application preferences | Medium | Multiple settings checkboxes |
| `password_dialog` | Password entry for encrypted docs | Low | Password text control |
| `sleep_timer_dialog` | Configure sleep timer | Low | Simple spin control |
| `view_note_dialog` | Display note content | Low | Read-only text display |
| `web_view_dialog` | Display tables as HTML | Medium | Uses wxWebView |

### Controls (app/controls.cpp)

| Control | Purpose | Notes |
|---------|---------|-------|
| `accessible_slider` | Screen reader accessible slider | Used by go_to_percent_dialog |

### App Infrastructure (app/app.cpp)

| Feature | Purpose | Notes |
|---------|---------|-------|
| IPC Server/Client | Single instance checking | Uses wxIPC |
| Document restoration | Restore previous documents on startup | Already in Rust but C++ has fallback |

### Main Window Handlers (app/main_window.cpp)

Many menu handlers still call C++ document_manager methods. These need review:

- Sleep timer functionality
- Some bookmark operations
- Some navigation that still uses C++ paths

## How to Build

### Rust (Active)

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

The project doesn't appear to have automated tests. Manual testing with various document formats is recommended.

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

## Porting Priority Recommendations

1. **High Priority** (used frequently):
   - `bookmark_dialog` - Core reading feature
   - `options_dialog` - User configuration
   - `password_dialog` - Required for encrypted PDFs

2. **Medium Priority** (useful but not critical):
   - `go_to_*` dialogs - Navigation helpers
   - `elements_dialog` - Document structure view
   - `note_entry_dialog` - Bookmark notes

3. **Lower Priority** (less common use):
   - `sleep_timer_dialog` - Niche feature
   - `view_note_dialog` - Simple display
   - `web_view_dialog` - Table display

## C++ Code Removal Checklist

After porting a dialog:

1. Remove class declaration from `app/dialogs.hpp`
2. Remove implementation from `app/dialogs.cpp`
3. Update any C++ code that calls the dialog (stub or remove)
4. Add comment indicating Rust location

## Notes

- The C++ CMake build is broken (missing `lib/` folder)
- All active development should target the Rust codebase
- When porting dialogs, match the existing Rust patterns in `src/ui/dialogs.rs`
- Translations use the same `.po` files - ensure translation keys match
