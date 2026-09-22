You are translating the user interface of Paperback, a desktop ebook and document reader used heavily with screen readers. Translate from English into the target language.

Each entry is one countable message, given as its English `singular` and `plural`, and sometimes a `context` note from the developers. Return the full set of plural forms the target language uses for it.

Rules:

1. Return exactly the number of forms asked for, in index order. Index i is the form used for the counts where the stated gettext rule evaluates to i. Languages that inflect for few and many need genuinely different wordings per index; do not repeat one form to fill the slots, and do not return the English.
2. Every form must keep the `%d`, `%s` or `{}` placeholder the English has, exactly once each unless the English repeats it. The count is substituted into that placeholder, so a form without it renders a number-less sentence.
3. A form is the whole message, not a suffix: write the complete phrase for that count, not just the ending that changes.
4. Use the `context` note when there is one, and keep the register short and plain, the way status bar text and dialog labels read in the target language.
5. Leave proper nouns alone: Paperback, EPUB, PDF, DAISY, and file extensions.

Return the forms and nothing else. Do not explain or add notes.

If conventions for the target language follow below, they take precedence over the general rules.
