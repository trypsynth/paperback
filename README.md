# Paperback
Paperback is a lightweight, fast, and accessible ebook and document reader. It’s written in C++ using wxWidgets to keep things cross-platform, screen reader-friendly, and totally free of bloat.

## Features
* Blazingly fast and built entirely with native code and no runtime dependencies.
* Supports a wide range of document formats, with more being added all the time.
* Simple tabbed interface, allowing you to open as many documents as you want.
* Fully accessible with screen readers.
* Full-featured find dialog for quickly searching your documents.
* Jump between EPUB sections with a single hotkey.
* Go to a specific line or percentage through your document with an simple dialog.
* Supports passing a file on the command line, great for "Open With".

## Building
It’s recommended to build from an x64 Visual Studio Developer Command Prompt so all paths are set up correctly. At least on my machine, building from a standard Windows command prompt tries to invoke `ld` for linking for some wacky reason.

```sh
mkdir build
cd build
cmake .. -GNinja
cmake --build . --config Release
```

This will give you a single `paperback.exe` binary—completely standalone. No dependencies, unless you want speech output, in which case `nvdaControllerClient64.dll` is optional.

## Contributing
Contributing is most certainly welcome! Whether it's in the form of an issue, puill request, discussion, or something else, I'll gladly take it! Thanks for your interest!

If you're modifying the code, I ask that you generally adhere to my coding style (tabs for indents, stars attached to the type, not the variable name, braces on the same line, etc.). There's a .clang-format file included in the repository to help with this, although it's worth noting that it currently doesn't run as a pre-commit hook, so must be done manually.

## License
This project is licensed under the [MIT license](LICENSE).
