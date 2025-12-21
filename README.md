# Paperback

[Paperback](https://paperback.dev) is a lightweight, fast, and accessible ebook and document reader designed for everyone, from casual readers to heavy power users. It's designed for screen reader accessibility, fast speeds, and a bloat-free experience.

## Features

* Written entirely in native programming languages and optimized for speed.
* Supports many common document formats, including but not limited to HTML, epub, CHM, PDF, DOCX, PPTX, and txt.
* Intuitive tabbed interface for managing multiple documents.
* Full screen reader accessibility.
* Robust find functionality for quick document searches.
* Seamless navigation between EPUB sections, headings, pages, links, lists, and more via hotkeys similar to what you find in a screen reader.
* Precise navigation to specific lines or percentages within documents.
* Seemless and light-weight installer that automatically sets up file associations for you.
* Translated into various different languages for the widest possible user support.

## Building

We use VCPKG for managing dependencies. Currently we manage our own VCPKG installation through a submodule. As such, make sure to clone Paperback recursively:

```batch
git clone --recursive https://github.com/trypsynth/paperback
```

If you've already cloned and forgot the --recursive flag, run the following in your paperback directory before trying to build:

```batch
git submodule update --init
```

You'll also need CMake and Ninja installed alongside a functional Visual Studio 2022 installation, in addition to a functional installation of Rust and Cargo. Once you have everything necessary, you can compile the project. It's recommended to build from an x64 Visual Studio Developer Command Prompt to ensure all paths are correctly configured.

```batch
mkdir build
cd build
cmake .. -GNinja
cmake --build .
```

This will generate paperback.exe and all its dependencies, including its readme in HTML format, in your build folder.

Optional tools:

* `pandoc` on your `PATH` to generate the HTML readme during the build.
* `gettext` tools (`xgettext`, `msgfmt`, `msgmerge`) on your `PATH` to generate the translation template and compile translations.
* InnoSetup installed to create the installer with the `release` target.

## Contributing

Contributions are welcome! Whether through issues, pull requests, discussions, or other means, your interest is most certainly appreciated.

## License

This project is licensed under the [MIT license](LICENSE.md).
