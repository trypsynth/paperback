---
layout: default.liquid
title: Translations
permalink: /translations
---
## Translation Guide
Thank you for your interest in translating Paperback! This page walks you through how to start a brand new translation, how to keep it updated when new strings land, and how to submit your work.

Prerequisites: Install the gettext tools (`xgettext`, `msgmerge`, `msgfmt`) and ensure they are available on your `PATH`.

All translation files live in the `po` directory. The template file is `paperback.pot`, and each language is stored as `po/<lang>.po` using the standard language code (for example: `es`, `pt_BR`, `de`).

## Starting a New Translation
1. Generate the `.pot` template (normally not necessary, but it keeps everything in sync):
```
cmake --build build --target update-pot
```
2. Create your language file:
```
cp paperback.pot your-lang-code.po
```
3. Edit the header in your `.po` file with the correct language name, your name, and email.
4. Translate the strings using your editor of choice. Keep placeholders and shortcut markers intact (see Notes below).
5. Verify the file compiles cleanly:
```
msgfmt --check --verbose es.po
```
6. Test your translation by recompiling Paperback and selecting it from the options dialog.

## Updating an Existing Translation
When new strings are added to Paperback:
1. Update the template (if not already done):
```
cmake --build build --target update-pot
```
2. Merge new strings into your translation:
```
msgmerge -U es.po paperback.pot
```
3. Translate the new or fuzzy strings. Anything marked with `fuzzy` needs a review, and any empty strings are brand new.
4. Remove fuzzy markers once done reviewing.
5. Compile and test (same as step 6 above).

## Submitting Your Translation
1. Fork the repository and create a branch.
2. Commit your updated `po/<lang>.po` file (and `paperback.pot` only if you regenerated it).
3. Open a pull request here: [Create a pull request](https://github.com/trypsynth/paperback/compare).

If you prefer not to use Git, you can also attach your `.po` file to an issue and we'll merge it for you.

## Notes
1. Keyboard shortcuts: Keep the `\t` and key name together.
2. Format strings: Keep the `%s`, `%d`, and similar placeholders.
3. Ampersands (&): Used for keyboard shortcuts. Choose different letters if the same key is used twice in a menu.
4. Punctuation and spacing: Match the source string for ellipses, colons, and spacing so UI alignment stays consistent.
5. Encoding: Files should stay UTF-8 without a BOM.
