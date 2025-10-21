# Paperback - version 0.5
## Introduction
Paperback is a lightweight, fast, and accessible ebook/document reader designed to make reading fun and seamless, regardless of the file format being used or the user's preferences.

## System Requirements
Paperback currently runs on Windows 7 through Windows 11. It's possible it runs on earlier versions of Windows too and/or can be built in such a way that it will, but this hasn't been tested yet. Support for other platforms is planned for a future version.

## Features
* Incredibly fast and standalone.
* Simple tabbed interface, allowing you to open as many documents as you want side-by-side.
* Saves your cursor position across every document you open.
* Optionally remembers what documents you had open when you closed the program, and restores them on next launch.
* Designed with screen reader users in mind.
* Ability to navigate by various navigation units, including HTML headings, PDF pages, and epub sections.
* Robust find dialog, including features such as history and regular expression support.
* Can be ran entirely portably, or installed with file associations automatically set up.

## Currently supported filetypes
Paperback supports several common file formats, with more planned for future releases.

* CHM help files.
* microsoft Word documents.
* Epub 2/3 books.
* HTML documents.
* Markdown documents.
* OpenDocument presentations.
* OpenDocument text files.
* PDF documents.
* PowerPoint presentations.
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
* Ctrl+G: bring up the go to line dialog.
* Ctrl+Shift+G: bring up the go to percent dialog, allowing you to go to a percentage through your document with a slider.
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
* Shift+P: go to the previous page.
* P: go to the next page.
* Shift+B: go to the previous bookmark.
* B: go to the next bookmark.
* Ctrl+Shift+B: toggle a bookmark at your current cursor position.
* Ctrl+B: bring up the jump to bookmark dialog.
* Shift+K:" go to the previous link.
* K: go to the next link.

### Tools menu
* Ctrl+W: view the word count of the currently focused document.
* Ctrl+I: bring up the document info dialog.
* Ctrl+T: show the table of contents.
* Ctrl+Comma: bring up the options dialog.

### About menu
* Ctrl+F1: show the about dialog.
* F1: pop up this readme file in your default web browser.
* Shift+F1: pop up this readme file in Paperback itself.
* Ctrl+D: open my PayPal donations page in your default web browser.

## Changelog
### Version 0.5
* Added Microsoft Word document support! [#27](https://github.com/trypsynth/paperback/issues/27).
* Added support for PowerPoint presentations! [#25](https://github.com/trypsynth/paperback/issues/25).
* Fixed certain menu items not being disabled with no documents open.
* Fixed the orientation of the go to percent slider. [#70](https://github.com/trypsynth/paperback/issues/70).
* Fixed the table of contents in Epub books with URL-encoded file paths and/or fragment IDs.
* Fixed whitespace being stripped from XHTML headings in weird ways.
* Fixed whitespace handling inside of nested pre tags in HTML documents.
* HTML and Markdown documents now support the table of contents feature! When you load an HTML/Markdown document, Paperback will build its own table of contents out of the structure of the headings in your document, and it will show that to you in the control+t dialog.
* HTML documents will now have the title as set in the title tag, if it exists. Otherwise, they'll continue to use the filename without the extension.
* Switched from UniversalSpeech to using a live region to report speech. This means no screen reader DLLs are shipped alongside the program anymore, and more screen readers will now be supported, such as Microsoft Narrator.
* Switched zip libraries to allow for opening a wider array of epub books. [#73](https://github.com/trypsynth/paperback/issues/73).
* The dialog asking you if you want to open your document as plain text has been completely redone, and it now allows you to open your document as plain text, HTML, or Markdown.
* The go to percent dialog now includes a text field allowing you to manually input a percentage to jump to. [#66](https://github.com/trypsynth/paperback/issues/66).
* The HTML parser will now recognize dd, dt, and dl as list elements.
* The table of contents in Epub books will once again be preserved exactly.
* The unicode non-breaking space is now considered when stripping blank lines. [#71](https://github.com/trypsynth/paperback/issues/71).
* You will no longer be asked how you want to open an unrecognized file every single time you load it, only the first time.

### Version 0.41
* Added an optional start menu icon to the installer.
* The table of contents should now be cleaner in a few cases, for example if you have a child and parent item with the same text at the same position you'll now only see the parent item.
* Fixed the table of contents in certain CHM documents.
* Fixed the table of contents in Epub 3 books with absolute paths in them. [#67](https://github.com/trypsynth/paperback/issues/67).
* CHM documents should now show their title as set in the metadata file.

### Version 0.4
* Added CHM file support! [#23](https://github.com/trypsynth/paperback/issues/23).
* Added bookmark support! You can have as many bookmarks throughout as many documents as you like. You can jump forward and backward through them with b and shift+b, set one with control+shift+b, and bring up a dialog to jump to a specific bookmark with control+b. [#13](https://github.com/trypsynth/paperback/issues/13).
* Added an installer alongside the portable zip file! The installer will install Paperback into your Program Files directory, and automatically set up file associations for you. [#33](https://github.com/trypsynth/paperback/issues/33).
* Text files with BOMs should now be decoded properly, and the BOM will no longer be displayed at the beginning of the text either.
* Added far more information to the status bar. It'll now show you your current line, character, and reading percentage. [#51](https://github.com/trypsynth/paperback/issues/51).
* HTML comments, as well as the contents of script and style tags, will no longer be shown in text output.
* If passing a relative path to Paperback on the command line, it will now resolve it properly.
* Percentage movement is now handled by its own slider-based dialog, accessible with control+shift+g. [#57](https://github.com/trypsynth/paperback/issues/57).
* Documents without known titles or authors will now always have a default.
* The position saving logic is now much smarter and should only write to the disk when absolutely necessary.
* The document you had focused when you closed Paperback is now remembered across application restarts.
* Input into the go to line and go to page dialogs should now be sanitized more strictly.
* Fixed table of contents navigation in epub 3 books with relative paths in their manifests.

### Version 0.3
* Fixed the table of contents in epub books with URL-encoded manifests. [#34](https://github.com/trypsynth/paperback/issues/34).
* Fixed heading navigation in HTML documents containing multi-byte Unicode characters. [#42](https://github.com/trypsynth/paperback/issues/42), [#59](https://github.com/trypsynth/paperback/issues/59), [#61](https://github.com/trypsynth/paperback/issues/61).
* Fixed high CPU usage in documents with long titles due to a regression in wxWidgets. [#60](https://github.com/trypsynth/paperback/issues/60).
* Fixed loading UTF-8 text files.
* Fixed nested TOC items in EPub books putting your cursor at the wrong position.
* Fixed a crash on application exit in certain cases. [#45](https://github.com/trypsynth/paperback/issues/45).
* Added a checkbox in the options dialog to enable or disable word wrap!
* It is now possible to donate to Paperback's development, either through the new donate item in the help menu or through the sponsor this project link at the bottom of the GitHub repository's main page.
* Markdown documents will now always have a title, and Paperback should now be able to load virtually any Markdown file. [#52](https://github.com/trypsynth/paperback/issues/52).
* PDF documents will now always have a title, even if the metadata is missing. [#56](https://github.com/trypsynth/paperback/issues/56).
* Switched PDF libraries to the one used in Chromium, leading to far more reliable PDF parsing across the board. [#41](https://github.com/trypsynth/paperback/issues/41).
* You can now only have one instance of Paperback running at a time. Running paperback.exe with a filename while it's already running will open that document in the already running instance.
* You can now press delete on a document in the tab control to close it.

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
