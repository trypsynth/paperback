//! A tagged page that tags none of its images still gets them, placed by height through [`UnclaimedImages`], since trusting its tree would lose them all: the menu in #857 draws five and declares no figure.

use crate::{
	document::{DocLen, DocPos, DocumentBuffer, Marker, MarkerType},
	pdf_engine::{MAX_FORM_DEPTH, ObjectKind, PageObject, PdfPage},
	t,
};

/// An undescribed image still gets a line: a reader who cannot see the page has no other way of knowing one is there.
pub(super) fn append_image(
	buffer: &mut DocumentBuffer,
	mtype: MarkerType,
	description: &str,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(DocPos, String)>,
	reference: &str,
) {
	// TRANSLATORS: Label inserted before a figure or image's description, e.g. "[Figure: a cat sleeping]"
	let label = if mtype == MarkerType::Figure { t("Figure") } else { t("Image") };
	let line = if description.is_empty() { format!("[{label}]") } else { format!("[{label}: {description}]") };
	let offset = buffer.end();
	current_lines_info.push((offset, line.clone()));
	buffer.add_marker(
		Marker::new(mtype, offset)
			.with_text(description.to_string())
			.with_length(DocLen::of(&line))
			.with_reference(reference.to_string()),
	);
	buffer.append(&line);
	buffer.append("\n");
	// Kept in step with the buffer because `place_links` measures its offsets against this, and a line written to one and not the other moves every link on the page.
	page_display_text.push_str(&line);
	page_display_text.push('\n');
}

/// The largest single image, not the images added together: a page tiled with small decorations is not a scan.
pub(super) fn page_largest_image_coverage(page: &PdfPage) -> f64 {
	let page_area = f64::from(page.width()) * f64::from(page.height());
	if page_area <= 0.0 {
		return 0.0;
	}
	let mut largest = 0.0_f64;
	for i in 0..page.object_count() {
		if let Some(object) = page.object(i) {
			collect_largest_image_area(&object, 0, &mut largest);
		}
	}
	(largest / page_area).min(1.0)
}

fn collect_largest_image_area(object: &PageObject, depth: u32, largest: &mut f64) {
	match object.kind() {
		ObjectKind::Image => {
			if let Some(bounds) = object.bounds() {
				let area =
					f64::from((bounds.right - bounds.left).abs()) * f64::from((bounds.top - bounds.bottom).abs());
				*largest = largest.max(area);
			}
		}
		ObjectKind::Form if depth < MAX_FORM_DEPTH => {
			for i in 0..object.form_object_count() {
				if let Some(child) = object.form_object(i) {
					collect_largest_image_area(&child, depth + 1, largest);
				}
			}
		}
		_ => {}
	}
}

pub(super) struct UnclaimedImages {
	images: std::collections::VecDeque<(f64, String)>,
	page_index: i32,
}

impl UnclaimedImages {
	pub(super) fn new(tops: &[f64], paths: &[String], page_index: i32) -> Self {
		Self { images: tops.iter().copied().zip(paths.iter().cloned()).collect(), page_index }
	}

	pub(super) fn is_empty(&self) -> bool {
		self.images.is_empty()
	}

	pub(super) fn place_above(
		&mut self,
		top: f64,
		buffer: &mut DocumentBuffer,
		page_display_text: &mut String,
		current_lines_info: &mut Vec<(DocPos, String)>,
	) {
		while self.images.front().is_some_and(|(image_top, _)| *image_top > top) {
			if let Some((_, path)) = self.images.pop_front() {
				let reference = format!("pdf:{}:{path}", self.page_index);
				append_image(buffer, MarkerType::Image, "", page_display_text, current_lines_info, &reference);
			}
		}
	}
}

/// One count per paragraph plus a last one for images below them all; both inputs must run down the page from the same edge.
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
	use super::{UnclaimedImages, append_image, images_before_each_paragraph};
	use crate::document::{DocumentBuffer, MarkerType};

	#[test]
	fn a_described_figure_is_written_as_a_line_of_its_own() {
		let mut buffer = DocumentBuffer::new();
		let mut page_text = String::new();
		let mut lines_info = Vec::new();
		append_image(&mut buffer, MarkerType::Figure, "a cat sleeping", &mut page_text, &mut lines_info, "pdf:2:0/1");
		assert_eq!(buffer.content, "[Figure: a cat sleeping]\n");
		assert_eq!(page_text, buffer.content, "the display text has to mirror the buffer for link placement");
		assert_eq!(lines_info.len(), 1);
		let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Figure).expect("Figure marker");
		assert_eq!(marker.position, 0);
		assert_eq!(marker.text, "a cat sleeping", "the marker keeps the description on its own");
		assert_eq!(marker.length, "[Figure: a cat sleeping]".len());
		assert_eq!(marker.reference, "pdf:2:0/1", "the marker names where the picture sits so it can be saved");
	}

	#[test]
	fn an_undescribed_image_still_gets_a_line() {
		let mut buffer = DocumentBuffer::new();
		let mut page_text = String::new();
		let mut lines_info = Vec::new();
		append_image(&mut buffer, MarkerType::Image, "", &mut page_text, &mut lines_info, "");
		assert_eq!(buffer.content, "[Image]\n");
		assert!(buffer.markers.iter().any(|m| m.mtype == MarkerType::Image));
	}

	/// Y grows up the page.
	#[test]
	fn images_land_between_the_paragraphs_they_were_drawn_between() {
		assert_eq!(images_before_each_paragraph(&[600.0, 400.0], &[700.0, 500.0, 300.0]), vec![0, 1, 1, 0]);
	}

	#[test]
	fn images_outside_the_text_go_at_the_ends() {
		assert_eq!(images_before_each_paragraph(&[800.0, 100.0], &[700.0, 500.0]), vec![1, 0, 1]);
	}

	#[test]
	fn no_image_is_dropped() {
		let counts = images_before_each_paragraph(&[200.0, 150.0, 100.0], &[700.0]);
		assert_eq!(counts.iter().sum::<usize>(), 3);
	}

	#[test]
	fn a_page_with_no_paragraphs_holds_every_image_back() {
		assert_eq!(images_before_each_paragraph(&[500.0], &[]), vec![1]);
	}

	#[test]
	fn unclaimed_images_go_in_front_of_the_first_block_below_each() {
		let mut images =
			UnclaimedImages::new(&[700.0, 500.0, 100.0], &["0".to_string(), "1".to_string(), "2".to_string()], 0);
		let mut buffer = DocumentBuffer::new();
		let mut page_text = String::new();
		let mut lines_info = Vec::new();
		let mut place = |top: f64, images: &mut UnclaimedImages, buffer: &mut DocumentBuffer| {
			images.place_above(top, buffer, &mut page_text, &mut lines_info);
		};
		place(600.0, &mut images, &mut buffer);
		assert_eq!(buffer.content.matches("[Image]").count(), 1);
		place(300.0, &mut images, &mut buffer);
		assert_eq!(buffer.content.matches("[Image]").count(), 2);
		assert!(!images.is_empty(), "the one at 100 is below both blocks");
		place(f64::NEG_INFINITY, &mut images, &mut buffer);
		assert_eq!(buffer.content.matches("[Image]").count(), 3);
		assert!(images.is_empty());
		assert_eq!(lines_info.len(), 3, "one line per image");
		assert_eq!(buffer.markers.iter().filter(|m| m.mtype == MarkerType::Image).count(), 3);
	}

	#[test]
	fn a_page_whose_tree_claims_its_figures_places_nothing() {
		let mut images = UnclaimedImages::new(&[], &[], 0);
		let mut buffer = DocumentBuffer::new();
		let mut page_text = String::new();
		let mut lines_info = Vec::new();
		assert!(images.is_empty());
		images.place_above(f64::NEG_INFINITY, &mut buffer, &mut page_text, &mut lines_info);
		assert_eq!(buffer.content, "");
		assert!(lines_info.is_empty());
	}
}
