---
layout: default.liquid
title: Translations
permalink: /translations
---

# Translation Guide

Thank you for your interest in translating Paperback! This page walks you through how to start a new translation, how to keep it updated, and how to submit your work.

All translation files live in the `po` directory of the repository. The template is `paperback.pot`, and each language is stored as `po/<lang>.po` using the standard language code (for example: `es`, `pt_BR`, `de`).

## Prerequisites

- gettext tools (`xgettext`, `msgmerge`, `msgfmt`) on your `PATH`, for merging and compiling translations.
- [Poedit](https://poedit.net/download) (optional but recommended), a graphical editor for `.po` files. Any text editor works too.
- Git and a GitHub account to submit your work.

## Starting a New Translation

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

## Updating an Existing Translation

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

## Translating the README

If you want to translate the user-facing documentation as well, create a `doc/readme-<lang>.md` file (for example `doc/readme-da.md`) using the English `doc/readme.md` as your starting point. The build system picks it up automatically and embeds the rendered HTML into the binary alongside the UI strings. Submit it in the same pull request as your `.po` file.

## Submitting Your Translation

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

## Notes

1. Keyboard shortcuts: Keep the `\t` and key name together.
2. Format strings: Keep `%s`, `%d`, and similar placeholders exactly as they appear.
3. Ampersands (`&`): Used for menu keyboard shortcuts. Choose different letters if the same key would appear twice in a menu.
4. Punctuation and spacing: Match the source string for ellipses, colons, and spacing so UI alignment stays consistent.
5. Encoding: Files must be UTF-8 without a BOM.
6. `.mo` files: Do not commit `.mo` files. They are compiled at build time and are not needed in the repository.
