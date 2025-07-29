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

This will generate a paperback.exe binary in your build directory, as well as all of the screen reader DLLs needed for fully functional speech output.
## Contributing
Contributions are welcome! Whether through issues, pull requests, discussions, or other means, your interest is appreciated.

When modifying the code, please adhere to the established coding style (tabs for indents, stars attached to the type, not the variable name, braces on the same line, etc.). A `.clang-format` file is included to assist with this. To format the code on Windows, simply run the fmt.bat script in the root of the repository.

## License
This project is licensed under the [MIT license](LICENSE.md).
