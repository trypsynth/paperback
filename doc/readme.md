# Paperback - version 1.0

## Introduction

Paperback is a lightweight, fast, and accessible reader for ebooks, documents, and audio books, for everyone, from casual readers to heavy power users. It's designed for screen reader accessibility, fast speeds, and a bloat-free experience.

## System Requirements

Paperback runs on Windows 10/11, all the modern versions of ARM macOS, Linux, iOS 17 and later, and Android 7 and later. The iOS and Android apps are on the App Store and Google Play.

## Features

* Completely standalone, not requiring any software to be installed on your computer to start reading.
* Incredibly fast, even on old hardware.
* Simple tabbed interface, allowing you to open as many documents as you want side-by-side.
* Saves your exact reading position across every document you open.
* Optionally remembers what documents you had open when you closed the program, and restores them on next launch.
* Includes navigation functionality similar to that found in the web browsing mode of many screen readers to quickly and easily navigate through documents.
* Includes a robust find dialog, including features such as history and regular expression support.
* Can be run entirely portably, or installed with file associations automatically set up.
* Supports a massive array of common file formats.
* Plays audio books, with adjustable speed and bookmarks that remember the exact time.
* Reads scanned PDF pages with the OCR built into Windows and macOS.
* Bookmarks and notes, so you can mark your place and come back to it.
* Every keyboard shortcut can be changed.
* Comes with `pb`, a command line tool that converts any supported document to HTML, Markdown, or plain text.

## Screen Reader Compatibility

Paperback works well with all major screen readers. There are, however, two known issues for JAWS users.

### JAWS and Braille Displays

If you use JAWS with a Braille display, you may find that long paragraphs are truncated when panning forward with your display's navigation keys. The read current paragraph command is also affected. This is a bug in JAWS's handling of the RICHEDIT50W text control, not something in Paperback itself, and one that took quite a while to surface a fix for given Vispero's enthusiasm for responding to issues with open source software.

The workaround, eventually surfaced through the JAWS discussion group after months of waiting, is to edit `paperback.jcf` and set "Braille Presentation and Panning" to "Always use DOM if available". You'll also want to enable "Pan Text by Paragraph", otherwise your display will stay on the active paragraph rather than advancing. With both settings in place, panning should work correctly.

### JAWS and Paperback's messages

Paperback says things like "No pages." or "This document has no audio." as accessibility notifications, which is what lets a screen reader speak them over whatever it is saying. JAWS only acts on those when "Enable accessible notification events" is switched on for the application, and on some machines it is not.

If JAWS says nothing when you press a key that should report something, open Settings Center with Paperback in front (`Insert+6`), search for "notification", and tick "Enable accessible notification events". That writes the setting to `paperback.jcf`, so it applies to Paperback alone.

## Currently supported file types

Paperback supports the following formats and extensions:

* Comic book archives (`.cbz`)
* CHM help files (`.chm`)
* DAISY books (`.opf`, `.zip`)
* EPUB books (`.epub`)
* FB2 ebooks (`.fb2`)
* HTML documents (`.htm`, `.html`, `.xhtml`)
* Manual pages, both `man` and BSD `mdoc` (`.1` to `.9`, `.man`, `.roff`, and the gzipped forms of each)
* Markdown documents (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word documents (`.docx`, `.docm`, `.doc`)
* M4B audiobooks (`.m4b`)
* MOBI/Kindle books (`.mobi`, `.azw`, `.azw3`)
* MP3 audiobooks (`.mp3`)
* OpenDocument presentations (`.odp`, `.fodp`)
* OpenDocument text files (`.odt`, `.fodt`)
* PDF documents (`.pdf`)
* PowerPoint presentations (`.pptx`, `.pptm`, `.ppt`)
* reStructuredText documents (`.rst`, `.rest`)
* RTF documents (`.rtf`)
* Windows Write documents (`.wri`)
* WinHelp files (`.hlp`)
* Plain text and log files (`.txt`, `.log`)

## Keyboard shortcuts

Paperback is designed for keyboard-first use. Here are the current shortcuts.

Shortcuts below are for Windows. Where macOS differs, the equivalent is noted in parentheses — mainly because Ctrl+G, Ctrl+W, and Alt+Left/Right are already claimed by other system or app conventions on that platform.

### File menu

* `Ctrl+O`: Open a document.
* `Ctrl+F4` (macOS: `Cmd+W`): Close the current document.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Close all open documents.
* `Ctrl+Shift+T`: Reopen the last closed document.
* `Ctrl+R`: Show the "All Documents" dialog (from Recent Documents).
* `Ctrl+Q`: Exit (Windows only; on macOS this is under the app menu instead).

### Go menu

* `Ctrl+F`: Show the Find dialog.
* `F3` (macOS: `Cmd+G`): Find next.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Find previous.
* `Ctrl+G` (macOS: `Cmd+L`): Go to line.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Go to percent.
* `Ctrl+P`: Go to page (when supported by the current document).
* `=`: Announce your current reading percentage and page, e.g. "15%, page 30". The page is left out for documents that have no page numbers.
* `Alt+Left` (macOS: `Cmd+[`): Go back in navigation history.
* `Alt+Right` (macOS: `Cmd+]`): Go forward in navigation history.
* `[`: Previous section.
* `]`: Next section.
* `Shift+H`: Previous heading.
* `H`: Next heading.
* `Shift+1` through `Shift+6`: Previous heading at level 1-6.
* `1` through `6`: Next heading at level 1-6.
* `Shift+P`: Previous page.
* `P`: Next page.
* `Shift+B`: Previous bookmark.
* `B`: Next bookmark.
* `/`: Set your temporary bookmark.
* `\`: Jump to your temporary bookmark.
* `Shift+N`: Previous note.
* `N`: Next note.
* `Ctrl+B`: Jump to all bookmarks and notes.
* `Ctrl+Alt+B`: Jump to bookmarks only.
* `Ctrl+Alt+M`: Jump to notes only.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, i.e. the physical Control key rather than Cmd): View note text at the current position.
* `Shift+K`: Previous link.
* `K`: Next link.
* `Shift+G`: Previous image.
* `G`: Next image.
* `Shift+F`: Previous figure.
* `F`: Next figure.
* `Shift+T`: Previous table.
* `T`: Next table.
* `Shift+M`: Previous formula.
* `M`: Next formula.
* `Shift+S`: Previous separator.
* `S`: Next separator.
* `Shift+L`: Previous list.
* `L`: Next list.
* `Shift+I`: Previous list item.
* `I`: Next list item.
* `Shift+,`: Go to the start of the current container (list or table).
* `,`: Go past the end of the current container (list or table).

### Tools menu

* `Ctrl+W` (macOS: `RawCtrl+W`, i.e. the physical Control key rather than Cmd): Show word count for the current document.
* `Ctrl+I`: Show document info.
* `Ctrl+T`: Show table of contents.
* `F7`: Show elements list.
* `Ctrl+Shift+C`: Open containing folder.
* `Ctrl+Shift+V`: Open current content in Web View.
* `Ctrl+U`: View the document source in a new tab.
* `Ctrl+Shift+E`: Export document data (`.paperback`).
* `Ctrl+Shift+I`: Import document data (`.paperback`).
* `Ctrl+E`: Export the current document to plain text.
* `Ctrl+Shift+B`: Toggle bookmark at the current selection/cursor.
* `Ctrl+Shift+N`: Add or edit bookmark note at the current selection/cursor.
* `Ctrl+Alt+W`: Toggle word wrap.
* `Ctrl+Space` (macOS: `RawCtrl+Space`, i.e. the physical Control key, since Cmd+Space opens Spotlight): Play/pause audio narration.
* `'`: Seek audio narration forward.
* `;`: Seek audio narration backward.
* `Shift+'`: Increase the audio seek amount.
* `Shift+;`: Decrease the audio seek amount.
* `Ctrl+Shift+.`: Speed up audio narration.
* `Ctrl+Shift+,`: Slow down audio narration.
* `F11` (macOS: `RawCtrl+Ctrl+F`, i.e. Control+Command+F): Toggle full screen.
* `Ctrl+,`: Open Settings (macOS: under the app menu).
* `Ctrl+Shift+S`: Toggle sleep timer.
* `Ctrl+Shift+O`: Recognize a range of scanned PDF pages with OCR.
* `Alt+F9` (macOS: `Cmd+F9`): Mark the beginning of a selection, so everything from here to wherever you get to can be copied in one go.
* `Alt+F10` (macOS: `Cmd+F10`): Copy everything from the marked beginning of the selection to the current position.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Jump back to the marked beginning of the selection, leaving the mark in place.

### Help menu

* `Ctrl+F1`: Show About dialog.
* `F1`: View help in your default browser.
* `Shift+F1`: View help in Paperback.
* `Ctrl+Shift+U`: Check for updates.
* `Ctrl+D`: Open the donation page in your default browser.

### Additional document-view keys

* `Delete` / `Numpad Delete` on the tab control: Close the selected document tab.
* `Enter` or `Space` in the document text: Follow a link or open a table or formula view at the cursor.
* `Enter` on a scanned PDF page: Recognize the page with OCR.
* `Shift+F10` or the Menu/Application key in the document text: Open the context menu.

## iOS and Android

The iOS and Android apps use the same reading engine as the desktop, so they open the same formats and remember your place the same way. They are built to be used with VoiceOver on iOS and TalkBack on Android.

### Opening documents

* Use the Open Book button, or open a document from the Files app or another app and choose Paperback.
* On Android, you can turn on the in-app file browser in Settings instead. It needs the All Files Access permission, and opens large files straight away instead of copying them first.
* Long-press the Open Book button to import or export a document's data (`.paperback`), the same files the desktop app uses.

### Reading and listening

Each app has two ways to read a document. In text mode, you read the text with your screen reader. In read aloud mode, Paperback reads the text to you with the voice you pick in Settings, and keeps going in the background and from the lock screen. Switch between them from the More Options menu.

Audio books, like DAISY, M4B, and MP3 books, play their own recording instead.

### The reading bar

The bar at the bottom of the screen has, from left to right:

* The navigation unit, such as paragraph, heading, page, or link. Swipe up or down on it to change it.
* Previous, play, and next buttons. Previous and next move by the navigation unit.
* The speech rate. Swipe up or down on it to change how fast Paperback reads.

You can also swipe up or down on the play button to move by the navigation unit, without reaching for the previous and next buttons. If that is all you use, the Hide previous and next buttons setting takes them out of your screen reader's way. The Swipe up moves forward setting picks which way a swipe goes.

### More options

The More Options menu is where everything else lives. Some items work a little differently on each app.

* **Switch to TTS Mode or Switch to Text Mode:** moves between read aloud mode and text mode, described above. In text mode, a Read Aloud item starts and pauses reading aloud without leaving text mode.
* **Table of Contents:** the book's chapters, opened at the one you're reading. Pick one to go straight to it. Entries with chapters under them can be expanded and collapsed with the screen reader's actions.
* **Elements:** a list of the document's headings or links. Switch between the two with the Type picker on iOS, or the tabs on Android, then pick one to go to it.
* **Find:** type what to look for, or pick an earlier search from Search History, and choose whether to match case, match whole words only, or use a regular expression. Find Previous and Find Next jump to a match and say where it landed, and Find stays open so you can keep going. In read aloud mode, Find also shows up as a navigation unit on the reading bar, so you can step through the matches from there too.
* **Go To:** jump to a line, a page, or a percentage through the document. Pick which with the Mode picker.
* **Recent Documents:** every document you've opened, each marked as currently open, closed, or file missing. Each one has two screen reader actions: Remove takes it off the list, and Locate lets you find a document whose file has moved. Clear Recent Documents empties the list without deleting any documents.
* **Word Count:** the number of words in the document.
* **Document Info:** the title, the author, the file name, and on iOS the line and character counts too.
* **Export:** saves the document as plain text, HTML, or Markdown.
* **Sleep Timer:** stops reading after 5, 10, 15, 30, 45, or 60 minutes, or a time of your own. Open it again while it's running to see how long is left, or to cancel it.
* **Help:** opens this readme.
* **Settings:**
    * **Text to speech:** the voice, speech rate, and pitch, a Play Sample button to hear them, and the pause between paragraphs. Android also lets you pick the speech engine. On iOS, this is also where the speech dictionary is: rules that change how words are spoken, for every voice or just some.
    * **Readability:** text size, line spacing, paragraph spacing, alignment, and high contrast text. iOS also has light and dark appearance.
    * **Behavior:** whether to reopen your documents when the app starts, which way a swipe on the play button moves, and whether to hide the previous and next buttons. Android also has the in-app file browser here.

### Keyboards and headsets

With a keyboard, the desktop shortcuts for opening books, recent documents, Find, Go To, the table of contents, word count, document info, export, and the sleep timer all work, using `Cmd` in place of `Ctrl` on iOS. So do the single-letter keys for moving by heading, page, link, and the rest, and `Space` plays and pauses. On iOS, the single-letter keys only reach Paperback while VoiceOver's single-letter Quick Nav is off.

On Android, a headset button plays and pauses with one press, moves forward with two, and goes back with three.

## Supported languages

Paperback is translated into many different languages, with more being added all the time. A complete list follows below.

To learn how to contribute, please read our [Translation Guide](translating.md).

* Bosnian
* Czech
* Dutch
* Finnish
* French
* German
* Japanese
* Polish
* Portuguese (Brazil)
* Russian
* Simplified Chinese
* Serbian
* Spanish
* Ukrainian
* Vietnamese

## Credits
### Development
* Quin Gillespie: primary developer and project founder.
* Aryan Choudhary: primary contributor.

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
* Jonathan Rodriguez
* Jonathan Schuster
* Keao Wright
* Michael Marshall
* Pratik Patel
* Roberto Perez
* Sean Randall
* Timothy Wynn
* Tyler Rodick

## Changelog

### Version 1.0

1.0 is the first release on all five platforms: Windows, macOS, Linux, iOS and Android, with the iOS and Android apps in the App Store and Google Play.

#### Added

##### General
* Linux support, as an AppImage or a tar.gz, with desktop integration so documents open from your file manager.
* Mark the beginning of a selection with `Alt+F9`, copy everything from there to wherever you have got to with `Alt+F10`, and go back to the mark with `Alt+Shift+F9`, for copying a long span of text without shift-arrowing through it. All three are under Tools > Select and copy.
* The `=` shortcut now announces the page as well as the percentage, e.g. "15%, page 30", and stays as it was for documents with no page numbers.
* The About box now shows Paperback's license and every translator.
* A Ukrainian translation.

##### New Formats
* Comic book archives (`.cbz`).
* M4B audiobooks, split into their chapters.
* Manual pages, both `man` and BSD `mdoc`, gzipped or not.
* MP3 audiobooks, split into chapters when the file has them.
* reStructuredText documents.
* Windows Write (`.wri`) files.
* WinHelp (`.hlp`) files.
* Word 6 and Word 95 documents.

##### OCR
* Scanned PDF pages can now be recognized with the OCR built into Windows and macOS. Press `Enter` on a scanned page to recognize it, or use Batch OCR (`Ctrl+Shift+O`) for a range of pages.

##### Navigation
* MathML formulas in EPUB and HTML are rendered as AsciiMath using MathCAT. Use `M` or `Shift+M` to navigate formulas, then `Enter` or `Space` to open the original MathML in Formula View.
* A Find All button in the Find dialog, listing every line with a match so you can jump straight to the one you want.
* Tables, Lists and Pages views in the elements list (`F7`).
* Go to Line, Go to Page and Go to Percent now take `+n` and `-n` to move relative to where you are.
* EPUB, MOBI and CHM books with no headings of their own now get heading navigation from their table of contents.
* KF8 (AZW3) books now support section navigation.
* EPUB pages that are only a picture now show a line for it, so you can land on them instead of skipping straight past.

##### Audio Books
* Playback speed controls, from half speed to three times as fast. Use `Ctrl+Shift+.` and `Ctrl+Shift+,`, or the Tools menu.
* Bookmarks and notes in audio-only books now remember the exact time you set them at.
* Next and previous position (`Alt+Left` and `Alt+Right`) now work in audio books.
* Progress through an audio book is now measured by its recording, so Go to Percent and the status bar match how far through it you really are.

##### Recent Documents
* A Clear Recent Documents item in the Recent Documents submenu.

##### PDF Documents
* A setting to keep every line of a PDF separate, rather than joining them into paragraphs.
* Images and figures in PDFs are now announced.
* PDFs that carry reading structure but tag none of their pictures now announce those pictures, rather than leaving them out of the book entirely.

##### Web View
* Any document can now be opened in the web view, not only EPUB, HTML and Markdown.

##### Readability
* Headings are now drawn at a size that matches their level, and images and tables are set apart from the text around them.

##### pb
* `pb --list-formats` lists every format pb can read.
* pb now says which file it could not read, and why.

#### Fixed

##### General
* Fixed a crash when closing Paperback.
* Closing Paperback now hides the window straight away, rather than leaving it on screen while it saves.
* Opening a document no longer leaves Reopen Last Closed enabled when there is nothing to reopen.
* Paperback no longer keeps retrying documents in your recent list that have gone missing, and caps how many recent documents it stores.
* The old INI settings file is now deleted once it has been moved over to the new format.
* The font and color dialogs' titles, and the Export As menu in Vietnamese, are now translated.
* Updating now brings the relaunched window to the front, instead of leaving it behind every other window in Alt+Tab.
* Word wrap now applies straight away on large documents, rather than reloading the whole thing.

##### Navigation
* `Alt+Left` now goes back to where you jumped from, rather than to an older position.
* Bookmark sounds now only play when you move over a bookmark, not when you land on the line it is on.
* Closing the table of contents, the elements list and the Go dialogs now takes you straight to the line you land on, rather than making you sit through the screen reader reading out the window again.
* Go to Line, Go to Page and Go to Percent now refuse numbers outside the document instead of quietly going somewhere else.
* NVDA no longer cuts off the announcement when a document has no pages.
* Pressing OK in the table of contents without moving now goes to the entry that was already selected.
* The table of contents, the elements list and the bookmarks list no longer lag or freeze on books with thousands of entries.
* Up and Down arrow now remember their column per document, instead of carrying it over when you switch tabs.

##### Audio Books
* Audio playback now uses `Control+Space` on macOS, since `Command+Space` belongs to Spotlight.

##### PDF Documents
* Fixed PDFs exported from Apple Pages reading as plain text, with none of the headings and lists they were written with.
* Fixed PDF paragraphs and headings splitting at every line, and words splitting apart at spaces.
* Fixed numbered PDF headings running together into one heading.
* Fixed PDFs whose structure tree leads to no text opening empty.
* Page headers and footers are no longer read out on every page of untagged PDFs.
* PDFs that tag their page headers and footers as ordinary text no longer repeat the title and the page number between two paragraphs on every page.
* PDFs now show their real title, rather than their file name.
* Lines set in a monospaced font, like code, are no longer joined into paragraphs.

##### MOBI/AZW3 Books
* Large MOBI books no longer run out of memory, and are no longer cut off after 20 MB.
* MOBI and AZW3 books now open much faster.
* Fixed MOBI books losing their chapter list.
* Fixed garbled text where MOBI books cross from one record to the next.

##### Web View
* The web view no longer loads the whole of a huge book at once.
* The web view now shows documents whole when the reader shows them whole, rather than only a slice of them.

##### Other Formats
* FictionBook (.fb2) books written in windows-1251, which is most of them, now open instead of failing to read at all.
* FictionBook books that use a namespace or an HTML entity they never declared now open, rather than being refused as broken.
* Books in legacy encodings now open much faster.
* Fixed some Chinese text files opening as garbled text.
* Password-protected OpenDocument files now ask for their password, rather than being reported as broken.
* Password-protected legacy PowerPoint files now open, and legacy PowerPoint slides no longer lose their text.
* Plain text files saved with an `.rtf` extension now open as text, rather than failing with an error.
* RTF control words no longer show up as text.

#### iOS and Android

The iOS and Android apps open every format the desktop does, and include:

* Read aloud, with your choice of voice, rate and pitch, a speech rate control right on the reading bar, and an optional pause between paragraphs.
* Playback of DAISY, M4B and MP3 audio books, which keeps going in the background and from the lock screen.
* Navigation by headings, pages, links, tables, lists and more from the reading bar, plus the table of contents and Find.
* A sleep timer, word count, and document export, plus a speech dictionary on iOS.
* Text size, spacing, and high contrast text options.
* Keyboard shortcuts that match the desktop.

### Version 0.9.2
* Audio books no longer make your screen reader read out a run of spaces when you focus the text field.
* Audio books now name the file as you step through them by section.
* Audio books now report their real length, rather than claiming every file in them runs for 24 hours.
* Closing the Web View with Escape no longer throws up a debug alert after you have followed a link inside it.
* Copying after Select All now gives you the whole document, instead of only the part of it currently loaded.
* Find now cuts straight to the line it found, rather than making you sit through the screen reader reading out the window again as focus returns to the book.
* Fixed EPUB's that carry a stray ZIP64 block refusing to open with "Invalid local file header".
* Fixed long documents walking back to their start while a screen reader read continuously through them.
* Links in the WebView now take you to the section they point at, rather than failing with "File not found".
* The automatic "Document reloaded" announcement no longer cuts your screen reader off mid sentence, instead waiting for it to finish what it was saying.
* The Settings dialog's General tab now tabs through its options in the order they appear on screen, with the update channel directly after the check for updates option.
* Windows will now always show "Paperback" in the Open With menu, rather than the program's full tagline.
* Word Count and Document Info now show how many files an audio book holds, and how long it runs in total.

### Version 0.9.1
* Bookmark and note sounds now play on macOS.
* DAISY books now play their audio on macOS, rather than opening and tracking their timeline in silence.
* Fixed curly quotes, em dashes and similar characters vanishing from RTF documents, running the surrounding words together as they went.
* Fixed RTF pictures leaking their raw data into the document as garbled text.
* Fixed the Recent Documents submenu keeping stale entries until something else happened to rebuild it.
* Keyboard accelerators are back in every translation, so Russian's menus have keyboard access again.
* Large CHM documents now open up to seven times faster.
* Opened documents are now registered with Windows, so they show up in the taskbar jump list and the Start menu's recent list.
* Options has been renamed to Settings, matching the mobile apps and, on macOS, the platform convention.
* Paperback now remembers its window position, size, and maximized state between runs.
* Plural forms are now translated, so messages that count things read properly in languages that need more than one form.
* Selecting a DAISY book's ncc.html now opens the complete audio book instead of just its text.
* The Customize Keyboard Shortcuts dialog's action names can now be translated.
* The document title now comes first in the title bar, so open books can be told apart in the taskbar and Alt+Tab.
* The update dialog is now translated.

### Version 0.9.0

#### Added

##### General
* A CLI tool, called pb, to quickly convert any of Paperback's supported formats to HTML, Markdown, or plain text.
* An option to reload documents that have been modified by other programs on disk.
* A View Source option to open a document's source in a new tab, useful for editing Markdown for example.
* Document text is now paginated, meaning you can load books with tens of millions of words in only a couple seconds now. Please report any weirdness found with this.

##### Platform Support
* ARM64 Windows support!
* Native macOS support!
* A full screen toggle.

##### All Documents Dialog
* A locate button to locate missing books that just changed their path.
* A status filter and status bar, so you can filter by document status and see how many documents are shown and selected.
* The `Ctrl+Shift+A` shortcut to deselect all documents.

##### Options and Readability
* A readability tab, with the following options:
    * Word wrap (moved from general);
    * Render tables inline (new in this release, see below);
    * Font;
    * Background color;
    * Line spacing;
    * Paragraph spacing;
    * Letter spacing;
    * Text alignment.
* A word wrap menu item and subsequent hotkey.
* A toggle to determine how you want tables displayed, and unified how tables are displayed across documents.

##### Navigation
* Support for navigating by container.
* An option to automatically move the cursor to the start of the line when navigating between lines, similar to browse mode in screen readers.
* The equals keyboard shortcut to announce your current percentage through a document.

##### Bookmarks
* Temporary bookmarks: you can have one per document, and they do persist. Use slash to set one and backslash to jump to it.

##### Word Count
* Estimated reading time in the word count dialog, as well as the ability to set your reading speed to make this metric actually useful.
* If a selection is active when you open the word count dialog, how many words you have selected will now be shown.

##### Keyboard Shortcuts
* The ability to customize every keyboard shortcut in the app through a simple dialog.
* A configurable keyboard shortcut to restore Paperback from the system tray.

##### Languages
* Dutch, Finnish, and Polish.

##### Export
* Expanded the export menu item to allow exporting to HTML and Markdown, in addition to plain text.

##### Updater
* A cancel button to the update-in-progress dialog.
* The updater now validates the downloaded file hasn't been tampered with.

##### Web View
* The webview is now opened at your current reading position.

##### DAISY Books
* Support for DAISY 2.0 books.
* Support for DAISY 2.02 audio playback.

##### Audio Books
* The ability to play audio books, currently supporting both DAISY audio (including DAISY audio + text) and zips of audio files.
* Keyboard shortcuts and menu items to play/pause narration, seek forward and backward, and adjust the seek amount.
* Options to sync the reading caret to audio playback, set the audio seek amount, and choose whether seeking past the end of a chapter continues into the next.

##### CHM Documents
* Support for lists, list items, figures, and images.

##### PowerPoint
* PowerPoint documents now support tables.

#### Fixed

##### General
* Documents encoded in legacy CJK encodings, such as GBK, Big5, and Shift_JIS, will now render properly instead of as a bunch of mojibake.
* "Reopen last closed" attempting to reopen the bundled readme.
* Your selected tab not getting properly focused after restarting Paperback.
* Paperback's handling of files on Windows network drives: pressing show file in folder now properly focuses the file on the network storage, and the paths no longer contain strange characters.
* .paperback files will no longer be forcefully loaded on document restoration; instead, you'll be asked for confirmation when one is found.
* Open containing folder now focuses the given file in explorer.
* Opening the readme will now respect your selected language.
* Paperback's user interface will now scale properly on high-DPI displays.
* The menu now properly updates, and focus moves to the text control, when opening help in Paperback.
* Switched to a much more secure method of IPC on Windows.
* The active document title will now be read when switching between tabs.
* Reduced memory usage on large documents by halving the size of the internal per-character index tables.

##### All Documents Dialog
* Escape not closing the Document Info and All Documents dialogs.
* The title bar not updating after closing a document from the all documents dialog.
* Readme.html will no longer be added to your all documents list when opened via Shift+F1.
* Removing documents from the recents dialog will now also close their active tab.
* Your search filter is now preserved after removing a document.

##### Navigation
* Page navigation announcing incorrect line text in some situations.
* Go to Line, Go to Page, and Go to Percent placing your cursor at the wrong position in large documents.
* Find and Find Next not respecting the loaded document window in large documents.

##### Bookmarks
* Bookmark/note sounds should now properly play exclusively when you navigate over a word containing one.

##### Readability
* Applying word wrap shooting you to the start of your document.

##### Web View
* The webview dialog not being resizable and popping up at a very small initial size.
* Images should now properly display in the embedded webview.

##### Updater
* The updater now properly shows the content of markdown code tags in release notes.

##### DAISY Books
* DAISY books showing incorrect info in the status bar.
* Loading DAISY books with bogus encoding declarations.

##### RTF Documents
* Parsing RTF documents with non-Latin characters in them.
* RTF `\pict` groups so embedded image data no longer leaks into the document text.

##### Mobi/AZW3 Books
* Filepos anchors in Mobi books splitting HTML tags and putting garbage in the book text.
* Links in legacy Mobi books.
* Majorly improved AZW3 parsing.

##### Word Documents
* Word documents with locale-specific style names not rendering their headings properly.

##### HTML/XHTML Documents
* dl, dt, and dd elements not producing line breaks in XHTML documents.

##### PDF Documents
* Paperback now falls back to plain text extraction for falsely-tagged PDFs.
* PDF documents containing control characters in their titles and/or bookmarks will no longer crash Paperback on open.

### Version 0.8.5
* Added page support to epub books.
* Added support for encrypted Microsoft Office documents. Currently Legacy word, modern Word and modern Powerpoint are supported, with legacy Powerpoint planned for the future.
* Added support for legacy Microsoft Word documents!
* Added support for legacy Powerpoint presentations!
* Added support for mobi and AZW3 books!
* Added support for tagged PDF files!
* Added the ctrl+q shortcut to exit the app.
* Added support for zipped books from Bookshare (both DAISY and Word)!
* Alt text for embedded images should now be properly shown.
* CHM documents now properly support internal link navigation.
* Fixed go to page being off by 1.
* Fixed the escape key not working to close the open as dialog.
* Fixed the reader context menu not showing up on right-click or the Applications key.
* Fixed the wrong document sometimes being focused when opening documents from the command line.
* Image only PDFs are once again detected and alert you of their existence.
* It is now possible to navigate through images and figures with g/shift+g and f/shift+f, respectively.
* Paperback will now respect your application dark mode setting.
* Removed DAISY XML support, as its no longer needed.
* Switched back to the native Win32 first letter navigation in the table of contents tree.
* The error loading dialog now shows more detailed error messages.
* The webview will now open much faster and smoother.

### Version 0.8.2
* Added page support to RTF documents!
* Fixed a bug where opening the webview in epubs containing external links would automatically activate them.
* Fixed a bug where the RTF parser wouldn't put a space between words in rare cases.
* Fixed paragraphs being split into multiple short lines in some PDF documents.
* PDF documents now have basic link and heading navigation support!
* RTF tabs and line feeds are now rendered exactly as they appear in the document.
* Switched back to the tried and true pdfium library for parsing PDF's, making PDF rendering much more reliable once again.

### Version 0.8.1
* Added Ctrl+Shift+T to reopen the last closed document.
* The All Documents dialog now supports selecting multiple documents to open at once.
* Fixed a few bugs with the RTF parser.
* Fixed file paths containing non-ASCII characters (such as Bosnian š, č, ć, ž) becoming corrupted when opening a file via a second Paperback instance.
* Fixed PDF text being read in the wrong order, and incorrect spacing around capitalized words.
* Fixed slow document loading when opening large files.
* Fixed the localization of the Yes/No buttons in confirmation dialogs.

### Version 0.8.0
* Added Japanese, simplified Chinese, and Vietnamese translations!
* Added an automatic updater that will now replace your currently installed version of Paperback instead of just downloading the new version!
* Added optional sound feedback for reaching a bookmark or a note, thanks Andre Louis for the sounds!
* Added RTF document support!
* Added support for DAISY XML documents.
* Added support for Flat Open Document Text files!
* Added support for Flat Open Document presentations!
* Added support for separators with s and shift+s.
* Any movement of greater than 300 characters will now automatically add to your navigation history.
* Fixed restoring Paperback's window from the system tray.
* Fixed Markdown documents showing raw text instead of rendered HTML in the Web View.
* Fixed tables not rendering properly in Markdown files.
* Image only PDFs will now warn you of their existence when you attempt to load one.
* Properly embed version information in the Paperback executable.
* Split the options dialog into tabs for ease of use and navigation.
* Switched to Hayro for parsing PDFs, leading to more reliability, speed, and fewer DLLs.
* Rewrote the entire app in Rust. The new codebase is safer, loads documents faster, and is easier to maintain and extend.
* The text control's context menu will now include reader-specific actions instead of generic items such as cut and paste.

### Version 0.7.0
* Added table support for HTML and XHTML-based documents! Navigate between tables using T and Shift+T, and press Enter to view one in a webview.
* Added a basic web rendering feature! Press Ctrl+Shift+V to open the current section of your document in a web-based renderer, useful for content like complex formatting or code samples.
* Added a Russian translation, thanks Ruslan Gulmagomedov!
* Added a Clear All button to the All Documents dialog.
* The update checker now displays release notes when a new version is available.
* Fixed restoring the window from the system tray.
* Fixed Yes/No button translations in confirmation dialogs.
* Fixed loading configs when running as administrator.
* Fixed comment handling in XML and HTML documents.
* Fixed TOC parsing in Epub 2 books.
* Fixed navigating to the next item with the same letter in the table of contents.
* Fixed the find dialog not hiding properly when using the next/previous buttons.
* Fixed epub TOC's occasionally throwing you to the wrong item.
* Fixed various whitespace handling issues in XML, HTML, and pre tags.
* Fixed off-by-one error in link navigation.
* Fixed some books having trailing whitespace on their lines.
* Fixed various parser issues.
* Bookmark-related menu items as well as the elements list are now properly disabled when no document is open.
* Improved list handling in various document formats.
* Improved the translation workflow for contributors.
* Many internal refactors, moving the majority of the application's business logic from C++ to Rust for improved performance and maintainability.

### Version 0.6.1
* Added password-protected PDF support!
* Added a very basic go to previous/next position feature. If you press enter on an internal link and it moves your cursor, that position will now be remembered, and can be navigated to with alt+left/right arrows.
* Added an elements list! Currently it only shows a tree of all the headings in your document or a list of links, but there are plans to expand it in the future.
* Added an option to start Paperback in maximized mode by default.
* Fixed links in some Epub documents not working properly.
* Fixed parsing Epub TOCs containing relative paths.
* Fixed some epub documents not showing a title or author.
* Fixed the titles of some epub chapters not showing up properly in the TOC dialog.
* Fixed you not being able to use the space bar to activate the OK/cancel buttons in the TOC dialog.
* Improved the handling of headings in Word documents.
* You will now get spoken feedback if the recent documents list is empty when you try to bring up the dialog.

### Version 0.6.0
* A new option to show the go menu in a far more compact form has been added to the options dialog, checked by default.
* Added an option to make navigation by structural elements wrap.
* Added an option to the tools menu to open the containing folder of the currently focused document.
* Added a quite simple, but very effective, updating system.
* Added a basic sleep timer feature, accessible with Ctrl+Shift+S.
* Added support for parsing FB2 ebooks!
* Added support for parsing OpenDocument presentations!
* Added support for parsing OpenDocument Text files!
* Bookmarks can now be made to bookmark an entire line, or to mark only some specified text. If you have no selection active when placing a bookmark, the behavior is like pre-0.6, and it'll mark the entire line. However, if you select some text, only that text will be included in the bookmark.
* Bookmarks can now have optional text notes attached to them! Navigate between bookmarks containing notes with N and Shift+N, or pop up the bookmarks dialog with all bookmarks, only notes, or only non-notes selected with specific hotkeys.
* Bookmarks in the bookmarks dialog will no longer have an annoying "bookmark x" prefix.
* Epub books containing HTML content pretending to be XML will now be handled properly.
* Fixed loading large Markdown documents.
* Fixed pressing space in the table of contents tree view activating the OK button.
* Fixed whitespace handling at the beginning of pre tags in both HTML and XHTML documents.
* Fixed the text control not regaining focus sometimes when returning to Paperback's window.
* Fixed the text field in the go to percent dialog not updating the slider's value.
* Fixed the rendering of custom HTML IDs in Markdown documents.
* HTML inside Markdown code blocks will now be rendered properly.
* If loading a book with a command line parameter while an existing Paperback instance is running, you'll no longer get an error if loading your document takes more than 5 seconds.
* If running Paperback as administrator, the configuration will now be properly loaded and saved.
* It is now possible to delete a bookmark directly from within the bookmarks dialog.
* It is now possible to import and export your bookmarks and reading position for a particular document. The generated file is named after the file with a .paperback extension. If such a file is found in the same directory as a file while loading it, it will be automatically loaded. Otherwise, you can manually import them using an item in the tools menu.
* Links inside documents are now fully supported! Use k and shift+k to move forward and backward through them, and press enter to open/activate one.
* Many internal refactors, making the app faster and the binary smaller.
* Markdown content is now preprocessed in order to be CommonMark compliant before rendering.
* Navigation by lists and their items is now fully supported! Use L and Shift+L to go by lists themselves, and I and Shift+I to go through list items.
* Numpad delete now works to remove documents from the tab bar in addition to normal delete.
* Paperback can now optionally minimize to your system tray! This option is off by default, but turning it on will make the minimize option in the system menu put Paperback in your tray, able to be restored by clicking on the spawned icon.
* Paperback is now fully translatable! The list of languages it supports is currently fairly small, but it's constantly growing!
* Paperback now has an official website, at [paperback.dev](https://paperback.dev)!
* PPTX documents will now show a basic table of contents, containing all of the slides.
* The full path to the opened document will now be shown in the document info dialog.
* The installer now includes an option to view the readme in your browser after installation.
* The recent documents list has been dramatically expanded! Instead of simply showing you the last 10 documents you opened, it'll now show you a customizable number, with the rest of the documents you've ever opened being accessible through a small dialog.
* Various small improvements to the parsers across the board, including putting a blank line between slides in PPTX presentations, fixing the newline handling inside of paragraphs in word documents, and adding bullet points to list items.

### Version 0.5.0
* Added Microsoft Word document support!
* Added support for PowerPoint presentations!
* Fixed certain menu items not being disabled with no documents open.
* Fixed the orientation of the go to percent slider.
* Fixed the table of contents in Epub books with URL-encoded file paths and/or fragment IDs.
* Fixed whitespace being stripped from XHTML headings in weird ways.
* Fixed whitespace handling inside of nested pre tags in HTML documents.
* HTML and Markdown documents now support the table of contents feature! When you load an HTML/Markdown document, Paperback will build its own table of contents out of the structure of the headings in your document, and it will show that to you in the ctrl+t dialog.
* HTML documents will now have the title as set in the title tag, if it exists. Otherwise, they'll continue to use the filename without the extension.
* Switched from UniversalSpeech to using a live region to report speech. This means no screen reader DLLs are shipped alongside the program anymore, and more screen readers will now be supported, such as Microsoft Narrator.
* Switched zip libraries to allow for opening a wider array of epub books.
* The dialog asking you if you want to open your document as plain text has been completely redone, and it now allows you to open your document as plain text, HTML, or Markdown.
* The go to percent dialog now includes a text field allowing you to manually input a percentage to jump to.
* The HTML parser will now recognize dd, dt, and dl as list elements.
* The table of contents in Epub books will once again be preserved exactly.
* The unicode non-breaking space is now considered when stripping blank lines.
* You will no longer be asked how you want to open an unrecognized file every single time you load it, only the first time.

### Version 0.4.1
* Added an optional start menu icon to the installer.
* The table of contents should now be cleaner in a few cases, for example if you have a child and parent item with the same text at the same position you'll now only see the parent item.
* Fixed the table of contents in certain CHM documents.
* Fixed the table of contents in Epub 3 books with absolute paths in them.
* CHM documents should now show their title as set in the metadata file.

### Version 0.4.0
* Added CHM file support!
* Added bookmark support! You can have as many bookmarks throughout as many documents as you like. You can jump forward and backward through them with b and shift+b, set one with control+shift+b, and bring up a dialog to jump to a specific bookmark with control+b.
* Added an installer alongside the portable zip file! The installer will install Paperback into your Program Files directory, and automatically set up file associations for you.
* Text files with BOMs should now be decoded properly, and the BOM will no longer be displayed at the beginning of the text either.
* Added far more information to the status bar. It'll now show you your current line, character, and reading percentage.
* HTML comments, as well as the contents of script and style tags, will no longer be shown in text output.
* If passing a relative path to Paperback on the command line, it will now resolve it properly.
* Percentage movement is now handled by its own slider-based dialog, accessible with control+shift+g.
* Documents without known titles or authors will now always have a default.
* The position saving logic is now much smarter and should only write to the disk when absolutely necessary.
* The document you had focused when you closed Paperback is now remembered across application restarts.
* Input into the go to line and go to page dialogs should now be sanitized more strictly.
* Fixed table of contents navigation in epub 3 books with relative paths in their manifests.

### Version 0.3.0
* Fixed the table of contents in epub books with URL-encoded manifests.
* Fixed heading navigation in HTML documents containing multi-byte Unicode characters.
* Fixed high CPU usage in documents with long titles due to a regression in wxWidgets.
* Fixed loading UTF-8 text files.
* Fixed nested TOC items in Epub books putting your cursor at the wrong position.
* Fixed a crash on application exit in certain cases.
* Added a checkbox in the options dialog to enable or disable word wrap!
* It is now possible to donate to Paperback's development, either through the new donate item in the help menu or through the sponsor this project link at the bottom of the GitHub repository's main page.
* Markdown documents will now always have a title, and Paperback should now be able to load virtually any Markdown file.
* PDF documents will now always have a title, even if the metadata is missing.
* Switched PDF libraries to the one used in Chromium, leading to far more reliable PDF parsing across the board.
* You can now only have one instance of Paperback running at a time. Running paperback.exe with a filename while it's already running will open that document in the already running instance.
* You can now press delete on a document in the tab control to close it.

### Version 0.2.1
* Added the total number of pages to the page label in the go to page dialog.
* Allow tabbing from the document content to your list of opened documents.
* Fixed the heading keystrokes sometimes opening recent documents if you had enough of them.
* Paperback will now remove unnecessary soft hyphens from text output.
* Fixed heading navigation sometimes putting you on the wrong character.

### Version 0.2.0
* Added markdown document support!
* Added PDF document support, including the ability to navigate between pages!
* Added keystrokes for navigating by headings in HTML content, including epub books and markdown documents. These keystrokes were designed to work similar to a screen reader.
* Fixed loading epubs with URL-encoded filenames in their manifests.
* Fixed loading epub 3 books with XHTML embedded inside of them.
* A message is now spoken if the document doesn't support a table of contents or sections, as opposed to the menu items being disabled.
* Added a recent documents menu! It currently stores your last 10 opened documents, and pressing enter on one will open it for reading.
* Completely rewrote the Find dialog, making it much simpler to use, while also adding a history of your last 25 searches and regular expression support!
* Previously opened documents are now remembered across application restarts. This is configurable through the new options item in the tools menu.
* Added shift+f1 to open the readme directly in Paperback itself.

### Version 0.1.0
* Initial release.
