//! `MathCAT`'s state is per thread, so it is initialized lazily, including on EPUB's rayon workers.

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
	// HTML parsing gives bare <math> the MathML namespace even without xmlns, and checking it excludes same-named elements in other contexts such as SVG.
	element.name() == "math" && &*element.name.ns == MATHML_NAMESPACE
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
	// The rules live in the binary's embedded zip vfs, not on disk, but MathCAT still requires a rules dir.
	let result = libmathcat::set_rules_dir("Rules")
		// Otherwise MathCAT re-reads its preference files on every get_braille call, resetting BrailleCode to Nemeth and discarding the ASCIIMath set below.
		.and_then(|()| libmathcat::set_preference("CheckRuleFiles", "None"))
		.and_then(|()| libmathcat::set_preference("Language", "en"))
		// ASCIIMath is a text notation, despite being exposed through MathCAT's braille API.
		.and_then(|()| libmathcat::set_preference("BrailleCode", "ASCIIMath"));
	if let Err(error) = result {
		tracing::warn!(%error, "failed to initialize embedded MathCAT rules");
		return false;
	}
	true
}

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
	// Whitespace isn't collapsed in the key because it can be significant inside mtext and attribute values.
	if let Some(hit) = CACHE.lock().unwrap().entries.get(mathml) {
		return hit.clone();
	}
	// Convert outside the lock so independent EPUB sections remain parallel.
	let result = libmathcat::set_mathml(mathml).and_then(|_| libmathcat::get_braille(""));
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

/// Re-serializes MathML without namespace prefixes: MathCAT's prefix handling is incomplete, formula view needs them gone, and roxmltree has no serializer.
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
	// Foreign subtrees and namespaced attributes are dropped rather than having their local names reinterpreted as MathML.
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
