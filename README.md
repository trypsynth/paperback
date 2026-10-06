# Paperback

[Paperback](https://paperback.dev) is an accessible reader for ebooks, documents, and audiobooks, designed for screen reader users first. It runs on Windows, macOS, Linux, iOS, and Android, and all five apps share one reading engine written in Rust.

This file is for people who work on Paperback. To use it, see the [user guide](doc/readme.md) or download it from [paperback.dev](https://paperback.dev). To contribute, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Features

- Opens EPUB, PDF, Word, PowerPoint, OpenDocument, RTF, HTML, Markdown, reStructuredText, FictionBook, MOBI and Kindle, CHM, WinHelp, DAISY, comic book archives, man pages, RSS and Atom feeds, Windows Write, plain text, and M4B and MP3 audiobooks. For the full list of file extensions, see the [user guide](doc/readme.md#currently-supported-file-types).
- Opens documents in tabs. Files opened from the shell or through a file association go to the window that's already running.
- Moves by heading, page, link, list, table, image, figure, formula, and more with single-letter keys, like a screen reader's browse mode. A table of contents and an elements list are also available.
- Finds text, with options for case, whole words, and regular expressions, and remembers past searches.
- Keeps bookmarks, notes, and navigation history, and goes to a line, page, or percentage.
- Plays audiobooks and DAISY audio at adjustable speeds, with a sleep timer.
- Reads scanned PDF pages with the OCR engine built into Windows and macOS.
- Reads MathML as AsciiMath through MathCAT.
- Reads untagged PDFs in order, joins their wrapped lines into paragraphs, finds headings by size and weight, and removes running headers and footers. You can turn off each of these.
- Lets you change the font, colors, spacing, alignment, word wrap, and every keyboard shortcut.
- Exports and imports each document's position, bookmarks, and notes as a `.paperback` file.
- Updates itself, from either the stable or the dev channel.
- Includes `pb`, a command-line tool that converts any supported document to plain text, HTML, or Markdown.
- Is translated into 15 languages.

## Repository layout

| Path | Contents |
|---|---|
| `crates/paperback-core` | The reading engine: the parsers, the document model, navigation, and settings. The mobile apps call it through [UniFFI](https://mozilla.github.io/uniffi-rs/). |
| `crates/paperback` | The desktop app, built on wxWidgets through [wxDragon](https://github.com/AllenDang/wxDragon). |
| `crates/paperback-formats` | Format names and file extensions, shared by the parsers and the packaging code. |
| `crates/pb` | The `pb` command-line converter. |
| `crates/xtask` | Build, release, and translation tasks. |
| `ios/` | The iOS app, written in SwiftUI. |
| `android/` | The Android app, written in Kotlin with Jetpack Compose. |
| `po/` | The translation template and a catalog for each language. |
| `doc/` | The user guide and its translations, which each app shows as its Help. |

## Build the desktop app

### Requirements

To build the desktop app, you need the following:

- Rust 1.91.1 or later, from [rustup](https://rustup.rs).
- The nightly Rust toolchain, which formats the code. To install it, run `rustup toolchain install nightly`. CI checks formatting with the nightly that `.github/workflows/ci.yml` names.
- CMake and Ninja, which compile wxWidgets.
- gettext, for `msgfmt`, which compiles the translations. Without it, the app builds but is English only.
- On Windows, the [WebView2 SDK](https://www.nuget.org/packages/Microsoft.Web.WebView2) on the linker's `LIB` path.
- On Linux, the GTK 3 and WebKitGTK development packages. On Debian or Ubuntu, install them with this command:

  ```
  sudo apt install cmake ninja-build gettext libgtk-3-dev libwebkit2gtk-4.1-dev libpng-dev libjpeg-dev libtiff-dev libgl1-mesa-dev libglu1-mesa-dev libxkbcommon-dev libwayland-dev libexpat1-dev libxtst-dev libsm-dev libice-dev libasound2-dev
  ```

### Fetch wxWidgets

Paperback builds against a pinned commit of wxWidgets master instead of a release, because it relies on accessibility fixes that haven't shipped in a release yet. `WXWIDGETS_DIR` in `.cargo/config.toml` names the commit. Before your first build, and whenever the pinned commit changes, fetch it:

```
cargo xtask wxwidgets
```

### Build and run

To build the app, run the following command:

```
cargo build --release
```

The app is `target/release/paperback`. To keep your own settings safe while you test, set `PAPERBACK_CONFIG_DIR` to another directory, and the app keeps its configuration there instead.

To build the packages a release ships, such as the Windows installer, the macOS disk image, and the Linux AppImage, run the following command:

```
cargo release
```

The Windows installer needs [Inno Setup](https://jrsoftware.org/isinfo.php). The Linux AppImage needs `appimagetool` on your `PATH`; without it, `cargo release` builds only the tarball.

### Run the tests

To run the tests, run the following command:

```
cargo test --workspace
```

On Windows, `crates/paperback/tests` also contains UI tests. They start the app, press keys, and check what UI Automation reports, including what screen readers are told. These tests are skipped by default. To run them, close Paperback, and then run the following command:

```
cargo test -p paperback -- --ignored
```

The UI tests take over the keyboard and focus, so don't use your computer until they finish.

### Use pb

`pb` converts a document to plain text, HTML, or Markdown. For example:

```
cargo run --release -p pb -- book.epub -f markdown -o book.md
```

`pb` can also convert only some pages, keep repeated page headers and footers, and read scanned PDF pages with OCR. For all of its options, run `pb --help`.

## Build the iOS app

To build the iOS app, you need a Mac with Xcode, and the iOS Rust targets:

```
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
```

Then build the reading engine:

```
cargo ios
```

`cargo ios` builds the reading engine for devices and the Simulator, and generates the Swift bindings and translations that the Xcode project uses. To build the app, open `ios/Paperback.xcodeproj` in Xcode.

## Build the Android app

To build the Android app, you need the Android SDK and NDK, [cargo-ndk](https://github.com/bbqsrc/cargo-ndk), and the Android Rust targets:

```
rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install cargo-ndk
```

Then build the reading engine and the app:

```
cargo android --debug
```

`cargo android` builds the reading engine and generates the Kotlin bindings and translations. The `--debug` and `--release` options also build the app, and `--install-debug` installs it on a connected device. To work in Android Studio, run `cargo android` once, and then open the `android/` directory.

## Translations

Paperback uses gettext catalogs through [patois](https://github.com/trypsynth/patois). Strings marked with `t()` in Rust, Swift, and Kotlin are collected into `po/paperback.pot`. To update the template, run the following command:

```
cargo gen-pot
```

Each language's strings are in `po/<lang>.po`, and its user guide is in `doc/readme-<lang>.md`. To find out how to translate Paperback, see [CONTRIBUTING.md](CONTRIBUTING.md#translations).

## Pre-commit hooks

Paperback uses [prek](https://github.com/j178/prek) to run the hooks in `prek.toml`. The hooks remove trailing whitespace, end files with a newline, and format the Rust code. To install them, run the following commands:

```
cargo install prek
prek install
```

## License

Paperback is licensed under the [MIT license](LICENSE.md).

The libraries Paperback uses, and their licenses, are credited at [paperback.dev/licenses](https://paperback.dev/licenses).
