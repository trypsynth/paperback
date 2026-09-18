//! `MathML` conversion shared by the XML/HTML text engines. `MathCAT`'s rules are embedded;
//! its per-thread state is initialized lazily, including on EPUB's rayon workers.

use std::{
	cell::OnceCell,
	collections::HashMap,
	sync::{LazyLock, Mutex},
};

use roxmltree::Node;

use crate::util::{
	html::{escape, escape_attr},
	text::collapse_whitespace,
};

const MATHML_NAMESPACE: &str = "http://www.w3.org/1998/Math/MathML";
const MAX_CACHE_BYTES: usize = 8 * 1024 * 1024;

pub(super) fn is_xml_formula(node: Node<'_, '_>) -> bool {
	node.is_element()
		&& node.tag_name().name() == "math"
		&& matches!(node.tag_name().namespace(), None | Some(MATHML_NAMESPACE))
}

pub(super) fn is_dom_formula(element: &scraper::node::Element) -> bool {
	// HTML parsing assigns the MathML namespace to bare <math> even without xmlns. Checking
	// that parsed namespace excludes same-named elements in other contexts, such as SVG.
	element.name() == "math" && element.name.ns.as_ref() == MATHML_NAMESPACE
}

#[derive(Default)]
struct FormulaCache {
	entries: HashMap<String, Option<String>>,
	bytes: usize,
}

// Share repeated expressions across sections without retaining an unbounded history of books.
static CACHE: LazyLock<Mutex<FormulaCache>> = LazyLock::new(|| Mutex::new(FormulaCache::default()));

thread_local! {
	static MATHCAT_INIT: OnceCell<bool> = const { OnceCell::new() };
}

fn init() -> bool {
	// TODO: make the output code configurable when math preferences are implemented.
	// `rules` is embedded in the zip vfs in the binary; it does not represent an on-disk directory.
	// It is nevertheless required for MathCat initialization.
	let result = libmathcat::set_rules_dir("Rules".to_string())
		// Without this, MathCAT re-reads its preference files on every get_braille call, which
		// resets BrailleCode to its shipped default (Nemeth) and discards the ASCIIMath set below,
		// so the reader hears Nemeth braille cells instead of AsciiMath.
		.and_then(|()| libmathcat::set_preference("CheckRuleFiles".to_string(), "None".to_string()))
		.and_then(|()| libmathcat::set_preference("Language".to_string(), "en".to_string()))
		// ASCIIMath is a text notation, despite being exposed through MathCAT's braille API.
		.and_then(|()| libmathcat::set_preference("BrailleCode".to_string(), "ASCIIMath".to_string()));
	if let Err(error) = result {
		tracing::warn!(%error, "failed to initialize embedded MathCAT rules");
		return false;
	}
	true
}

/// `AsciiMath` when conversion succeeds, then the author's alternative text, then raw text.
/// An empty expression with no fallback emits neither text nor a marker.
pub(super) fn formula_text(
	mathml: &str,
	alttext: Option<&str>,
	text_content: impl FnOnce() -> String,
) -> Option<String> {
	asciimath(mathml).or_else(|| alttext.and_then(normalized_text)).or_else(|| normalized_text(&text_content()))
}

fn normalized_text(text: &str) -> Option<String> {
	let text = collapse_whitespace(text).trim().to_string();
	(!text.is_empty()).then_some(text)
}

fn asciimath(mathml: &str) -> Option<String> {
	if !MATHCAT_INIT.with(|cell| *cell.get_or_init(init)) {
		return None;
	}
	// Do not collapse whitespace in the key: whitespace inside mtext and attribute values can
	// be significant. Serialization already canonicalizes XML element/attribute quoting.
	if let Some(hit) = CACHE.lock().unwrap().entries.get(mathml) {
		return hit.clone();
	}
	// Convert outside the lock so independent EPUB sections remain parallel.
	let result = libmathcat::set_mathml(mathml.to_string()).and_then(|_| libmathcat::get_braille(String::new()));
	let rendered = match result {
		Ok(text) => normalized_text(&text),
		Err(error) => {
			tracing::debug!(%error, "MathCAT rejected an expression; using alternative text");
			None
		}
	};
	let bytes = mathml.len() + rendered.as_ref().map_or(0, String::len);
	if bytes <= MAX_CACHE_BYTES {
		let mut cache = CACHE.lock().unwrap();
		if !cache.entries.contains_key(mathml) {
			if cache.bytes + bytes > MAX_CACHE_BYTES {
				// TODO: move to an LRU cache instead of wiping completely.
				cache.entries.clear();
				cache.bytes = 0;
			}
			cache.bytes += bytes;
			cache.entries.insert(mathml.to_string(), rendered.clone());
		}
	}
	rendered
}

/// MathML expressions may use XML namespace prefixes declared on an ancestor element. These may
/// need to be stripped for MathCat (as its prefix handling is incomplete), and must be stripped for formula view.
///
/// Therefore, we can't pass the source MathML verbatim and must re-serialize it instead. Because Roxmltree
/// doesn't implement its own serializer, we implement just enough of one ourselves to do the job.
pub(super) fn xml_formula_fragment(node: Node<'_, '_>) -> String {
	let mut output = String::new();
	serialize_mathml(node, &mut output);
	output
}

fn serialize_mathml(node: Node<'_, '_>, output: &mut String) {
	if node.is_text() {
		output.push_str(&escape(node.text().unwrap_or("")));
		return;
	}
	// Foreign XML subtrees and custom namespaced attributes need a separate representation
	// for the HTML view. Omit them here rather than reinterpret their local names as MathML.
	if !node.is_element() || !matches!(node.tag_name().namespace(), None | Some(MATHML_NAMESPACE)) {
		return;
	}
	let name = node.tag_name().name();
	output.push('<');
	output.push_str(name);
	if name == "math" {
		output.push_str(" xmlns=\"");
		output.push_str(MATHML_NAMESPACE);
		output.push('"');
	}
	for attr in node.attributes() {
		let prefix = match attr.namespace() {
			None => "",
			Some("http://www.w3.org/XML/1998/namespace") => "xml:",
			Some(_) => continue,
		};
		output.push(' ');
		output.push_str(prefix);
		output.push_str(attr.name());
		output.push_str("=\"");
		output.push_str(&escape_attr(attr.value()).replace('<', "&lt;"));
		output.push('"');
	}
	output.push('>');
	for child in node.children() {
		serialize_mathml(child, output);
	}
	output.push_str("</");
	output.push_str(name);
	output.push('>');
}

#[cfg(test)]
mod tests;
