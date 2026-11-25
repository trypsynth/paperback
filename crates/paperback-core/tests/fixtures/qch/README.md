# Qt help test files

These are Qt's own test files for its help system, copied unchanged from [qttools](https://github.com/qt/qttools) at commit [`e4b6ac9f2e932c5dde510fadcdc8c0d54762dac9`](https://github.com/qt/qttools/tree/e4b6ac9f2e932c5dde510fadcdc8c0d54762dac9). They were made by real Qt releases, including Qt 3.3.8 and 4.3.0, whose file layouts no current Qt writes, so they test what hand-built test files can only imitate.

| File | From |
|---|---|
| `test.qch` | `tests/auto/qhelpcontentmodel/data/test.qch` |
| `qmake-3.3.8.qch` | `tests/auto/qhelpcontentmodel/data/qmake-3.3.8.qch` |
| `qmake-4.3.0.qch` | `tests/auto/qhelpcontentmodel/data/qmake-4.3.0.qch` |
| `linguist-3.3.8.qch` | `tests/auto/qhelpenginecore/data/linguist-3.3.8.qch` |
| `qtqml.qch` | `tests/auto/helpengineplugin/data/qtqml.qch` |

Copyright (C) The Qt Company Ltd. Licensed under the GNU General Public License version 3 only (`GPL-3.0-only`, in [GPL-3.0-only.txt](GPL-3.0-only.txt)), or the Qt commercial license, as qttools' `REUSE.toml` declares for its test data.

These files are under the GPL, not Paperback's MIT license. They are test data that Paperback's tests read; they are not part of Paperback's code or of any build of it. Qt publishes them only in this form, and qttools at the commit above is where to get them from Qt.
