# Paperback

[Paperback](https://paperback.dev) is a lightweight, fast, and accessible ebook and document reader for everyone, from casual readers to heavy power users. It's designed for screen reader accessibility, fast speeds, and a bloat-free experience.

## Features

* Written entirely in Rust, a modern, fast, and safe systems programming language
* Every aspect is optimized for speed.
* Supports many common document formats, including but not limited to HTML, epub, CHM, PDF, DOCX, PPTX, ODT, and txt.
* Intuitive tabbed interface for managing multiple documents.
* Full screen reader accessibility.
* Robust find functionality for quick document searches.
* Seamless navigation between EPUB sections, headings, pages, links, lists, tables, and more via hotkeys similar to what you find in a screen reader.
* Precise navigation to specific lines or percentages within documents.
* Seamless and light-weight installer that automatically sets up file associations for you.
* Translated into numerous different languages for the widest possible user support.

## Building

To build, you'll need cargo, as well as CMake and Ninja for building wxDragon.

```batch
cargo build --release
```

to generate the binary in the release folder, and

```batch
cargo release
```

### Optional tools:

The following tools aren't required to build a functioning Paperback on a basic level, but will help you make a complete release build.

* `pandoc` on your `PATH` to generate the HTML readme.
* `gettext` tools (`xgettext`, `msgfmt`, `msgmerge`) on your `PATH` to generate the translation template and compile translations.
* InnoSetup installed to create the installer.

### Linux

Building on Linux requires wxWidgets 3.2+ with GTK3 backend. The wxDragon build system will handle compiling the wxWidgets bindings.

```bash
cargo build --release
```

Optional tools:
- `pandoc` for HTML readme generation
- `gettext` tools for translations

### Linux (Nix)

**Run directly:**
```bash
nix run github:trypsynth/paperback
```

**Install to profile:**
```bash
nix profile install github:trypsynth/paperback
```

**Build from source:**
```bash
# Clone repository
git clone https://github.com/trypsynth/paperback
cd paperback

# Build and run
nix run

# Or build without running:
nix build

# Build Flatpak:
flatpak-builder --force-clean --repo=repo build io.github.trypsynth.Paperback.yaml
```

## Contributing

Contributions are welcome! Whether through issues, pull requests, discussions, or other means, your interest is most certainly appreciated. Thanks for using Paperback!

## License

This project is licensed under the [MIT license](LICENSE.md).
