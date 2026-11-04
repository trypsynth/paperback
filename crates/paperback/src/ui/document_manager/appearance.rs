use std::{rc::Rc, sync::Mutex};

use wxdragon::prelude::*;

use super::DocumentManager;
use crate::{
	text_window::TextWindow,
	ui::{readability::readability_style, reader_input, text_render::style_and_fill},
};

impl DocumentManager {
	pub fn apply_font(&self, font: &Font) {
		for tab in &self.tabs {
			tab.text_ctrl.set_font(font);
			tab.text_ctrl.refresh();
		}
	}

	pub fn apply_color(&self, color: i32) {
		for tab in &self.tabs {
			tab.text_ctrl.apply_foreground_color(color);
			tab.text_ctrl.refresh();
		}
	}

	pub fn apply_bg_color(&self, color: i32) {
		for tab in &self.tabs {
			tab.text_ctrl.apply_background_color(color);
			tab.text_ctrl.refresh();
		}
	}

	pub fn apply_line_spacing(&self, line_spacing: i32) {
		for tab in &self.tabs {
			tab.text_ctrl.apply_line_spacing(line_spacing);
			tab.text_ctrl.refresh();
		}
	}

	pub fn apply_word_wrap(&mut self, self_rc: &Rc<Mutex<Self>>, word_wrap: bool) {
		let style = readability_style(&self.config.lock().unwrap());
		for tab in &mut self.tabs {
			let old_ctrl = tab.text_ctrl;
			let current_doc_pos = tab.window.to_doc(old_ctrl.get_insertion_point());
			// A wrap toggle changes layout, not the document, so the same whole text is loaded again.
			let doc_len = tab.session.document_len();
			let slice = tab.session.get_window(paperback_core::document::DocPos::ZERO, doc_len.as_pos());
			let text_ctrl =
				reader_input::build_text_ctrl(tab.panel, word_wrap, self_rc, self.frame, self.from_keyboard.clone());
			let sizer = BoxSizer::builder(Orientation::Vertical).build();
			text_ctrl.add_to_sizer(sizer);
			tab.panel.set_sizer(sizer, true);
			// The native control only keeps text once it has been laid out in the shown panel.
			tab.panel.layout();
			tab.window = TextWindow::of(&slice);
			style_and_fill(text_ctrl, &slice, &tab.window, &style);
			let max_pos = text_ctrl.get_last_position();
			let pos = tab.window.to_local(current_doc_pos).min(max_pos);
			text_ctrl.set_insertion_point(pos);
			text_ctrl.show_position(pos);
			old_ctrl.destroy();
			tab.text_ctrl = text_ctrl;
		}
	}
}
