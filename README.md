# Paperback
Paperback is a lightweight, fast, and accessible ebook and document reader built with C++ and wxWidgets. It's designed for cross-platform compatibility, screen reader accessibility, and a bloat-free experience.

## Features
* Written entirely in C++ and optimized for speed.
* Supports many common document formats, including but not limited to HTML, epub, PDF, and txt.
* Intuitive tabbed interface for managing multiple documents.
* Full screen reader accessibility.
* Robust find functionality for quick document searches.
* Seamless navigation between EPUB sections, headings, and pages via hotkeys.
* Precise navigation to specific lines or percentages within documents.
* Command-line file opening for 'Open With' integration.

## Building

### Windows (VCPKG)

We use VCPKG for managing dependencies. Currently we manage our own VCPKG installation through a submodule. As such, make sure to clone Paperback recursively:

```batch
git clone --recursive https://github.com/trypsynth/paperback
```

If you've already cloned and forgot the --recursive flag, run the following in your paperback directory before trying to build:

```batch
git submodule update --init
```

You'll also need CMake and Ninja installed alongside a functional Visual Studio 2022 installation. Once you have everything necessary, you can compile the project. It's recommended to build from an x64 Visual Studio Developer Command Prompt to ensure all paths are correctly configured.

```batch
mkdir build
cd build
cmake .. -GNinja
cmake --build .
```

This will generate paperback.exe and all its dependencieds, including its readme in HTML format, in your build folder.

Optional tools:
- `pandoc` on your `PATH` to generate the HTML readme during the build.
- `gettext` tools (`xgettext`, `msgfmt`, `msgmerge`) on your `PATH` to generate the translation template and compile translations.

### Linux

For building with CMake, you'll need CMake 3.21+, a C++20 compiler, and dependencies:
- wxWidgets 3.2+
- chmlib, lexbor, mbedtls, pdfium, pugixml, nlohmann-json

```bash
cmake -B build -DUSE_SYSTEM_LIBS=ON -DCMAKE_BUILD_TYPE=Release
cmake --build build
sudo cmake --install build
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
git clone --recursive https://github.com/trypsynth/paperback
cd paperback

# Build and run
nix run .#paperback

# Or build specific outputs:
nix build .#paperback  # Nix derivation

# Build Flatpak:
flatpak-builder --force-clean --repo=repo build io.github.trypsynth.Paperback.yaml
```

## Contributing
Contributions are welcome! Whether through issues, pull requests, discussions, or other means, your interest is appreciated.

When modifying the code, please adhere to the established coding style (tabs for indents, stars attached to the type, not the variable name, braces on the same line, etc.). A `.clang-format` file is included to assist with this. To format the code on Windows, simply run the fmt.bat script in the root of the repository.

## License
This project is licensed under the [MIT license](LICENSE.md).
