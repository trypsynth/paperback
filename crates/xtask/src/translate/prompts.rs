//! What each kind of request asks the model for: the system prompt that tells it the rules, and
//! the schema its answer has to fit.

use serde_json::{Value, json};

pub(super) fn translations_schema() -> Value {
	json!({
		"type": "object",
		"properties": {
			"translations": {
				"type": "array",
				"items": {
					"type": "object",
					"properties": {
						"id": { "type": "integer" },
						"text": { "type": "string" }
					},
					"required": ["id", "text"],
					"additionalProperties": false
				}
			}
		},
		"required": ["translations"],
		"additionalProperties": false
	})
}

/// Schema for a plural batch.
///
/// The form count is deliberately not pinned here. The obvious spelling - `minItems`/`maxItems`
/// set to `nplurals` - is rejected outright: the API supports `minItems` of 0 or 1 only, and a
/// request asking for `[2, 5]` comes back as a 400 for the whole batch. So the schema asks only
/// for a non-empty array of strings, the prompt states the exact count, and [`check_plural`]
/// enforces it on the way back, which it has to do regardless.
pub(super) fn plural_schema() -> Value {
	json!({
		"type": "object",
		"properties": {
			"translations": {
				"type": "array",
				"items": {
					"type": "object",
					"properties": {
						"id": { "type": "integer" },
						"forms": {
							"type": "array",
							"items": { "type": "string" },
							"minItems": 1
						}
					},
					"required": ["id", "forms"],
					"additionalProperties": false
				}
			}
		},
		"required": ["translations"],
		"additionalProperties": false
	})
}

pub(super) fn markdown_schema() -> Value {
	json!({
		"type": "object",
		"properties": { "markdown": { "type": "string" } },
		"required": ["markdown"],
		"additionalProperties": false
	})
}

pub(super) const fn phrase_system_prompt() -> &'static str {
	"You are translating the user interface of Paperback, a desktop ebook and document reader \
	 used heavily with screen readers. Translate from English into the target language.\n\
	 \n\
	 You receive a JSON array of entries. Each has an `id`, the English `text`, and sometimes a \
	 `context` note written by the developers. Return a translation for every id you are given.\n\
	 \n\
	 Rules, in order of importance:\n\
	 \n\
	 1. `&` immediately before a letter marks that letter as the keyboard accelerator for a menu \
	 item or button. If the English has one, the translation MUST have exactly one too. Put it \
	 before a letter that actually occurs in your translated text, preferring the first letter of \
	 a main word. Never drop it, never leave it before a letter the translation does not contain, \
	 and never add one where the English had none. This is a real accessibility feature, not \
	 decoration.\n\
	 2. A literal `\\t` (backslash then t) separates a label from its keyboard shortcut, as in \
	 `&Open\\tCtrl+O`. Translate only the part before it. Reproduce the `\\t` and everything after \
	 it byte for byte: key names like Ctrl, Shift, Alt, Enter and F1 are not translated.\n\
	 3. Preserve every `%s`, `%d` and `{}` placeholder. The count of each must match the English \
	 exactly. Their order may change to suit the target grammar.\n\
	 4. Use the `context` note when there is one. It exists because the string is ambiguous \
	 without it, and it usually says where the string appears or what a placeholder holds.\n\
	 5. Keep the register of desktop software in the target language, and keep it short: these are \
	 menu items, buttons, status bar text and dialog labels, sitting in a fixed amount of space.\n\
	 6. Be consistent. The same English term should get the same translation everywhere in the \
	 batch.\n\
	 7. Leave proper nouns alone: Paperback, EPUB, PDF, DAISY, HTML, Markdown, file extensions, \
	 and URLs.\n\
	 \n\
	 Translate the text and nothing else. Do not explain, comment, or add notes."
}

pub(super) const fn plural_system_prompt() -> &'static str {
	"You are translating the user interface of Paperback, a desktop ebook and document reader \
	 used heavily with screen readers. Translate from English into the target language.\n\
	 \n\
	 Each entry is one countable message, given as its English `singular` and `plural`, and \
	 sometimes a `context` note from the developers. Return the full set of plural forms the \
	 target language uses for it.\n\
	 \n\
	 Rules:\n\
	 \n\
	 1. Return exactly the number of forms asked for, in index order. Index i is the form used \
	 for the counts where the stated gettext rule evaluates to i. Languages that inflect for \
	 few and many need genuinely different wordings per index; do not repeat one form to fill \
	 the slots, and do not return the English.\n\
	 2. Every form must keep the `%d`, `%s` or `{}` placeholder the English has, exactly once \
	 each unless the English repeats it. The count is substituted into that placeholder, so a \
	 form without it renders a number-less sentence.\n\
	 3. A form is the whole message, not a suffix: write the complete phrase for that count, \
	 not just the ending that changes.\n\
	 4. Use the `context` note when there is one, and keep the register short and plain, the \
	 way status bar text and dialog labels read in the target language.\n\
	 5. Leave proper nouns alone: Paperback, EPUB, PDF, DAISY, and file extensions.\n\
	 \n\
	 Return the forms and nothing else. Do not explain or add notes."
}

pub(super) const fn markdown_system_prompt() -> &'static str {
	"You are translating the user documentation for Paperback, a desktop ebook and document \
	 reader, from English into the target language.\n\
	 \n\
	 You receive part of a Markdown document. Return the same document translated, as Markdown.\n\
	 \n\
	 Rules:\n\
	 \n\
	 1. Preserve the Markdown structure exactly: heading levels, list nesting, tables, emphasis, \
	 blockquotes, and the blank lines between blocks.\n\
	 2. Do not translate anything inside backtick code spans or fenced code blocks. Commands, \
	 file names, file extensions, keyboard shortcuts and configuration keys stay verbatim. This \
	 includes key names that are ordinary words: `Alt+Left`, `Ctrl+Space`, `Shift+Home` and \
	 `Page Down` keep their English key names, because they name physical keys rather than \
	 describing a direction.\n\
	 3. In links, translate the link text but never the URL.\n\
	 4. Leave proper nouns alone: Paperback, EPUB, PDF, DAISY, and the names of formats and \
	 programs.\n\
	 5. Translate the prose fully and naturally. Do not summarise, expand, or add notes.\n\
	 \n\
	 Return only the translated Markdown."
}

#[cfg(test)]
mod tests {
	use super::*;

	// The count cannot live in the schema: the API takes minItems of 0 or 1 only, and rejects
	// the whole request otherwise. check_plural is what enforces it.
	#[test]
	fn the_plural_schema_asks_only_for_a_non_empty_array() {
		let schema = plural_schema();
		let forms = &schema["properties"]["translations"]["items"]["properties"]["forms"];
		assert_eq!(forms["minItems"], 1);
		assert!(forms["maxItems"].is_null());
	}
}
