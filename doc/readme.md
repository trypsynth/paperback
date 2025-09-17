# Paperback - version 0.21
## Introduction
Paperback is a lightweight, fast, and accessible ebook/document reader designed to make reading fun and seamless, regardless of the file format being used or the user's reading preferences.

## System Requirements
Paperback currently runs on Windows 7 through Windows 11. It's possible it runs on earlier versions of Windows too and/or can be built in such a way that it will, but this hasn't been tested yet. Support for other platforms is planned.

## Features:
* Incredibly fast and standalone.
* Simple tabbed interface, allowing you to open as many documents as you want side-by-side.
* Saves your cursor position across every document you open.
* Optionally remembers what documents you had open when you closed the program, and restores them on next launch.
* Designed with screen reader users in mind.
* Ability to navigate through epub sections, as well as HTML headings and pages in a PDF.
* Robust find dialog, including features such as history and regular expression support.
* Intuitive go-to dialog, allowing you to jump to a particular line or percent in your document.
* Usable from the Windows open with dialog.

## Currently supported filetypes:
Paperback supports several common file formats, with more planned for future releases.

* Epub 2/3 books.
* HTML documents.
* Markdown documents.
* PDF documents.
* Text documents.

## Hotkeys
Paperback's user interface was designed specifically with keyboard and screen reader users in mind. As such, every action has an associated hotkey. Below, you'll find a list of all of them, grouped by menu structure.

### File menu
* Ctrl+O: open a document.
* Ctrl+F4: close the currently focused document.
* Ctrl+Shift+F4: close all currently opened documents.
* Ctrl+E: export the currently focused document to plain text.

### Go menu
* Ctrl+F: show the find dialog.
* F3: find next.
* Shift+F3: find previous.
* Ctrl+G: bring up the go-to dialog, allowing you to go to either a line number or a percentage.
* Ctrl+P: brings up a dialog allowing you to specify a page number to jump to, if the document supports pages.
* Left bracket: go to the previous section.
* Right bracket: go to the next section.
* Shift+H: go to the previous heading.
* H: Go to the next heading.
* Shift+1: go to the previous heading level 1.
* 1: go to the next heading level 1.
* Shift+2: go to the previous heading level 2.
* 2: go to the next heading level 2.
* Shift+3: go to the previous heading level 3.
* 3: go to the next heading level 3.
* Shift+4: go to the previous heading level 4.
* 4: go to the next heading level 4.
* Shift+5: go to the previous heading level 5.
* 5: go to the next heading level 5.
* Shift+6: go to the previous heading level 6.
* 6: go to the next heading level 6.
* P: go to the next page.
* Shift+P: go to the previous page.

### Tools menu
* Ctrl+W: view the word count of the currently focused document.
* Ctrl+I: bring up the document info dialog.
* Ctrl+T: show the table of contents.
* Ctrl+Comma: bring up the options dialog.

### About menu
* Ctrl+F1: show the about dialog.
* F1: pop up this readme file in your default web browser.
* Shift+F1: pop up this readme file in Paperback itself.

## Changelog
### Version 0.3
* Fixed the table of contents in epub books with URL-encoded manifests. [#34](https://github.com/trypsynth/paperback/issues/34).
* Fixed heading navigation in HTML documents containing multi-byte Unicode characters. [#42](https://github.com/trypsynth/paperback/issues/42), [#59](https://github.com/trypsynth/paperback/issues/59), [#61](https://github.com/trypsynth/paperback/issues/61).
* Fixed high CPU usage in documents with long titles due to a regression in wxWidgets. [#60](https://github.com/trypsynth/paperback/issues/60).
* Fixed loading UTF-8 text files.
* Fixed nested TOC items in EPub books putting your cursor at the wrong position.
* Fixed a crash on application exit in certain cases. [#45](https://github.com/trypsynth/paperback/issues/45).
* Added a checkbox in the options dialog to enable or disable word wrap!
* Markdown documents will now always have a title, and Paperback should now be able to load virtually any Markdown file. [#52](https://github.com/trypsynth/paperback/issues/52).
* PDF documents will now always have a title, even if the metadata is missing. [#56](https://github.com/trypsynth/paperback/issues/56).
* Switched PDF libraries to the one used in Chromium, leading to far more reliable PDF parsing across the board. [#41](https://github.com/trypsynth/paperback/issues/41).
* You can now only have one instance of Paperback running at a time. Running paperback.exe with a filename while it's already running will open that document in the already running instance.

### Version 0.21
* Added the total number of pages to the page label in the go to page dialog. [#46](https://github.com/trypsynth/paperback/issues/46).
* Allow tabbing from the document content to your list of opened documents. [#19](https://github.com/trypsynth/paperback/issues/19).
* Fixed the heading keystrokes sometimes opening recent documents if you had enough of them. [#47](https://github.com/trypsynth/paperback/issues/47).
* Paperback will now remove unnecessary soft hyphens from text output.
* Fixed heading navigation sometimes putting you on the wrong character.

### Version 0.2
* Added markdown document support! [#22](https://github.com/trypsynth/paperback/issues/22).
* Added PDF document support, including the ability to navigate between pages! [#12](https://github.com/trypsynth/paperback/issues/12), [#37](https://github.com/trypsynth/paperback/issues/37).
* Added keystrokes for navigating by headings in HTML content, including epub books and markdown documents. These keystrokes were designed to work similar to a screen reader. [#3](https://github.com/trypsynth/paperback/issues/3).
* Fixed loading epubs with URL-encoded filenames in their manifests. [#20](https://github.com/trypsynth/paperback/issues/20).
* Fixed loading epub 3 books with XHTML embedded inside of them. [#35](https://github.com/trypsynth/paperback/issues/35).
* A message is now spoken if the document doesn't support a table of contents or sections, as opposed to the menu items being disabled. [#39](https://github.com/trypsynth/paperback/issues/39).
* Added a recent documents menu! It currently stores your last 10 opened documents, and pressing enter on one will open it for reading. [#32](https://github.com/trypsynth/paperback/issues/32).
* Completely rewrote the Find dialog, making it much simpler to use, while also adding a history of your last 25 searches and regular expression support! [#21](https://github.com/trypsynth/paperback/issues/21).
* Previously opened documents are now remembered across application restarts. This is configurable through the new options item in the tools menu. [#18](https://github.com/trypsynth/paperback/issues/18).
* Added shift+f1 to open the readme directly in Paperback itself.

### Version 0.1
* Initial release.
