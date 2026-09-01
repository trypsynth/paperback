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

use super::{
	checks::{check, check_plural},
	markdown::{restore_code_spans, split_markdown},
	prompts::{
		markdown_schema, markdown_system_prompt, phrase_system_prompt, plural_schema, plural_system_prompt,
		translations_schema,
	},
};

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

	/// The `output_config` for a request, with `effort` included only where the model takes it.
	///
	/// `effort` is not universal: the Haiku tier rejects it outright with "This model does not
	/// support the effort parameter", a 400 on every request rather than a warning. It is worth
	/// setting on the models that do have it - it holds down thinking tokens on a task whose
	/// rules are all supplied up front - but it has to be conditional, because the model is
	/// configurable and the cheap default is exactly the one that refuses it.
	fn output_config(&self, format: &Value) -> Value {
		let mut config = json!({ "format": format });
		if supports_effort(&self.model) {
			config["effort"] = json!("low");
		}
		config
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
			"output_config": self.output_config(&json!({ "type": "json_schema", "schema": translations_schema() })),
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
			"output_config": self.output_config(&json!({ "type": "json_schema", "schema": plural_schema() })),
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
			"output_config": self.output_config(&json!({ "type": "json_schema", "schema": markdown_schema() })),
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

/// Whether `output_config.effort` is accepted by this model.
///
/// The Haiku tier does not take it and returns a 400 for the whole request, so this is a
/// correctness check, not a tuning one. Written as "everything except Haiku" rather than a list
/// of models that do support it, so a newer model set through `PAPERBACK_TRANSLATE_MODEL` gets
/// the parameter by default instead of silently losing it.
fn supports_effort(model: &str) -> bool {
	!model.contains("haiku")
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
		assert_eq!(request["output_config"]["effort"], "low", "a model that accepts effort gets it");
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

	// Sending `effort` to a model that does not take it is a 400 on every single request, so
	// this is correctness rather than tuning. Haiku is the default, which is what made it bite.
	#[test]
	fn effort_is_sent_only_to_models_that_accept_it() {
		assert!(!supports_effort("claude-haiku-4-5"));
		assert!(supports_effort("claude-opus-5"));
		assert!(supports_effort("claude-sonnet-5"));
	}

	#[test]
	fn the_opus_request_still_carries_effort() {
		let client = ClaudeClient { api_key: "test".to_string(), model: "claude-opus-5".to_string() };
		let phrases = vec![Phrase { source: "Ready".to_string(), context: None }];
		let request = client.phrase_request(&phrases, "French").unwrap();
		assert_eq!(request["output_config"]["effort"], "low");
		assert_eq!(request["output_config"]["format"]["type"], "json_schema");
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
