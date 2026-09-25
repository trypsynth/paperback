---
layout: default.liquid
title: Translations
permalink: /translations
description: How to translate Paperback into your language, how the machine translator works, how to steer it, and how to opt out of it.
---

# Translation Guide

Thank you for your interest in translating Paperback. This page covers starting a new translation, keeping it up to date, and submitting your work. It also explains what the machine translator does, how to make it write your language the way you would, and how to switch it off for your language entirely.

All translation files live in the `po` directory of the repository. The template is `paperback.pot`, and each language is `po/<lang>.po`.

Use the code as it already appears in the `po` directory. Most are a bare language code (`es`, `de`, `fi`), and where a region is part of it, the existing files are the guide: `pt_br` is lower case and `zh_CN` is not. A new file should follow whichever form its language normally uses.

## Prerequisites

- gettext tools (`xgettext`, `msgmerge`, `msgfmt`) on your `PATH`, for merging and compiling translations.
- [Poedit](https://poedit.net/download) (optional but recommended), a graphical editor for `.po` files. Any text editor works too.
- Git and a GitHub account to submit your work.

## Starting a new translation

1. Fork the repository on GitHub and clone your fork locally:

```bash
git clone https://github.com/trypsynth/paperback.git
cd paperback
```

2. Create a branch for your translation:

```bash
git checkout -b danish-translation
```

3. Copy the template to create your language file:

```bash
cp po/paperback.pot po/da.po
```

4. Edit the header in your `.po` file with the correct language name, your name, and email.

5. Translate the strings. Keep placeholders and shortcut markers intact (see [Notes](#notes) below). Poedit will highlight warnings and errors as you go.

6. Verify the file compiles cleanly:

```bash
msgfmt --check --verbose po/da.po
```

7. Test your translation by recompiling Paperback and selecting it from the options dialog.

## Updating an existing translation

When new strings are added to Paperback:

1. Switch to your translation branch and pull the latest changes:

```bash
git checkout danish-translation
git pull origin danish-translation
```

2. Sync with the upstream repository if you have not already:

```bash
git remote add upstream https://github.com/trypsynth/paperback.git  # first time only
git fetch upstream
git merge upstream/master
```

3. Merge new strings into your translation:

```bash
msgmerge -U po/da.po po/paperback.pot
```

4. Translate any new or fuzzy strings. Entries marked `fuzzy` need review; empty entries are brand new.

5. Remove fuzzy markers once reviewed, then compile and test (same as steps 6 and 7 above).

## The machine translator

Any language **not** listed in `po/human-maintained-locales.txt` is filled in automatically. A GitHub Actions workflow runs on every push to master that changes a translatable source file, and it does this:

1. Regenerates `po/paperback.pot` from the source.
2. Runs `msgmerge` over every `.po` file, with `--no-fuzzy-matching`, so a new string arrives as a blank entry rather than a guess copied from a similar old one.
3. Sends the blank entries to the Claude API, one batch per language, passing each string's `#. TRANSLATORS:` note along with it.
4. Also re-translates any entry whose existing translation provably dropped a `%s`, a `{}`, an `&` accelerator or a `\t` shortcut suffix.
5. Machine-translates `doc/readme.md` into `doc/readme-<lang>.md` for the same languages.
6. Opens or updates a pull request called **chore: machine-translate missing strings**.

Three things about it are worth knowing.

**Machine translations are marked `fuzzy`, and fuzzy entries still ship.** The `fuzzy` marker means "a human has not reviewed this", not "this is switched off". If your language is machine translated, what the model wrote is what users are reading.

**A filled entry is never revisited.** The translator only touches entries that are blank or provably damaged. A translation that is simply wrong, but has the right placeholders, stays exactly as it is until a person changes it. So reviewing the machine output is worth doing, and a correction you make by hand will not be overwritten.

**Your own work is never touched.** A non-fuzzy entry was written by a person, and nothing in the normal flow overwrites one.

## Steering the machine translator

If your language is machine translated and the wording is not how you would put it, you can fix that once instead of correcting it every release. Write your conventions in `po/style/<lang>.md`, and that file is given to the translator every time your language runs.

Write it in English, because the model is what reads it. `po/style/nl.md` and `po/style/de.md` are worked examples. The things worth putting in it:

- **Register.** Formal or informal, and which pronoun forms follow from that.
- **How labels should read.** Whether menu items and buttons take an infinitive, an imperative, or a noun.
- **Capitalisation.** Most languages use sentence case for labels where English uses Title Case, and saying so stops the English shape being carried over.
- **Words that stay in English.** Format and product names, and any term your users expect untranslated.
- **A glossary.** English to your language, for the terms you want rendered the same way every time.
- **Anything structural.** Compound spelling, article rules, word order, quotation marks.

Its conventions take precedence over the translator's general rules. If the file is missing, nothing changes. To check it has been picked up:

```bash
cargo xtask translate --dry-run
```

Your language's line will say `with po/style/<lang>.md`.

A style note only affects entries translated after you add it. It does not rewrite what is already in your `.po`, because a filled entry is never revisited.

## Opting out of machine translation

If you would rather maintain your language entirely by hand, add your locale code on its own line to `po/human-maintained-locales.txt`:

```
bs
cs
da
fi
```

The translator then skips your language completely, both `po/<lang>.po` and `doc/readme-<lang>.md`, and will not fill in a blank or a fuzzy entry even if one is sitting there. Untranslated strings fall back to English in the interface, which is the trade: nothing is written for you, and nothing is written over you.

Removing your line opts back in. Neither direction is a commitment, and switching either way is a one-line pull request.

## Translating the manual

The user manual is `doc/readme.md`. To translate it, create `doc/readme-<lang>.md` (for example `doc/readme-da.md`) using the English one as your starting point. The build picks it up automatically, publishes it at `paperback.dev/readme-<lang>.html`, and embeds it in the program for the Help menu. Submit it in the same pull request as your `.po` file.

A machine-translated manual is written one heading at a time, and every section is checked against its English source before anything is written: the same headings at the same levels, the same number of list items, the same number of code fences. A section that comes back short is retried, and if it keeps coming back short the run reports which section failed and writes nothing, rather than publishing a manual with a piece missing.

That check also runs against hand-written manuals when we go looking, which is how we find a section that was added to the English manual and never made it into a translation. If yours is a few sections behind, that is fine and nobody is counting; it just means the English text shows through in places.

## Notes

1. **Keyboard shortcuts.** Six strings carry their shortcut inside the translation, after a `\t`: Undo, Redo, Cut, Copy, Paste and Select All. In those the text after the tab is what the toolkit turns into the actual key, so keep `Ctrl+Z` and the rest exactly as the English has them. Translating them stops the shortcut working.
2. **Key names elsewhere.** Every other menu shortcut is generated from the user's configuration rather than from the catalogue, in English (`Ctrl`, `Shift`, `Alt`). Nothing you write in the `.po` changes that, so the manual should use the same English key names as the program shows. Localized key names are planned; if you want them for your language, say so in [the translation discussion](https://github.com/trypsynth/paperback/discussions/910).
3. **Format strings.** Keep `%s`, `%d` and `{}` exactly as they appear, and in an order that makes sense in your language.
4. **Ampersands.** `&` marks the accelerator letter in a menu item or a button. Keep one, and choose a different letter if the English letter would collide with another item in the same menu.
5. **Punctuation and spacing.** Match the source for ellipses, colons and trailing spaces, so the interface lines up.
6. **Encoding.** UTF-8 without a BOM.
7. **`.mo` files.** Do not commit them. They are compiled at build time.

## Submitting your translation

1. Commit your `.po` file and push:

```bash
git add po/da.po
git commit -m "Add Danish translation"
git push origin danish-translation
```

2. Open a pull request at [https://github.com/trypsynth/paperback/compare](https://github.com/trypsynth/paperback/compare). Make sure the fields are set correctly:

   - base repository: `trypsynth/paperback`
   - base: `master`
   - head repository: your fork
   - compare: your translation branch

If you prefer not to use Git, attach your `.po` file to an issue and we will merge it for you.

Questions, or anything that does not make sense here, go in [the translation discussion](https://github.com/trypsynth/paperback/discussions/910).
