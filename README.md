# Paperback
Paperback is a lightweight, fast, and accessible ebook and document reader built with C++ and wxWidgets. It's designed for cross-platform compatibility, screen reader accessibility, and a bloat-free experience.

## Features
* Blazing fast performance with native code and minimal runtime dependencies.
* Extensive document format support, with ongoing additions.
* Intuitive tabbed interface for managing multiple documents.
* Full screen reader accessibility.
* Robust find functionality for quick document searches.
* Seamless navigation between EPUB sections via hotkeys.
* Precise navigation to specific lines or percentages within documents.
* Command-line file opening for 'Open With' integration.

## Building
It's recommended to build from an x64 Visual Studio Developer Command Prompt to ensure all paths are correctly configured. Building from a standard Windows command prompt may lead to issues with `ld` for linking.

```sh
mkdir build
cd build
cmake .. -GNinja
cmake --build . --config Release
```

This process generates a standalone `paperback.exe` binary with no external dependencies, except for the optional `nvdaControllerClient64.dll` if speech output is desired. Dependencies are managed using vcpkg.

## Contributing
Contributions are welcome! Whether through issues, pull requests, discussions, or other means, your interest is appreciated.

When modifying the code, please adhere to the established coding style (tabs for indents, stars attached to the type, not the variable name, braces on the same line, etc.). A `.clang-format` file is included to assist with this, though it currently requires manual execution as it's not a pre-commit hook.

## License
This project is licensed under the [MIT license](LICENSE).
