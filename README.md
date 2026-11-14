# Paperback

[Paperback](https://paperback.dev) is an accessible reader for ebooks, documents, and audiobooks, designed for screen reader users first. It runs on Windows, macOS, Linux, iOS, and Android, with all five apps sharing one reading engine written in Rust.

This file is for people who work on Paperback. To use it, see the [user guide](doc/readme.md) or download it from [paperback.dev](https://paperback.dev). To contribute, see [CONTRIBUTING.md](CONTRIBUTING.md).

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
- On Linux, the GTK 3 and WebKitGTK development packages.

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

To build the iOS app, you need a Mac with Xcode. To build it and start it in the Simulator, run:

```
cargo ios run
```

This builds the reading engine for devices and the Simulator, generates the Swift bindings and translations that the Xcode project uses, then builds the app and starts it. It uses the booted simulator, or the newest iPhone simulator if none is booted. To pick one, add `--simulator` with its name or UDID, like `--simulator "iPhone 17"`. To keep printing the app's output until it quits, add `--console`.

`cargo ios` (with no `run`) builds the Rust Parts. Afterwards, you can also open `ios/Paperback.xcodeproj` and build in Xcode.

Simulators aren't very accessible with VO; this is more for agents testing their work; humans probably want to run on a device instead.

### Run on your iPhone or iPad

A free Apple account is enough; you don't need to join the Apple Developer Program. Once per Mac, sign in to Xcode under Xcode > Settings > Accounts. Once per device, connect it with a cable, unlock it and tap Trust, then turn on Developer Mode under Settings > Privacy & Security on the device. Then run:

```
cargo ios run --device
```

The first run sets up signing. It reads the team that is signed into Xcode, creates a bundle ID, and saves both in `~/.config/paperback-dev/ios-signing.xcconfig`. Every Paperback checkout uses that file to avoid running out of limits on free accounts when parallel checkouts and agent sessions are in use. Xcode then creates your certificate and provisioning profile and registers the device. If you're in several teams,  pick one with `--team`, the command will list them if the flag is required and missing. If several devices are paired, pick one with `--device` and its name. If the build says "No Accounts", sign in again under Xcode > Settings > Accounts. The saved login can expire while the account is still listed in Xcode, which is when this message appears.

When an app signed with a free account is started for the first time, iOS asks you to trust its developer. To do so, open Settings > General > VPN & Device Management on the device and trust your certificate. These apps stop opening after 7 days; run the command again to reinstall.

Don't set a team or bundle ID in the Xcode project, including through its Signing & Capabilities tab. A value there would override every other contributor's settings, so the build fails when it finds one.

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

Paperback uses gettext catalogs through [patois](https://github.com/accessware/patois). Strings marked with `t()` in Rust, Swift, and Kotlin are collected into `po/paperback.pot`. To update the template, run the following command:

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
