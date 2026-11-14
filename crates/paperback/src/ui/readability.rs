//! The font, colours and line spacing the reader applies, and their stored forms.

use paperback_core::config::{ConfigManager, ReadabilityFont};
use wxdragon::prelude::*;

pub struct ReadabilityStyle {
	pub rf: ReadabilityFont,
	pub line_spacing: i32,
	pub bg_color: i32,
}

pub fn readability_style(cfg: &ConfigManager) -> ReadabilityStyle {
	ReadabilityStyle {
		rf: cfg.get_readability_font(),
		line_spacing: cfg.get_line_spacing(),
		bg_color: cfg.get_bg_color(),
	}
}

pub fn build_font_from_readability(rf: &ReadabilityFont) -> Option<Font> {
	if rf.is_default() {
		return None;
	}
	let point_size = if rf.point_size > 0 { rf.point_size } else { 10 };
	let mut font = Font::new_with_details(
		point_size,
		FontFamily::Default.as_i32(),
		rf.style,
		rf.weight,
		rf.underlined,
		&rf.face_name,
	)?;
	if rf.strikethrough {
		font.set_strikethrough(true);
	}
	if rf.encoding != 0 {
		font.set_encoding(rf.encoding);
	}
	Some(font)
}

/// The default colour is stored as a negative number.
pub fn stored_colour(color: i32) -> Option<Colour> {
	let [_, r, g, b] = color.to_be_bytes();
	(color >= 0).then(|| Colour::rgb(r, g, b))
}
