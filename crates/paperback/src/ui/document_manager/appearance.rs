//! Applying reader appearance settings across every open tab: font, colours, alignment, spacing,
//! and the word-wrap toggle that rebuilds each tab's text control. Split out of the main
//! `DocumentManager` impl.

use std::{rc::Rc, sync::Mutex};

use wxdragon::prelude::*;

use super::DocumentManager;
use crate::{
	text_window::TextWindow,
	ui::{
		readability::{
			apply_bg_color_to_ctrl, apply_foreground_color_to_ctrl, apply_letter_spacing_to_ctrl,
			apply_line_spacing_to_ctrl, apply_paragraph_spacing_to_ctrl, apply_readability_format_to_ctrl,
			apply_text_alignment_to_ctrl, build_font_from_readability,
		},
		reader_input,
		text_render::fill_text_ctrl_with_formatting,
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
			apply_foreground_color_to_ctrl(tab.text_ctrl, color);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_bg_color(&self, color: i32) {
		for tab in &self.tabs {
			apply_bg_color_to_ctrl(tab.text_ctrl, color);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_text_alignment(&self, alignment: i32) {
		for tab in &self.tabs {
			apply_text_alignment_to_ctrl(tab.text_ctrl, alignment);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_letter_spacing(&self, spacing: i32) {
		for tab in &self.tabs {
			apply_letter_spacing_to_ctrl(tab.text_ctrl, spacing);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_paragraph_spacing(&self, spacing: i32) {
		for tab in &self.tabs {
			apply_paragraph_spacing_to_ctrl(tab.text_ctrl, spacing);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_line_spacing(&self, line_spacing: i32) {
		for tab in &self.tabs {
			apply_line_spacing_to_ctrl(tab.text_ctrl, line_spacing);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_word_wrap(&mut self, self_rc: &Rc<Mutex<Self>>, word_wrap: bool) {
		let (rf, line_spacing, bg_color, text_alignment, letter_spacing, paragraph_spacing) = {
			let cfg = self.config.lock().unwrap();
			(
				cfg.get_readability_font(),
				cfg.get_line_spacing(),
				cfg.get_bg_color(),
				cfg.get_text_alignment(),
				cfg.get_letter_spacing(),
				cfg.get_paragraph_spacing(),
			)
		};
		for tab in &mut self.tabs {
			let old_ctrl = tab.text_ctrl;
			let current_doc_pos = tab.window.to_doc(old_ctrl.get_insertion_point());
			// Re-slice just the currently loaded window rather than the whole document: a wrap
			// toggle doesn't change what range is loaded, only how it's laid out.
			let slice = tab.session.get_window(tab.window.start(), tab.window.end());
			let text_ctrl =
				reader_input::build_text_ctrl(tab.panel, word_wrap, self_rc, self.frame, self.from_keyboard.clone());
			let sizer = BoxSizer::builder(Orientation::Vertical).build();
			sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 0);
			tab.panel.set_sizer(sizer, true);
			fill_text_ctrl_with_formatting(text_ctrl, &slice);
			tab.window = TextWindow::new(slice.start, slice.end);
			if let Some(font) = build_font_from_readability(&rf) {
				text_ctrl.set_font(&font);
			}
			apply_foreground_color_to_ctrl(text_ctrl, rf.color);
			apply_bg_color_to_ctrl(text_ctrl, bg_color);
			apply_readability_format_to_ctrl(
				text_ctrl,
				line_spacing,
				paragraph_spacing,
				letter_spacing,
				text_alignment,
			);
			let max_pos = text_ctrl.get_last_position();
			let pos = tab.window.to_local(current_doc_pos).clamp(0, max_pos);
			tab.panel.layout();
			text_ctrl.set_insertion_point(pos);
			text_ctrl.show_position(pos);
			old_ctrl.destroy();
			tab.text_ctrl = text_ctrl;
		}
	}
}
