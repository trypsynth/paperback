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
* Seemless and light-weight installer that automatically sets up file associations for you.
* Translated into numerous different languages for the widest possible user support.

## Building

To build, you'll need cargo, as well as CMake and Ninja for building wxDragon.

On Linux, you will also need clang/libclang (for bindgen), GTK3, WebKitGTK, Expat, and TIFF development packages (for example `clang`, `libclang-dev`, `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libexpat1-dev`, and `libtiff-dev` on Debian/Ubuntu).

```bash
cargo build --release
```

to generate the binary in the release folder, and

```bash
cargo release
```

`cargo release` produces platform-specific archives:

* Windows: `paperback_windows.zip` and `paperback_setup.exe`
* Linux: `paperback_linux.zip`
* macOS: `paperback_mac.zip`

### Optional tools:

The following tools aren't required to build a functioning Paperback on a basic level, but will help you make a complete release build.

* `pandoc` on your `PATH` to generate the HTML readme.
* `gettext` tools (`xgettext`, `msgfmt`, `msgmerge`) on your `PATH` to generate the translation template and compile translations.
* InnoSetup installed to create the installer.

## Contributing

Contributions are welcome! Whether through issues, pull requests, discussions, or other means, your interest is most certainly appreciated. Thanks for using Paperback!

## License

This project is licensed under the [MIT license](LICENSE.md).
