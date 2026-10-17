Notes for the machine translator on Hungarian. Written in English, because the model reads it.

The base is the Microsoft Hungarian Localization Style Guide and Microsoft Terminology, because
most Paperback users are on Windows. Where Apple's macOS and iOS Hungarian differs, the choice
made here is noted. Spelling follows `A magyar helyesírás szabályai`, 12th edition (AkH. 12).

## Register

- Address the reader formally, in the polite third person (önözés): `Válassza ki`, `Adja meg`,
  `Biztosan törölni szeretné?`. Never use the informal forms (`Válaszd ki`, `Add meg`).
- Avoid the pronoun `Ön` and possessive suffixes where Hungarian can do without them.
  "Your reading position" is `az olvasási pozíció`, not `az Ön olvasási pozíciója`.
- Write plain, everyday Hungarian, not officialese: `ha`, not `amennyiben`; `szeretne`, not
  `kíván`; `megpróbál`, not `megkísérel`; never `kerül végrehajtásra` or similar passive
  constructions with `kerül`.
- Drop "Please". "Please select a bookmark to jump to." is `Válassza ki, melyik könyvjelzőre
  szeretne ugrani.`
- Prefer a suffix to a heavy postposition: `a billentyűzettel`, not `a billentyűzet
  segítségével`.
- Use generic verbs that work with every input method: `válassza ki`, not `kattintson rá`, unless
  the English really means clicking.

## Interface labels

- Menu items, buttons, and commands use the action noun (`-ás`/`-és`), the way Windows and macOS
  both do: `Megnyitás`, `Bezárás`, `Mentés másként`, `Exportálás`, `Újratöltés`.
- "Previous X" and "Next X" are `Előző X` and `Következő X`.
- "Go to X" and "Jump to X" are `Ugrás` plus the target in the right case: `Ugrás az oldalra`,
  `Ugrás a könyvjelzőre`. A bare "Go to" with a dialog is `Ugrás...`.
- "Toggle X" is `X be- és kikapcsolása`.
- Checkbox labels that start with "Show" are `X megjelenítése`. Others are noun phrases:
  "Word wrap" is `Sortörés`.
- Help text in the status bar that describes a command, written in English as an imperative
  without a final period, is an action-noun phrase without a period: "Copy from the beginning of
  the selection to here" is `Másolás a kijelölés elejétől az aktuális helyig`.
- Status messages and screen reader announcements are short sentences or verbless statements:
  "No next note." is `Nincs következő jegyzet.`, "Temporary bookmark set." is
  `Ideiglenes könyvjelző beállítva.`
- Errors: "Cannot X" and "Could not X" use the `nem -ható/-hető` form where it fits:
  `A fájl nem nyitható meg.` "Failed to X" is `Nem sikerült` plus the infinitive:
  `Nem sikerült exportálni a beállításokat.` "Could not find X" is `X nem található.`
- Questions are polite and short: "Do you want to continue?" is `Folytatja?`.

## Capitalization

- Sentence-style capitalization only: the first word of a label, title, or heading is
  capitalized and nothing else. "Previous Heading Level 1" is `Előző 1. szintű címsor`.
- With a slash, only the first item is capitalized: `Keresés/csere`.
- Format, product, and program names keep their own spelling: EPUB, PDF, DAISY, MOBI,
  Markdown, Windows, macOS, iOS, Android, Linux, VoiceOver, TalkBack, JAWS, NVDA.
- `internet`, `web`, `e-könyv`, and the names of keys in running text are lowercase common
  nouns.

## Grammar

- Hungarian uses the definite article where English has none: "Using Paperback" is
  `A Paperback használata`. Use `a` before a consonant sound and `az` before a vowel sound.
- After a number, the noun stays singular: `3 dokumentum`, `%d oldal`. This also applies to
  every plural form. The header is `Plural-Forms: nplurals=2; plural=(n != 1);`, and in most
  messages both forms are identical: `%d dokumentum.`
- A placeholder can never take a suffix or an article, because its value is unknown. Rephrase
  so that it stands where it needs neither, usually after a colon: "Checking %s" is
  `Ellenőrzés: %s`, "Couldn't extract the document from the archive: {}" is
  `Nem sikerült kibontani a dokumentumot az archívumból: {}`. Use `a(z)` only if no rephrasing
  works.
- With "nem", a prefixed verbal adjective splits: `nem küldhető el`, never `nem elküldhető`.
- Suffixes on names follow pronunciation. Attach them directly to names written and pronounced
  in a Hungarian-compatible way (`Paperbacket`, `Paperbackben`, `Windowson`, `Androidon`,
  `Linuxon`, `VoiceOverrel`, `TalkBackkel`) and with a hyphen to acronyms and names ending in a
  letter that is not pronounced (`PDF-ben`, `EPUB-ot`, `HTML-t`, `URL-t`, `NVDA-val`,
  `JAWS-szal`, `macOS-en`, `iOS-en`).

## Compounds

- Follow AkH. 12. A compound of two words is written as one word, however long:
  `dokumentuminformáció`, `szövegfelismerés`. An adjective with its noun is not a compound and
  stays two words: `olvasási pozíció`, `ideiglenes könyvjelző`.
- A compound of more than two words is one word up to six syllables and takes a hyphen from
  seven syllables up (the 6-3 rule): `könyvjelzőjegyzet`, `könyvjelzőlista`, but
  `billentyűparancs-beállítások`.
- A compound with an acronym, a format name, or a product name always takes a hyphen:
  `PDF-fájl`, `EPUB-könyv`, `HTML-forrás`, `OCR-szöveg`, `Windows-telepítő`.
- Prefer a short phrase to a long, artificial compound: `a billentyűparancsok testreszabása`,
  not `billentyűparancstestreszabás`.

## Punctuation

- Hungarian quotation marks: `„így”`. It opens with U+201E and closes with U+201D. Never use
  ASCII `"` or `“…”`. Inside quotation marks use `»…«`.
- Keep the three-dot ellipsis `...` exactly as the English writes it, with no space before it.
- Keep colons and terminal periods as the English has them. After a colon, a complete sentence
  starts with a capital letter.
- No em dash. A dash between clauses is the en dash with a space on each side: ` – `. A range is
  the en dash without spaces: `10–20`.
- Do not use a comma before `és`, `vagy`, or `stb.` in a list. Always put a comma before `illetve`,
  `valamint`, and between clauses.
- Spell out `és` instead of `&` in running text, because screen readers read symbols. In a menu
  label `&` is the accelerator marker, not a symbol.
- Thousands are separated by a space, decimals by a comma: `1 000`, `1,5`.

## Keyboard

- Key names stay exactly as the English source writes them: `Ctrl`, `Alt`, `Shift`, `Enter`,
  `Esc`, `Page Down`, `Cmd`, `Option`. Microsoft Hungarian writes `Control`, but Paperback
  generates its menu shortcuts in English, and the manual must match what the user sees.
  In the manual this applies to everything in backticks. In running text outside backticks,
  where the English names a key in words, write it in Hungarian: "Alt+Left/Right" is
  `Alt+Balra/Jobbra nyíl`, "the Up and Down arrows" is `a felfelé és a lefelé nyíl`, "the space
  bar" is `a szóköz billentyű`, "the context menu key" is `a helyimenü-billentyű`.
- Keep `Space` in English in the catalog too: the window hotkey dialog reads its key field back
  and recognizes only the English word.
- Accelerators (`&`): pick a letter that is unique within the menu or dialog, in this order of
  preference: the first letter of the label, the first letter of another word, the first letter
  of a later part of a compound, then any other plain letter. Use an accented letter (`á`, `é`,
  `ő`, `ű`) only when no other letter is free.

## Microsoft and Apple differences

Where the two differ, the Microsoft form is used:

- Cancel → `Mégse` (Apple: `Mégsem`)
- Select All → `Az összes kijelölése` (Apple: `Összes kijelölése`)
- Go Back → `Vissza` (Apple: `Visszalépés`); Go Forward → `Előre` (Apple: `Előrelépés`)
- Section → `szakasz` (Apple: `szekció`)

## Menus

- File → `Fájl`; Edit → `Szerkesztés`; View → `Nézet`; Go → `Ugrás`; Bookmarks → `Könyvjelzők`;
  Audio → `Hang`; Tools → `Eszközök`; Help → `Súgó`
- Menu bar accelerators: `&Fájl`, `&Szerkesztés`, `&Nézet`, `&Ugrás`, `&Könyvjelzők`, `&Hang`,
  `&Eszközök`, `Sú&gó`

## Glossary

English → Hungarian. Use these consistently.

- announce → bejelent (a screen reader announcement is `bejelentés`)
- app, application → alkalmazás; on iOS and Android `app` is acceptable in running text
- app name placeholder: when a `%s` or `{}` is the application's name, write it as if it were
  `Paperback`, with the article: "A new version of %s is available" is `A %s új verziója
  elérhető.`
- archive → archívum; open from archive → megnyitás archívumból
- audio narration → audionarráció; audiobook → hangoskönyv
- audio seek amount → léptetési időegység (the setting lists units of time); seek
  forward/backward → előretekerés/visszatekerés
- batch OCR → kötegelt szövegfelismerés
- bookmark → könyvjelző; note → jegyzet; bookmark note → könyvjelzőjegyzet; bookmark with note →
  könyvjelző jegyzettel; temporary bookmark → ideiglenes könyvjelző
- braille display → braille-kijelző
- clipboard → vágólap
- container → tároló; list → lista; list item → listaelem; table → táblázat; figure → ábra;
  image → kép; formula → képlet; link → hivatkozás, never `link`; web link → webhivatkozás
- document data (.paperback file) → dokumentumadatok; document info → dokumentuminformáció
- elements list → elemlista
- e-book → e-könyv
- export to X, export as X → exportálás X-ként, the same form for every format: `Exportálás
  egyszerű szövegként`, `Exportálás HTML-ként`, `Exportálás Markdownként`; never `HTML-be` or
  `Markdownba`
- find → keresés; find next/previous → következő/előző keresése; match case → kis- és nagybetűk
  megkülönböztetése; whole word → csak teljes szavak
- file manager → fájlkezelő; file browser (in the app) → fájlböngésző
- heading → címsor; heading level 2 → 2. szintű címsor
- keyboard shortcut → billentyűparancs
- log → napló; log level → naplózási szint
- OCR → szövegfelismerés (the acronym `OCR` is fine where space is short)
- open from URL → megnyitás webcímről; URL (on its own) → webcím
- page → oldal; line → sor; paragraph → bekezdés; word → szó; character → karakter
- percent → százalék
- reading position → olvasási pozíció, never `olvasási hely`
- read aloud → felolvasás; speech rate → beszédsebesség; pitch → hangmagasság; voice (a
  text-to-speech voice) → beszédhang, never `hang`; text to speech → szövegfelolvasás
- recent documents → legutóbbi dokumentumok
- screen reader → képernyőolvasó
- section → szakasz
- selection → kijelölés; selection start → kijelölés eleje
- settings → beállítások; option → beállítás
- share → megosztás
- sleep timer → elalvási időzítő
- swipe (touch gesture, on iOS and Android alike) → legyintés; swipe up → felfelé legyintés;
  never `csúsztatás`, which is a different gesture
- system tray → rendszertálca
- table of contents → tartalomjegyzék
- word wrap → sortörés

## JAWS

When the text names something in JAWS, use the exact wording of the Hungarian JAWS (JAWS 2026 HUN),
even where it reads oddly, because that is what the user sees and searches for:

- Settings Center → `Beállítási központ`
- panning (moving a braille display along the text) → `léptetés`; pan forward → `előre léptet`
- Braille Presentation and Panning → `Braille-leképezés és léptetés`
- Always use DOM if available. → `Ha van, mindig a DOM használatával.`
- Pan Text by Paragraph → `Szöveg léptetése bekezdésenként`
- Enable Accessible Notification Events → `Elérhető értesítési események engedélyezése`; in Settings
  Center the user searches for `értesítés`
- the Say Paragraph command → `Bekezdés felolvasása`
