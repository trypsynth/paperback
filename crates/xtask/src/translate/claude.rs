//! Machine translation through the Claude API.
//!
//! This replaces a dedicated machine-translation API, and the reason is not price. A
//! translation endpoint takes a string and a target language and nothing else, which for a
//! gettext catalog throws away most of what the catalog knows. Three things were being lost:
//!
//! - The `#. TRANSLATORS:` comments. Every one of them was written to disambiguate a string
//!   that can't be translated correctly without it ("Ready" the status, not "Ready" the
//!   button), and there was no field to put them in. There are ~30k characters of them.
//! - The `&` accelerator markers. A translation API sees punctuation and drops or relocates
//!   it, and nothing downstream checks: 216 of Russian's 251 accelerator strings had lost
//!   theirs, so most of the Russian menu had no keyboard access at all.
//! - The `\tCtrl+O` shortcut suffixes, which are UI wiring rather than prose and must survive
//!   byte-for-byte.
//!
//! A model can be told about all three, and the checks at the bottom of this file verify it
//! did rather than trusting it. Anything that fails a check is returned as `None` and left for
//! the next run, which is the same contract the previous backend had for placeholder damage.

use std::{error::Error, thread, time::Duration};

use serde::Deserialize;
use serde_json::{Value, json};

const API_URL: &str = "https://api.anthropic.com/v1/messages";
/// The API version header, which pins the request/response shape. Unrelated to the model.
const API_VERSION: &str = "2023-06-01";

/// Overridable with `PAPERBACK_TRANSLATE_MODEL`, e.g. `claude-opus-5` for a language the
/// results look weak in.
///
/// Short UI strings with the rules stated up front, and a checked output, is squarely what the
/// cheapest model is for: a full rebuild of all seven locales lands well under a dollar, and
/// the accelerator, placeholder and shortcut checks below don't care which model produced the
/// text. What a larger model buys here is phrasing, not correctness.
const DEFAULT_MODEL: &str = "claude-haiku-4-5";

/// Strings per request. Well under what the model can hold; the point is to bound `max_tokens`
/// and to lose only one batch, not a whole language, when a request fails.
const BATCH_LIMIT: usize = 60;

/// Plural entries are batched smaller: each one comes back as several forms rather than one
/// string, so the same number of entries is several times the output tokens.
const PLURAL_BATCH_LIMIT: usize = 20;

/// Source characters per README chunk. The README is split on section headings and sent a few
/// sections at a time so no single response has to be enormous.
const README_CHUNK_CHARS: usize = 6000;

const MAX_TOKENS: u32 = 16000;

/// Retries for the statuses that are worth retrying (429 and 5xx). Raw HTTP gets none of the
/// automatic backoff the official SDKs have, so it is done here.
const MAX_ATTEMPTS: u32 = 4;

/// One string to translate, with whatever the catalog knows about it.
pub struct Phrase {
	pub source: String,
	/// The `#. TRANSLATORS:` comment from the pot, when the string has one.
	pub context: Option<String>,
}

/// One plural string to translate: the English singular and plural, and how many forms the
/// target language wants back.
pub struct PluralPhrase {
	pub singular: String,
	pub plural: String,
	pub context: Option<String>,
}

pub struct ClaudeClient {
	api_key: String,
	model: String,
}

impl ClaudeClient {
	#[must_use]
	pub fn new(api_key: String) -> Self {
		let model = std::env::var("PAPERBACK_TRANSLATE_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
		Self { api_key, model }
	}

	#[must_use]
	pub fn model(&self) -> &str {
		&self.model
	}

	/// Translates `phrases` into `language`, in batches. Returns one result per input, in
	/// order, with `None` where the result failed a check (see [`check`]).
	pub fn translate_phrases(&self, phrases: &[Phrase], language: &str) -> Result<Vec<Option<String>>, Box<dyn Error>> {
		let mut out = Vec::with_capacity(phrases.len());
		for chunk in phrases.chunks(BATCH_LIMIT) {
			out.extend(self.translate_chunk(chunk, language)?);
		}
		Ok(out)
	}

	/// Builds the request body for one batch. Split out from the call so its shape can be
	/// asserted in a test: a request that is subtly wrong (a misplaced `format`, a missing
	/// `id`) otherwise only shows up as a 400 from a live call, which the test suite never
	/// makes.
	fn phrase_request(&self, phrases: &[Phrase], language: &str) -> Result<Value, Box<dyn Error>> {
		let items: Vec<Value> = phrases
			.iter()
			.enumerate()
			.map(|(i, p)| {
				p.context.as_ref().map_or_else(
					|| json!({ "id": i, "text": p.source }),
					|context| json!({ "id": i, "text": p.source, "context": context }),
				)
			})
			.collect();
		Ok(json!({
			"model": self.model,
			"max_tokens": MAX_TOKENS,
			// A well-specified transformation with the rules supplied up front, which is what
			// low effort is for. Raising it costs thinking tokens on every batch and has
			// nothing extra to work out.
			"output_config": {
				"effort": "low",
				"format": { "type": "json_schema", "schema": translations_schema() }
			},
			// The rules are identical for every batch and every language, so they sit in a
			// cacheable system block rather than being repeated in each user message.
			"system": [{
				"type": "text",
				"text": phrase_system_prompt(),
				"cache_control": { "type": "ephemeral" }
			}],
			"messages": [{
				"role": "user",
				"content": format!(
					"Target language: {language}\n\nTranslate every entry:\n{}",
					serde_json::to_string_pretty(&items)?
				)
			}]
		}))
	}

	fn translate_chunk(&self, phrases: &[Phrase], language: &str) -> Result<Vec<Option<String>>, Box<dyn Error>> {
		let request = self.phrase_request(phrases, language)?;
		let text = self.send(&request)?;
		#[derive(Deserialize)]
		struct Item {
			id: usize,
			text: String,
		}
		#[derive(Deserialize)]
		struct Payload {
			translations: Vec<Item>,
		}
		let payload: Payload = serde_json::from_str(&text)
			.map_err(|e| format!("Claude returned a response that did not match the schema: {e} (body: {text})"))?;
		// Keyed by id rather than zipped by position: the schema fixes the shape of each item
		// but not that the array comes back complete or in order.
		let mut out: Vec<Option<String>> = vec![None; phrases.len()];
		for item in payload.translations {
			if let Some(slot) = out.get_mut(item.id) {
				*slot = check(&phrases[item.id].source, &item.text);
			}
		}
		Ok(out)
	}

	/// Translates plural strings, asking for `nplurals` forms of each.
	///
	/// Separate from [`Self::translate_phrases`] because the unit is different: one English
	/// singular/plural pair goes in, and a whole set of forms comes back, as many as the target
	/// language uses. That count is not a property of the string - Russian wants three where
	/// French wants two - so it is read from the file's `Plural-Forms` header and passed in
	/// rather than assumed.
	pub fn translate_plurals(
		&self,
		phrases: &[PluralPhrase],
		language: &str,
		nplurals: usize,
		plural_rule: &str,
	) -> Result<Vec<Option<Vec<String>>>, Box<dyn Error>> {
		let mut out = Vec::with_capacity(phrases.len());
		for chunk in phrases.chunks(PLURAL_BATCH_LIMIT) {
			out.extend(self.translate_plural_chunk(chunk, language, nplurals, plural_rule)?);
		}
		Ok(out)
	}

	fn translate_plural_chunk(
		&self,
		phrases: &[PluralPhrase],
		language: &str,
		nplurals: usize,
		plural_rule: &str,
	) -> Result<Vec<Option<Vec<String>>>, Box<dyn Error>> {
		let items: Vec<Value> = phrases
			.iter()
			.enumerate()
			.map(|(i, p)| {
				let mut item = json!({ "id": i, "singular": p.singular, "plural": p.plural });
				if let Some(context) = &p.context {
					item["context"] = json!(context);
				}
				item
			})
			.collect();
		let request = json!({
			"model": self.model,
			"max_tokens": MAX_TOKENS,
			"output_config": {
				"effort": "low",
				"format": { "type": "json_schema", "schema": plural_schema(nplurals) }
			},
			"system": [{
				"type": "text",
				"text": plural_system_prompt(),
				"cache_control": { "type": "ephemeral" }
			}],
			"messages": [{
				"role": "user",
				"content": format!(
					"Target language: {language}\n\
					 This language has {nplurals} plural forms. Its gettext rule is:\n{plural_rule}\n\n\
					 Return exactly {nplurals} forms for each entry, in index order: forms[i] is used \
					 for the counts where that rule yields i.\n\n{}",
					serde_json::to_string_pretty(&items)?
				)
			}]
		});
		let text = self.send(&request)?;
		#[derive(Deserialize)]
		struct Item {
			id: usize,
			forms: Vec<String>,
		}
		#[derive(Deserialize)]
		struct Payload {
			translations: Vec<Item>,
		}
		let payload: Payload = serde_json::from_str(&text)
			.map_err(|e| format!("Claude returned a response that did not match the schema: {e} (body: {text})"))?;
		let mut out: Vec<Option<Vec<String>>> = vec![None; phrases.len()];
		for item in payload.translations {
			let Some(slot) = out.get_mut(item.id) else { continue };
			*slot = check_plural(&phrases[item.id], &item.forms, nplurals);
		}
		Ok(out)
	}

	/// Translates a Markdown document, section by section, and reassembles it.
	///
	/// Markdown goes to the model as Markdown. The previous backend had to run the README
	/// through `pandoc` into HTML, translate that, and convert it back, because its API had no
	/// other way to protect code spans and fenced blocks from being translated; that round trip
	/// is now just an instruction.
	pub fn translate_markdown(&self, markdown: &str, language: &str) -> Result<String, Box<dyn Error>> {
		let mut translated: Vec<String> = Vec::new();
		for chunk in split_markdown(markdown, README_CHUNK_CHARS) {
			translated.push(self.translate_markdown_chunk(&chunk, language)?);
		}
		let joined = translated.join("\n\n");
		Ok(restore_code_spans(markdown, &joined))
	}

	fn translate_markdown_chunk(&self, markdown: &str, language: &str) -> Result<String, Box<dyn Error>> {
		let request = json!({
			"model": self.model,
			"max_tokens": MAX_TOKENS,
			"output_config": {
				"effort": "low",
				"format": { "type": "json_schema", "schema": markdown_schema() }
			},
			"system": [{
				"type": "text",
				"text": markdown_system_prompt(),
				"cache_control": { "type": "ephemeral" }
			}],
			"messages": [{
				"role": "user",
				"content": format!("Target language: {language}\n\n<document>\n{markdown}\n</document>")
			}]
		});
		let text = self.send(&request)?;
		#[derive(Deserialize)]
		struct Payload {
			markdown: String,
		}
		let payload: Payload = serde_json::from_str(&text)
			.map_err(|e| format!("Claude returned a response that did not match the schema: {e} (body: {text})"))?;
		Ok(payload.markdown)
	}

	/// Posts `request` and returns the first text block, retrying the statuses that deserve it.
	fn send(&self, request: &Value) -> Result<String, Box<dyn Error>> {
		let mut attempt = 0;
		loop {
			attempt += 1;
			let response = ureq::post(API_URL)
				.config()
				.http_status_as_error(false)
				.build()
				.header("x-api-key", &self.api_key)
				.header("anthropic-version", API_VERSION)
				.header("content-type", "application/json")
				.send_json(request);
			let mut response = match response {
				Ok(response) => response,
				// A transport-level failure (DNS, TLS, dropped connection) is as retryable as
				// a 503 and arrives as a different kind of value, so it rejoins the same path.
				Err(e) if attempt < MAX_ATTEMPTS => {
					backoff(attempt, None);
					let _ = e;
					continue;
				}
				Err(e) => return Err(format!("Claude request failed: {e}").into()),
			};
			let status = response.status();
			let retry_after =
				response.headers().get("retry-after").and_then(|v| v.to_str().ok()).and_then(|v| v.parse::<u64>().ok());
			let body = response.body_mut().read_to_string().unwrap_or_default();
			if status.is_success() {
				return first_text_block(&body);
			}
			// 429 and 5xx are transient. Everything else (401, 400, ...) will fail the same
			// way however many times it is sent, so it surfaces immediately with the API's own
			// error text, which says which field was wrong.
			let retryable = status.as_u16() == 429 || status.is_server_error();
			if retryable && attempt < MAX_ATTEMPTS {
				backoff(attempt, retry_after);
				continue;
			}
			return Err(format!("Claude request returned HTTP {status}: {body}").into());
		}
	}
}

fn backoff(attempt: u32, retry_after: Option<u64>) {
	// Honour the server's own retry-after when it sends one; it knows when the limit resets.
	let seconds = retry_after.unwrap_or_else(|| 2u64.pow(attempt));
	thread::sleep(Duration::from_secs(seconds.min(60)));
}

/// Pulls the response text out of a successful `/v1/messages` body.
///
/// `content` is a list of blocks and the text is not necessarily the first: with thinking on,
/// a `thinking` block precedes it. Filtering by type rather than indexing avoids returning a
/// reasoning summary as if it were the translation.
fn first_text_block(body: &str) -> Result<String, Box<dyn Error>> {
	let parsed: Value =
		serde_json::from_str(body).map_err(|e| format!("Claude response was not valid JSON: {e} (body: {body})"))?;
	// A refusal is a 200 with no usable content, so it needs its own message rather than
	// being reported as a malformed response.
	if parsed.get("stop_reason").and_then(Value::as_str) == Some("refusal") {
		return Err(
			format!("Claude declined the request: {}", parsed.get("stop_details").unwrap_or(&Value::Null)).into()
		);
	}
	parsed
		.get("content")
		.and_then(Value::as_array)
		.and_then(|blocks| {
			blocks
				.iter()
				.find(|b| b.get("type").and_then(Value::as_str) == Some("text"))
				.and_then(|b| b.get("text"))
				.and_then(Value::as_str)
		})
		.map(str::to_string)
		.ok_or_else(|| format!("Claude response had no text block (body: {body})").into())
}

fn translations_schema() -> Value {
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

/// Schema for a plural batch. `minItems`/`maxItems` pin the form count at `nplurals`, so a
/// response with too few forms is rejected by the API rather than arriving here to be caught.
fn plural_schema(nplurals: usize) -> Value {
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
							"minItems": nplurals,
							"maxItems": nplurals
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

fn markdown_schema() -> Value {
	json!({
		"type": "object",
		"properties": { "markdown": { "type": "string" } },
		"required": ["markdown"],
		"additionalProperties": false
	})
}

const fn phrase_system_prompt() -> &'static str {
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

const fn plural_system_prompt() -> &'static str {
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

const fn markdown_system_prompt() -> &'static str {
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

/// Puts the source's inline code spans back, so a `` `Alt+Left` `` stays `Alt+Left`.
///
/// The prompt says code spans are verbatim, and the model mostly obeys, but the exceptions are
/// the ones that matter: `Alt+Left` came back as `Alt+Gauche` in French, `Alt+Links` in Dutch
/// and `Alt+Влево` in Russian. Those read as translations and are not keys anyone can press.
/// The rule is absolute, so it is enforced here rather than left to the prompt.
///
/// Only runs when the span counts match. A different count means the model added or dropped
/// one, so the nth span in the translation isn't the nth in the source and positional
/// restoration would put text in the wrong place; the translation is then left exactly as it
/// came back, for a human to look at.
fn restore_code_spans(source: &str, translated: &str) -> String {
	let source_spans = code_spans(source);
	let translated_spans = code_spans(translated);
	if source_spans.len() != translated_spans.len() {
		return translated.to_string();
	}
	let mut out = String::with_capacity(translated.len());
	let mut last = 0;
	for ((range, _), (_, original)) in translated_spans.into_iter().zip(source_spans) {
		out.push_str(&translated[last..range.start]);
		out.push_str(original);
		last = range.end;
	}
	out.push_str(&translated[last..]);
	out
}

/// The inner text of every single-backtick inline code span, with its byte range.
///
/// Deliberately single-line: a span never spans a newline in Markdown, and stopping at one
/// keeps a stray unmatched backtick from swallowing the rest of the document.
fn code_spans(text: &str) -> Vec<(std::ops::Range<usize>, &str)> {
	let bytes = text.as_bytes();
	let mut out = Vec::new();
	let mut i = 0;
	while i < bytes.len() {
		if bytes[i] != b'`' {
			i += 1;
			continue;
		}
		let start = i + 1;
		let mut j = start;
		while j < bytes.len() && bytes[j] != b'`' && bytes[j] != b'\n' {
			j += 1;
		}
		if j < bytes.len() && bytes[j] == b'`' && j > start {
			out.push((start..j, &text[start..j]));
			i = j + 1;
		} else {
			i += 1;
		}
	}
	out
}

/// Splits Markdown into chunks of at most `limit` characters, breaking only at `##` headings so
/// a chunk is always a whole number of sections and the model never sees a half-open construct.
/// A single section longer than the limit is left whole rather than cut mid-paragraph.
fn split_markdown(markdown: &str, limit: usize) -> Vec<String> {
	let mut sections: Vec<String> = Vec::new();
	let mut current = String::new();
	for line in markdown.lines() {
		if line.starts_with("## ") && !current.trim().is_empty() {
			sections.push(current.trim_end().to_string());
			current = String::new();
		}
		current.push_str(line);
		current.push('\n');
	}
	if !current.trim().is_empty() {
		sections.push(current.trim_end().to_string());
	}
	let mut chunks: Vec<String> = Vec::new();
	for section in sections {
		match chunks.last_mut() {
			Some(last) if last.len() + section.len() + 2 <= limit => {
				last.push_str("\n\n");
				last.push_str(&section);
			}
			_ => chunks.push(section),
		}
	}
	chunks
}

/// Whether an existing translation is provably damaged: it dropped a placeholder, an
/// accelerator, or a shortcut suffix that the English has.
///
/// The same rules [`check`] applies to fresh output, pointed at what is already in the catalog.
/// Entries that fail this were never translated as such - they are what `msgmerge` copied in
/// from a similar string before anything checked - and since nothing in the normal flow will
/// ever revisit them, this is what marks them for re-translation.
#[must_use]
pub fn is_damaged(source: &str, translated: &str) -> bool {
	!translated.is_empty() && check(source, translated).is_none()
}

/// The plural counterpart of [`check`]: every form has to be present and carry the English's
/// placeholders, or the whole set is rejected.
///
/// All-or-nothing on purpose. A partly-good set written into the file leaves some `msgstr[N]`
/// filled and others blank, which reads as translated to every tool that looks at it while
/// gettext quietly falls back to the English for the missing counts.
fn check_plural(phrase: &PluralPhrase, forms: &[String], nplurals: usize) -> Option<Vec<String>> {
	if forms.len() != nplurals || forms.iter().any(|f| f.trim().is_empty()) {
		return None;
	}
	// Checked against the English plural rather than the singular: the placeholder belongs to
	// the countable message as a whole, and `1 document.` / `%d documents.` legitimately
	// differ in whether the singular spells the number out.
	let expected = placeholder_counts(&phrase.plural);
	forms.iter().all(|f| placeholder_counts(f) == expected).then(|| forms.to_vec())
}

/// Accepts a translation only if it kept the parts that aren't prose, returning `None` when it
/// didn't so the caller leaves the entry for the next run instead of writing damage into the
/// catalog.
fn check(source: &str, translated: &str) -> Option<String> {
	if translated.trim().is_empty() {
		return None;
	}
	if placeholder_counts(source) != placeholder_counts(translated) {
		return None;
	}
	if accelerator_count(source) != accelerator_count(translated) {
		return None;
	}
	if shortcut_suffix(source) != shortcut_suffix(translated) {
		return None;
	}
	Some(translated.to_string())
}

/// Counts of `%s`/`%d`/`{}`, the placeholder styles this project's strings use.
fn placeholder_counts(s: &str) -> (usize, usize, usize) {
	(s.matches("%s").count(), s.matches("%d").count(), s.matches("{}").count())
}

/// How many accelerator markers a string has: an `&` directly before an alphanumeric.
///
/// `&&` is an escaped literal ampersand rather than a marker, and is skipped along with the
/// character after it so `A && B` counts as none rather than one.
fn accelerator_count(s: &str) -> usize {
	let chars: Vec<char> = s.chars().collect();
	let mut count = 0;
	let mut i = 0;
	while i < chars.len() {
		if chars[i] == '&' {
			match chars.get(i + 1) {
				Some('&') => i += 2,
				Some(c) if c.is_alphanumeric() => {
					count += 1;
					i += 2;
				}
				_ => i += 1,
			}
			continue;
		}
		i += 1;
	}
	count
}

/// The `\tCtrl+O` shortcut suffix, which has to come back byte for byte. `None` when the string
/// has no tab, so strings without a shortcut compare equal to each other.
fn shortcut_suffix(s: &str) -> Option<&str> {
	s.split_once('\t').map(|(_, suffix)| suffix)
}

/// Maps a `po/<lang>.po` filename stem to the language name used in the prompt.
///
/// A name rather than a code: the code is what a translation API's language parameter wanted,
/// and there is no such parameter here. Unlike that list, this one does not have to be checked
/// against a provider's supported-languages endpoint, which is what previously left `bs`, `sr`
/// and `vi` at the mercy of whether the vendor had got to them yet.
#[must_use]
pub fn language_name(po_lang: &str) -> Option<&'static str> {
	Some(match po_lang {
		"bs" => "Bosnian",
		"cs" => "Czech",
		"de" => "German",
		"es" => "Spanish",
		"fi" => "Finnish",
		"fr" => "French",
		"ja" => "Japanese",
		"nl" => "Dutch",
		"pl" => "Polish",
		"pt_br" => "Brazilian Portuguese",
		"ru" => "Russian",
		"sr" => "Serbian",
		"vi" => "Vietnamese",
		"zh_CN" => "Simplified Chinese",
		_ => return None,
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn placeholder_counts_catch_a_dropped_token() {
		assert_eq!(placeholder_counts("%s Heading level %d"), placeholder_counts("Ebene %d von %s"));
		assert_ne!(
			placeholder_counts("Remove the {} selected documents?"),
			placeholder_counts("Sollen die gewahlten Dokumente entfernt werden?")
		);
	}

	// The failure this backend exists to stop: `E&xit` came back as a Russian word with no
	// accelerator at all, 216 times in one locale.
	#[test]
	fn a_dropped_accelerator_is_rejected() {
		assert_eq!(check("E&xit", "Выход"), None);
		assert_eq!(check("E&xit", "В&ыход"), Some("В&ыход".to_string()));
	}

	#[test]
	fn an_invented_accelerator_is_rejected() {
		assert_eq!(check("Ready", "&Listo"), None);
		assert_eq!(check("Ready", "Listo"), Some("Listo".to_string()));
	}

	#[test]
	fn an_escaped_ampersand_is_not_an_accelerator() {
		assert_eq!(accelerator_count("Search && Replace"), 0);
		assert_eq!(accelerator_count("&Search && Replace"), 1);
		assert_eq!(check("Search && Replace", "Buscar && Reemplazar"), Some("Buscar && Reemplazar".to_string()));
	}

	#[test]
	fn the_accelerator_may_move_to_a_different_letter() {
		// Which letter it lands on is the model's call, since it has to be a letter the
		// translation actually contains. Only the count is enforced.
		assert_eq!(check("&Open", "&Abrir"), Some("&Abrir".to_string()));
		assert_eq!(check("&Open", "A&brir"), Some("A&brir".to_string()));
	}

	#[test]
	fn a_shortcut_suffix_must_come_back_untouched() {
		assert_eq!(check("&Copy\tCtrl+C", "&Copiar\tCtrl+C"), Some("&Copiar\tCtrl+C".to_string()));
		assert_eq!(check("&Copy\tCtrl+C", "&Copiar\tCtrl+D"), None, "a rewritten key combination");
		assert_eq!(check("&Copy\tCtrl+C", "&Copiar"), None, "a dropped shortcut");
	}

	#[test]
	fn an_empty_translation_is_rejected() {
		assert_eq!(check("Ready", "   "), None);
	}

	#[test]
	fn markdown_splits_on_section_headings() {
		let doc = "# Title\n\nIntro.\n\n## One\n\nBody one.\n\n## Two\n\nBody two.\n";
		let chunks = split_markdown(doc, 30);
		assert_eq!(chunks.len(), 3, "each section should be its own chunk at this limit");
		assert!(chunks[1].starts_with("## One"));
		assert!(chunks[2].starts_with("## Two"));
	}

	#[test]
	fn markdown_sections_pack_together_under_the_limit() {
		let doc = "# Title\n\nIntro.\n\n## One\n\nBody one.\n\n## Two\n\nBody two.\n";
		let chunks = split_markdown(doc, 10_000);
		assert_eq!(chunks.len(), 1, "the whole document fits in one chunk");
		assert_eq!(chunks[0].trim(), doc.trim());
	}

	// The real case: French came back with `Alt+Gauche` where the source said `Alt+Left`.
	#[test]
	fn a_translated_key_name_is_put_back() {
		let source = "Press `Alt+Left` to go back, or `Ctrl+Space` to play.";
		let translated = "Appuyez sur `Alt+Gauche` pour revenir, ou `Ctrl+Espace` pour lire.";
		assert_eq!(
			restore_code_spans(source, translated),
			"Appuyez sur `Alt+Left` pour revenir, ou `Ctrl+Space` pour lire."
		);
	}

	#[test]
	fn prose_around_a_restored_span_is_left_alone() {
		let source = "The `readme.md` file.";
		let translated = "Het `readme.md` bestand.";
		assert_eq!(restore_code_spans(source, translated), translated);
	}

	// A changed count means the nth span in the translation is no longer the nth in the
	// source, so restoring by position would drop text into the wrong place.
	#[test]
	fn a_mismatched_span_count_leaves_the_translation_untouched() {
		let source = "Press `Alt+Left` then `Ctrl+C`.";
		let translated = "Appuyez sur `Alt+Gauche`.";
		assert_eq!(restore_code_spans(source, translated), translated);
	}

	#[test]
	fn code_spans_do_not_run_past_a_newline() {
		// A stray unmatched backtick must not swallow the rest of the document.
		let spans = code_spans("a ` stray\nand `real` one");
		assert_eq!(spans.len(), 1);
		assert_eq!(spans[0].1, "real");
	}

	#[test]
	fn an_empty_span_is_not_a_span() {
		assert!(code_spans("nothing `` here").is_empty());
	}

	#[test]
	fn restoring_handles_multibyte_text_around_the_spans() {
		let source = "Press `Alt+Left` now.";
		let translated = "Нажмите `Alt+Влево` сейчас.";
		assert_eq!(restore_code_spans(source, translated), "Нажмите `Alt+Left` сейчас.");
	}

	#[test]
	fn markdown_round_trips_when_nothing_needs_splitting() {
		let doc = "# Title\n\nIntro.\n\n## One\n\nBody one.";
		assert_eq!(split_markdown(doc, 10_000).join("\n\n"), doc);
	}

	#[test]
	fn every_shipped_locale_resolves_to_a_language_name() {
		for lang in ["bs", "cs", "de", "es", "fi", "fr", "ja", "nl", "pl", "pt_br", "ru", "sr", "vi", "zh_CN"] {
			assert!(language_name(lang).is_some(), "{lang} has no language name");
		}
		assert_eq!(language_name("xx"), None);
	}

	#[test]
	fn a_refusal_is_reported_as_a_refusal() {
		let body = r#"{"stop_reason":"refusal","stop_details":{"type":"refusal"},"content":[]}"#;
		let err = first_text_block(body).unwrap_err().to_string();
		assert!(err.contains("declined"), "got: {err}");
	}

	// With thinking on, a thinking block precedes the text block.
	#[test]
	fn the_text_block_is_found_past_a_thinking_block() {
		let body = r#"{"content":[{"type":"thinking","thinking":""},{"type":"text","text":"{}"}]}"#;
		assert_eq!(first_text_block(body).unwrap(), "{}");
	}

	// The request shape is otherwise only checked by the API itself, which the test suite
	// never talks to. This pins the parts that have to be exactly where they are.
	#[test]
	fn the_request_body_has_the_shape_the_api_expects() {
		let client = ClaudeClient { api_key: "test".to_string(), model: "test-model".to_string() };
		let phrases = vec![
			Phrase { source: "&Settings".to_string(), context: Some("Menu item".to_string()) },
			Phrase { source: "Ready".to_string(), context: None },
		];
		let request = client.phrase_request(&phrases, "Russian").unwrap();
		assert_eq!(request["model"], "test-model", "the configured model has to reach the request");
		assert_eq!(request["output_config"]["effort"], "low");
		// `format` nests inside output_config; as a top-level `output_format` it is the
		// deprecated spelling and would be rejected.
		assert_eq!(request["output_config"]["format"]["type"], "json_schema");
		assert!(request["output_config"]["format"]["schema"].is_object());
		// The system prompt is a block list so it can carry cache_control, not a bare string.
		assert_eq!(request["system"][0]["cache_control"]["type"], "ephemeral");
		assert_eq!(request["messages"][0]["role"], "user");
		let content = request["messages"][0]["content"].as_str().unwrap();
		assert!(content.contains("Target language: Russian"));
		assert!(content.contains("&Settings"), "the source string must reach the request");
		assert!(content.contains("Menu item"), "the translator note must reach the request");
		assert!(content.contains("\"id\": 0") && content.contains("\"id\": 1"), "every entry needs its id");
	}

	// Real entries from the catalog, each one something msgmerge copied in from a similar
	// string before anything checked.
	#[test]
	fn damage_already_in_the_catalog_is_recognised() {
		assert!(is_damaged("&Status:", "Estado"), "dropped accelerator");
		assert!(is_damaged("Choose &Background Color...", "Seleccionar color de fondo..."), "dropped accelerator");
		assert!(is_damaged("{} minutes", "minutos"), "dropped placeholder");
		// An *extra* placeholder is the dangerous direction: nothing fills it.
		assert!(is_damaged("Page %d", "Página %d: %s"), "invented placeholder");
		assert!(is_damaged("&Copy\tCtrl+C", "&Copiar\tCtrl+D"), "rewritten shortcut");
	}

	#[test]
	fn a_sound_translation_is_not_flagged_as_damaged() {
		assert!(!is_damaged("E&xit", "В&ыход"));
		assert!(!is_damaged("Ready", "Listo"));
		assert!(!is_damaged("Page %d", "Página %d"));
		assert!(!is_damaged("&Copy\tCtrl+C", "&Copiar\tCtrl+C"));
		// An untranslated entry is the normal flow's job, not the repair pass's.
		assert!(!is_damaged("Ready", ""));
	}

	fn plural(singular: &str, plural: &str) -> PluralPhrase {
		PluralPhrase { singular: singular.to_string(), plural: plural.to_string(), context: None }
	}

	#[test]
	fn a_complete_plural_set_is_accepted() {
		let p = plural("%d document.", "%d documents.");
		let forms = ["%d документ.".to_string(), "%d документа.".to_string(), "%d документов.".to_string()];
		assert_eq!(check_plural(&p, &forms, 3), Some(forms.to_vec()));
	}

	// Russian needs three. Two written into a three-form entry leaves msgstr[2] blank, which
	// looks translated to every tool that inspects the file while gettext falls back to the
	// English for those counts.
	#[test]
	fn a_short_plural_set_is_rejected() {
		let p = plural("%d document.", "%d documents.");
		let forms = ["%d документ.".to_string(), "%d документа.".to_string()];
		assert_eq!(check_plural(&p, &forms, 3), None);
	}

	#[test]
	fn a_plural_set_with_a_blank_form_is_rejected() {
		let p = plural("%d document.", "%d documents.");
		let forms = ["a".to_string(), "  ".to_string()];
		assert_eq!(check_plural(&p, &forms, 2), None);
	}

	// The count is substituted into the placeholder, so a form without it renders a sentence
	// with no number in it.
	#[test]
	fn a_plural_form_that_dropped_the_placeholder_is_rejected() {
		let p = plural("%d document.", "%d documents.");
		let forms = ["%d документ.".to_string(), "документа.".to_string()];
		assert_eq!(check_plural(&p, &forms, 2), None);
	}

	// Checked against the English plural, not the singular: `1 document.` spells the number
	// out and has no placeholder, which would reject every correct translation.
	#[test]
	fn the_placeholder_check_uses_the_english_plural() {
		let p = plural("1 document.", "%d documents.");
		let forms = ["%d документ.".to_string(), "%d документа.".to_string()];
		assert!(check_plural(&p, &forms, 2).is_some());
	}

	#[test]
	fn the_plural_schema_pins_the_form_count() {
		let schema = plural_schema(3);
		let forms = &schema["properties"]["translations"]["items"]["properties"]["forms"];
		assert_eq!(forms["minItems"], 3);
		assert_eq!(forms["maxItems"], 3);
	}

	// A typo here is a 404 on every request, and only at runtime.
	#[test]
	fn the_default_model_is_a_real_model_id() {
		assert_eq!(DEFAULT_MODEL, "claude-haiku-4-5");
	}

	#[test]
	fn a_phrase_without_a_note_carries_no_context_field() {
		let client = ClaudeClient { api_key: "test".to_string(), model: "test-model".to_string() };
		let phrases = vec![Phrase { source: "Ready".to_string(), context: None }];
		let request = client.phrase_request(&phrases, "French").unwrap();
		let content = request["messages"][0]["content"].as_str().unwrap();
		assert!(!content.contains("context"), "an absent note should be absent, not empty");
	}
}
