//! Reading-appearance settings: font, background color, text alignment, and the
//! line/paragraph/letter spacing steps shown in the options dialog's readability tab.

use super::ConfigManager;
use crate::config::settings::ReadabilityFont;

impl ConfigManager {
	pub fn get_readability_font(&self) -> ReadabilityFont {
		if !self.initialized {
			return ReadabilityFont::default();
		}
		let data = self.data.borrow();
		ReadabilityFont {
			face_name: data.app.font_face_name.clone(),
			point_size: data.app.font_point_size.try_into().unwrap_or(0),
			style: data.app.font_style.try_into().unwrap_or(0),
			weight: data.app.font_weight.try_into().unwrap_or(0),
			underlined: data.app.font_underlined,
			strikethrough: data.app.font_strikethrough,
			color: data.app.font_color.try_into().unwrap_or(-1),
			encoding: data
				.app
				.extra
				.get("font_encoding")
				.and_then(toml::Value::as_integer)
				.and_then(|i| i32::try_from(i).ok())
				.unwrap_or(0),
		}
	}

	pub fn set_readability_font(&self, font: &ReadabilityFont) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			data.app.font_face_name = font.face_name.clone();
			data.app.font_point_size = i64::from(font.point_size);
			data.app.font_style = i64::from(font.style);
			data.app.font_weight = i64::from(font.weight);
			data.app.font_underlined = font.underlined;
			data.app.font_strikethrough = font.strikethrough;
			data.app.font_color = i64::from(font.color);
			data.app.extra.insert("font_encoding".to_string(), toml::Value::Integer(i64::from(font.encoding)));
		}
		self.dirty.set(true);
	}

	pub fn get_line_spacing(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.line_spacing.try_into().unwrap_or(0)
	}

	pub fn set_line_spacing(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.line_spacing = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_bg_color(&self) -> i32 {
		if !self.initialized {
			return -1;
		}
		self.data.borrow().app.bg_color.try_into().unwrap_or(-1)
	}

	pub fn set_bg_color(&self, color: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.bg_color = i64::from(color);
		self.dirty.set(true);
	}

	pub fn get_text_alignment(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.text_alignment.try_into().unwrap_or(0)
	}

	pub fn set_text_alignment(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.text_alignment = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_letter_spacing(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.letter_spacing.clamp(0, 2).try_into().unwrap_or(0)
	}

	pub fn set_letter_spacing(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.letter_spacing = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_paragraph_spacing(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.paragraph_spacing.clamp(0, 2).try_into().unwrap_or(0)
	}

	pub fn set_paragraph_spacing(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.paragraph_spacing = i64::from(value);
		self.dirty.set(true);
	}
}
