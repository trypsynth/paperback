# Paperback

[Paperback](https://paperback.dev) is a lightweight, fast, and accessible reader for ebooks, documents, and audiobooks, for everyone from casual readers to heavy power users. It is built for screen readers first, and it is fast and free of bloat.

It runs on Windows, macOS, Linux, iOS and Android. The desktop app is in this repository along with the iOS and Android apps, which share its reading engine.

This file is for people working on Paperback. If you want to use it, the [user guide](doc/readme.md) covers features, supported formats and keyboard shortcuts, and [paperback.dev](https://paperback.dev) has downloads.

## Features

- Opens EPUB, PDF, Word, PowerPoint, OpenDocument, RTF, HTML, Markdown, reStructuredText, FictionBook, MOBI/Kindle, CHM, WinHelp, DAISY, comic book archives, man pages, Windows Write, plain text, and M4B and MP3 audiobooks. The [user guide](doc/readme.md#currently-supported-file-types) has the full list of extensions.
- Tabs, with one running instance that files opened from the shell or a file association go to.
- Navigation by heading, page, link, list, table, image, figure, formula and more, with single-letter keys like a screen reader's browse mode, plus a table of contents and an elements list.
- Find with match case, whole word, regular expressions and history.
- Bookmarks and notes, navigation history, and go to line, page or percentage.
- Audiobooks and DAISY audio with adjustable speed, and a sleep timer.
- OCR for scanned PDF pages, using the engine built into Windows and macOS.
- MathML read as AsciiMath through MathCAT.
- Untagged PDFs read in order, with wrapped lines joined into paragraphs, headings found by size and weight, and running headers and footers removed (each can be turned off).
- Readability settings: fonts, colors, spacing, alignment and word wrap.
- Every keyboard shortcut can be changed.
- Per-document state (position, bookmarks, notes) that can be exported and imported as `.paperback` files.
- An auto-updater with stable and dev channels.
- `pb`, a command line tool that converts any supported document to plain text, HTML or Markdown.
- Translated into 15 languages.

## Repository layout

| Path | What it is |
|---|---|
| `crates/paperback-core` | The reading engine: every parser, the document model, navigation, settings. Shared by all five platforms; the mobile apps reach it through [UniFFI](https://mozilla.github.io/uniffi-rs/). |
| `crates/paperback` | The desktop app, built on wxWidgets through [wxDragon](https://github.com/AllenDang/wxDragon). |
| `crates/paperback-formats` | Format names and extensions, shared by the parsers and the packaging scripts. |
| `crates/pb` | The `pb` command line converter. |
| `crates/xtask` | Build, release, translation and packaging tasks. |
| `ios/` | The iOS app (SwiftUI). |
| `android/` | The Android app (Kotlin and Jetpack Compose). |
| `po/` | Translations: the template and one catalog per language. |
| `doc/` | The user guide and its translations, built into each app's Help. |

## Desktop

### Requirements

- Rust 1.91.1 or later (edition 2024), from [rustup](https://rustup.rs).
- The nightly toolchain, for formatting: `rustup toolchain install nightly`. CI checks formatting with a pinned nightly; `.github/workflows/ci.yml` names it.
- CMake and Ninja, to compile wxWidgets.
- gettext (`msgfmt`) on `PATH`, to compile the translations. Without it the build still succeeds, but the app is English only.
- On Windows, the [WebView2 SDK](https://www.nuget.org/packages/Microsoft.Web.WebView2) on the linker's `LIB` path.
- On Linux, the GTK 3 and WebKitGTK development packages. On Debian and Ubuntu:

  ```
  sudo apt install cmake ninja-build gettext libgtk-3-dev libwebkit2gtk-4.1-dev libpng-dev libjpeg-dev libtiff-dev libgl1-mesa-dev libglu1-mesa-dev libxkbcommon-dev libwayland-dev libexpat1-dev libxtst-dev libsm-dev libice-dev libasound2-dev
  ```

### wxWidgets

Paperback builds against a pinned commit of wxWidgets master rather than its last release, for accessibility fixes that have not shipped in one. The commit is named in `WXWIDGETS_DIR` in `.cargo/config.toml`. Fetch it before the first build, and again whenever the pin moves:

```
cargo xtask wxwidgets
```

### Building and running

```
cargo xtask wxwidgets
cargo build --release
```

The app is `target/release/paperback`. Setting `PAPERBACK_CONFIG_DIR` gives it a separate config directory, which is handy for testing without touching your own settings.

To build everything a release ships (the Windows installer and zip, the macOS disk image, the Linux tarball and AppImage, translations and the help files):

```
cargo release
```

A Windows installer needs [Inno Setup](https://jrsoftware.org/isinfo.php). A Linux AppImage needs `appimagetool` on `PATH`; without it only the tarball is built.

### Tests

```
cargo test --workspace
```

On Windows, `crates/paperback/tests` also holds UI tests that start the real app, press keys, and check what UI Automation reports: focus, the status bar, and what screen readers are told. They are skipped by default and run with:

```
cargo test -p paperback -- --ignored
```

Close Paperback first; the tests refuse to start while it runs. They take over the keyboard and focus, so leave the desktop alone until they finish.

### pb

`pb` converts a document to plain text, HTML or Markdown:

```
cargo run --release -p pb -- book.epub -f markdown -o book.md
```

It can convert only some pages (`--pages 5-10,80-end`), keep repeated page headers and footers (`--keep-repeated`), and read scanned PDF pages with OCR (`--ocr-image-pages`, `--ocr-text-pages`). `pb --help` lists everything.

## iOS

Building needs macOS with Xcode, and the iOS Rust targets:

```
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
cargo ios
```

`cargo ios` builds the reading engine for devices and the Simulator, and generates the Swift bindings and translations the Xcode project expects. Then open `ios/Paperback.xcodeproj` and build. `cargo ios-release` archives and exports a build for App Store Connect.

## Android

Building needs the Android SDK and NDK, [cargo-ndk](https://github.com/bbqsrc/cargo-ndk), and the Android Rust targets:

```
rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install cargo-ndk
cargo android --debug
```

`cargo android` builds the reading engine for both architectures and generates the Kotlin bindings and translations. `--debug` or `--release` then builds the app with Gradle, `--install-debug` installs it on a connected device, and `--build-aab` makes the bundle for Google Play. To work in Android Studio, run `cargo android` once and open `android/`.

## Translations

Paperback uses gettext catalogs through [patois](https://github.com/trypsynth/patois). Strings marked with `t()` in Rust, Swift and Kotlin are collected into `po/paperback.pot`:

```
cargo gen-pot
```

Translations live in `po/<lang>.po`, and the user guide's in `doc/readme-<lang>.md`. A comment starting with `TRANSLATORS:` directly above a string is passed on to translators, so add one wherever a string could be read more than one way.

Missing strings are machine-translated by a GitHub workflow that opens a pull request on every push to master. Languages listed in `po/human-maintained-locales.txt` are left entirely to their translators, and `po/style/<lang>.md` holds a language's own conventions for the machine translation to follow.

## Pre-commit hooks

The repository uses [prek](https://github.com/j178/prek) to run its hooks, configured in `prek.toml`:

```
cargo install prek
prek install
```

They strip trailing whitespace, make sure files end with a newline, and format Rust code with `cargo +nightly fmt --all`.

## Contributing

Issues, pull requests and discussions are all welcome. Please run the tests and the formatter before opening a pull request, and keep comments to explaining why the code does what it does.

## License

Paperback is licensed under the [MIT license](LICENSE.md).

It is built on other people's work. The libraries it uses, and their licenses, are credited at [paperback.dev/licenses](https://paperback.dev/licenses).
