# Paperback - Claude Code Project Guide

## Project Overview

Paperback is an accessible, lightweight ebook and document reader for desktop. Pure Rust, wxdragon (wxWidgets) for GUI. Accessibility-first design targeting screen reader users.

## Build & Workflow

```bash
cargo build --release     # Always use --release
cargo run --release
cargo clippy --release    # Lint check
```

Build takes ~50s. No automated tests - manual testing with actual document files.

**Pre-existing warnings** (do not fix unless asked):
- `SystemOptions` unused import in `main.rs`
- Several clippy pedantic warnings in `ui/help.rs`, `update.rs`, `document_manager.rs`

## Architecture: Layered

```
Parsing Layer     parser/*.rs          format files → Document
Session Layer     session.rs           runtime state, position tracking, link resolution
Core Logic        reader_core.rs       navigation algorithms, search
Configuration     config.rs            persistent settings, per-doc state
UI Layer          ui/*.rs              wxWidgets dialogs, main window, menus
IPC/Update        ipc.rs, update.rs    single-instance, GitHub update checks
Utilities         text.rs, encoding.rs, zip.rs, html_to_text.rs, xml_to_text.rs
```

## Key Files Reference

| File | Purpose |
|------|---------|
| `src/main.rs` | Entry point, IPC mutex, app init |
| `src/document.rs` | Core data types: `Document`, `DocumentBuffer`, `Marker`, `MarkerType` |
| `src/session.rs` | `DocumentSession` - runtime state wrapping parsed doc |
| `src/reader_core.rs` | `reader_navigate()`, `reader_search()`, bookmark nav |
| `src/config.rs` | `ConfigManager`, all config keys, migration logic |
| `src/parser.rs` | `Parser` trait, `ParserRegistry`, `ConverterOutput` trait |
| `src/ipc.rs` | Single-instance IPC (port 4242, localhost) |
| `src/update.rs` | GitHub releases API, download, version comparison |
| `src/live_region.rs` | Screen reader announcements |
| `src/translation_manager.rs` | Gettext `.mo` file loading, language detection |
| `src/ui/app.rs` | `PaperbackApp` main struct |
| `src/ui/main_window.rs` | Main frame, menu bar, status bar |
| `src/ui/dialogs.rs` | All user-facing dialogs |
| `src/ui/document_manager.rs` | Open/close/recent document management |
| `src/ui/menu_ids.rs` | All menu/command ID constants |
| `src/ui/utils.rs` | UI helpers, `main_window_parent()` |
| `src/ui/help.rs` | Update dialog, `execute_update()` - launches installer |
| `src/html_to_text.rs` | `HtmlToText` - HTML → text + markers |
| `src/xml_to_text.rs` | `XmlToText` - XML → text + markers |

## Core Data Structures

### `MarkerType` (document.rs)
```rust
Heading1=0, Heading2=1, Heading3=2, Heading4=3, Heading5=4, Heading6=5,
PageBreak=6, SectionBreak=7, TocItem=8, Link=9,
List=10, ListItem=11, Table=12, Separator=13
```

### `Marker` (document.rs)
```rust
pub struct Marker {
    pub mtype: MarkerType,
    pub position: usize,    // Position in display_len units
    pub text: String,       // Heading text, link text, etc.
    pub reference: String,  // URL/path (links), HTML (tables)
    pub level: i32,         // Heading level (1-6), list depth
    pub length: usize,      // Content length
}
```

### `DocumentBuffer` (document.rs)
```rust
pub struct DocumentBuffer {
    pub content: String,            // Full UTF-8 document text
    pub markers: Vec<Marker>,       // Sorted by position
    content_display_len: usize,     // Display length (CJK-aware)
}
```

### `Document` (document.rs)
```rust
pub struct Document {
    pub title: String,
    pub author: String,
    pub buffer: DocumentBuffer,
    pub toc_items: Vec<TocItem>,
    pub id_positions: HashMap<String, usize>,   // anchor-id → position
    pub spine_items: Vec<String>,               // EPUB spine order
    pub manifest_items: HashMap<String, String>, // EPUB manifest
    pub stats: DocumentStats,
}
```

### `ParserContext` (document.rs)
```rust
pub struct ParserContext {
    pub file_path: String,
    pub password: Option<String>,
    pub forced_extension: Option<String>,
}
```

### `ParserFlags` (bitflags, document.rs)
```
SUPPORTS_SECTIONS | SUPPORTS_TOC | SUPPORTS_PAGES | SUPPORTS_LISTS
```

### `DocumentSession` (session.rs)
```rust
pub struct DocumentSession {
    handle: DocumentHandle,         // Immutable parsed doc
    file_path: String,
    history: Vec<i64>,              // Navigation positions
    history_index: usize,
    parser_flags: ParserFlags,
    last_stable_position: Option<i64>,
}
```
Constants: `MAX_HISTORY_LEN = 10`, `HISTORY_DISTANCE_THRESHOLD = 300`

### `NavigationResult` (session.rs)
```rust
pub struct NavigationResult {
    pub found: bool,
    pub wrapped: bool,
    pub offset: i64,
    pub marker_text: String,
    pub marker_level: i32,
    pub marker_index: i32,
    pub not_supported: bool,
}
```

### `LinkActivationResult` (session.rs)
```rust
pub struct LinkActivationResult {
    pub found: bool,
    pub action: LinkAction,   // Internal / External / NotFound
    pub offset: i64,          // Internal links
    pub url: String,          // External links
}
```

### `Bookmark` (config.rs)
```rust
pub struct Bookmark {
    pub start: i64,
    pub end: i64,
    pub note: String,   // base64-encoded
}
```

## Parser System

### `Parser` trait (parser.rs)
```rust
pub trait Parser: Send + Sync {
    fn name(&self) -> &str;
    fn extensions(&self) -> &[&str];
    fn supported_flags(&self) -> ParserFlags;
    fn parse(&self, context: &ParserContext) -> Result<Document>;
}
```

### `ConverterOutput` trait (parser.rs)
```rust
pub trait ConverterOutput {
    fn get_headings(&self) -> &[HeadingInfo];
    fn get_links(&self) -> &[LinkInfo];
    fn get_tables(&self) -> &[TableInfo];
    fn get_separators(&self) -> &[SeparatorInfo];
    fn get_lists(&self) -> &[ListInfo];
    fn get_list_items(&self) -> &[ListItemInfo];
}
```
Both `HtmlToText` and `XmlToText` implement this.

### Transferring converter output to `DocumentBuffer`
```rust
// Standard (includes links):
add_converter_markers(&mut buffer, &converter_output);

// EPUB/CHM use custom link loops instead:
add_converter_markers_excluding_links(&mut buffer, &converter_output);
// ...then handle links manually for proper resolution
```

### Registered parsers (parser.rs `ParserRegistry`)
`ChmParser`, `DocxParser`, `EpubParser`, `Fb2Parser`, `HtmlParser`, `XmlParser`,
`PdfParser`, `MarkdownParser`, `OdpParser`, `FodpParser`, `OdtParser`, `FodtParser`,
`PptxParser`, `RtfParser`, `TextParser`

### Entry points
- `parse_document(context)` — main entry, also computes stats
- `get_parser_flags_for_context(context)` — returns supported features without parsing
- `parser_supports_extension(ext)` — quick boolean check
- `build_file_filter_string()` — wx file dialog filter string
- `is_external_url(href)` — checks http/https/ftp/mailto schemes

## Navigation (reader_core.rs)

### `NavRequest`
```rust
pub struct NavRequest {
    pub position: i64,
    pub wrap: bool,
    pub direction: NavDirection,    // Next / Previous
    pub target: NavTarget,
    pub level_filter: i32,          // 0=any, 1-6 for headings
}
```

### `NavTarget` variants
`Section`, `Page`, `Heading`, `List`, `ListItem`, `Link`, `Table`, `Separator`

### Search flags
```
FORWARD | MATCH_CASE | WHOLE_WORD | REGEX
```
- `reader_search()` — single pass, returns -1 if not found
- `reader_search_with_wrap()` — wraps, returns `(found, wrapped, position)`
- Positions are UTF-16 units (for compatibility)

## Configuration (config.rs)

Config stored in `%APPDATA%/Paperback/Paperback.ini` (installed) or exe dir (portable). Uses wxdragon's INI-style `Config`. Document sections named by SHA1 hash of file path.

### App-level keys (`/app` path)

| Key | Type | Default |
|-----|------|---------|
| `restore_previous_documents` | bool | true |
| `word_wrap` | bool | false |
| `minimize_to_tray` | bool | false |
| `start_maximized` | bool | false |
| `compact_go_menu` | bool | true |
| `navigation_wrap` | bool | false |
| `check_for_updates_on_startup` | bool | true |
| `find_match_case` | bool | false |
| `find_whole_word` | bool | false |
| `find_use_regex` | bool | false |
| `recent_documents_to_show` | int | 25 |
| `sleep_timer_duration` | int | 30 |
| `version` | int | 2 (current) |
| `language` | string | system default |
| `active_document` | string | "" |
| `update_channel` | string | "stable" |

### Per-document keys (section = `doc_<SHA1>`)

| Key | Type |
|-----|------|
| `last_position` | i64 |
| `navigation_history` | CSV of i64 |
| `navigation_history_index` | i64 |
| `bookmarks` | CSV: `start:end:encoded_note,...` |
| `format` | string (extension override) |
| `password` | string |
| `opened` | bool |
| `path` | string (full file path) |

### Special sections
- `/recent_documents` — `doc0`, `doc1`, ... → section names
- `/opened_documents` — `File0`, `File1`, ... → file paths
- `/find_history` — `item0`, `item1`, ... → search terms

### Config access pattern
```rust
let config = config.lock().unwrap();
let value = config.get_app_bool("setting_name", default_value);
config.set_app_bool("setting_name", value);
// Also: get_app_string, set_app_string, get_app_int, set_app_int
```

### Migration
- v0→v1: Move root-level positions to document sections
- v1→v2: Add end position to bookmarks

## UI Patterns (wxdragon)

### Dialog pattern
```rust
pub fn show_example_dialog(parent: &Frame, ...) -> Option<Result> {
    let dialog = Dialog::builder(parent, &t("Title")).build();
    let content_sizer = BoxSizer::new(Orientation::Vertical);
    // ... add controls to content_sizer
    dialog.set_sizer_and_fit(content_sizer, true);
    dialog.centre();
    if dialog.show_modal() == wxdragon::id::ID_OK {
        Some(result)
    } else {
        None
    }
}
```

### Sizers
- `BoxSizer::new(Orientation::Vertical/Horizontal)` for linear layouts
- `StaticBoxSizer` for labelled groups
- `.add(control, flags)` with `wxSizerFlags` for alignment/expansion

### Common controls
`TextCtrl`, `Button`, `Choice`, `ComboBox`, `CheckBox`, `ListCtrl`, `StaticText`, `WebView`

### Standard IDs
`ID_OK`, `ID_CANCEL`, `ID_EXIT`, `ID_ABOUT` — from `wxdragon::id`

### Getting parent window
```rust
use crate::ui::utils::main_window_parent;
let parent = main_window_parent();
```

### Launching external processes (Windows)
```rust
use std::process::Command;
// Installer:
Command::new(&path).arg("/silent").spawn()?;
// Background PowerShell (no window):
Command::new("powershell.exe")
    .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &script])
    .creation_flags(0x0800_0000)  // CREATE_NO_WINDOW
    .spawn()?;
```

## Menu IDs (menu_ids.rs)

Generated by `menu_ids!()` macro to prevent collisions. Key ranges:

| Range | Feature |
|-------|---------|
| 0–99 | File (OPEN, CLOSE, CLOSE_ALL) |
| 100–199 | Recent documents (RECENT_DOCUMENT_BASE) |
| 200–209 | Find (FIND, FIND_NEXT, FIND_PREVIOUS) |
| 210–219 | Go to (GO_TO_LINE, GO_TO_PERCENT, GO_TO_PAGE) |
| 220–229 | History (GO_BACK, GO_FORWARD) |
| 230–239 | Sections (PREVIOUS/NEXT_SECTION) |
| 240–269 | Headings (PREVIOUS/NEXT_HEADING, HEADING_1–6) |
| 270–279 | Pages (PREVIOUS/NEXT_PAGE) |
| 280–289 | Bookmarks/Notes (PREVIOUS/NEXT_BOOKMARK, JUMP_TO_*) |
| 290–299 | Links (PREVIOUS/NEXT_LINK) |
| 300–319 | Elements (TABLE, SEPARATOR, LIST, LIST_ITEM) |
| 400–409 | Document info (WORD_COUNT, TOC, ELEMENTS_LIST) |
| 410–419 | Import/Export (EXPORT_TO_PLAIN_TEXT) |
| 420–429 | Bookmark tools (TOGGLE_BOOKMARK, BOOKMARK_WITH_NOTE) |
| 430–439 | Settings (OPTIONS, SLEEP_TIMER) |
| 500–599 | Help (CHECK_FOR_UPDATES, DONATE) |
| 900–999 | System tray (RESTORE) |

## Translation

```rust
use wxdragon::translations::translate as t;
let label = t("Some English text");
```

- `.po` source files in `/po/` (e.g., `es.po`, `zh_CN.po`)
- Compiled to `.mo` binary files in `langs/<code>/LC_MESSAGES/paperback.mo`
- `translation_manager.rs` scans `langs/` at startup, detects system language
- Language settable via config key `language`
- All user-visible strings must use `t()` — **never hardcode display strings**
- `msgid` in `.po` is the exact English string passed to `t()`

## Accessibility

- `live_region` crate (v0.1.4) for screen reader announcements
- Used in UI layer for navigation feedback and status changes
- All dialogs must have accessible labels on controls
- Keyboard navigation required throughout — no mouse-only interactions
- `src/live_region.rs` wraps the crate for app use

## IPC (ipc.rs)

- Mutex name: `"paperback_running"` — prevents multiple instances
- Port: `4242` on `localhost`
- Topic: `"open_file"`
- Commands: `Activate` (raise window) or `OpenFile(PathBuf)`
- `normalize_cli_path()` — canonicalizes paths, handles relative → absolute

## Update System (update.rs)

Flow: check GitHub releases API → compare version → download to temp → launch.

- **Stable**: fetches `/releases/latest`, compares semver tags
- **Dev**: fetches `/releases/tags/latest`, compares commit hashes in release notes
- Asset selection: `paperback_setup.exe` (installer) or `paperback.zip` (portable)
- Installer launched with `/silent` flag
- HTTP client: `ureq` with rustls (no OpenSSL), 30s connect / 600s transfer timeout
- `UpdateChannel` enum: `Stable` | `Dev` (stored as `"stable"` / `"dev"` in config)

## Key Dependencies (Cargo.toml)

| Crate | Purpose |
|-------|---------|
| `wxdragon` | wxWidgets GUI bindings (with `webview` feature) |
| `anyhow` | Error handling throughout |
| `ureq` | HTTP client (rustls backend) |
| `serde` | JSON deserialization (GitHub API responses) |
| `regex` | Text search |
| `scraper` | HTML parsing (CSS selectors) |
| `roxmltree` | Fast XML parsing |
| `pulldown-cmark` | Markdown parsing |
| `encoding_rs` | Legacy charset detection/conversion |
| `zip` | ZIP archive reading |
| `base64` | Bookmark note encoding |
| `sha1` | Config section names (per-doc hash) |
| `bitflags` | `ParserFlags`, search flags |
| `hayro-syntax`, `hayro-interpret` | PDF text extraction |
| `libchm` | CHM decompilation |
| `live-region` | Screen reader live region announcements |
| `windows` | Win32 API (accessibility, COM) |

Release profile: LTO, strip, opt-level `z`, single codegen unit, panic=abort.

## Common Gotchas

- **Positions are display-length units**, not byte offsets — use `display_len()` for CJK correctness
- **Search positions are UTF-16 units** (reader_core.rs) — different from display_len
- **EPUB/CHM link handling** uses `add_converter_markers_excluding_links` + custom loops — don't switch to `add_converter_markers` for these formats
- **Config sections** for per-doc state use SHA1 of file path, not the path directly
- **All display strings go through `t()`** — no exceptions, even for error messages shown to users
- **Dialogs need `set_sizer_and_fit` + `centre()`** before `show_modal()`
- **`main_window_parent()`** returns `Option<Frame>` — always handle the `None` case
- **Windows-only code** gated with `#[cfg(target_os = "windows")]` — keep it that way
- **Bookmark notes** are base64-encoded in config storage
- **`compact_go_menu`** controls whether heading-level submenus appear in the Go menu
