# Paperback

[Paperback](https://paperback.dev) is a lightweight, fast, and accessible ebook and document reader for everyone, from casual readers to heavy power users. Designed for screen reader accessibility and a blazing fast bloat-free reading experience.

## Features

- Written entirely in Rust to ensure it's modern, fast, and memory-safe
- Supports a wide range of document formats:
  - HTML documents (htm/html/xhtml)
  - EPUB books (epub)
  - FictionBook ebooks (fb2)
  - CHM help files (chm)
  - DAISY books (opf/zip)
  - PDF documents (pdf)
  - Word documents (doc/docx/docm)
  - PowerPoint presentations (ppt/pptx/pptm)
  - OpenDocument text files (odt/fodt)
  - OpenDocument presentations (odp/fodp)
  - RTF documents (rtf)
  - MOBI/Kindle books (mobi/azw/azw3)
  - Markdown documents (md/markdown/mdx/mdown/mdwn/mkd/mkdn/mkdown/ronn)
  - Plain text and log files (txt/log)
- Intuitive tabbed interface for managing multiple documents, with single-instance behavior so opening a file from the shell or a file association reuses the running window
- Full screen reader accessibility, including live-region status announcements for actions like search results and navigation
- Robust find functionality with match case, whole word, and regular expression options, plus persisted search history
- Seamless navigation between EPUB sections, headings (per level), pages, links, lists, list items, images, figures, tables, and separators via hotkeys similar to screen reader conventions, with a table of contents and elements list for quick jumps
- Precise navigation to specific lines or percentages within documents, plus per-document navigation history (back/forward)
- Bookmarks and notes, with optional sound feedback and a dedicated dialog to jump to any of them
- Extensive readability customization: custom fonts and colors, line/paragraph/letter spacing, text alignment, word wrap, and inline vs. placeholder table rendering
- Password-protected document support, and per-document settings (position, bookmarks, format overrides) that can be exported/imported via `.paperback` files
- Recently closed and recently opened document tracking, with quick reopen
- Built-in auto-update checker with stable/dev channels
- System tray support on Windows and Linux
- A `pb` CLI tool for scripted conversion of documents to plain text, HTML, or Markdown
- Lightweight installer that automatically sets up file associations
- Translated into numerous languages

## Workspace layout

This is a Cargo workspace. The main crates are:

| Crate | Description |
|---|---|
| `paperback-core` | Core document parsing and reading logic (library) |
| `paperback` | The GUI application (wxWidgets via wxDragon) |
| `pb` | CLI tool to convert documents to text or HTML |
| `xtask` | Build and release automation (`cargo release`) |

## Requirements

- Rust 1.87+ (edition 2024). Install via [rustup](https://rustup.rs).
- Rust nightly toolchain, used by the `cargo fmt` pre-commit hook. Install with:
  ```
  rustup toolchain install nightly
  ```
- CMake and Ninja, required to compile wxWidgets via wxDragon.

### Optional tools

These are not needed for a basic build but are required for a complete release:

- `pandoc` on `PATH`: generates the HTML readme
- gettext tools (`xgettext`, `msgfmt`, `msgmerge`) on `PATH`: generates the translation template and compiles translations
- InnoSetup: creates the Windows installer

## Building

```
cargo build --release
```

This produces the binary in `target/release/`. To build a full release package (zip, translations, etc.):

```
cargo release
```

This runs the `xtask` crate via the `cargo release` alias defined in `.cargo/config.toml`.

## Pre-commit hooks

This project uses [prek](https://github.com/LorenzoLeonardini/prek), a Rust-based pre-commit hook runner. Hooks are configured in `prek.toml`.

Install prek and set up the hooks:

```
cargo install prek
prek install
```

The following hooks run on every commit:

- `trailing-whitespace`: strips trailing whitespace
- `end-of-file-fixer`: ensures files end with a newline
- `cargo fmt` (nightly): formats all Rust code with `cargo +nightly fmt --all`

## Linux

Building on Linux requires wxWidgets 3.2+ with the GTK3 backend. The wxDragon build system handles compiling the wxWidgets bindings automatically.

### Flatpak

```bash
flatpak-builder --force-clean --repo=repo-flatpak build dev.paperback.desktop.yaml
flatpak build-bundle repo-flatpak paperback.flatpak dev.paperback.desktop
flatpak --user install paperback.flatpak
```

## Contributing

Contributions are welcome! Whether through issues, pull requests, or discussions, your interest is appreciated. Thanks for using Paperback!

## License

This project is licensed under the [MIT license](LICENSE.md).
