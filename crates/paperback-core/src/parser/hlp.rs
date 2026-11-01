//! WinHelp stores a hotspot's destination as the hash of the target's context string, never an offset, so links resolve through the file's `|CONTEXT` index into [`Document::id_positions`] keys.

use std::{collections::HashMap, fs};

use anyhow::{Context, Result};
use libhlp::{Block, Format, HlpFile, Paragraph, Run, Table, Target, Topic};

use crate::{
	document::{
		DocLen, DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem, format_marker_types,
	},
	parser::{Parser, util::path::extract_title_from_path},
	t,
};

/// The prefix stops a bare topic number reading as a fragment when the reader resolves a link.
const OFFSET_KEY: &str = "topic";

/// Topic numbers are distinct in both format generations while offsets exist only from WinHelp 3.1, so the table of contents points at these.
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

fn failed(path: &str, error: &libhlp::HlpError) -> anyhow::Error {
	tracing::warn!(path = %path, error = %error, "failed to read winhelp file");
	// TRANSLATORS: Error shown when a WinHelp (.hlp) file cannot be read; {} is the underlying error
	anyhow::anyhow!(t("Failed to parse WinHelp file: {}").replace("{}", &error.to_string()))
}

struct Built {
	buffer: DocumentBuffer,
	id_positions: HashMap<String, DocPos>,
	toc_items: Vec<TocItem>,
}

struct Builder<'a> {
	/// Taking the `|CONTEXT` map rather than the file keeps the walk testable without one.
	contexts: &'a HashMap<u32, u32>,
	buffer: DocumentBuffer,
	starts: Vec<DocPos>,
}

impl<'a> Builder<'a> {
	const fn new(contexts: &'a HashMap<u32, u32>) -> Self {
		Self { contexts, buffer: DocumentBuffer::new(), starts: Vec::new() }
	}

	fn add_topic(&mut self, topic: &Topic) {
		let start = self.buffer.end();
		self.starts.push(start);
		if !topic.title.is_empty() {
			// A WinHelp topic almost always repeats its title as its first line, and adding it again would be announced twice.
			if !starts_with_title(topic) {
				self.buffer.append(&topic.title);
				self.buffer.append("\n");
			}
			self.buffer.add_marker(
				Marker::new(MarkerType::Heading1, start)
					.with_text(topic.title.clone())
					.with_length(DocLen::of(&topic.title)),
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

	fn add_table(&mut self, table: &Table) {
		let start = self.buffer.end();
		for row in &table.rows {
			for (index, cell) in row.cells.iter().enumerate() {
				if index > 0 {
					self.buffer.append("\t");
				}
				// A cell's paragraphs join with a space so the row stays on one line.
				for (index, paragraph) in cell.paragraphs.iter().enumerate() {
					if index > 0 {
						self.buffer.append(" ");
					}
					self.add_paragraph(paragraph);
				}
			}
			self.buffer.append("\n");
		}
		let length = self.buffer.end().since(start);
		if !length.is_zero() {
			self.buffer.add_marker(Marker::new(MarkerType::Table, start).with_length(length));
		}
	}

	fn add_paragraph(&mut self, paragraph: &Paragraph) {
		// Runs split at every formatting command, so neighbours sharing emphasis are merged into one marker per visible span.
		let mut span: Option<(Format, DocPos)> = None;
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
				span = Some((format, self.buffer.end()));
			}
			self.add_run(run, &mut links);
		}
		if let Some((open, start)) = span {
			self.close_span(open, start);
		}
		// Emphasis markers are only added when their span closes; adding links first would have exporters interleave the two rather than nest them.
		for marker in links {
			self.buffer.add_marker(marker);
		}
	}

	fn close_span(&mut self, format: Format, start: DocPos) {
		let length = self.buffer.end().since(start);
		if length.is_zero() {
			return;
		}
		for mtype in format_marker_types(format.bold, format.italic, format.underline) {
			self.buffer.add_marker(Marker::new(mtype, start).with_length(length));
		}
	}

	fn add_run(&mut self, run: &Run, links: &mut Vec<Marker>) {
		match run {
			Run::Text { text, link, .. } => {
				let position = self.buffer.end();
				self.buffer.append(text);
				let Some(link) = link else { return };
				// A key rather than a position, so the target topic needn't be read yet; an undefined target leaves it empty, a link to nowhere.
				let reference = match &link.target {
					// WinHelp 3.1 names a target by the hash of its context string.
					Target::Context(hash) => self.contexts.get(hash).copied().map(offset_key),
					// WinHelp 3.0 names it by topic number instead.
					Target::Number(number) => Some(number_key(*number)),
					// A macro runs rather than going anywhere, and another help file has nothing here to land on.
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
				let position = self.buffer.end();
				self.buffer.add_marker(Marker::new(MarkerType::Image, position).with_length(DocLen::of(&line)));
				self.buffer.append(&line);
			}
		}
	}

	fn finish(self, topics: &[Topic]) -> Built {
		let mut id_positions = HashMap::new();
		let mut toc_items = Vec::new();
		for (topic, &start) in topics.iter().zip(&self.starts) {
			id_positions.insert(number_key(topic.number), start);
			// Every WinHelp 3.0 topic reports offset zero; only the first claims the key, and nothing in such a file references it.
			id_positions.entry(offset_key(topic.offset)).or_insert(start);
			if !topic.title.is_empty() {
				toc_items.push(TocItem::new(topic.title.clone(), number_key(topic.number), start));
			}
		}
		Built { buffer: self.buffer, id_positions, toc_items }
	}
}

fn offset_key(offset: u32) -> String {
	format!("{OFFSET_KEY}{offset}")
}

fn number_key(number: u32) -> String {
	format!("{NUMBER_KEY}{number}")
}

fn starts_with_title(topic: &Topic) -> bool {
	topic.blocks.first().is_some_and(|block| block.text().trim_start().starts_with(topic.title.trim()))
}

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

	// An untitled topic still has to be reachable by section navigation.
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

	#[test]
	fn neighbouring_runs_sharing_emphasis_get_one_marker() {
		let runs = vec![styled_run("one ", bold()), styled_run("two", bold())];
		let built = build(&[topic(16, 0, "", vec![paragraph(runs)])]);
		let bolds = markers(&built, MarkerType::Bold);
		assert_eq!(bolds.len(), 1);
		assert_eq!(bolds[0].length, 7);
	}

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
		// The first topic writes its title as well as its body, so the second starts nine characters in.
		assert_eq!(built.id_positions.get("topic55"), Some(&9.into()));
	}

	// A WinHelp 3.0 file has no context index at all for the hash path to fall back on.
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
		assert_eq!(built.id_positions.get("topicnum17"), Some(&9.into()));
	}

	// Every WinHelp 3.0 topic reports offset zero, so keying only by offset would collapse the contents onto the first topic.
	#[test]
	fn topics_sharing_an_offset_stay_reachable_by_number() {
		let topics = vec![
			topic(16, 0, "First", vec![paragraph(vec![text_run("First")])]),
			topic(17, 0, "Second", vec![paragraph(vec![text_run("Second")])]),
		];
		let built = build(&topics);
		assert_eq!(built.id_positions.get("topicnum16"), Some(&0.into()));
		assert_eq!(built.id_positions.get("topicnum17"), Some(&6.into()));
		let offsets: Vec<&str> = built.toc_items.iter().map(|i| i.reference.as_str()).collect();
		assert_eq!(offsets, ["topicnum16", "topicnum17"]);
	}

	// Still worth announcing, but activating it must not land somewhere arbitrary.
	#[test]
	fn a_hotspot_with_no_target_gets_no_reference() {
		let link = Link::jump(Target::Context(0x999));
		let runs = vec![Run::Text { text: "dead".into(), format: Format::default(), link: Some(link) }];
		let built = build(&[topic(16, 0, "", vec![paragraph(runs)])]);
		let links = markers(&built, MarkerType::Link);
		assert_eq!(links.len(), 1);
		assert!(links[0].reference.is_empty());
	}

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
