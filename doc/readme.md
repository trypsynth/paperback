# Paperback - version 0.1
## Introduction
Paperback is a lightweight, fast, and accessible ebook/document reader designed to make reading fun and seamless, regardless of the file format being used or the user's reading preferences.

## System Requirements
Paperback currently runs on Windows 10/11. It's possible it runs on earlier versions of Windows too and/or can be built in such a way that it will, but this hasn't been tested yet. macOS support is planned for a future version.

## Features:
* Incredibly fast and standalone.
* Simple tabbed interface, allowing you to open as many documents as you want at once.
* Saves your cursor position across every document you open.
* Designed with screen reader users in mind.
* Fast and efficient text finding throughout your entire document.
* Ability to navigate through epub sections, with other navigation units planned.
* Usable with the Windows open with dialog.

## Currently supported filetypes:
Paperback supports several common file formats, with more planned for future releases.

* Epub 2/3 books.
* HTML documents.
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
* Left bracket: go to the previous epub section.
* Right bracket: go to the next epub section.

### Tools menu
* Ctrl+W: view the word count of the currently focused document.
* Ctrl+I: bring up the document info dialog.
* Ctrl+T: show the table of contents.

### About menu
* Ctrl+F1: show the about dialog.
* F1: pop up this readme file in your default web browser.

## Changelog
### Version 0.2
* Added markdown document support!
* Fixed loading epubs with URL-encoded filenames in their manifests.

### Version 0.1
* Initial release.
