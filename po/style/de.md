Notes for the machine translator on German. Written in English, because the model reads it.

Every rule here is taken from what Steffen Schultz already wrote in `po/de.po` and
`doc/readme-de.md`, so following them keeps new text consistent with the German that is already
shipped rather than introducing a second style beside it.

## Register

- Address the reader as `du`: `dir`, `dein`. Never `Sie` or `Ihr`. The German readme uses `du`
  and the catalogue has no `Sie` in it at all.
- Prefer an impersonal sentence where German allows one, the way the catalogue does. "Saves your
  exact reading position" is `Speichert die genaue Leseposition`, not `Speichert deine genaue
  Leseposition`. Use `du` only where a pronoun is genuinely needed.

## Punctuation

- German quotation marks: `„so“`, never `"so"` and never `“so”`.
- Keep the ellipsis on menu items that open a dialog, exactly as the English has it.

## Keyboard shortcuts

- Key names stay as the English source writes them: `Ctrl`, `Shift`, `Alt`, `Enter`, `Esc`.
  Do not translate them to `Strg`, `Umschalt` or `Eingabe`.
- This is not an oversight. The German interface itself labels the menu accelerators `Ctrl+O`
  and so on, so a manual that said `Strg+O` would disagree with the program it documents.
  Whatever is inside backticks is what the user presses and reads on screen.

## Anglicisms

- Use the German term when the catalogue already has one. OCR is `Texterkennung`, so "Batch OCR"
  is `Texterkennung (Stapelverarbeitung)`, not `Batch-OCR`.
- When an English word really is the normal German usage, get its gender right. It is
  `der Workaround`, though `die Umgehungslösung` reads better and is preferred.
- Software that runs without installation is `portabel`, never `tragbar`, which means carryable.
- Software that is small and quick is `schlank`, never `leicht`, which reads as easy or
  lightweight in the physical sense.

## Consistency with the interface

The manual names menus, dialogs and buttons that the interface also names. Take the wording from
`po/de.po` rather than translating the English again, so the manual and the program agree. For
example the File menu is `Menü „Datei“` and the Go to menu is `Menü „Gehe zu“`.

## Glossary

English → German. Use these consistently.

- announce → ankündigen
- bookmark → Lesezeichen
- clipboard → Zwischenablage
- heading → Überschrift
- keyboard shortcut → Tastenkombination
- note → Notiz
- OCR → Texterkennung
- portable → portabel
- reading position → Leseposition
- screen reader → Bildschirmleser
- table of contents → Inhaltsverzeichnis

Format and product names stay in English: EPUB, PDF, DAISY, MOBI, Markdown, RTF, WinHelp,
VoiceOver, TalkBack, JAWS, NVDA.
