# Paperback
Paperback is a light-weight, fast, and accessible ebook and document reader. It is written in C++ with wxWidgets for great cross-platform accessibility and to avoid bloat.

## Building
From an x64 Visual Studio Developer command prompt:

```
mkdir build
cd build
cmake ..
cmake --build . --config Release
```

This will produce a single binary called paperback.exe. It is entirely standalone and needs no dependencies.

## License
This project is licensed under the MIT license. See the [LICENSE file](LICENSE) for more details.
