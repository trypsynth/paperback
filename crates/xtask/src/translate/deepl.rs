use std::{collections::HashSet, error::Error};

use serde::{Deserialize, Serialize};

/// Maximum texts per `/v2/translate` call (`DeepL`'s documented per-request limit).
const BATCH_LIMIT: usize = 50;

pub struct DeepLClient {
	api_key: String,
	base_url: &'static str,
}

impl DeepLClient {
	/// Free-tier keys end in `:fx`, which is how `DeepL` distinguishes the free
	/// (`api-free.deepl.com`) and paid (`api.deepl.com`) endpoints; picking the wrong one
	/// for a given key fails auth, so this reads the key itself rather than needing a
	/// separate "which plan" setting.
	#[must_use]
	pub fn new(api_key: String) -> Self {
		let base_url =
			if api_key.trim().ends_with(":fx") { "https://api-free.deepl.com" } else { "https://api.deepl.com" };
		Self { api_key, base_url }
	}

	pub fn supported_target_languages(&self) -> Result<HashSet<String>, Box<dyn Error>> {
		#[derive(Deserialize)]
		struct Lang {
			language: String,
		}
		let url = format!("{}/v2/languages", self.base_url);
		let body = ureq::get(&url)
			.config()
			.http_status_as_error(false)
			.build()
			.header("Authorization", &format!("DeepL-Auth-Key {}", self.api_key))
			.query("type", "target")
			.call()
			.map_err(|e| format!("DeepL languages request failed: {e}"))?;
		let body_text = read_body_or_error("languages", body)?;
		let langs: Vec<Lang> = serde_json::from_str(&body_text)
			.map_err(|e| format!("DeepL languages response was not valid JSON: {e} (body: {body_text})"))?;
		Ok(langs.into_iter().map(|l| l.language.to_uppercase()).collect())
	}

	/// Translates `texts` (English source) to `target_lang`, batching in groups of
	/// [`BATCH_LIMIT`]. Returns translations in the same order as `texts`.
	pub fn translate_batch(&self, texts: &[String], target_lang: &str) -> Result<Vec<String>, Box<dyn Error>> {
		let mut out = Vec::with_capacity(texts.len());
		for chunk in texts.chunks(BATCH_LIMIT) {
			out.extend(self.translate_chunk(chunk, target_lang)?);
		}
		Ok(out)
	}

	fn translate_chunk(&self, texts: &[String], target_lang: &str) -> Result<Vec<String>, Box<dyn Error>> {
		#[derive(Serialize)]
		struct Req<'a> {
			text: Vec<String>,
			target_lang: &'a str,
			source_lang: &'a str,
			tag_handling: &'a str,
			ignore_tags: &'a [&'a str],
			preserve_formatting: u8,
		}
		#[derive(Deserialize)]
		struct TranslationItem {
			text: String,
		}
		#[derive(Deserialize)]
		struct Resp {
			translations: Vec<TranslationItem>,
		}

		let protected: Vec<String> = texts.iter().map(|t| protect_placeholders(t)).collect();
		let url = format!("{}/v2/translate", self.base_url);
		let req = Req {
			text: protected,
			target_lang,
			source_lang: "EN",
			tag_handling: "xml",
			ignore_tags: &["x"],
			preserve_formatting: 1,
		};
		let body = ureq::post(&url)
			.config()
			.http_status_as_error(false)
			.build()
			.header("Authorization", &format!("DeepL-Auth-Key {}", self.api_key))
			.send_json(&req)
			.map_err(|e| format!("DeepL translate request failed: {e}"))?;
		let body_text = read_body_or_error("translate", body)?;
		let resp: Resp = serde_json::from_str(&body_text)
			.map_err(|e| format!("DeepL translate response was not valid JSON: {e} (body: {body_text})"))?;
		if resp.translations.len() != texts.len() {
			return Err(format!(
				"DeepL returned {} translations for {} input strings",
				resp.translations.len(),
				texts.len()
			)
			.into());
		}
		Ok(resp.translations.into_iter().map(|t| unprotect_placeholders(&t.text)).collect())
	}
}

/// Reads the response body as text regardless of status (requests are sent with
/// `http_status_as_error(false)` for exactly this), surfacing `DeepL`'s own error message
/// on a non-2xx status instead of just a bare status code — `DeepL`'s error bodies say
/// things like which field was invalid, which is otherwise invisible.
fn read_body_or_error(
	endpoint: &str,
	mut response: ureq::http::Response<ureq::Body>,
) -> Result<String, Box<dyn Error>> {
	let status = response.status();
	let text = response.body_mut().read_to_string().unwrap_or_default();
	if !status.is_success() {
		return Err(format!("DeepL {endpoint} request returned HTTP {status}: {text}").into());
	}
	Ok(text)
}

/// Maps a `po/<lang>.po` filename stem to the `DeepL` target language code(s) it could
/// resolve to, most-preferred first, then picks the first one `DeepL`'s own
/// `/v2/languages?type=target` response actually lists. Returns `None` when nothing
/// matches (e.g. `bs`/`sr`/`vi` — `DeepL` doesn't cover them as of writing) so those
/// languages are simply skipped rather than needing a hardcoded exclusion list that goes
/// stale if `DeepL` adds coverage later.
#[must_use]
pub fn resolve_target_lang(po_lang: &str, supported: &HashSet<String>) -> Option<String> {
	let candidates: Vec<String> = match po_lang {
		"pt_br" => vec!["PT-BR".to_string(), "PT".to_string()],
		"zh_CN" => vec!["ZH".to_string(), "ZH-HANS".to_string()],
		other => vec![other.to_uppercase()],
	};
	candidates.into_iter().find(|c| supported.contains(c))
}

/// Wraps `DeepL`'s XML-protection tag around the placeholder tokens `%s`/`%d`/`{}` (the
/// only styles used in this project's strings) so `tag_handling=xml` + `ignore_tags=["x"]`
/// keeps them byte-for-byte instead of risking a reworded or reordered placeholder.
/// Escapes the surrounding text for XML first, since the whole payload is parsed as XML
/// once tags are added.
fn protect_placeholders(s: &str) -> String {
	xml_escape(s).replace("%s", "<x>%s</x>").replace("%d", "<x>%d</x>").replace("{}", "<x>{}</x>")
}

/// Inverse of [`protect_placeholders`]: strips the wrapper tags `DeepL` echoes back
/// unchanged, then un-escapes the XML entities.
fn unprotect_placeholders(s: &str) -> String {
	xml_unescape(&s.replace("<x>", "").replace("</x>", ""))
}

fn xml_escape(s: &str) -> String {
	s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn xml_unescape(s: &str) -> String {
	s.replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn resolves_direct_upper_case_codes() {
		let supported: HashSet<String> = ["DE", "FR", "JA"].iter().map(|s| (*s).to_string()).collect();
		assert_eq!(resolve_target_lang("de", &supported), Some("DE".to_string()));
		assert_eq!(resolve_target_lang("fr", &supported), Some("FR".to_string()));
	}

	#[test]
	fn resolves_pt_br_to_hyphenated_form() {
		let supported: HashSet<String> = std::iter::once("PT-BR".to_string()).collect();
		assert_eq!(resolve_target_lang("pt_br", &supported), Some("PT-BR".to_string()));
	}

	#[test]
	fn resolves_zh_cn_falling_back_to_zh_hans() {
		let supported: HashSet<String> = std::iter::once("ZH-HANS".to_string()).collect();
		assert_eq!(resolve_target_lang("zh_CN", &supported), Some("ZH-HANS".to_string()));
	}

	#[test]
	fn unsupported_languages_resolve_to_none() {
		let supported: HashSet<String> = ["DE", "FR"].iter().map(|s| (*s).to_string()).collect();
		assert_eq!(resolve_target_lang("bs", &supported), None);
		assert_eq!(resolve_target_lang("sr", &supported), None);
		assert_eq!(resolve_target_lang("vi", &supported), None);
	}

	#[test]
	fn placeholder_round_trip_preserves_tokens() {
		let original = "%s Heading level %d";
		let protected = protect_placeholders(original);
		assert_eq!(protected, "<x>%s</x> Heading level <x>%d</x>");
		assert_eq!(unprotect_placeholders(&protected), original);
	}

	#[test]
	fn brace_placeholder_round_trip() {
		let original = "Remove the {} selected documents?";
		let protected = protect_placeholders(original);
		assert_eq!(protected, "Remove the <x>{}</x> selected documents?");
		assert_eq!(unprotect_placeholders(&protected), original);
	}

	#[test]
	fn xml_special_chars_round_trip() {
		let original = "A & B <tag>";
		let protected = protect_placeholders(original);
		assert_eq!(unprotect_placeholders(&protected), original);
	}
}
