//! The line-oriented block parser: turns reStructuredText source into HTML. See the module
//! doc comment on [`super`] for why this is hand-written rather than a wrapped crate.

use std::{
	collections::{HashMap, HashSet},
	fmt::Write,
};

/// Converts reStructuredText source to HTML for the shared `HtmlToText` converter.
///
/// Headings and hyperlink targets get an `id` attribute (their label, normalized), and a
/// reference to one becomes an `<a href="#label">` rather than plain text: `HtmlToText` reads
/// those ids into `Document::id_positions`, which is the same mechanism EPUB/HTML/CHM use for
/// in-book navigation, so an RST cross-reference becomes a genuine jump-to-position link rather
/// than inert text. A reference to `http://`/`https://`/`mailto:` (or embedding one directly, as
/// `` `text <https://...>`_ `` does) stays an ordinary external href, which the reader's link
/// resolver (`is_external_url`) already knows to open outside the book.
#[must_use]
pub fn rst_to_html(source: &str) -> String {
	let lines: Vec<String> = source.lines().map(str::to_string).collect();
	let len = lines.len();
	let mut conv = Converter {
		lines,
		targets: HashMap::new(),
		internal_anchors: HashSet::new(),
		substitutions: HashMap::new(),
		levels: Vec::new(),
	};
	conv.collect_definitions();
	conv.render_blocks(0, len, 0)
}

struct Converter {
	lines: Vec<String>,
	targets: HashMap<String, String>,
	/// Labels of explicit targets with no URL of their own (`.. _label:` with nothing after the
	/// colon), which bind instead to whichever block follows them in the document.
	internal_anchors: HashSet<String>,
	substitutions: HashMap<String, String>,
	levels: Vec<char>,
}

const ADMONITIONS: &[&str] = &[
	"note",
	"warning",
	"important",
	"tip",
	"caution",
	"danger",
	"attention",
	"error",
	"hint",
	"admonition",
	"seealso",
	"deprecated",
	"versionadded",
	"versionchanged",
	"todo",
];

const LITERAL_DIRECTIVES: &[&str] = &["code-block", "code", "sourcecode", "math", "math-block", "productionlist"];

/// Directives whose body is metadata, cross-references to other files, or raw markup for another
/// output format, none of which have anything to show a reader here.
const SKIPPED_DIRECTIVES: &[&str] = &[
	"toctree",
	"include",
	"raw",
	"meta",
	"highlight",
	"highlightlang",
	"index",
	"default-role",
	"role",
	"sectnum",
	"contents",
	"codeauthor",
	"moduleauthor",
	"currentmodule",
	"module",
	"tabularcolumns",
	"cssclass",
	"class",
];

impl Converter {
	fn line(&self, i: usize) -> &str {
		&self.lines[i]
	}

	/// A first pass collecting `.. _label: url` targets, `.. |name| replace:: text` substitutions
	/// and every section title's implicit target, wherever in the document they appear: RST
	/// allows all three to be referenced before their definition.
	fn collect_definitions(&mut self) {
		let mut i = 0;
		while i < self.lines.len() {
			let line = self.line(i).to_string();
			let trimmed = line.trim_start();
			if let Some(rest) = trimmed.strip_prefix(".. _") {
				let (label, mut url) = split_once_colon(rest);
				let indent = line_indent(&line);
				let mut j = i + 1;
				while j < self.lines.len() && is_continuation(self.line(j), indent) {
					url.push(' ');
					url.push_str(self.line(j).trim());
					j += 1;
				}
				let label = normalize_label(&label);
				let url = url.trim().trim_start_matches('<').trim_end_matches('>').trim().to_string();
				if !label.is_empty() {
					if url.is_empty() {
						// An anchor-only target: it points at whichever block follows it rather
						// than at a URL of its own.
						self.internal_anchors.insert(label.clone());
						self.targets.insert(label.clone(), format!("#{label}"));
					} else {
						self.targets.insert(label, url);
					}
				}
				i = j;
				continue;
			}
			if let Some(rest) = trimmed.strip_prefix(".. |")
				&& let Some(bar) = rest.find('|')
			{
				let name = rest[..bar].trim().to_ascii_lowercase();
				let after = rest[bar + 1..].trim_start();
				if let Some(directive_rest) = after.strip_prefix("replace::") {
					self.substitutions.insert(name, directive_rest.trim().to_string());
				} else if !name.is_empty() {
					// Image or other substitution directives carry no plain-text value.
					self.substitutions.entry(name).or_default();
				}
			}
			i += 1;
		}
		self.collect_section_targets();
	}

	/// Every section title implicitly names a hyperlink target (its own text, normalized), so a
	/// `` `Some Section`_ `` reference resolves without an explicit `.. _Some Section:` target,
	/// as an internal `#label` link the heading's own `id` attribute (set in [`Self::try_title`])
	/// answers to. Runs after explicit targets are collected, so a real target with the same
	/// label wins.
	fn collect_section_targets(&mut self) {
		let is_text = |s: &str| !s.trim().is_empty() && adornment_char(s).is_none();
		let mut i = 0;
		while i < self.lines.len() {
			let title = if i + 2 < self.lines.len()
				&& adornment_char(self.line(i)).is_some()
				&& is_text(self.line(i + 1))
				&& adornment_char(self.line(i + 2)).is_some_and(|u| Some(u) == adornment_char(self.line(i)))
			{
				Some(self.line(i + 1).trim().to_string())
			} else if i + 1 < self.lines.len() && is_text(self.line(i)) {
				let title_len = self.line(i).trim_end().chars().count();
				adornment_char(self.line(i + 1))
					.filter(|_| self.line(i + 1).trim().chars().count() >= title_len)
					.map(|_| self.line(i).trim().to_string())
			} else {
				None
			};
			if let Some(title) = title {
				let label = normalize_label(&title);
				self.targets.entry(label.clone()).or_insert_with(|| format!("#{label}"));
			}
			i += 1;
		}
	}

	/// The heading level for an adornment character, assigned by the order distinct characters
	/// are first seen. Real documents nearly always use one character per level consistently
	/// (`=` for parts, `-` for chapters, and so on), so this is simpler than docutils' full
	/// nesting algorithm while matching it for every document written that way.
	fn level_for(&mut self, adornment: char) -> usize {
		if let Some(pos) = self.levels.iter().position(|&c| c == adornment) {
			pos + 1
		} else {
			self.levels.push(adornment);
			self.levels.len()
		}
		.min(6)
	}

	/// Renders every sibling block in `[start, end)`, all belonging to the same indentation
	/// context, which is at least `indent` columns deep.
	fn render_blocks(&mut self, start: usize, end: usize, indent: usize) -> String {
		let mut html = String::new();
		let mut i = start;
		while i < end {
			if self.line(i).trim().is_empty() {
				i += 1;
				continue;
			}
			let (block_html, next) = self.render_one_block(i, end, indent);
			html.push_str(&block_html);
			// `next` must exceed `i` even if a block type matched but consumed nothing, or a
			// malformed document could loop here forever instead of degrading to plain text.
			i = next.max(i + 1);
		}
		html
	}

	/// Renders the single block starting at line `i`, returning its HTML and the index of the
	/// line after it.
	fn render_one_block(&mut self, i: usize, end: usize, indent: usize) -> (String, usize) {
		if let Some(result) = self.try_title(i, end) {
			return result;
		}
		if let Some(result) = self.try_transition(i, end) {
			return result;
		}
		if let Some(result) = self.try_table(i, end, indent) {
			return result;
		}
		if let Some(result) = self.try_directive_or_target_or_comment(i, end, indent) {
			return result;
		}
		if let Some(result) = self.try_field_list(i, end, indent) {
			return result;
		}
		if let Some(result) = self.try_bullet_list(i, end, indent) {
			return result;
		}
		if let Some(result) = self.try_enum_list(i, end, indent) {
			return result;
		}
		if let Some(result) = self.try_definition_list(i, end, indent) {
			return result;
		}
		// A more deeply indented run with nothing else claiming it is a block quote.
		let this_indent = line_indent(self.line(i));
		if this_indent > indent {
			let quote_end = self.indented_run_end(i, end, this_indent);
			let body = self.render_blocks(i, quote_end, this_indent);
			return (format!("<blockquote>{body}</blockquote>"), quote_end);
		}
		self.paragraph(i, end, indent)
	}

	fn try_title(&mut self, i: usize, end: usize) -> Option<(String, usize)> {
		let is_text = |s: &str| !s.trim().is_empty() && adornment_char(s).is_none();
		// Overline + text + underline.
		if i + 2 < end
			&& let Some(over) = adornment_char(self.line(i))
			&& is_text(self.line(i + 1))
			&& let Some(under) = adornment_char(self.line(i + 2))
			&& over == under
		{
			let text = self.line(i + 1).trim().to_string();
			let level = self.level_for(over);
			let id = escape_html(&normalize_label(&text));
			let inline = self.inline(&text);
			return Some((format!(r#"<h{level} id="{id}">{inline}</h{level}>"#), i + 3));
		}
		// Text + underline.
		if i + 1 < end
			&& is_text(self.line(i))
			&& let Some(under) = adornment_char(self.line(i + 1))
		{
			let title_len = self.line(i).trim_end().chars().count();
			let adorn_len = self.line(i + 1).trim().chars().count();
			if adorn_len >= title_len {
				let text = self.line(i).trim().to_string();
				let level = self.level_for(under);
				let id = escape_html(&normalize_label(&text));
				let inline = self.inline(&text);
				return Some((format!(r#"<h{level} id="{id}">{inline}</h{level}>"#), i + 2));
			}
		}
		None
	}

	/// A line of 4 or more repeated punctuation characters standing on its own, once
	/// [`try_title`] has had first refusal at reading it as a title's overline or underline.
	fn try_transition(&self, i: usize, end: usize) -> Option<(String, usize)> {
		let line = self.line(i);
		adornment_char(line)?;
		if line.trim().chars().count() < 4 {
			return None;
		}
		let before_blank = i == 0 || self.line(i - 1).trim().is_empty();
		let after_blank = i + 1 >= end || self.line(i + 1).trim().is_empty();
		(before_blank && after_blank).then(|| ("<hr>".to_string(), i + 1))
	}

	/// A grid table (`+---+---+`) or simple table (`===  ===`) border line, and everything else
	/// through the end of the block. Full parsing into `<table>` rows/columns isn't attempted, but
	/// preserving the source's own line breaks in a literal block is enough to keep a table
	/// readable (and, for a screen reader, keeps it from being read as one run-on line of dashes
	/// and pipes).
	fn try_table(&self, i: usize, end: usize, indent: usize) -> Option<(String, usize)> {
		let line = self.line(i);
		if line_indent(line) != indent || !is_table_border(line) {
			return None;
		}
		let mut j = i + 1;
		while j < end && !self.line(j).trim().is_empty() {
			j += 1;
		}
		Some((format!("<pre><code>{}</code></pre>", escape_html(&self.literal_text(i, j))), j))
	}

	fn try_directive_or_target_or_comment(&mut self, i: usize, end: usize, indent: usize) -> Option<(String, usize)> {
		let line = self.line(i).to_string();
		let trimmed = line.trim_start();
		let rest = trimmed.strip_prefix("..")?;
		let rest = rest.strip_prefix(' ').unwrap_or(rest);
		let body_end = self.indented_or_blank_run_end(i + 1, end, indent);

		// Hyperlink target / anonymous target: its URL (if any) was already collected. An
		// anchor-only one (no URL) binds to whichever block follows it, so that block is rendered
		// here and gets the anchor's `id` attached, rather than the target simply vanishing.
		if let Some(label_part) = rest.strip_prefix('_') {
			let (label, _) = split_once_colon(label_part);
			let label = normalize_label(&label);
			if self.internal_anchors.contains(&label) {
				// `body_end` is the target's own (empty) body; the block it binds to starts
				// after the blank line separating them.
				let mut next_start = body_end;
				while next_start < end && self.line(next_start).trim().is_empty() {
					next_start += 1;
				}
				if next_start < end {
					let (html, next) = self.render_one_block(next_start, end, indent);
					return Some((attach_id(html, &label), next));
				}
			}
			return Some((String::new(), body_end));
		}
		// Substitution definition: already collected, but its body (e.g. image options) is skipped.
		if rest.starts_with('|') {
			return Some((String::new(), body_end));
		}
		// Footnote or citation definition (`.. [1] body` / `.. [CIT2002] body`): resolving it back
		// to its `[1]_` reference isn't attempted, but the body text itself is real content and
		// must not silently vanish the way a comment does.
		if let Some(after_bracket) = rest.strip_prefix('[')
			&& let Some(close) = after_bracket.find(']')
		{
			let label = after_bracket[..close].to_string();
			let mut text = after_bracket[close + 1..].trim_start().to_string();
			for k in (i + 1)..body_end {
				let continuation = self.line(k).trim();
				if !continuation.is_empty() {
					if !text.is_empty() {
						text.push(' ');
					}
					text.push_str(continuation);
				}
			}
			let full = format!("[{label}] {}", text.trim());
			return Some((format!("<p>{}</p>", self.inline(&full)), body_end));
		}
		// Directive: `name:: arguments`.
		if let Some(colon_pos) = rest.find("::") {
			let name = rest[..colon_pos].trim();
			if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
				let arg = rest[colon_pos + 2..].trim().to_string();
				let name = name.to_string();
				return Some(self.directive(&name, &arg, i + 1, body_end, indent));
			}
		}
		// Anything else starting with `..` is a comment.
		Some((String::new(), body_end))
	}

	fn directive(
		&mut self,
		name: &str,
		arg: &str,
		body_start: usize,
		body_end: usize,
		indent: usize,
	) -> (String, usize) {
		let name_lower = name.to_ascii_lowercase();
		let (options, content_start) = self.read_field_options(body_start, body_end);

		if name_lower == "image" || name_lower == "figure" {
			let alt = options.get("alt").cloned().unwrap_or_default();
			let img = format!("<img alt=\"{}\">", escape_html(&alt));
			if name_lower == "image" {
				return (img, body_end);
			}
			let caption_indent = self.first_indent_or(content_start, body_end, indent + 1);
			let caption = self.render_blocks(content_start, body_end, caption_indent);
			return (format!("<figure>{img}<figcaption>{caption}</figcaption></figure>"), body_end);
		}
		if LITERAL_DIRECTIVES.contains(&name_lower.as_str()) {
			let text = self.literal_text(content_start, body_end);
			return (format!("<pre><code>{}</code></pre>", escape_html(&text)), body_end);
		}
		if ADMONITIONS.contains(&name_lower.as_str()) {
			let label =
				if name_lower == "admonition" && !arg.is_empty() { arg.to_string() } else { capitalize(&name_lower) };
			let body_indent = self.first_indent_or(content_start, body_end, indent + 1);
			let body = self.render_blocks(content_start, body_end, body_indent);
			return (
				format!("<blockquote><p><strong>{}</strong></p>{body}</blockquote>", escape_html(&label)),
				body_end,
			);
		}
		if SKIPPED_DIRECTIVES.contains(&name_lower.as_str()) {
			return (String::new(), body_end);
		}
		if name_lower == "rubric" {
			return (format!("<p><strong>{}</strong></p>", escape_html(arg)), body_end);
		}
		// Unknown or purely structural directives (only, container, topic, compound, ...): show
		// their body, since hiding it would silently drop real content from the reader.
		let body_indent = self.first_indent_or(content_start, body_end, indent + 1);
		let body = self.render_blocks(content_start, body_end, body_indent);
		(body, body_end)
	}

	fn first_indent_or(&self, start: usize, end: usize, default: usize) -> usize {
		(start..end).find(|&i| !self.line(i).trim().is_empty()).map_or(default, |i| line_indent(self.line(i)))
	}

	/// Reads any `:field: value` option lines right after a directive's own line, returning them
	/// and the index of the first line after them.
	fn read_field_options(&self, start: usize, end: usize) -> (HashMap<String, String>, usize) {
		let mut options = HashMap::new();
		let mut i = start;
		while i < end {
			let line = self.line(i);
			if line.trim().is_empty() {
				i += 1;
				continue;
			}
			let trimmed = line.trim_start();
			if let Some(rest) = trimmed.strip_prefix(':')
				&& let Some(close) = rest.find(':')
			{
				let key = rest[..close].trim().to_ascii_lowercase();
				let value = rest[close + 1..].trim().to_string();
				options.insert(key, value);
				i += 1;
				continue;
			}
			break;
		}
		(options, i)
	}

	fn literal_text(&self, start: usize, end: usize) -> String {
		let min_indent = (start..end)
			.filter(|&i| !self.line(i).trim().is_empty())
			.map(|i| line_indent(self.line(i)))
			.min()
			.unwrap_or(0);
		let mut out = String::new();
		for i in start..end {
			let line = self.line(i);
			let dedented = if line.len() >= min_indent { &line[min_indent.min(line.len())..] } else { line };
			out.push_str(dedented);
			if i + 1 < end {
				out.push('\n');
			}
		}
		out
	}

	fn try_field_list(&self, i: usize, end: usize, indent: usize) -> Option<(String, usize)> {
		let line = self.line(i);
		if line_indent(line) != indent {
			return None;
		}
		field_marker_close(line)?;
		let mut items = String::new();
		let mut j = i;
		loop {
			let line = self.line(j);
			let trimmed = line.trim_start();
			let Some(close) = field_marker_close(line) else { break };
			let rest = &trimmed[1..];
			let field = rest[..close].to_string();
			let value_start = rest[close + 1..].trim().to_string();
			let body_end = self.indented_or_blank_run_end(j + 1, end, indent);
			let mut value = value_start;
			for k in (j + 1)..body_end {
				if !self.line(k).trim().is_empty() {
					if !value.is_empty() {
						value.push(' ');
					}
					value.push_str(self.line(k).trim());
				}
			}
			let _ = write!(items, "<dt>{}</dt><dd>{}</dd>", escape_html(&field), self.inline(&value));
			j = body_end;
			if j >= end || self.line(j).trim().is_empty() {
				break;
			}
		}
		Some((format!("<dl>{items}</dl>"), j))
	}

	fn try_bullet_list(&mut self, i: usize, end: usize, indent: usize) -> Option<(String, usize)> {
		let line = self.line(i);
		if line_indent(line) != indent {
			return None;
		}
		let bullet = bullet_marker(line)?;
		let mut html = String::new();
		let mut j = i;
		while j < end {
			let line = self.line(j);
			if line.trim().is_empty() {
				j += 1;
				continue;
			}
			if line_indent(line) != indent {
				break;
			}
			let Some((marker, content_col)) = bullet_item_start(line) else { break };
			if marker != bullet {
				break;
			}
			let item_end = self.item_extent(j, end, indent);
			let body = self.render_item_body(j, item_end, content_col);
			let _ = write!(html, "<li>{body}</li>");
			j = item_end;
		}
		Some((format!("<ul>{html}</ul>"), j))
	}

	fn try_enum_list(&mut self, i: usize, end: usize, indent: usize) -> Option<(String, usize)> {
		let line = self.line(i);
		if line_indent(line) != indent {
			return None;
		}
		enum_item_start(line)?;
		let mut html = String::new();
		let mut j = i;
		while j < end {
			let line = self.line(j);
			if line.trim().is_empty() {
				j += 1;
				continue;
			}
			if line_indent(line) != indent {
				break;
			}
			let Some(content_col) = enum_item_start(line) else { break };
			let item_end = self.item_extent(j, end, indent);
			let body = self.render_item_body(j, item_end, content_col);
			let _ = write!(html, "<li>{body}</li>");
			j = item_end;
		}
		Some((format!("<ol>{html}</ol>"), j))
	}

	/// The lines making up one list item: from `j` (its marker line) up to (but not including)
	/// the next item at the list's own `indent`, or a dedent back to or below it.
	fn item_extent(&self, j: usize, end: usize, indent: usize) -> usize {
		let mut k = j + 1;
		while k < end {
			let line = self.line(k);
			if line.trim().is_empty() {
				k += 1;
				continue;
			}
			if line_indent(line) <= indent {
				break;
			}
			k += 1;
		}
		while k > j + 1 && self.line(k - 1).trim().is_empty() {
			k -= 1;
		}
		k
	}

	/// Renders one list item's body: its marker line's own remainder, re-based to `content_col`
	/// so any continuation lines and nested blocks line up with it. Temporarily rewrites the
	/// marker line in place (padding it out to `content_col`) so the shared block renderer can
	/// treat it uniformly with the rest of the item, then restores it.
	fn render_item_body(&mut self, start: usize, end: usize, content_col: usize) -> String {
		let original = self.lines[start].clone();
		let rest = if original.len() >= content_col { original[content_col..].to_string() } else { String::new() };
		if rest.trim().is_empty() {
			return self.render_blocks(start + 1, end, content_col);
		}
		self.lines[start] = format!("{}{}", " ".repeat(content_col), rest);
		let html = self.render_blocks(start, end, content_col);
		self.lines[start] = original;
		html
	}

	fn try_definition_list(&mut self, i: usize, end: usize, indent: usize) -> Option<(String, usize)> {
		let line = self.line(i);
		if line_indent(line) != indent || line.trim().is_empty() {
			return None;
		}
		if adornment_char(line).is_some() || bullet_marker(line).is_some() || enum_item_start(line).is_some() {
			return None;
		}
		if line.trim_start().starts_with("..") || line.trim_start().starts_with(':') {
			return None;
		}
		let next = i + 1;
		if next >= end || self.line(next).trim().is_empty() || line_indent(self.line(next)) <= indent {
			return None;
		}
		let mut html = String::new();
		let mut j = i;
		loop {
			if j >= end || self.line(j).trim().is_empty() || line_indent(self.line(j)) != indent {
				break;
			}
			let term = self.line(j).trim().to_string();
			if j + 1 >= end || self.line(j + 1).trim().is_empty() {
				break;
			}
			let def_indent = line_indent(self.line(j + 1));
			if def_indent <= indent {
				break;
			}
			let def_end = self.indented_run_end(j + 1, end, def_indent);
			let body = self.render_blocks(j + 1, def_end, def_indent);
			let _ = write!(html, "<dt>{}</dt><dd>{body}</dd>", self.inline(&term));
			j = def_end;
		}
		if html.is_empty() { None } else { Some((format!("<dl>{html}</dl>"), j)) }
	}

	fn paragraph(&self, i: usize, end: usize, indent: usize) -> (String, usize) {
		let mut j = i;
		let mut text = String::new();
		while j < end {
			let line = self.line(j);
			if line.trim().is_empty() || line_indent(line) != indent {
				break;
			}
			if j > i && Self::starts_new_block(line) {
				break;
			}
			if !text.is_empty() {
				text.push(' ');
			}
			text.push_str(line.trim());
			j += 1;
		}
		// A paragraph ending in `::` introduces a literal block: docutils folds the `::` into a
		// trailing `:` (or drops it if it was already preceded by whitespace) before showing it.
		let literal_follows = text.trim_end().ends_with("::");
		if literal_follows {
			let stripped = text.trim_end();
			let stripped = &stripped[..stripped.len() - 2];
			text = if stripped.ends_with(' ') || stripped.is_empty() {
				stripped.trim_end().to_string()
			} else {
				format!("{stripped}:")
			};
		}
		let mut html = if text.is_empty() { String::new() } else { format!("<p>{}</p>", self.inline(&text)) };
		let mut next = j;
		if literal_follows {
			let mut k = j;
			while k < end && self.line(k).trim().is_empty() {
				k += 1;
			}
			if k < end && line_indent(self.line(k)) > indent {
				let block_indent = line_indent(self.line(k));
				let block_end = self.indented_run_end(k, end, block_indent);
				let literal = self.literal_text(k, block_end);
				let _ = write!(html, "<pre><code>{}</code></pre>", escape_html(&literal));
				next = block_end;
			}
		}
		(html, next)
	}

	/// Whether `line` looks like the start of some other block type, so a paragraph already in
	/// progress knows to stop even without a blank line separating them (real documents aren't
	/// always strict about that blank line).
	fn starts_new_block(line: &str) -> bool {
		adornment_char(line).is_some()
			|| bullet_marker(line).is_some()
			|| enum_item_start(line).is_some()
			|| line.trim_start().starts_with("..")
			|| field_marker_close(line).is_some()
	}

	/// The end of a run of lines all indented at least `min_indent`, stopping at end-of-range, a
	/// dedent back below it, or two consecutive blank lines.
	fn indented_run_end(&self, start: usize, end: usize, min_indent: usize) -> usize {
		let mut i = start;
		let mut blank_run = 0;
		while i < end {
			let line = self.line(i);
			if line.trim().is_empty() {
				blank_run += 1;
				if blank_run >= 2 {
					break;
				}
				i += 1;
				continue;
			}
			blank_run = 0;
			if line_indent(line) < min_indent {
				break;
			}
			i += 1;
		}
		while i > start && self.line(i - 1).trim().is_empty() {
			i -= 1;
		}
		i
	}

	/// Like [`Self::indented_run_end`], but for content that must be either blank or indented
	/// *more than* `base_indent` (a directive's own line, or a field-list marker's line).
	fn indented_or_blank_run_end(&self, start: usize, end: usize, base_indent: usize) -> usize {
		let mut i = start;
		while i < end {
			let line = self.line(i);
			if line.trim().is_empty() {
				i += 1;
				continue;
			}
			if line_indent(line) <= base_indent {
				break;
			}
			i += 1;
		}
		while i > start && self.line(i - 1).trim().is_empty() {
			i -= 1;
		}
		i
	}

	fn inline(&self, text: &str) -> String {
		inline::render(text, &self.targets, &self.substitutions)
	}
}

fn line_indent(line: &str) -> usize {
	line.chars().take_while(|c| *c == ' ').count()
}

fn is_continuation(line: &str, target_indent: usize) -> bool {
	!line.trim().is_empty() && line_indent(line) > target_indent
}

/// Splits `label: rest` at the first colon, as used by `.. _label: url` and its anonymous form.
fn split_once_colon(s: &str) -> (String, String) {
	s.find(':').map_or_else(
		|| (s.trim().to_string(), String::new()),
		|pos| (s[..pos].trim().to_string(), s[pos + 1..].to_string()),
	)
}

fn normalize_label(label: &str) -> String {
	label.trim().trim_matches('`').trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

/// The byte offset (within `line.trim_start()`, after the leading `:`) of a field-list marker's
/// closing `:` (`:name: value`), or `None` if `line` isn't one.
///
/// Distinguishes a field marker from an interpreted-text role like `:func:` starting a wrapped
/// paragraph line - `` See :func:`sorted`, and\n:func:`min`, ... `` is a real Sphinx idiom, and
/// its second line must stay part of the paragraph rather than being read as a one-field list. A
/// role's second colon is always immediately followed by the backtick that opens its interpreted
/// text; a field marker's is followed by whitespace, another field, or nothing.
fn field_marker_close(line: &str) -> Option<usize> {
	let rest = line.trim_start().strip_prefix(':')?;
	let close = rest.find(':')?;
	if rest[..close].trim().is_empty() || rest[close + 1..].starts_with('`') {
		return None;
	}
	Some(close)
}

/// Whether `line` opens a grid table (`+------+------+` or `+======+======+`) or a simple table
/// (`=====  =====`, two or more `=` runs separated by whitespace). A single `=====` run alone is
/// deliberately not matched here: it's indistinguishable from a section title's underline without
/// looking at what follows, and titles are far more common.
fn is_table_border(line: &str) -> bool {
	let trimmed = line.trim();
	if trimmed.len() >= 3
		&& trimmed.starts_with('+')
		&& trimmed.ends_with('+')
		&& trimmed.chars().all(|c| matches!(c, '+' | '-' | '='))
		&& trimmed.contains(['-', '='])
	{
		return true;
	}
	let groups: Vec<&str> = trimmed.split_whitespace().collect();
	groups.len() >= 2 && groups.iter().all(|g| !g.is_empty() && g.chars().all(|c| c == '='))
}

/// The repeated punctuation character a section-title or transition line is made of, or `None`.
fn adornment_char(line: &str) -> Option<char> {
	let trimmed = line.trim_end();
	if trimmed.is_empty() {
		return None;
	}
	let first = trimmed.chars().next()?;
	if first.is_alphanumeric() || first.is_whitespace() {
		return None;
	}
	trimmed.chars().all(|c| c == first).then_some(first)
}

/// The bullet character of a `- `/`* `/`+ ` list item line, if it is one.
fn bullet_marker(line: &str) -> Option<char> {
	bullet_item_start(line).map(|(c, _)| c)
}

fn bullet_item_start(line: &str) -> Option<(char, usize)> {
	let indent = line_indent(line);
	let rest = &line[indent..];
	let mut chars = rest.chars();
	let marker = chars.next()?;
	if !matches!(marker, '-' | '*' | '+') {
		return None;
	}
	let after = chars.next();
	if after != Some(' ') {
		return None;
	}
	let spaces = rest[1..].chars().take_while(|c| *c == ' ').count();
	Some((marker, indent + 1 + spaces))
}

/// The column where an enumerated list item's content starts, if `line` opens one: `1.`, `1)`,
/// `a.`, `#.` and similar, followed by whitespace.
fn enum_item_start(line: &str) -> Option<usize> {
	let indent = line_indent(line);
	let rest = &line[indent..];
	let end = rest.find(['.', ')'])?;
	let token = &rest[..end];
	let valid = token == "#"
		|| (!token.is_empty() && token.chars().all(|c| c.is_ascii_digit()))
		|| (token.chars().count() <= 2 && !token.is_empty() && token.chars().all(|c| c.is_ascii_alphabetic()));
	if !valid {
		return None;
	}
	let after_marker = &rest[end + 1..];
	if !after_marker.starts_with(' ') {
		return None;
	}
	let spaces = after_marker.chars().take_while(|c| *c == ' ').count();
	Some(indent + end + 1 + spaces)
}

fn capitalize(s: &str) -> String {
	let mut chars = s.chars();
	match chars.next() {
		Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
		None => String::new(),
	}
}

fn escape_html(s: &str) -> String {
	s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Inserts an `id="label"` attribute into `html`'s very first opening tag, so an anchor-only
/// target (see [`Converter::try_directive_or_target_or_comment`]) binds to the block that follows
/// it. `html` always starts with a tag here (every block renderer opens on one), but falls back to
/// returning it untouched rather than panicking if that ever stops holding.
fn attach_id(html: String, label: &str) -> String {
	let Some(name_start) = html.strip_prefix('<').map(|_| 1) else { return html };
	let name_end = html[name_start..].find([' ', '>']).map_or(html.len(), |offset| name_start + offset);
	if name_end >= html.len() {
		return html;
	}
	format!(r#"{} id="{}"{}"#, &html[..name_end], escape_html(label), &html[name_end..])
}

mod inline;

#[cfg(test)]
mod tests;
