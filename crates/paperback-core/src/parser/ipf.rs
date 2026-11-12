//! OS/2 help files: books (`.inf`) and application help (`.hlp`), as IBM's IPF compiler writes them.
//!
//! Every panel, hidden ones included, has an entry in the table of contents, and links name their target by that entry's index, so each panel's start is keyed by it in [`Document::id_positions`].

use std::{collections::HashMap, fs};

use anyhow::{Context, Result};
use libinf::{Block, BlockKind, InfFile, Link, Run, Style, TocEntry};

use crate::{
	document::{
		DocLen, DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem, format_marker_types,
	},
	parser::{
		Parser, is_external_url,
		layout::build_toc_tree,
		text::TextParser,
		util::{path::extract_title_from_path, toc::heading_level_to_marker_type},
	},
	t,
};

/// The prefix stops a bare panel number reading as a fragment when the reader resolves a link.
const PANEL_KEY: &str = "panel";

/// A picture's reference: where it starts in the file's image area, which saving or reading it decodes from the file again.
pub(crate) const PICTURE_PREFIX: &str = "ipf-picture:";

pub struct IpfParser;

impl Parser for IpfParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open OS/2 help file '{}'", context.file_path))?;
		// `.inf` is also the extension of Windows setup files, which are plain text.
		if !libinf::is_ipf(&bytes) {
			return TextParser.parse(context);
		}
		parse_ipf(&context.file_path, bytes)
	}
}

/// Also reached from the WinHelp parser, since OS/2 application help shares its `.hlp` extension.
pub(super) fn parse_ipf(path: &str, bytes: Vec<u8>) -> Result<Document> {
	tracing::debug!(path = %path, "parsing os/2 help file");
	let book = InfFile::from_bytes(bytes).map_err(|error| failed(path, &error))?;
	tracing::debug!(path = %path, kind = ?book.kind(), code_page = book.code_page().number(), panels = book.toc().len(), "os/2 help structure read");
	let mut builder = Builder::default();
	for (index, entry) in book.toc().iter().enumerate() {
		match book.panel(index) {
			Ok(panel) => builder.add_panel(index, &Heading::of(entry), &panel.blocks),
			// One damaged panel shouldn't cost the reader the rest of the book.
			Err(error) => {
				tracing::warn!(path = %path, panel = index, error = %error, "failed to read os/2 help panel");
				builder.add_panel(index, &Heading::of(entry), &[]);
			}
		}
	}
	let title = book
		.title()
		.filter(|title| !title.is_empty())
		.map_or_else(|| extract_title_from_path(path), ToString::to_string);
	let mut document = Document::new().with_title(title);
	document.set_buffer(builder.buffer);
	document.id_positions = builder.id_positions;
	document.toc_items = build_toc_tree(builder.toc);
	tracing::debug!(path = %path, "parsed os/2 help file successfully");
	Ok(document)
}

fn failed(path: &str, error: &libinf::InfError) -> anyhow::Error {
	tracing::warn!(path = %path, error = %error, "failed to read os/2 help file");
	// TRANSLATORS: Error shown when an OS/2 help file (.inf or .hlp) cannot be read; {} is the underlying error
	anyhow::anyhow!(t("Failed to parse OS/2 help file: {}").replace("{}", &error.to_string()))
}

fn panel_key(index: usize) -> String {
	format!("{PANEL_KEY}{index}")
}

/// What a panel's table of contents entry says about it, apart from where its text is.
struct Heading<'a> {
	title: &'a str,
	level: u8,
	hidden: bool,
}

impl<'a> Heading<'a> {
	fn of(entry: &'a TocEntry) -> Self {
		Self { title: entry.title.trim(), level: entry.level, hidden: entry.hidden }
	}
}

#[derive(Default)]
struct Builder {
	buffer: DocumentBuffer,
	id_positions: HashMap<String, DocPos>,
	toc: Vec<(u32, TocItem)>,
}

impl Builder {
	fn add_panel(&mut self, index: usize, heading: &Heading<'_>, blocks: &[Block]) {
		let start = self.buffer.end();
		self.id_positions.insert(panel_key(index), start);
		let title = heading.title;
		if !title.is_empty() {
			// VIEW shows a panel's heading in its window's title bar, so the text never repeats it.
			self.buffer.append(title);
			self.buffer.append("\n");
			self.buffer.add_marker(
				Marker::new(heading_level_to_marker_type(i32::from(heading.level)), start)
					.with_text(title.to_string())
					.with_length(DocLen::of(title)),
			);
		}
		self.buffer.add_marker(Marker::new(MarkerType::SectionBreak, start).with_text(title.to_string()));
		// A hidden panel is a pop-up or one only reached through links, which VIEW's contents leave out.
		if !heading.hidden && !title.is_empty() {
			self.toc.push((u32::from(heading.level), TocItem::new(title.to_string(), panel_key(index), start)));
		}
		for block in blocks {
			self.add_block(block);
		}
	}

	fn add_block(&mut self, block: &Block) {
		if block.runs.is_empty() {
			return;
		}
		let start = self.buffer.end();
		// Emphasis spans merge across runs that share a style, and links go in after, so exporters nest the two rather than interleave them.
		let mut span: Option<(Style, DocPos)> = None;
		let mut links = Vec::new();
		for run in &block.runs {
			let style = match run {
				Run::Text { style, .. } => *style,
				_ => Style::default(),
			};
			if span.is_none_or(|(open, _)| open != style) {
				if let Some((open, at)) = span {
					self.close_span(open, at);
				}
				span = Some((style, self.buffer.end()));
			}
			self.add_run(run, &mut links);
		}
		if let Some((open, at)) = span {
			self.close_span(open, at);
		}
		for marker in links {
			self.buffer.add_marker(marker);
		}
		if block.kind != BlockKind::Paragraph {
			let length = self.buffer.end().since(start);
			if !length.is_zero() {
				self.buffer.add_marker(Marker::new(MarkerType::Preformatted, start).with_length(length));
			}
		}
		self.buffer.append("\n");
	}

	fn close_span(&mut self, style: Style, start: DocPos) {
		let length = self.buffer.end().since(start);
		if length.is_zero() {
			return;
		}
		for mtype in format_marker_types(style.bold, style.italic, style.underline) {
			self.buffer.add_marker(Marker::new(mtype, start).with_length(length));
		}
	}

	fn add_run(&mut self, run: &Run, links: &mut Vec<Marker>) {
		match run {
			Run::Text { text, link, .. } => {
				let position = self.buffer.end();
				self.buffer.append(text);
				let Some(link) = link else { return };
				// A message to the application or a panel in another book has nothing here to land on, so it is a link to nowhere, as is a program unless it is given a web address.
				let reference = match link {
					Link::Panel(index) | Link::Footnote(index) => panel_key(*index),
					Link::Launch(command) => web_address(command).unwrap_or_default().to_string(),
					_ => String::new(),
				};
				links.push(Marker::new(MarkerType::Link, position).with_text(text.clone()).with_reference(reference));
			}
			Run::LineBreak => self.buffer.append("\n"),
			Run::Image(image) => {
				// TRANSLATORS: Label standing in for a picture in an OS/2 help panel, shown as "[Image]"
				let line = format!("[{}]", t("Image"));
				let position = self.buffer.end();
				self.buffer.add_marker(
					Marker::new(MarkerType::Image, position)
						.with_length(DocLen::of(&line))
						.with_reference(format!("{PICTURE_PREFIX}{}", image.offset)),
				);
				self.buffer.append(&line);
			}
		}
	}
}

/// Books link to web pages by starting a browser with the address, as in `explore.exe http://www.sgi.com/`.
fn web_address(command: &str) -> Option<&str> {
	command.split_whitespace().find(|word| is_external_url(word))
}

/// The picture at `reference` in the help file at `path`, as a Windows bitmap.
pub(crate) fn picture_bytes(path: &str, reference: &str) -> Result<Vec<u8>> {
	let offset = reference
		.strip_prefix(PICTURE_PREFIX)
		.and_then(|offset| offset.parse().ok())
		.context("malformed OS/2 help picture reference")?;
	let book = InfFile::open(path).map_err(|error| failed(path, &error))?;
	let picture = book.picture(libinf::ImageRef { offset }).map_err(|error| failed(path, &error))?;
	Ok(picture.to_bmp())
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::util::test_support::TempDir;

	fn text(text: &str, link: Option<Link>) -> Run {
		Run::Text { text: text.to_string(), style: Style::default(), link }
	}

	fn paragraph(runs: Vec<Run>) -> Block {
		Block { kind: BlockKind::Paragraph, runs }
	}

	fn built(panels: &[(Heading<'_>, Vec<Block>)]) -> Builder {
		let mut builder = Builder::default();
		for (index, (heading, blocks)) in panels.iter().enumerate() {
			builder.add_panel(index, heading, blocks);
		}
		builder
	}

	fn marker(builder: &Builder, mtype: MarkerType) -> &Marker {
		builder.buffer.markers.iter().find(|marker| marker.mtype == mtype).unwrap()
	}

	#[test]
	fn panels_get_headings_and_hidden_ones_stay_out_of_the_contents() {
		let builder = built(&[
			(
				Heading { title: "Commands", level: 1, hidden: false },
				vec![paragraph(vec![text("Every command.", None)])],
			),
			(Heading { title: "Pop-up", level: 2, hidden: true }, vec![paragraph(vec![text("A footnote.", None)])]),
		]);
		assert_eq!(
			builder.buffer.content,
			"Commands
Every command.
Pop-up
A footnote.
"
		);
		let toc = build_toc_tree(builder.toc);
		assert_eq!(toc.len(), 1);
		assert_eq!(toc[0].reference, "panel0");
		assert_eq!(builder.id_positions["panel1"].get(), 24);
		assert!(
			builder.buffer.markers.iter().any(|marker| marker.mtype == MarkerType::Heading2 && marker.text == "Pop-up")
		);
	}

	#[test]
	fn links_lead_to_panels_and_web_pages_or_nowhere() {
		let runs = vec![
			text("See", Some(Link::Panel(3))),
			text(" the site", Some(Link::Launch("explore.exe http://www.os2world.com/".into()))),
			text(" or the program", Some(Link::Inform(7))),
		];
		let builder = built(&[(Heading { title: "", level: 1, hidden: false }, vec![paragraph(runs)])]);
		let references: Vec<&str> = builder
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::Link)
			.map(|marker| marker.reference.as_str())
			.collect();
		assert_eq!(references, ["panel3", "http://www.os2world.com/", ""]);
	}

	#[test]
	fn examples_are_preformatted_and_pictures_named_by_their_place() {
		let example =
			Block { kind: BlockKind::Example, runs: vec![text("dir /s", None), Run::LineBreak, text("  copy", None)] };
		let picture = paragraph(vec![Run::Image(libinf::ImageRef { offset: 4096 })]);
		let builder = built(&[(Heading { title: "", level: 1, hidden: false }, vec![example, picture])]);
		assert_eq!(
			builder.buffer.content,
			"dir /s
  copy
[Image]
"
		);
		assert_eq!(marker(&builder, MarkerType::Preformatted).length.get(), 13);
		assert_eq!(marker(&builder, MarkerType::Image).reference, "ipf-picture:4096");
	}

	#[test]
	fn a_windows_setup_file_opens_as_text() {
		let dir = TempDir::new("ipf_setup");
		let path = dir.path().join("driver.inf");
		fs::write(
			&path,
			"[Version]
Signature=\"$Windows NT$\"
",
		)
		.unwrap();
		let document = IpfParser.parse(&ParserContext::new(path.to_string_lossy().into_owned())).unwrap();
		assert_eq!(
			document.buffer.content,
			"[Version]
Signature=\"$Windows NT$\"
"
		);
	}

	#[rstest]
	#[case("explore.exe  http://www.sgi.com/", Some("http://www.sgi.com/"))]
	#[case("nEtScApE.eXe mailto:cla@langanke.de", Some("mailto:cla@langanke.de"))]
	#[case("view.exe cmdref ANSI", None)]
	#[case("netscape.exe ftp://ftp.netlabs.org/pub", None)]
	fn a_launched_program_leads_to_its_web_address(#[case] command: &str, #[case] expected: Option<&str>) {
		assert_eq!(web_address(command), expected);
	}
}
