# Paperback
Paperback is a light-weight, fast, and accessible ebook and document reader. It is written in C++ with wxWidgets for great cross-platform accessibility and to avoid bloat.

## Features:
* Extremely fast and build with completely native code.
* Supports many different document formats, with more being added all the time.
* Simple tabbed interface allowing you to open many documents at once.
* Fully accessible with screen readers.
* Fully-featured find dialog, allowing you to quickly search your documents.
* Ability to easily jump between epub sections with a single hotkey.
* Allows you to go to a line or a percentage through your document with a simple dialog.
* Allows passing a file on the command line for easy use with open with.

## Building:
Building from an x64 Visual Studio Developer command prompt is recommended to set up the paths correctly.

```
mkdir build
cd build
cmake .. -GNinja
cmake --build . --config Release
```

This will produce a single binary called paperback.exe. It is entirely standalone and needs no dependencies other than optionally nvdaControllerClient64.dll for speech output.

## License
This project is licensed under the [MIT license](LICENSE).
