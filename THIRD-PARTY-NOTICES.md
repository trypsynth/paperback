# Third-party notices

Paperback itself is under the [MIT license](LICENSE.md). It is built on other people's work, and
this file records what that work is and what it is licensed under. Nothing Paperback ships is
under a copyleft license that would place conditions on the distributed binaries.

## Native libraries

These are not Rust crates. They ship alongside the application as compiled libraries.

| Library | Used for | License |
| --- | --- | --- |
| [PDFium](https://pdfium.googlesource.com/pdfium/) | Reading PDFs, including the tag tree that makes a tagged PDF readable | BSD-3-Clause |
| [wxWidgets](https://www.wxwidgets.org/) | The desktop interface | wxWindows Library Licence (LGPL with a binary-distribution exception) |

The prebuilt PDFium binaries come from
[bblanchon/pdfium-binaries](https://github.com/bblanchon/pdfium-binaries).

## Rust dependencies

The dependency tree is 538 packages. Rather than list every one, here is the breakdown by
license, which is what matters for redistribution:

| License | Packages |
| --- | --- |
| MIT, Apache-2.0, or a choice between them | 469 |
| MPL-2.0 | 28 |
| Unicode-3.0 | 18 |
| BSD (2- and 3-clause), ISC, Zlib, 0BSD, Unlicense, CC0 | 20 |
| Other permissive (bzip2, CDLA-Permissive-2.0) | 2 |

To regenerate that breakdown:

```
cargo metadata --format-version 1 --all-features
```

The MPL-2.0 packages are [`uniffi`](https://github.com/mozilla/uniffi-rs) and its supporting
crates, which generate the mobile bindings, and [`symphonia`](https://github.com/pdeljanov/Symphonia),
which decodes audiobooks. MPL-2.0 is a file-level copyleft: it places conditions on changes to
those crates' own files, not on the application that links them. Paperback does not modify them.
