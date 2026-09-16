//! The Options dialog's font and background-color helpers: the descriptions shown next to each
//! setting and the system font picker that changes the readability font.

use std::{fmt::Write, mem};

use paperback_core::config::ReadabilityFont;
use patois::t;
use wxdragon::prelude::*;

pub(super) fn color_description(color: i32) -> String {
	if color < 0 {
		// TRANSLATORS: Description text shown when the background color is set to default
		t("Background: Default")
	} else {
		let r = ((color >> 16) & 0xFF) as u8;
		let g = ((color >> 8) & 0xFF) as u8;
		let b = (color & 0xFF) as u8;
		format!("#{r:02X}{g:02X}{b:02X}")
	}
}

pub(super) fn font_description(rf: &ReadabilityFont) -> String {
	if rf.is_default() {
		// TRANSLATORS: Description text shown when the font is set to default
		return t("Font: Default");
	}
	// TRANSLATORS: Fallback font name
	let face = if rf.face_name.is_empty() { t("Default") } else { rf.face_name.clone() };
	// TRANSLATORS: Font description prefix; {} is the font face name
	let mut desc = t("Font: {}").replace("{}", &face);
	if rf.point_size > 0 {
		// TRANSLATORS: Point size attribute; {} is the numeric size, "pt" is the unit abbreviation
		let size_desc = t("{}pt").replace("{}", &rf.point_size.to_string());
		let _ = write!(desc, ", {size_desc}");
	}
	if rf.weight >= FontWeight::Bold as i32 {
		// TRANSLATORS: Font weight attribute name
		let _ = write!(desc, ", {}", t("Bold"));
	}
	if rf.style == FontStyle::Italic as i32 || rf.style == FontStyle::Slant as i32 {
		// TRANSLATORS: Font style attribute name
		let _ = write!(desc, ", {}", t("Italic"));
	}
	if rf.underlined {
		// TRANSLATORS: Font underline attribute name
		let _ = write!(desc, ", {}", t("Underlined"));
	}
	if rf.strikethrough {
		// TRANSLATORS: Font strikethrough attribute name
		let _ = write!(desc, ", {}", t("Strikethrough"));
	}
	desc
}

pub(super) fn show_font_picker(parent: Dialog, current: &ReadabilityFont) -> Option<ReadabilityFont> {
	let mut font_data = FontData::new();
	if current.color >= 0 {
		let r = ((current.color >> 16) & 0xFF) as u8;
		let g = ((current.color >> 8) & 0xFF) as u8;
		let b = (current.color & 0xFF) as u8;
		font_data.set_colour(&Colour::rgb(r, g, b));
	}
	if !current.is_default() {
		let style = match current.style {
			s if s == FontStyle::Italic as i32 => FontStyle::Italic,
			s if s == FontStyle::Slant as i32 => FontStyle::Slant,
			_ => FontStyle::Normal,
		};
		let weight = match current.weight {
			w if w == FontWeight::Bold as i32 => FontWeight::Bold,
			w if w == FontWeight::Light as i32 => FontWeight::Light,
			w if w == FontWeight::ExtraBold as i32 => FontWeight::ExtraBold,
			_ => FontWeight::Normal,
		};
		let point_size = if current.point_size > 0 { current.point_size } else { 10 };
		if let Some(mut font) = Font::builder()
			.with_face_name(&current.face_name)
			.with_point_size(point_size)
			.with_style(style)
			.with_weight(weight)
			.with_underline(current.underlined)
			.with_strikethrough(current.strikethrough)
			.build()
		{
			if current.encoding != 0 {
				font.set_encoding(current.encoding);
			}
			font_data.set_initial_font(&font);
		}
	}
	let dlg = FontDialog::builder(&parent)
		// TRANSLATORS: Title of the system dialog for picking a font
		.with_title(&t("Choose a font"))
		.with_font_data(&font_data)
		.build();
	if dlg.show_modal() != ID_OK {
		return None;
	}
	let font = dlg.get_font()?;
	let chosen_color = dlg.get_font_data().map_or(-1, |fd| {
		let c = fd.get_chosen_colour();
		// Prevent double-free: this FontData pointer is owned by the dialog, not by us
		mem::forget(fd);
		c.map_or(-1, |col| (i32::from(col.r) << 16) | (i32::from(col.g) << 8) | i32::from(col.b))
	});
	Some(ReadabilityFont {
		face_name: font.get_face_name(),
		point_size: font.get_point_size(),
		style: font.get_style() as i32,
		weight: font.get_weight() as i32,
		underlined: font.is_underlined(),
		strikethrough: font.is_strikethrough(),
		color: chosen_color,
		encoding: font.get_encoding(),
	})
}
