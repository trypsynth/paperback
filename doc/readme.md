# Paperback - version 0.6.1
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

* CHM help files
* microsoft Word documents
* Epub books (version 2 and 3)
* FB2 ebooks
* HTML documents
* Markdown documents
* OpenDocument presentations
* OpenDocument text files
* PDF documents
* PowerPoint presentations
* Text documents

## Hotkeys
Paperback's user interface was designed specifically with keyboard and screen reader users in mind. As such, every action has an associated hotkey. Below, you'll find a list of all of them, grouped by menu structure.

### File menu
* Ctrl+O: open a document.
* Ctrl+F4: close the currently focused document.
* Ctrl+Shift+F4: close all currently opened documents.
* Ctrl+R: bring up the all documents dialog.

### Go menu
* Ctrl+F: show the find dialog.
* F3: find next.
* Shift+F3: find previous.
* Ctrl+G: bring up the go to line dialog.
* Ctrl+Shift+G: bring up the go to percent dialog.
* Ctrl+P: Bring up the go to page dialog, if supported in your focused document.
* Alt+left: go to the previous item in the navigation history.
* Alt+right: go to the next item in the navigation history.
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
* Shift+N: go to the previous note.
* N: go to the next note.
* Ctrl+B: bring up the jump to bookmark dialog, with both bookmarks and notes selected.
* Ctrl+Alt+B: bring up the jump to bookmark dialog, with only bookmarks selected.
* Ctrl+Alt+M: bring up the jump to bookmark dialog, with only notes selected.
* Ctrl+Shift+W: view the currently focused note in a read-only text field.
* Shift+K:" go to the previous link.
* K: go to the next link.
* Shift+L: go to the previous list.
* L: go to the next list.
* Shift+I: go to the previous list item.
* I: go to the next list item.

### Tools menu
* Ctrl+W: view the word count of the currently focused document.
* Ctrl+I: bring up the document info dialog.
* Ctrl+T: show the table of contents.
* F7: bring up the elements list.
* Ctrl+Shift+E: export your currently focused document's bookmarks and reading position to a file for easy sharing.
* Ctrl+Shift+I: import metadata about a book from a .paperback file.
* Ctrl+E: export your focused document's contents to a text file.
* Ctrl+Shift+B: toggle a bookmark at your current cursor position.
* Ctrl+Shift+N: add/edit a bookmarked note at your current cursor position.
* Ctrl+Comma: bring up the options dialog.
* Ctrl+Shift+S: toggle the sleep timer.

### Help menu
* Ctrl+F1: show the about dialog.
* F1: pop up this readme file in your default web browser.
* Shift+F1: pop up this readme file in Paperback itself.
* Ctrl+Shift+U: check for updates.
* Ctrl+D: open my PayPal donations page in your default web browser.

## Credits
### Development
* Quin Gillespie: primary developer and project founder.
* Aryan Choudhary: added multiple new document formats, link and list support, more.

### Donations
The following people have made donations of some size to Paperback development. If you make a donation your name won't automatically be added here, I only add people who want their donation made public.

Note: I consider a public GitHub sponsor grounds for automatic inclusion in this list.

* Alex Hall
* Brandon McGinty
* Brian Hartgen
* Debbie Yuille
* Devin Prater
* Felix Steindorff
* Hamish Mackenzie
* James Scholes
* Jayson Smith
* Jonathan Schuster
* Pratik Patel 
* Roberto Perez
* Sean Randall
* Timothy Wynn

### Translations
* Tarik Hadžirović: Bosnian translation
* Martin Courcelles: French translation
* Ruslan Gulmagomedov: Russian translation
* Nikola Jović: Serbian translation
* Steffen Schultz: German translation

## Changelog
### Version 0.6.1
* Added password-protected PDF support! [#169](https://github.com/trypsynth/paperback/issues/169).
* Added a very basic go to previous/next position feature. If you press enter on an internal link and it moves your cursor, that position will now be remembered, and can be navigated to with alt+left/right arrows. [#115](https://github.com/trypsynth/paperback/issues/115), [#174](https://github.com/trypsynth/paperback/pull/174).
* Added an elements list! Currently it only shows a tree of all the headings in your document or a list of links, but there are plans to expand it in the future. [#173](https://github.com/trypsynth/paperback/issues/173), [#177](https://github.com/trypsynth/paperback/pull/177).
* Added an option to start Paperback in maximized mode by default. [#164](https://github.com/trypsynth/paperback/issues/164), [#172](https://github.com/trypsynth/paperback/pull/172).
* Fixed links in some Epub documents not working properly. [#167](https://github.com/trypsynth/paperback/issues/167), [#171](https://github.com/trypsynth/paperback/pull/171), [#178](https://github.com/trypsynth/paperback/issues/178), [#180](https://github.com/trypsynth/paperback/pull/180).
* Fixed parsing Epub TOCs containing relative paths. [#187](https://github.com/trypsynth/paperback/issues/187).
* Fixed some epub documents not showing a title or author. [#175](https://github.com/trypsynth/paperback/issues/175).
* Fixed the titles of some epub chapters not showing up properly in the TOC dialog. [#176](https://github.com/trypsynth/paperback/pull/176).
* Fixed you not being able to use the space bar to activate the OK/cancel buttons in the TOC dialog. [#170](https://github.com/trypsynth/paperback/issues/170).
* Improved the handling of headings in Word documents. [#183](https://github.com/trypsynth/paperback/pull/183).
* You will now get spoken feedback if the recent documents list is empty when you try to bring up the dialog. [#185](https://github.com/trypsynth/paperback/issues/185).

### Version 0.6.0
* A new option to show the go menu in a far more compact form has been added to the options dialog, checked by default. 
* Added an option to make navigation by structural elements wrap. [#116](https://github.com/trypsynth/paperback/pull/116).
* Added an option to the tools menu to open the containing folder of the currently focused document. [#142](https://github.com/trypsynth/paperback/pull/142).
* Added a quite simple, but very effective, updating system. [#28](https://github.com/trypsynth/paperback/issues/28).
* Added a basic sleep timer feature, accessible with Ctrl+Shift+S. [#117](https://github.com/trypsynth/paperback/issues/117), [#118](https://github.com/trypsynth/paperback/pull/118).
* Added support for parsing FB2 ebooks! [#30](https://github.com/trypsynth/paperback/issues/30), [#107](https://github.com/trypsynth/paperback/pull/107).
* Added support for parsing OpenDocument presentations! [#105](https://github.com/trypsynth/paperback/issues/105), [#106](https://github.com/trypsynth/paperback/pull/106).
* Added support for parsing OpenDocument Text files! [#29](https://github.com/trypsynth/paperback/issues/29), [#90](https://github.com/trypsynth/paperback/pull/90).
* Bookmarks can now be made to bookmark an entire line, or to mark only some specified text. If you have no selection active when placing a bookmark, the behavior is like pre-0.6, and it'll mark the entire line. However, if you select some text, only that text will be included in the bookmark. [#99](https://github.com/trypsynth/paperback/issues/99).
* Bookmarks can now have optional text notes attached to them! Navigate between bookmarks containing notes with N and Shift+N, or pop up the bookmarks dialog with all bookmarks, only notes, or only non-notes selected with specific hotkeys. [#68](https://github.com/trypsynth/paperback/issues/68), [#128](https://github.com/trypsynth/paperback/issues/128), [#156](https://github.com/trypsynth/paperback/issues/156), [#157](https://github.com/trypsynth/paperback/issues/157), [#158](https://github.com/trypsynth/paperback/pull/158), [#159](https://github.com/trypsynth/paperback/issues/159), [#161](https://github.com/trypsynth/paperback/pull/161).
* Bookmarks in the bookmarks dialog will no longer have an annoying "bookmark x" prefix. [#86](https://github.com/trypsynth/paperback/issues/86).
* Epub books containing HTML content pretending to be XML will now be handled properly. [#96](https://github.com/trypsynth/paperback/issues/96).
* Fixed loading large Markdown documents. [#97](https://github.com/trypsynth/paperback/issues/97).
* Fixd pressing space in the table of contents tree view activating the OK button. [#121](https://github.com/trypsynth/paperback/issues/121), [#123](https://github.com/trypsynth/paperback/pull/123).
* Fixed whitespace handling at the beginning of pre tags in both HTML and XHTML documents.
* Fixed the text control not regaining focus sometimes when returning to Paperback's window. [#138](https://github.com/trypsynth/paperback/issues/138).
* Fixed the text field in the go to percent dialog not updating the slider's value.
* Fixed the rendering of custom HTML IDs in Markdown documents. [#113](https://github.com/trypsynth/paperback/issues/113).
* HTML inside Markdown code blocks will now be rendered properly. [#79](https://github.com/trypsynth/paperback/issues/79).
* If loading a book with a command line parameter while an existing Paperback instance is running, you'll no longer get an error if loading your document takes more than 5 seconds.
* If running Paperback as administrator, the configuration will now be properly loaded and saved. [#148](https://github.com/trypsynth/paperback/issues/148), [#149](https://github.com/trypsynth/paperback/pull/149).
* It is now possible to delete a bookmark directly from within the bookmarks dialog. [#100](https://github.com/trypsynth/paperback/issues/100), [#103](https://github.com/trypsynth/paperback/pull/103).
* It is now possible to import and export your bookmarks and reading position for a particular document. The generated file is named after the file with a .paperback extension. If such a file is found in the same directory as a file while loading it, it will be automatically loaded. Otherwise, you can manually import them using an item in the tools menu. [#146](https://github.com/trypsynth/paperback/issues/146), [#147](https://github.com/trypsynth/paperback/pull/147).
* Links inside documents are now fully supported! Use k and shift+k to move forward and backward through them, and press enter to open/activate one. [#74](https://github.com/trypsynth/paperback/issues/74), [#87](https://github.com/trypsynth/paperback/pull/87), [#126](https://github.com/trypsynth/paperback/issues/126), [#129](https://github.com/trypsynth/paperback/issues/129), [#130](https://github.com/trypsynth/paperback/issues/130).
* Many internal refactors, making the app faster and the binary smaller.
* Markdown content is now preprocessed in order to be CommonMark compliant before rendering.
* Navigation by lists and their items is now fully supported! Use L and Shift+L to go by lists themselves, and I and Shift+I to go through list items. [#119](https://github.com/trypsynth/paperback/issues/119), [#124](https://github.com/trypsynth/paperback/pull/124).
* Numpad delete now works to remove documents from the tab bar in addition to normal delete.
* Paperback can now optionally minimize to your system tray! This option is off by default, but turning it on will make the minimize option in the system menu put Paperback in your tray, able to be restored by clicking on the spawned icon. [#49](https://github.com/trypsynth/paperback/issues/49), [#85](https://github.com/trypsynth/paperback/pull/85).
* Paperback is now fully translatable! The list of languages it supports is currently fairly small, but it's constantly growing! [#75](https://github.com/trypsynth/paperback/issues/75), [#92](https://github.com/trypsynth/paperback/pull/92), [#95](https://github.com/trypsynth/paperback/pull/95), [#134](https://github.com/trypsynth/paperback/pull/134), [#137](https://github.com/trypsynth/paperback/pull/137), [#141](https://github.com/trypsynth/paperback/pull/141), [#152](https://github.com/trypsynth/paperback/pull/152).
* Paperback now has an official website, at [paperback.dev](https://paperback.dev)!
* PPTX documents will now show a basic table of contents, containing all of the slides. [#122](https://github.com/trypsynth/paperback/issues/122).
* The full path to the opened document will now be shown in the document info dialog. [#139](https://github.com/trypsynth/paperback/issues/139), [#140](https://github.com/trypsynth/paperback/pull/140).
* The installer now includes an option to view the readme in your browser after installation.
* The recent documents list has been dramatically expanded! Instead of simply showing you the last 10 documents you opened, it'll now show you a customizable number, with the rest of the documents you've ever opened being accessible through a small dialog. [#78](https://github.com/trypsynth/paperback/issues/78), [#80](https://github.com/trypsynth/paperback/pull/80), [#84](https://github.com/trypsynth/paperback/pull/84), [#135](https://github.com/trypsynth/paperback/pull/135).
* Various small improvements to the parsers across the board, including putting a blank line between slides in PPTX presentations, fixing the newline handling inside of paragraphs in word documents, and adding bullet points to list items.

### Version 0.5.0
* Added Microsoft Word document support! [#27](https://github.com/trypsynth/paperback/issues/27).
* Added support for PowerPoint presentations! [#25](https://github.com/trypsynth/paperback/issues/25).
* Fixed certain menu items not being disabled with no documents open.
* Fixed the orientation of the go to percent slider. [#70](https://github.com/trypsynth/paperback/issues/70).
* Fixed the table of contents in Epub books with URL-encoded file paths and/or fragment IDs.
* Fixed whitespace being stripped from XHTML headings in weird ways.
* Fixed whitespace handling inside of nested pre tags in HTML documents.
* HTML and Markdown documents now support the table of contents feature! When you load an HTML/Markdown document, Paperback will build its own table of contents out of the structure of the headings in your document, and it will show that to you in the ctrl+t dialog.
* HTML documents will now have the title as set in the title tag, if it exists. Otherwise, they'll continue to use the filename without the extension.
* Switched from UniversalSpeech to using a live region to report speech. This means no screen reader DLLs are shipped alongside the program anymore, and more screen readers will now be supported, such as Microsoft Narrator.
* Switched zip libraries to allow for opening a wider array of epub books. [#73](https://github.com/trypsynth/paperback/issues/73).
* The dialog asking you if you want to open your document as plain text has been completely redone, and it now allows you to open your document as plain text, HTML, or Markdown.
* The go to percent dialog now includes a text field allowing you to manually input a percentage to jump to. [#66](https://github.com/trypsynth/paperback/issues/66).
* The HTML parser will now recognize dd, dt, and dl as list elements.
* The table of contents in Epub books will once again be preserved exactly.
* The unicode non-breaking space is now considered when stripping blank lines. [#71](https://github.com/trypsynth/paperback/issues/71).
* You will no longer be asked how you want to open an unrecognized file every single time you load it, only the first time.

### Version 0.4.1
* Added an optional start menu icon to the installer.
* The table of contents should now be cleaner in a few cases, for example if you have a child and parent item with the same text at the same position you'll now only see the parent item.
* Fixed the table of contents in certain CHM documents.
* Fixed the table of contents in Epub 3 books with absolute paths in them. [#67](https://github.com/trypsynth/paperback/issues/67).
* CHM documents should now show their title as set in the metadata file.

### Version 0.4.0
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

### Version 0.3.0
* Fixed the table of contents in epub books with URL-encoded manifests. [#34](https://github.com/trypsynth/paperback/issues/34).
* Fixed heading navigation in HTML documents containing multi-byte Unicode characters. [#42](https://github.com/trypsynth/paperback/issues/42), [#59](https://github.com/trypsynth/paperback/issues/59), [#61](https://github.com/trypsynth/paperback/issues/61).
* Fixed high CPU usage in documents with long titles due to a regression in wxWidgets. [#60](https://github.com/trypsynth/paperback/issues/60).
* Fixed loading UTF-8 text files.
* Fixed nested TOC items in Epub books putting your cursor at the wrong position.
* Fixed a crash on application exit in certain cases. [#45](https://github.com/trypsynth/paperback/issues/45).
* Added a checkbox in the options dialog to enable or disable word wrap!
* It is now possible to donate to Paperback's development, either through the new donate item in the help menu or through the sponsor this project link at the bottom of the GitHub repository's main page.
* Markdown documents will now always have a title, and Paperback should now be able to load virtually any Markdown file. [#52](https://github.com/trypsynth/paperback/issues/52).
* PDF documents will now always have a title, even if the metadata is missing. [#56](https://github.com/trypsynth/paperback/issues/56).
* Switched PDF libraries to the one used in Chromium, leading to far more reliable PDF parsing across the board. [#41](https://github.com/trypsynth/paperback/issues/41).
* You can now only have one instance of Paperback running at a time. Running paperback.exe with a filename while it's already running will open that document in the already running instance.
* You can now press delete on a document in the tab control to close it.

### Version 0.2.1
* Added the total number of pages to the page label in the go to page dialog. [#46](https://github.com/trypsynth/paperback/issues/46).
* Allow tabbing from the document content to your list of opened documents. [#19](https://github.com/trypsynth/paperback/issues/19).
* Fixed the heading keystrokes sometimes opening recent documents if you had enough of them. [#47](https://github.com/trypsynth/paperback/issues/47).
* Paperback will now remove unnecessary soft hyphens from text output.
* Fixed heading navigation sometimes putting you on the wrong character.

### Version 0.2.0
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

### Version 0.1.0
* Initial release.
