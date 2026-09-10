//! The images a PDF page draws: finding them, and writing them into the document.
//!
//! A tagged PDF says both where its images are and what they show. Its structure tree carries a
//! `/Figure` element in reading order, usually with an `/Alt` description written for exactly
//! this purpose, so [`super::structure`] places one by walking the tree and calls
//! [`append_image`] as it meets one.
//!
//! An untagged PDF says neither. All it has is image objects with page coordinates, so
//! [`page_image_tops`] collects those and the caller places each one among the page's text lines
//! by how far down the page it sits.

use std::ffi::c_ulong;

use pdfium::{
	PdfiumPage, PdfiumPageObject, lib,
	pdfium_constants::{FPDF_PAGEOBJ_FORM, FPDF_PAGEOBJ_IMAGE},
};

use crate::{
	document::{DocumentBuffer, Marker, MarkerType},
	t,
	util::text::display_len,
};

/// How far to follow form `XObject`s into each other looking for the images they draw. A form
/// holding a form is ordinary enough (a placed logo, an annotation's appearance stream), while a
/// chain of them this deep is a malformed file rather than a page anyone meant to draw.
const MAX_FORM_DEPTH: u32 = 8;

/// Shortest side, in PDF user units, an image must have to be worth a line of its own. A page
/// laid out from HTML draws its spacers and its hairline rules as images a fraction of a point
/// across; announcing one of those as an image gives the reader a stop with nothing at it.
const MIN_IMAGE_SIDE: f32 = 4.0;

/// Writes one image's line into the page, with a marker spanning it.
///
/// `description` is the image's alt text, and is empty when it has none. An undescribed image
/// still gets a line: a reader who cannot see the page has no other way of knowing that one is
/// there, and the line is what image navigation and the elements list stop on.
pub(super) fn append_image(
	buffer: &mut DocumentBuffer,
	mtype: MarkerType,
	description: &str,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
) {
	// TRANSLATORS: Label inserted before a figure or image's description, e.g. "[Figure: a cat sleeping]"
	let label = if mtype == MarkerType::Figure { t("Figure") } else { t("Image") };
	let line = if description.is_empty() { format!("[{label}]") } else { format!("[{label}: {description}]") };
	let offset = buffer.current_position();
	current_lines_info.push((offset, line.clone()));
	buffer.add_marker(Marker::new(mtype, offset).with_text(description.to_string()).with_length(display_len(&line)));
	buffer.append(&line);
	buffer.append("\n");
	// Kept in step with the buffer because `place_links` measures its offsets against this, and
	// a line written to one and not the other moves every link on the page.
	page_display_text.push_str(&line);
	page_display_text.push('\n');
}

/// The top edge, in PDF user units, of every image the page draws, ordered down the page.
///
/// Images reached through a form `XObject` count. A page whose content is wrapped in one form (a
/// scanner's output and a placed-artwork export both look like this) draws no image object of its
/// own, so stopping at the top level would find nothing on it at all.
pub(super) fn page_image_tops(page: &PdfiumPage) -> Vec<f64> {
	let mut tops = Vec::new();
	let object_count = lib().FPDFPage_CountObjects(page);
	for i in 0..object_count {
		// Bound before the `if let` so that the library handle `lib()` returns is dropped with
		// this statement rather than held for the body.
		let object = lib().FPDFPage_GetObject(page, i);
		if let Ok(object) = object {
			collect_image_tops(&object, 0, &mut tops);
		}
	}
	// Y grows up the page, so the image nearest the top of it has the largest one.
	tops.sort_unstable_by(|a, b| b.total_cmp(a));
	tops
}

fn collect_image_tops(object: &PdfiumPageObject, depth: u32, tops: &mut Vec<f64>) {
	let object_type = lib().FPDFPageObj_GetType(object);
	match object_type {
		FPDF_PAGEOBJ_IMAGE => {
			let (mut left, mut bottom, mut right, mut top) = (0.0, 0.0, 0.0, 0.0);
			if lib().FPDFPageObj_GetBounds(object, &mut left, &mut bottom, &mut right, &mut top).is_ok()
				&& (right - left).abs() >= MIN_IMAGE_SIDE
				&& (top - bottom).abs() >= MIN_IMAGE_SIDE
			{
				tops.push(f64::from(top));
			}
		}
		FPDF_PAGEOBJ_FORM if depth < MAX_FORM_DEPTH => {
			let child_count = u32::try_from(lib().FPDFFormObj_CountObjects(object)).unwrap_or(0);
			for i in 0..child_count {
				let child = lib().FPDFFormObj_GetObject(object, c_ulong::from(i));
				if let Ok(child) = child {
					collect_image_tops(&child, depth + 1, tops);
				}
			}
		}
		_ => {}
	}
}

/// Writes `count` undescribed images into the page, one line each.
pub(super) fn append_images(
	count: usize,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
) {
	for _ in 0..count {
		append_image(buffer, MarkerType::Image, "", page_display_text, current_lines_info);
	}
}

/// How many of `image_tops` belong in front of each paragraph on an untagged page, with one
/// last count for the images that sit below every paragraph. Both arguments run down the page,
/// and both are measured from the same page edge, so an image belongs in front of the first
/// paragraph that starts below it.
///
/// This is all an untagged page has to go on. Its text arrives as lines with coordinates and its
/// images as objects with coordinates, and nothing in it says which of the two the reader was
/// meant to meet first.
pub(super) fn images_before_each_paragraph(image_tops: &[f64], paragraph_tops: &[f64]) -> Vec<usize> {
	let mut counts = vec![0; paragraph_tops.len() + 1];
	let mut image = 0;
	for (index, paragraph_top) in paragraph_tops.iter().enumerate() {
		while image < image_tops.len() && image_tops[image] > *paragraph_top {
			counts[index] += 1;
			image += 1;
		}
	}
	counts[paragraph_tops.len()] = image_tops.len() - image;
	counts
}

#[cfg(test)]
mod tests {
	use super::{append_image, images_before_each_paragraph};
	use crate::document::{DocumentBuffer, MarkerType};

	/// A described figure reads as its description, and the marker spans the whole line so that
	/// arrowing onto any part of it lands on the figure.
	#[test]
	fn a_described_figure_is_written_as_a_line_of_its_own() {
		let mut buffer = DocumentBuffer::new();
		let mut page_text = String::new();
		let mut lines_info = Vec::new();
		append_image(&mut buffer, MarkerType::Figure, "a cat sleeping", &mut page_text, &mut lines_info);
		assert_eq!(buffer.content, "[Figure: a cat sleeping]\n");
		assert_eq!(page_text, buffer.content, "the display text has to mirror the buffer for link placement");
		assert_eq!(lines_info.len(), 1);
		let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Figure).expect("Figure marker");
		assert_eq!(marker.position, 0);
		assert_eq!(marker.text, "a cat sleeping", "the marker keeps the description on its own");
		assert_eq!(marker.length, "[Figure: a cat sleeping]".len());
	}

	/// An image nobody described still gets a line: it is the only sign a reader who cannot see
	/// the page has that one is there.
	#[test]
	fn an_undescribed_image_still_gets_a_line() {
		let mut buffer = DocumentBuffer::new();
		let mut page_text = String::new();
		let mut lines_info = Vec::new();
		append_image(&mut buffer, MarkerType::Image, "", &mut page_text, &mut lines_info);
		assert_eq!(buffer.content, "[Image]\n");
		assert!(buffer.markers.iter().any(|m| m.mtype == MarkerType::Image));
	}

	/// Y grows up the page, so an image goes in front of the first paragraph set below it.
	#[test]
	fn images_land_between_the_paragraphs_they_were_drawn_between() {
		// Paragraphs at 700, 500 and 300; images at 600 (between the first two) and 400.
		assert_eq!(images_before_each_paragraph(&[600.0, 400.0], &[700.0, 500.0, 300.0]), vec![0, 1, 1, 0]);
	}

	/// An image above everything on the page opens it, and one below everything closes it.
	#[test]
	fn images_outside_the_text_go_at_the_ends() {
		assert_eq!(images_before_each_paragraph(&[800.0, 100.0], &[700.0, 500.0]), vec![1, 0, 1]);
	}

	/// A page whose images all sit below its text still writes every one of them out.
	#[test]
	fn no_image_is_dropped() {
		let counts = images_before_each_paragraph(&[200.0, 150.0, 100.0], &[700.0]);
		assert_eq!(counts.iter().sum::<usize>(), 3);
	}

	/// A page with no text places nothing, and the caller leaves it to the image-only placeholder.
	#[test]
	fn a_page_with_no_paragraphs_holds_every_image_back() {
		assert_eq!(images_before_each_paragraph(&[500.0], &[]), vec![1]);
	}
}
