//! A whole parsed document back-translated from braille to print, such as an eBraille book read by the EPUB parser. Each line is translated on its own, and everything that points into the text, markers, the table of contents and anchors, is moved to where its braille's print now begins.

use louis::Translator;
use rayon::prelude::*;

use super::{BLANK, is_cell, translate_paragraph};
use crate::{
	document::{DocPos, Document, DocumentBuffer, MarkerType, TocItem},
	util::html,
};

pub(crate) fn back_translate_document(document: &mut Document, translator: &Translator) {
	let lines: Vec<&str> = document.buffer.content.split('\n').collect();
	let translated: Vec<(String, Vec<usize>)> = lines.par_iter().map(|line| translate_line(translator, line)).collect();
	let mut content = String::with_capacity(document.buffer.content.len());
	let (mut old_starts, mut new_starts) = (Vec::with_capacity(lines.len()), Vec::with_capacity(lines.len()));
	let (mut old_at, mut new_at) = (0, 0);
	for (index, (line, (text, _))) in lines.iter().zip(&translated).enumerate() {
		if index > 0 {
			content.push('\n');
			old_at += 1;
			new_at += 1;
		}
		old_starts.push(old_at);
		new_starts.push(new_at);
		content.push_str(text);
		old_at += line.chars().count();
		new_at += text.chars().count();
	}
	let map = |position: usize| {
		let line = old_starts.partition_point(|start| *start <= position).saturating_sub(1);
		let (text, positions) = &translated[line];
		new_starts[line] + positions.get(position - old_starts[line]).copied().unwrap_or_else(|| text.chars().count())
	};
	// The translator's positions are counts of characters, as positions in the buffer are.
	let moved = |position: DocPos| DocPos::from_raw(map(position.get()));
	let mut markers = std::mem::take(&mut document.buffer.markers);
	for marker in &mut markers {
		let start = marker.position;
		let new_start = moved(start);
		marker.length = moved(start + marker.length).since(new_start);
		marker.prefix_length = moved(start + marker.prefix_length).since(new_start);
		marker.position = new_start;
		marker.text = translate_text(translator, &marker.text);
		// A table's HTML is what the table view, copying a table and HTML export show.
		if marker.mtype == MarkerType::Table {
			marker.reference = translate_markup(translator, &marker.reference);
		}
	}
	document.buffer = DocumentBuffer::with_content(content);
	document.buffer.markers = markers;
	for position in document.id_positions.values_mut() {
		*position = moved(*position);
	}
	move_toc(&mut document.toc_items, translator, &moved);
}

/// Back-translates braille, such as a heading's text or a page's number, and leaves alone what holds none.
pub(crate) fn translate_text(translator: &Translator, text: &str) -> String {
	if text.chars().any(is_cell) { translate_line(translator, text).0.trim().to_owned() } else { text.to_owned() }
}

fn move_toc(items: &mut [TocItem], translator: &Translator, moved: &impl Fn(DocPos) -> DocPos) {
	for item in items {
		item.offset = moved(item.offset);
		item.name = translate_text(translator, &item.name);
		move_toc(&mut item.children, translator, moved);
	}
}

/// A stretch of a line, with its print and where each of its characters went if it was braille.
type Run = (Vec<char>, Option<(String, Vec<usize>)>);

fn translate_line(translator: &Translator, line: &str) -> (String, Vec<usize>) {
	let mut output = String::with_capacity(line.len());
	let mut positions = Vec::with_capacity(line.len());
	let mut written = 0;
	for (run, translation) in translate_runs(translator, line) {
		if let Some((text, run_positions)) = translation {
			positions.extend(run_positions.iter().map(|position| written + position));
			written += text.chars().count();
			output.push_str(&text);
		} else {
			positions.extend(written..written + run.len());
			written += run.len();
			output.extend(run);
		}
	}
	(output, positions)
}

/// The print is escaped, or a "<" or a quote the braille stands for would break the HTML around it.
fn translate_markup(translator: &Translator, markup: &str) -> String {
	translate_runs(translator, markup)
		.into_iter()
		.map(|(run, translation)| match translation {
			Some((text, _)) => html::escape(&text).replace('"', "&quot;"),
			None => run.into_iter().collect(),
		})
		.collect()
}

/// Only the braille in a line is translated, so print beside it passes through as it is, such as the "[Figure: ...]" the EPUB converter writes around a picture's braille description, or a table's tags. Spaces between braille words are blank cells to a table.
fn translate_runs(translator: &Translator, line: &str) -> Vec<Run> {
	let characters: Vec<char> = line.chars().collect();
	let braille = |character: char| is_cell(character) || character == ' ';
	let mut runs = Vec::new();
	let mut start = 0;
	while start < characters.len() {
		let in_braille = braille(characters[start]);
		let end = characters[start..]
			.iter()
			.position(|&character| braille(character) != in_braille)
			.map_or(characters.len(), |length| start + length);
		let run = characters[start..end].to_vec();
		let translation = (in_braille && run.iter().copied().any(is_cell)).then(|| {
			let cells: String = run.iter().map(|&character| if character == ' ' { BLANK } else { character }).collect();
			translate_paragraph(translator, &cells)
		});
		runs.push((run, translation));
		start = end;
	}
	runs
}
