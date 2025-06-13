# Paperback
Paperback is a lightweight, fast, and accessible ebook and document reader. It’s written in C++ using wxWidgets to keep things cross-platform, screen reader-friendly, and totally free of bloat.

## Features
* Blazing fast and built entirely with native code.
* Supports a wide range of document formats, with more being added all the time.
* Simple tabbed interface; open as many documents as you want.
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

## License
This project is licensed under the [MIT license](LICENSE).
