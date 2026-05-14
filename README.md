# Paperback

[Paperback](https://paperback.dev) is a lightweight, fast, and accessible ebook and document reader for everyone, from casual readers to heavy power users. Designed for screen reader accessibility and a blazing fast bloat-free reading experience.

## Features

- Written entirely in Rust to ensure it's modern, fast, and memory-safe
- Supports many common document formats: HTML, EPUB, FB2, CHM, PDF, DOCX, PPTX, DOC, PPT, ODT, RTF, Markdown, and plain text
- Intuitive tabbed interface for managing multiple documents
- Full screen reader accessibility
- Robust find functionality for quick document searches
- Seamless navigation between EPUB sections, headings, pages, links, lists, images, figures, and tables via hotkeys similar to screen reader conventions
- Precise navigation to specific lines or percentages within documents
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

### Nix

Run directly:
```bash
nix run github:trypsynth/paperback
```

Install to profile:
```bash
nix profile install github:trypsynth/paperback
```

Build from source:
```bash
nix run    # build and run
nix build  # build only
```

### Flatpak

```bash
flatpak-builder --force-clean --repo=repo-flatpak build io.github.trypsynth.Paperback.yaml
flatpak build-bundle repo-flatpak paperback.flatpak io.github.trypsynth.Paperback
flatpak --user install paperback.flatpak
```

## Contributing

Contributions are welcome! Whether through issues, pull requests, or discussions, your interest is appreciated. Thanks for using Paperback!

## License

This project is licensed under the [MIT license](LICENSE.md).
