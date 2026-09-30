//! WinHelp (`.hlp`) documents, read through the `libhlp` crate.
//!
//! A help file is a set of topics rather than one flowing document, so each topic becomes a
//! section with its title as a heading, and the topics are concatenated in the order the
//! file stores them.
//!
//! Links are the awkward part. WinHelp stores a hotspot's destination as the hash of the
//! target's context string, never as an offset, so `libhlp` resolves the hash through the
//! file's own `|CONTEXT` index and this parser turns the topic offset that comes back into
//! a key in [`Document::id_positions`] that the reader's link machinery can look up.

use std::{collections::HashMap, fs};

use anyhow::{Context, Result};
use libhlp::{Block, Format, HlpFile, Paragraph, Run, Table, Target, Topic};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem, format_marker_types},
	parser::{Parser, util::path::extract_title_from_path},
	t,
	util::text::display_len,
};

/// Prefix for the [`Document::id_positions`] key naming a topic by its offset.
///
/// WinHelp has no file paths to key link targets by, so a topic's own numbering stands in.
/// The prefix keeps those keys clear of anything else and stops a bare number reading as a
/// fragment when the reader resolves a link.
const OFFSET_KEY: &str = "topic";

/// Prefix for the key naming a topic by its number.
///
/// Every topic has a distinct number in both format generations, where topic offsets exist
/// only in WinHelp 3.1 and later, so this is what the table of contents points at.
const NUMBER_KEY: &str = "topicnum";

pub struct HlpParser;

impl Parser for HlpParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing hlp file");
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open WinHelp file '{}'", context.file_path))?;
		let hlp = HlpFile::from_bytes(bytes).map_err(|e| failed(&context.file_path, &e))?;
		let topics = hlp.topics().map_err(|e| failed(&context.file_path, &e))?;
		tracing::debug!(
			path = %context.file_path,
			version = hlp.version(),
			topics = topics.len(),
			"hlp structure read"
		);
		let mut builder = Builder::new(hlp.contexts());
		for topic in &topics {
			builder.add_topic(topic);
		}
		let built = builder.finish(&topics);
		let title = hlp.title().map_or_else(|| extract_title_from_path(&context.file_path), ToString::to_string);
		let mut document = Document::new().with_title(title);
		document.set_buffer(built.buffer);
		document.id_positions = built.id_positions;
		document.toc_items = built.toc_items;
		tracing::debug!(path = %context.file_path, "parsed hlp file successfully");
		Ok(document)
	}
}

/// Report a `libhlp` failure as a parse error, logging the detail the user does not see.
fn failed(path: &str, error: &libhlp::HlpError) -> anyhow::Error {
	tracing::warn!(path = %path, error = %error, "failed to read winhelp file");
	// TRANSLATORS: Error shown when a WinHelp (.hlp) file cannot be read; {} is the underlying error
	anyhow::anyhow!(t("Failed to parse WinHelp file: {}").replace("{}", &error.to_string()))
}

/// What the walk produced.
struct Built {
	buffer: DocumentBuffer,
	id_positions: HashMap<String, usize>,
	toc_items: Vec<TocItem>,
}

/// Builds the document's text and markers one topic at a time.
struct Builder<'a> {
	/// The file's `|CONTEXT` index, mapping a hotspot's context hash to a topic offset.
	/// Taking the map rather than the file keeps the walk testable without one.
	contexts: &'a HashMap<u32, u32>,
	buffer: DocumentBuffer,
	/// Where each topic started, in the same order as the topics themselves.
	starts: Vec<usize>,
}

impl<'a> Builder<'a> {
	const fn new(contexts: &'a HashMap<u32, u32>) -> Self {
		Self { contexts, buffer: DocumentBuffer::new(), starts: Vec::new() }
	}

	fn add_topic(&mut self, topic: &Topic) {
		let start = self.buffer.current_position();
		self.starts.push(start);
		if !topic.title.is_empty() {
			// A WinHelp topic almost always repeats its title as the first line of its own
			// text, and adding it again would have a reader announce it twice.
			if !starts_with_title(topic) {
				self.buffer.append(&topic.title);
				self.buffer.append("\n");
			}
			self.buffer.add_marker(
				Marker::new(MarkerType::Heading1, start)
					.with_text(topic.title.clone())
					.with_length(display_len(&topic.title)),
			);
		}
		self.buffer.add_marker(Marker::new(MarkerType::SectionBreak, start).with_text(section_label(topic)));
		for block in &topic.blocks {
			self.add_block(block);
		}
	}

	fn add_block(&mut self, block: &Block) {
		match block {
			Block::Paragraph(paragraph) => {
				self.add_paragraph(paragraph);
				self.buffer.append("\n");
			}
			Block::Table(table) => self.add_table(table),
		}
	}

	/// Lay a table out as one line per row with tabs between cells, and mark the whole of
	/// it so the reader can jump table to table.
	fn add_table(&mut self, table: &Table) {
		let start = self.buffer.current_position();
		for row in &table.rows {
			for (index, cell) in row.cells.iter().enumerate() {
				if index > 0 {
					self.buffer.append("\t");
				}
				// A cell holding several paragraphs still has to stay on the row's one
				// line, so they run together with a space rather than a break.
				for (index, paragraph) in cell.paragraphs.iter().enumerate() {
					if index > 0 {
						self.buffer.append(" ");
					}
					self.add_paragraph(paragraph);
				}
			}
			self.buffer.append("\n");
		}
		let length = self.buffer.current_position().saturating_sub(start);
		if length > 0 {
			self.buffer.add_marker(Marker::new(MarkerType::Table, start).with_length(length));
		}
	}

	fn add_paragraph(&mut self, paragraph: &Paragraph) {
		// Runs split wherever a formatting command falls, so neighbouring runs usually
		// share their emphasis. Merging them leaves one marker per visible span rather
		// than one per command.
		let mut span: Option<(Format, usize)> = None;
		let mut links = Vec::new();
		for run in &paragraph.runs {
			let format = match run {
				Run::Text { format, .. } => *format,
				_ => Format::default(),
			};
			if span.is_none_or(|(open, _)| open != format) {
				if let Some((open, start)) = span {
					self.close_span(open, start);
				}
				span = Some((format, self.buffer.current_position()));
			}
			self.add_run(run, &mut links);
		}
		if let Some((open, start)) = span {
			self.close_span(open, start);
		}
		// An emphasis marker is only added once its span closes, so a hotspot's own marker
		// would otherwise land first and leave exporters interleaving the two rather than
		// wrapping one around the other.
		for marker in links {
			self.buffer.add_marker(marker);
		}
	}

	/// Emit the emphasis markers for a finished run of same-formatted text.
	fn close_span(&mut self, format: Format, start: usize) {
		let length = self.buffer.current_position().saturating_sub(start);
		if length == 0 {
			return;
		}
		for mtype in format_marker_types(format.bold, format.italic, format.underline) {
			self.buffer.add_marker(Marker::new(mtype, start).with_length(length));
		}
	}

	/// Append one run. A hotspot's marker goes into `links` rather than straight into the
	/// buffer, so the caller can order it after the emphasis covering the same text.
	fn add_run(&mut self, run: &Run, links: &mut Vec<Marker>) {
		match run {
			Run::Text { text, link, .. } => {
				let position = self.buffer.current_position();
				self.buffer.append(text);
				let Some(link) = link else { return };
				// A hotspot's reference is a key rather than a position, so it can be
				// written now even though the target topic may not have been read yet. A
				// target this file never defines leaves the reference empty, which the
				// reader treats as a link that goes nowhere.
				let reference = match &link.target {
					// WinHelp 3.1 names a target by the hash of its context string.
					Target::Context(hash) => self.contexts.get(hash).copied().map(offset_key),
					// WinHelp 3.0 names it by topic number instead.
					Target::Number(number) => Some(number_key(*number)),
					// A macro runs rather than going anywhere, and a jump into another help
					// file has nothing in this document to land on.
					Target::External { .. } | Target::Macro(_) => None,
				}
				.unwrap_or_default();
				links.push(Marker::new(MarkerType::Link, position).with_text(text.clone()).with_reference(reference));
			}
			Run::Tab => self.buffer.append("\t"),
			Run::LineBreak => self.buffer.append("\n"),
			Run::Image(_) => {
				// TRANSLATORS: Label standing in for a picture in a WinHelp topic, shown as "[Image]"
				let line = format!("[{}]", t("Image"));
				let position = self.buffer.current_position();
				self.buffer.add_marker(Marker::new(MarkerType::Image, position).with_length(display_len(&line)));
				self.buffer.append(&line);
			}
		}
	}

	fn finish(self, topics: &[Topic]) -> Built {
		let mut id_positions = HashMap::new();
		let mut toc_items = Vec::new();
		for (topic, &start) in topics.iter().zip(&self.starts) {
			id_positions.insert(number_key(topic.number), start);
			// A WinHelp 3.0 file has no topic offsets, so every topic there reports zero.
			// Only the first can claim the key, and nothing in such a file references it.
			id_positions.entry(offset_key(topic.offset)).or_insert(start);
			if !topic.title.is_empty() {
				toc_items.push(TocItem::new(topic.title.clone(), number_key(topic.number), start));
			}
		}
		Built { buffer: self.buffer, id_positions, toc_items }
	}
}

/// The key a WinHelp 3.1 hotspot's resolved target is stored under.
fn offset_key(offset: u32) -> String {
	format!("{OFFSET_KEY}{offset}")
}

/// The key naming a topic by its number, which every topic has in either format.
fn number_key(number: u32) -> String {
	format!("{NUMBER_KEY}{number}")
}

/// Whether the topic's own text already opens with its title.
fn starts_with_title(topic: &Topic) -> bool {
	topic.blocks.first().is_some_and(|block| block.text().trim_start().starts_with(topic.title.trim()))
}

/// What a section break announces. An untitled topic still needs naming.
fn section_label(topic: &Topic) -> String {
	if topic.title.is_empty() {
		// TRANSLATORS: Name given to a WinHelp topic that carries no title of its own; {} is the topic's number
		t("Topic {}").replace("{}", &topic.number.to_string())
	} else {
		topic.title.clone()
	}
}

#[cfg(test)]
mod tests {
	use libhlp::{Cell, Link, Row};

	use super::*;

	fn text_run(text: &str) -> Run {
		Run::Text { text: text.to_string(), format: Format::default(), link: None }
	}

	fn styled_run(text: &str, format: Format) -> Run {
		Run::Text { text: text.to_string(), format, link: None }
	}

	fn bold() -> Format {
		Format { bold: true, ..Format::default() }
	}

	fn paragraph(runs: Vec<Run>) -> Block {
		Block::Paragraph(Paragraph { runs })
	}

	fn topic(number: u32, offset: u32, title: &str, blocks: Vec<Block>) -> Topic {
		Topic { number, offset, title: title.to_string(), blocks }
	}

	/// Run the builder over `topics` with no context index, as a file with no links has.
	fn build(topics: &[Topic]) -> Built {
		build_with(topics, &HashMap::new())
	}

	fn build_with(topics: &[Topic], contexts: &HashMap<u32, u32>) -> Built {
		let mut builder = Builder::new(contexts);
		for topic in topics {
			builder.add_topic(topic);
		}
		builder.finish(topics)
	}

	fn markers(built: &Built, mtype: MarkerType) -> Vec<&Marker> {
		built.buffer.markers.iter().filter(|m| m.mtype == mtype).collect()
	}

	#[test]
	fn each_topic_becomes_a_section_with_a_heading() {
		let topics = vec![
			topic(16, 0, "First", vec![paragraph(vec![text_run("First")])]),
			topic(17, 40, "Second", vec![paragraph(vec![text_run("Second")])]),
		];
		let built = build(&topics);
		assert_eq!(built.buffer.content, "First\nSecond\n");
		let headings = markers(&built, MarkerType::Heading1);
		assert_eq!(headings.len(), 2);
		assert_eq!(headings[0].text, "First");
		assert_eq!(headings[1].position, 6);
		assert_eq!(markers(&built, MarkerType::SectionBreak).len(), 2);
	}

	// A WinHelp topic almost always opens with its own title, so writing the title out as
	// well would have a reader announce it twice over.
	#[test]
	fn a_title_the_body_repeats_is_not_written_twice() {
		let built = build(&[topic(16, 0, "Intro", vec![paragraph(vec![text_run("Intro")])])]);
		assert_eq!(built.buffer.content, "Intro\n");
	}

	#[test]
	fn a_title_the_body_omits_is_written_in() {
		let built = build(&[topic(16, 0, "Intro", vec![paragraph(vec![text_run("Body only.")])])]);
		assert_eq!(built.buffer.content, "Intro\nBody only.\n");
	}

	// An untitled topic still has to be reachable by section navigation, so its break gets
	// the topic's number rather than an empty name.
	#[test]
	fn an_untitled_topic_still_gets_a_named_section() {
		let built = build(&[topic(19, 0, "", vec![paragraph(vec![text_run("Body.")])])]);
		assert!(markers(&built, MarkerType::Heading1).is_empty());
		let sections = markers(&built, MarkerType::SectionBreak);
		assert_eq!(sections.len(), 1);
		assert!(sections[0].text.contains("19"));
	}

	#[test]
	fn emphasis_becomes_markers_over_the_text_it_covers() {
		let runs = vec![text_run("plain "), styled_run("loud", bold()), text_run(" plain")];
		let built = build(&[topic(16, 0, "", vec![paragraph(runs)])]);
		let bolds = markers(&built, MarkerType::Bold);
		assert_eq!(bolds.len(), 1);
		assert_eq!(bolds[0].position, 6);
		assert_eq!(bolds[0].length, 4);
	}

	// Runs split wherever a formatting command falls, so a sentence often arrives as
	// several runs that look identical. One marker per run would be noise.
	#[test]
	fn neighbouring_runs_sharing_emphasis_get_one_marker() {
		let runs = vec![styled_run("one ", bold()), styled_run("two", bold())];
		let built = build(&[topic(16, 0, "", vec![paragraph(runs)])]);
		let bolds = markers(&built, MarkerType::Bold);
		assert_eq!(bolds.len(), 1);
		assert_eq!(bolds[0].length, 7);
	}

	// WinHelp names a link's target by a hash, and the reader navigates by looking a key up
	// in id_positions, so the two have to agree on what that key is.
	#[test]
	fn a_hotspot_references_the_topic_its_hash_resolves_to() {
		let link = Link::jump(Target::Context(0xABC));
		let runs = vec![Run::Text { text: "go".into(), format: Format::default(), link: Some(link) }];
		let topics = vec![
			topic(16, 0, "Start", vec![paragraph(runs)]),
			topic(17, 55, "Target", vec![paragraph(vec![text_run("Target")])]),
		];
		let contexts = HashMap::from([(0xABC, 55)]);
		let built = build_with(&topics, &contexts);
		let links = markers(&built, MarkerType::Link);
		assert_eq!(links.len(), 1);
		assert_eq!(links[0].text, "go");
		assert_eq!(links[0].reference, "topic55");
		// The key the link names has to be the one the target topic registered. The first
		// topic writes its title as well as its body, so the second starts nine
		// characters in.
		assert_eq!(built.id_positions.get("topic55"), Some(&9));
	}

	// A WinHelp 3.0 hotspot names its target by topic number rather than by context hash,
	// and the file has no context index at all for the other path to fall back on.
	#[test]
	fn a_30_hotspot_references_the_topic_number_it_names() {
		let link = Link::jump(Target::Number(17));
		let runs = vec![Run::Text { text: "go".into(), format: Format::default(), link: Some(link) }];
		let topics = vec![
			topic(16, 0, "Start", vec![paragraph(runs)]),
			// A 3.0 file reports zero for every topic offset, so the number is the identity.
			topic(17, 0, "Target", vec![paragraph(vec![text_run("Target")])]),
		];
		let built = build(&topics);
		let links = markers(&built, MarkerType::Link);
		assert_eq!(links[0].reference, "topicnum17");
		assert_eq!(built.id_positions.get("topicnum17"), Some(&9));
	}

	// Every topic in a WinHelp 3.0 file reports offset zero, so keying only by offset would
	// collapse the whole contents onto the first topic.
	#[test]
	fn topics_sharing_an_offset_stay_reachable_by_number() {
		let topics = vec![
			topic(16, 0, "First", vec![paragraph(vec![text_run("First")])]),
			topic(17, 0, "Second", vec![paragraph(vec![text_run("Second")])]),
		];
		let built = build(&topics);
		assert_eq!(built.id_positions.get("topicnum16"), Some(&0));
		assert_eq!(built.id_positions.get("topicnum17"), Some(&6));
		let offsets: Vec<&str> = built.toc_items.iter().map(|i| i.reference.as_str()).collect();
		assert_eq!(offsets, ["topicnum16", "topicnum17"]);
	}

	// A hotspot whose context the file never defines is still a hotspot worth announcing,
	// but activating it must not land somewhere arbitrary.
	#[test]
	fn a_hotspot_with_no_target_gets_no_reference() {
		let link = Link::jump(Target::Context(0x999));
		let runs = vec![Run::Text { text: "dead".into(), format: Format::default(), link: Some(link) }];
		let built = build(&[topic(16, 0, "", vec![paragraph(runs)])]);
		let links = markers(&built, MarkerType::Link);
		assert_eq!(links.len(), 1);
		assert!(links[0].reference.is_empty());
	}

	// A macro hotspot runs a WinHelp macro rather than going anywhere, so it has no topic
	// to reference even though the file resolved it.
	#[test]
	fn a_macro_hotspot_gets_no_reference() {
		let link = Link::jump(Target::Macro("Exit()".into()));
		let runs = vec![Run::Text { text: "quit".into(), format: Format::default(), link: Some(link) }];
		let built = build(&[topic(16, 0, "", vec![paragraph(runs)])]);
		assert!(markers(&built, MarkerType::Link)[0].reference.is_empty());
	}

	#[test]
	fn a_table_lays_out_as_rows_and_is_marked_whole() {
		let cell = |t: &str| Cell { paragraphs: vec![Paragraph { runs: vec![text_run(t)] }] };
		let table = Table {
			rows: vec![
				Row { cells: vec![cell("Name"), cell("Value")] },
				Row { cells: vec![cell("width"), cell("80")] },
			],
		};
		let built = build(&[topic(16, 0, "", vec![Block::Table(table)])]);
		assert_eq!(built.buffer.content, "Name\tValue\nwidth\t80\n");
		let tables = markers(&built, MarkerType::Table);
		assert_eq!(tables.len(), 1);
		assert_eq!(tables[0].position, 0);
		assert_eq!(tables[0].length, built.buffer.content.chars().count());
	}

	#[test]
	fn a_picture_gets_a_placeholder_and_a_marker() {
		let runs = vec![text_run("before "), Run::Image(Some(3)), text_run(" after")];
		let built = build(&[topic(16, 0, "", vec![paragraph(runs)])]);
		assert!(built.buffer.content.contains("before ["));
		let images = markers(&built, MarkerType::Image);
		assert_eq!(images.len(), 1);
		assert_eq!(images[0].position, 7);
	}

	#[test]
	fn tabs_and_line_breaks_reach_the_text() {
		let runs = vec![text_run("a"), Run::Tab, text_run("b"), Run::LineBreak, text_run("c")];
		let built = build(&[topic(16, 0, "", vec![paragraph(runs)])]);
		assert_eq!(built.buffer.content, "a\tb\nc\n");
	}

	#[test]
	fn titled_topics_make_the_table_of_contents() {
		let topics = vec![
			topic(16, 0, "First", vec![paragraph(vec![text_run("First")])]),
			topic(17, 40, "", vec![paragraph(vec![text_run("Untitled.")])]),
			topic(18, 90, "Third", vec![paragraph(vec![text_run("Third")])]),
		];
		let built = build(&topics);
		let names: Vec<&str> = built.toc_items.iter().map(|i| i.name.as_str()).collect();
		assert_eq!(names, ["First", "Third"]);
		// Topic numbers are unique in both format generations, where topic offsets exist
		// only in WinHelp 3.1, so the contents points at the number.
		assert_eq!(built.toc_items[1].reference, "topicnum18");
	}

	#[test]
	fn a_file_with_no_topics_builds_an_empty_document() {
		let built = build(&[]);
		assert!(built.buffer.content.is_empty());
		assert!(built.toc_items.is_empty());
		assert!(built.id_positions.is_empty());
	}
}
