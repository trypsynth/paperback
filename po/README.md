# Paperback Translation Guide
Thank you for your interest in translating Paperback! This is a complete guide walking you through all the steps of how to easily add a new language, as well as how to update it when new strings get added.

## Starting a New Translation
1. Generate the .pot template (normally not necessary, just in case):
```bash
cmake --build build --target update-pot
```
2. Create your language file:
```bash
# Replace 'es' with your language code.
cp paperback.pot es.po
```
3. Edit the header in your `.po` file to have correct information about your language and who contributed it.
4. Translate the strings using your editor of choice.
5. Test your translation by recompiling Paperback and selecting it from the options dialog.

## Updating an Existing Translation
When new strings are added to Paperback:
1. Update the template (if not already done):
```bash
cmake --build build --target update-pot
```
2. Merge new strings into your translation:
```bash
msgmerge -U es.po paperback.pot
```
3. Translate the new/fuzzy strings. Anything marked with fuzzy needs updated, and any empty strings are brand new.
4. Remove fuzzy markers once done reviewing.
5. Compile and test (same as step 5 above).

## Notes
1. Keyboard Shortcuts: Keep the `\t` and key name:
2. Format Strings: Keep the `%s`, `%d`, and similar placeholders:
3. Ampersands (&): Used for keyboard shortcuts. Choose different letters if the same key is used twice in a menu.

## License
All translations are distributed under the same MIT license as Paperback.
