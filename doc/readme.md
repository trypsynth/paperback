# Paperback - version 0.21
## Introduction
Paperback is a lightweight, fast, and accessible ebook/document reader designed to make reading fun and seamless, regardless of the file format being used or the user's reading preferences.

## System Requirements
Paperback currently runs on Windows 10/11. It's possible it runs on earlier versions of Windows too and/or can be built in such a way that it will, but this hasn't been tested yet. macOS support is planned for a future version.

## Features:
* Incredibly fast and standalone.
* Simple tabbed interface, allowing you to open as many documents as you want at once.
* Saves your cursor position across every document you open.
* Optionally remembers what documents you had open when you closed the program, and restores them on next launch.
* Designed with screen reader users in mind.
* Ability to navigate through epub sections, as well as pages in a PDF. More navigation units, such as HTML headings, are planned.
* Includes a robust find dialog, including features like history and regular expression support.
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
* Ctrl+G: bring up the go to dialog, allowing you to go to either a line number or a percentage.
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
