use std::{rc::Rc, sync::Mutex};

use wxdragon::prelude::*;

use super::DocumentManager;
use crate::{
	text_window::TextWindow,
	ui::{
		readability::{
			apply_bg_color_to_ctrl, apply_foreground_color_to_ctrl, apply_letter_spacing_to_ctrl,
			apply_line_spacing_to_ctrl, apply_paragraph_spacing_to_ctrl, apply_text_alignment_to_ctrl,
			readability_style,
		},
		reader_input,
		text_render::style_and_fill,
	},
};

impl DocumentManager {
	pub fn apply_font(&self, font: &Font) {
		for tab in &self.tabs {
			tab.text_ctrl.set_font(font);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_color(&self, color: i32) {
		for tab in &self.tabs {
			apply_foreground_color_to_ctrl(tab.text_ctrl.widget(), color);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_bg_color(&self, color: i32) {
		for tab in &self.tabs {
			apply_bg_color_to_ctrl(tab.text_ctrl.widget(), color);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_text_alignment(&self, alignment: i32) {
		for tab in &self.tabs {
			apply_text_alignment_to_ctrl(tab.text_ctrl.widget(), alignment);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_letter_spacing(&self, spacing: i32) {
		for tab in &self.tabs {
			apply_letter_spacing_to_ctrl(tab.text_ctrl.widget(), spacing);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_paragraph_spacing(&self, spacing: i32) {
		for tab in &self.tabs {
			apply_paragraph_spacing_to_ctrl(tab.text_ctrl.widget(), spacing);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_line_spacing(&self, line_spacing: i32) {
		for tab in &self.tabs {
			apply_line_spacing_to_ctrl(tab.text_ctrl.widget(), line_spacing);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_word_wrap(&mut self, self_rc: &Rc<Mutex<Self>>, word_wrap: bool) {
		let style = readability_style(&self.config.lock().unwrap());
		for tab in &mut self.tabs {
			let old_ctrl = tab.text_ctrl;
			let current_doc_pos = tab.window.to_doc(old_ctrl.get_insertion_point());
			// A wrap toggle changes layout, not the loaded range, so only the current window is re-sliced.
			let slice = tab.session.get_window(tab.window.start(), tab.window.end());
			let text_ctrl =
				reader_input::build_text_ctrl(tab.panel, word_wrap, self_rc, self.frame, self.from_keyboard.clone());
			let sizer = BoxSizer::builder(Orientation::Vertical).build();
			sizer.add(&text_ctrl.widget(), 1, SizerFlag::Expand | SizerFlag::All, 0);
			tab.panel.set_sizer(sizer, true);
			tab.window = TextWindow::of(&slice);
			style_and_fill(text_ctrl, &slice, &tab.window, &style);
			let max_pos = text_ctrl.get_last_position();
			let pos = tab.window.to_local(current_doc_pos).min(max_pos);
			tab.panel.layout();
			text_ctrl.set_insertion_point(pos);
			text_ctrl.show_position(pos);
			old_ctrl.destroy();
			tab.text_ctrl = text_ctrl;
		}
	}
}
