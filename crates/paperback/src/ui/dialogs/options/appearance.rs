//! The Appearance tab: how text looks.

use std::{
	cell::{Cell, RefCell},
	rc::Rc,
};

use paperback_core::config::{ConfigManager, ReadabilityFont};
use patois::t;
use wxdragon::prelude::*;

use super::{
	OPTION_PADDING,
	font::{color_description, font_description, show_font_picker},
	labeled_row,
};
use crate::ui::readability::stored_colour;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppearanceSettings {
	pub word_wrap: bool,
	pub readability_font: ReadabilityFont,
	/// A packed RGB colour, or negative for the system's.
	pub bg_color: i32,
	pub line_spacing: i32,
}

impl AppearanceSettings {
	pub fn load(config: &ConfigManager) -> Self {
		Self {
			word_wrap: config.get_app_bool("word_wrap", false),
			readability_font: config.get_readability_font(),
			bg_color: config.get_bg_color(),
			line_spacing: config.get_line_spacing(),
		}
	}

	pub fn save(&self, config: &ConfigManager) {
		config.set_app_bool("word_wrap", self.word_wrap);
		config.set_readability_font(&self.readability_font);
		config.set_bg_color(self.bg_color);
		config.set_line_spacing(self.line_spacing);
	}
}

pub(super) struct AppearanceTab {
	word_wrap: CheckBox,
	font: Rc<RefCell<ReadabilityFont>>,
	bg_color: Rc<Cell<i32>>,
	line_spacing: Choice,
}

impl AppearanceTab {
	/// `dialog` is the parent of the font and colour pickers this tab opens.
	pub(super) fn new(panel: Panel, dialog: Dialog, settings: &AppearanceSettings, padding: i32) -> Self {
		// TRANSLATORS: Option to toggle word wrapping of text
		let word_wrap = CheckBox::builder(&panel).with_label(&t("&Word wrap")).build();
		// TRANSLATORS: Label/header for the Font options section
		let font_box = StaticBox::builder(&panel).with_label(&t("Font")).build();
		let font_sizer = StaticBoxSizerBuilder::new_with_box(&font_box, Orientation::Vertical).build();
		let font_preview = StaticText::builder(&panel).with_label("").build();
		// TRANSLATORS: Button label to pick a font
		let choose_font = Button::builder(&panel).with_label(&t("Choose &Font...")).build();
		// TRANSLATORS: Button label to restore font settings to default values
		let reset_font = Button::builder(&panel).with_label(&t("&Reset to Default Font")).build();
		font_sizer.add(&font_preview, 0, SizerFlag::All, OPTION_PADDING);
		font_sizer.add(&choose_font, 0, SizerFlag::All, OPTION_PADDING);
		font_sizer.add(&reset_font, 0, SizerFlag::All, OPTION_PADDING);
		// TRANSLATORS: Label/header for the Background Color options section
		let bg_box = StaticBox::builder(&panel).with_label(&t("Background Color")).build();
		let bg_sizer = StaticBoxSizerBuilder::new_with_box(&bg_box, Orientation::Vertical).build();
		let bg_preview = StaticText::builder(&panel).with_label("").build();
		// TRANSLATORS: Button label to pick a background color
		let choose_bg = Button::builder(&panel).with_label(&t("Choose &Background Color...")).build();
		// TRANSLATORS: Button label to restore background color to default values
		let reset_bg = Button::builder(&panel).with_label(&t("Reset to &Default Background")).build();
		bg_sizer.add(&bg_preview, 0, SizerFlag::All, OPTION_PADDING);
		bg_sizer.add(&choose_bg, 0, SizerFlag::All, OPTION_PADDING);
		bg_sizer.add(&reset_bg, 0, SizerFlag::All, OPTION_PADDING);
		// TRANSLATORS: Label for the line spacing dropdown
		let (line_spacing_label, line_spacing) = labeled_choice(
			panel,
			&t("&Line spacing:"),
			&[
				// TRANSLATORS: Default spacing option (as opposed to relaxed/wide), shown in the line, paragraph, and letter spacing dropdowns
				t("Normal"),
				// TRANSLATORS: 1.5x line spacing option
				t("1.5×"),
				// TRANSLATORS: Double line spacing option
				t("Double"),
			],
		);
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&word_wrap, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&font_sizer, 0, SizerFlag::Expand | SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&bg_sizer, 0, SizerFlag::Expand | SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(line_spacing_label, &line_spacing, padding), 0, SizerFlag::All, OPTION_PADDING);
		panel.set_sizer(sizer, true);
		word_wrap.set_value(settings.word_wrap);
		line_spacing.set_selection(settings.line_spacing.clamp(0, 2).unsigned_abs());
		font_preview.set_label(&font_description(&settings.readability_font));
		bg_preview.set_label(&color_description(settings.bg_color));
		let font = Rc::new(RefCell::new(settings.readability_font.clone()));
		let bg_color = Rc::new(Cell::new(settings.bg_color));
		let font_state = Rc::clone(&font);
		choose_font.on_click(move |_| {
			let current = font_state.borrow().clone();
			if let Some(selected) = show_font_picker(dialog, &current) {
				font_preview.set_label(&font_description(&selected));
				*font_state.borrow_mut() = selected;
			}
		});
		let font_state = Rc::clone(&font);
		reset_font.on_click(move |_| {
			let default_font = ReadabilityFont::default();
			font_preview.set_label(&font_description(&default_font));
			*font_state.borrow_mut() = default_font;
		});
		let bg_state = Rc::clone(&bg_color);
		choose_bg.on_click(move |_| {
			// TRANSLATORS: Title of the system dialog for picking a background colour
			let mut picker = ColourDialog::builder(&dialog).with_title(&t("Choose a colour"));
			if let Some(colour) = stored_colour(bg_state.get()) {
				picker = picker.with_initial_colour(colour);
			}
			let picker = picker.build();
			if picker.show_modal() == ID_OK
				&& let Some(c) = picker.get_colour()
			{
				let packed = (i32::from(c.r) << 16) | (i32::from(c.g) << 8) | i32::from(c.b);
				bg_state.set(packed);
				bg_preview.set_label(&color_description(packed));
			}
		});
		let bg_state = Rc::clone(&bg_color);
		reset_bg.on_click(move |_| {
			bg_state.set(-1);
			bg_preview.set_label(&color_description(-1));
		});
		Self { word_wrap, font, bg_color, line_spacing }
	}

	pub(super) fn values(&self) -> AppearanceSettings {
		let selection = |choice: &Choice| choice.get_selection().and_then(|i| i32::try_from(i).ok()).unwrap_or(0);
		AppearanceSettings {
			word_wrap: self.word_wrap.is_checked(),
			readability_font: self.font.borrow().clone(),
			bg_color: self.bg_color.get(),
			line_spacing: selection(&self.line_spacing),
		}
	}
}

/// A label and a dropdown of `options`, created in that order so the label names the dropdown.
fn labeled_choice(panel: Panel, label_text: &str, options: &[String]) -> (StaticText, Choice) {
	let label = StaticText::builder(&panel).with_label(label_text).build();
	let choice = Choice::builder(&panel).build();
	for option in options {
		choice.append(option);
	}
	#[cfg(target_os = "macos")]
	choice.set_accessibility_label(label_text.replace('&', "").trim_end_matches(':').trim());
	(label, choice)
}
