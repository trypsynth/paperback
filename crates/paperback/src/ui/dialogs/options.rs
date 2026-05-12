use std::{
	cell::{Cell, RefCell},
	fmt::Write,
	mem,
	rc::Rc,
};

use paperback_core::config::{ConfigManager, ReadabilityFont};
use wxdragon::{prelude::*, translations::translate as t};

use super::DIALOG_PADDING;
use crate::translation_manager::TranslationManager;

#[derive(Clone, Debug)]
pub struct OptionsDialogResult {
	pub restore_previous_documents: bool,
	pub word_wrap: bool,
	pub minimize_to_tray: bool,
	pub start_maximized: bool,
	pub compact_go_menu: bool,
	pub navigation_wrap: bool,
	pub check_for_updates_on_startup: bool,
	pub bookmark_sounds: bool,
	pub recent_documents_to_show: i32,
	pub reading_speed_wpm: i32,
	pub language: String,
	pub update_channel: paperback_core::config::UpdateChannel,
	pub readability_font: ReadabilityFont,
	pub line_spacing: i32,
	pub bg_color: i32,
	pub text_alignment: i32,
	pub letter_spacing: i32,
	pub paragraph_spacing: i32,
}

struct OptionsDialogUi {
	dialog: Dialog,
	notebook: Notebook,
	restore_docs_check: CheckBox,
	word_wrap_check: CheckBox,
	minimize_to_tray_check: CheckBox,
	start_maximized_check: CheckBox,
	compact_go_menu_check: CheckBox,
	navigation_wrap_check: CheckBox,
	check_for_updates_check: CheckBox,
	bookmark_sounds_check: CheckBox,
	recent_docs_ctrl: SpinCtrl,
	reading_speed_ctrl: SpinCtrl,
	language_combo: ComboBox,
	update_channel_combo: ComboBox,
	language_codes: Vec<String>,
	current_language: String,
	ok_button: Button,
	cancel_button: Button,
	readability_font: Rc<RefCell<ReadabilityFont>>,
	line_spacing_ctrl: Choice,
	bg_color: Rc<Cell<i32>>,
	text_alignment_ctrl: Choice,
	letter_spacing_ctrl: Choice,
	paragraph_spacing_ctrl: Choice,
}

pub fn show_options_dialog(parent: &Frame, config: &ConfigManager) -> Option<OptionsDialogResult> {
	let ui = build_options_dialog_ui(parent, config);
	finalize_options_dialog_layout(&ui);
	if ui.dialog.show_modal() != wxdragon::id::ID_OK {
		return None;
	}
	let language = resolve_options_language(&ui);
	let update_channel = match ui.update_channel_combo.get_selection() {
		Some(1) => paperback_core::config::UpdateChannel::Dev,
		_ => paperback_core::config::UpdateChannel::Stable,
	};
	let readability_font = ui.readability_font.borrow().clone();
	let line_spacing = ui.line_spacing_ctrl.get_selection().unwrap_or(0) as i32;
	let bg_color = ui.bg_color.get();
	let text_alignment = ui.text_alignment_ctrl.get_selection().unwrap_or(0) as i32;
	let letter_spacing = ui.letter_spacing_ctrl.get_selection().unwrap_or(0) as i32;
	let paragraph_spacing = ui.paragraph_spacing_ctrl.get_selection().unwrap_or(0) as i32;
	Some(OptionsDialogResult {
		restore_previous_documents: ui.restore_docs_check.is_checked(),
		word_wrap: ui.word_wrap_check.is_checked(),
		minimize_to_tray: ui.minimize_to_tray_check.is_checked(),
		start_maximized: ui.start_maximized_check.is_checked(),
		compact_go_menu: ui.compact_go_menu_check.is_checked(),
		navigation_wrap: ui.navigation_wrap_check.is_checked(),
		check_for_updates_on_startup: ui.check_for_updates_check.is_checked(),
		bookmark_sounds: ui.bookmark_sounds_check.is_checked(),
		recent_documents_to_show: ui.recent_docs_ctrl.value(),
		reading_speed_wpm: ui.reading_speed_ctrl.value(),
		language,
		update_channel,
		readability_font,
		line_spacing,
		bg_color,
		text_alignment,
		letter_spacing,
		paragraph_spacing,
	})
}

fn build_options_dialog_ui(parent: &Frame, config: &ConfigManager) -> OptionsDialogUi {
	let dialog = Dialog::builder(parent, &t("Options")).build();
	let notebook = Notebook::builder(&dialog).with_style(NotebookStyle::Top).build();
	let general_panel = Panel::builder(&notebook).build();
	let reading_panel = Panel::builder(&notebook).build();
	let readability_panel = Panel::builder(&notebook).build();
	let general_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let reading_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let readability_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let restore_docs_check =
		CheckBox::builder(&general_panel).with_label(&t("&Restore previously opened documents on startup")).build();
	let word_wrap_check = CheckBox::builder(&readability_panel).with_label(&t("&Word wrap")).build();
	let minimize_to_tray_check = CheckBox::builder(&general_panel).with_label(&t("&Minimize to system tray")).build();
	let start_maximized_check = CheckBox::builder(&general_panel).with_label(&t("&Start maximized")).build();
	let compact_go_menu_check = CheckBox::builder(&reading_panel).with_label(&t("Show compact &go menu")).build();
	let navigation_wrap_check = CheckBox::builder(&reading_panel).with_label(&t("&Wrap navigation")).build();
	let bookmark_sounds_check =
		CheckBox::builder(&reading_panel).with_label(&t("Play &sounds on bookmarks and notes")).build();
	let check_for_updates_check =
		CheckBox::builder(&general_panel).with_label(&t("Check for &updates on startup")).build();
	let option_padding = 5;
	for check in [&restore_docs_check, &start_maximized_check, &minimize_to_tray_check, &check_for_updates_check] {
		general_sizer.add(check, 0, SizerFlag::All, option_padding);
	}
	for check in [&navigation_wrap_check, &compact_go_menu_check, &bookmark_sounds_check] {
		reading_sizer.add(check, 0, SizerFlag::All, option_padding);
	}
	let reading_speed_label =
		StaticText::builder(&reading_panel).with_label(&t("&Reading speed (words per minute):")).build();
	let reading_speed_ctrl = SpinCtrl::builder(&reading_panel).with_range(1, 2000).build();
	let reading_speed_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	reading_speed_sizer.add(&reading_speed_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	reading_speed_sizer.add(&reading_speed_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	reading_sizer.add_sizer(&reading_speed_sizer, 0, SizerFlag::All, option_padding);
	let max_recent_docs = 100;
	let recent_docs_label =
		StaticText::builder(&general_panel).with_label(&t("Number of &recent documents to show:")).build();
	let recent_docs_ctrl = SpinCtrl::builder(&general_panel).with_range(0, max_recent_docs).build();
	let recent_docs_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	recent_docs_sizer.add(&recent_docs_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	recent_docs_sizer.add(&recent_docs_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	general_sizer.add_sizer(&recent_docs_sizer, 0, SizerFlag::All, option_padding);
	let language_label = StaticText::builder(&general_panel).with_label(&t("&Language:")).build();
	let language_combo = ComboBox::builder(&general_panel).with_style(ComboBoxStyle::ReadOnly).build();
	let languages = TranslationManager::instance().lock().unwrap().available_languages();
	let mut language_codes = Vec::new();
	for lang in &languages {
		language_combo.append(&lang.native_name);
		language_codes.push(lang.code.clone());
	}
	let language_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	language_sizer.add(&language_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	language_sizer.add(&language_combo, 0, SizerFlag::AlignCenterVertical, 0);
	general_sizer.add_sizer(&language_sizer, 0, SizerFlag::All, option_padding);
	let channel_label = StaticText::builder(&general_panel).with_label(&t("Update Channel:")).build();
	let update_channel_combo = ComboBox::builder(&general_panel).with_style(ComboBoxStyle::ReadOnly).build();
	update_channel_combo.append(&t("Stable"));
	update_channel_combo.append(&t("Dev"));
	let channel_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	channel_sizer.add(&channel_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	channel_sizer.add(&update_channel_combo, 0, SizerFlag::AlignCenterVertical, 0);
	general_sizer.add_sizer(&channel_sizer, 0, SizerFlag::All, option_padding);
	let font_group_box = StaticBox::builder(&readability_panel).with_label(&t("Font")).build();
	let font_group_sizer = StaticBoxSizerBuilder::new_with_box(&font_group_box, Orientation::Vertical).build();
	let font_preview_label = StaticText::builder(&readability_panel).with_label("").build();
	let choose_font_button = Button::builder(&readability_panel).with_label(&t("Choose &Font...")).build();
	let reset_font_button = Button::builder(&readability_panel).with_label(&t("&Reset to Default Font")).build();
	font_group_sizer.add(&font_preview_label, 0, SizerFlag::All, option_padding);
	font_group_sizer.add(&choose_font_button, 0, SizerFlag::All, option_padding);
	font_group_sizer.add(&reset_font_button, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&font_group_sizer, 0, SizerFlag::Expand | SizerFlag::All, option_padding);
	let bg_group_box = StaticBox::builder(&readability_panel).with_label(&t("Background Color")).build();
	let bg_group_sizer = StaticBoxSizerBuilder::new_with_box(&bg_group_box, Orientation::Vertical).build();
	let bg_color_label = StaticText::builder(&readability_panel).with_label("").build();
	let choose_bg_button = Button::builder(&readability_panel).with_label(&t("Choose &Background Color...")).build();
	let reset_bg_button = Button::builder(&readability_panel).with_label(&t("Reset to &Default Background")).build();
	bg_group_sizer.add(&bg_color_label, 0, SizerFlag::All, option_padding);
	bg_group_sizer.add(&choose_bg_button, 0, SizerFlag::All, option_padding);
	bg_group_sizer.add(&reset_bg_button, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&bg_group_sizer, 0, SizerFlag::Expand | SizerFlag::All, option_padding);
	let line_spacing_label = StaticText::builder(&readability_panel).with_label(&t("&Line spacing:")).build();
	let line_spacing_ctrl = Choice::builder(&readability_panel).build();
	line_spacing_ctrl.append(&t("Normal"));
	line_spacing_ctrl.append(&t("1.5\u{00d7}"));
	line_spacing_ctrl.append(&t("Double"));
	let line_spacing_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	line_spacing_sizer.add(&line_spacing_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	line_spacing_sizer.add(&line_spacing_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	let paragraph_spacing_label = StaticText::builder(&readability_panel).with_label(&t("&Paragraph spacing:")).build();
	let paragraph_spacing_ctrl = Choice::builder(&readability_panel).build();
	paragraph_spacing_ctrl.append(&t("Normal"));
	paragraph_spacing_ctrl.append(&t("Relaxed"));
	paragraph_spacing_ctrl.append(&t("Wide"));
	let paragraph_spacing_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	paragraph_spacing_sizer.add(
		&paragraph_spacing_label,
		0,
		SizerFlag::AlignCenterVertical | SizerFlag::Right,
		DIALOG_PADDING,
	);
	paragraph_spacing_sizer.add(&paragraph_spacing_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	let letter_spacing_label = StaticText::builder(&readability_panel).with_label(&t("L&etter spacing:")).build();
	let letter_spacing_ctrl = Choice::builder(&readability_panel).build();
	letter_spacing_ctrl.append(&t("Normal"));
	letter_spacing_ctrl.append(&t("Wide"));
	letter_spacing_ctrl.append(&t("Very Wide"));
	let letter_spacing_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	letter_spacing_sizer.add(
		&letter_spacing_label,
		0,
		SizerFlag::AlignCenterVertical | SizerFlag::Right,
		DIALOG_PADDING,
	);
	letter_spacing_sizer.add(&letter_spacing_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	let text_alignment_label = StaticText::builder(&readability_panel).with_label(&t("Text &alignment:")).build();
	let text_alignment_ctrl = Choice::builder(&readability_panel).build();
	text_alignment_ctrl.append(&t("Left"));
	text_alignment_ctrl.append(&t("Center"));
	text_alignment_ctrl.append(&t("Right"));
	text_alignment_ctrl.append(&t("Justify"));
	let text_alignment_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	text_alignment_sizer.add(
		&text_alignment_label,
		0,
		SizerFlag::AlignCenterVertical | SizerFlag::Right,
		DIALOG_PADDING,
	);
	text_alignment_sizer.add(&text_alignment_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	readability_sizer.add(&word_wrap_check, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&line_spacing_sizer, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&paragraph_spacing_sizer, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&letter_spacing_sizer, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&text_alignment_sizer, 0, SizerFlag::All, option_padding);
	readability_panel.set_sizer(readability_sizer, true);
	general_panel.set_sizer(general_sizer, true);
	reading_panel.set_sizer(reading_sizer, true);
	notebook.add_page(&general_panel, &t("General"), true, None);
	notebook.add_page(&reading_panel, &t("Reading"), false, None);
	notebook.add_page(&readability_panel, &t("Readability"), false, None);
	restore_docs_check.set_value(config.get_app_bool("restore_previous_documents", true));
	word_wrap_check.set_value(config.get_app_bool("word_wrap", false));
	minimize_to_tray_check.set_value(config.get_app_bool("minimize_to_tray", false));
	start_maximized_check.set_value(config.get_app_bool("start_maximized", false));
	compact_go_menu_check.set_value(config.get_app_bool("compact_go_menu", true));
	navigation_wrap_check.set_value(config.get_app_bool("navigation_wrap", false));
	bookmark_sounds_check.set_value(config.get_app_bool("bookmark_sounds", true));
	check_for_updates_check.set_value(config.get_app_bool("check_for_updates_on_startup", true));
	recent_docs_ctrl.set_value(config.get_app_int("recent_documents_to_show", 25).clamp(0, max_recent_docs));
	reading_speed_ctrl.set_value(config.get_app_int("reading_speed_wpm", 150).clamp(1, 2000));
	let stored_language = config.get_app_string("language", "");
	let current_language = if stored_language.is_empty() {
		TranslationManager::instance().lock().unwrap().current_language()
	} else {
		stored_language
	};
	if let Some(index) = language_codes.iter().position(|code| code == &current_language) {
		language_combo.set_selection(u32::try_from(index).unwrap_or(0));
	}
	let current_channel = config.get_update_channel();
	let channel_index = match current_channel {
		paperback_core::config::UpdateChannel::Stable => 0,
		paperback_core::config::UpdateChannel::Dev => 1,
	};
	update_channel_combo.set_selection(channel_index);
	let initial_font = config.get_readability_font();
	font_preview_label.set_label(&font_description(&initial_font));
	let readability_font = Rc::new(RefCell::new(initial_font));
	let stored_line_spacing = config.get_line_spacing().clamp(0, 2) as u32;
	line_spacing_ctrl.set_selection(stored_line_spacing);
	paragraph_spacing_ctrl.set_selection(config.get_paragraph_spacing().clamp(0, 2) as u32);
	letter_spacing_ctrl.set_selection(config.get_letter_spacing().clamp(0, 2) as u32);
	text_alignment_ctrl.set_selection(config.get_text_alignment().clamp(0, 3) as u32);
	let stored_bg = config.get_bg_color();
	bg_color_label.set_label(&color_description(stored_bg));
	let bg_color = Rc::new(Cell::new(stored_bg));
	let font_state = Rc::clone(&readability_font);
	let preview_label = font_preview_label;
	let dialog_ref = dialog;
	choose_font_button.on_click(move |_| {
		let current = font_state.borrow().clone();
		if let Some(selected) = show_font_picker(dialog_ref, &current) {
			preview_label.set_label(&font_description(&selected));
			*font_state.borrow_mut() = selected;
		}
	});
	let font_state_reset = Rc::clone(&readability_font);
	let preview_label_reset = preview_label;
	reset_font_button.on_click(move |_| {
		let default_font = ReadabilityFont::default();
		preview_label_reset.set_label(&font_description(&default_font));
		*font_state_reset.borrow_mut() = default_font;
	});
	let bg_state = Rc::clone(&bg_color);
	let bg_label = bg_color_label;
	let dialog_for_bg = dialog_ref;
	choose_bg_button.on_click(move |_| {
		let current = bg_state.get();
		let initial = if current >= 0 {
			let r = ((current >> 16) & 0xFF) as u8;
			let g = ((current >> 8) & 0xFF) as u8;
			let b = (current & 0xFF) as u8;
			Some(Colour::rgb(r, g, b))
		} else {
			None
		};
		let mut dlg = ColourDialog::builder(&dialog_for_bg);
		if let Some(c) = initial {
			dlg = dlg.with_initial_colour(c);
		}
		let dlg = dlg.build();
		if dlg.show_modal() == wxdragon::id::ID_OK {
			if let Some(c) = dlg.get_colour() {
				let packed = ((c.r as i32) << 16) | ((c.g as i32) << 8) | c.b as i32;
				bg_state.set(packed);
				bg_label.set_label(&color_description(packed));
			}
		}
	});
	let bg_state_reset = Rc::clone(&bg_color);
	let bg_label_reset = bg_label;
	reset_bg_button.on_click(move |_| {
		bg_state_reset.set(-1);
		bg_label_reset.set_label(&color_description(-1));
	});
	let ok_button = Button::builder(&dialog_ref).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog_ref).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	ok_button.set_default();
	OptionsDialogUi {
		dialog: dialog_ref,
		notebook,
		restore_docs_check,
		word_wrap_check,
		minimize_to_tray_check,
		start_maximized_check,
		compact_go_menu_check,
		navigation_wrap_check,
		check_for_updates_check,
		bookmark_sounds_check,
		recent_docs_ctrl,
		reading_speed_ctrl,
		language_combo,
		update_channel_combo,
		language_codes,
		current_language,
		ok_button,
		cancel_button,
		readability_font,
		line_spacing_ctrl,
		bg_color,
		text_alignment_ctrl,
		letter_spacing_ctrl,
		paragraph_spacing_ctrl,
	}
}

fn finalize_options_dialog_layout(ui: &OptionsDialogUi) {
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ui.ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&ui.cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&ui.notebook, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	ui.dialog.set_sizer_and_fit(content_sizer, true);
	ui.dialog.centre();
}

fn resolve_options_language(ui: &OptionsDialogUi) -> String {
	ui.language_combo
		.get_selection()
		.and_then(|index| usize::try_from(index).ok())
		.and_then(|index| ui.language_codes.get(index).cloned())
		.unwrap_or_else(|| ui.current_language.clone())
}

fn color_description(color: i32) -> String {
	if color < 0 {
		t("Background: Default")
	} else {
		let r = ((color >> 16) & 0xFF) as u8;
		let g = ((color >> 8) & 0xFF) as u8;
		let b = (color & 0xFF) as u8;
		format!("#{r:02X}{g:02X}{b:02X}")
	}
}

fn font_description(rf: &ReadabilityFont) -> String {
	if rf.is_default() {
		return t("Font: Default");
	}
	let face = if rf.face_name.is_empty() { t("Default") } else { rf.face_name.clone() };
	let mut desc = format!("Font: {face}");
	if rf.point_size > 0 {
		let _ = write!(desc, ", {}pt", rf.point_size);
	}
	if rf.weight >= FontWeight::Bold as i32 {
		let _ = write!(desc, ", {}", t("Bold"));
	}
	if rf.style == FontStyle::Italic as i32 || rf.style == FontStyle::Slant as i32 {
		let _ = write!(desc, ", {}", t("Italic"));
	}
	if rf.underlined {
		let _ = write!(desc, ", {}", t("Underlined"));
	}
	if rf.strikethrough {
		let _ = write!(desc, ", {}", t("Strikethrough"));
	}
	desc
}

fn show_font_picker(parent: Dialog, current: &ReadabilityFont) -> Option<ReadabilityFont> {
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
	let dlg = FontDialog::builder(&parent).with_font_data(&font_data).build();
	if dlg.show_modal() != wxdragon::id::ID_OK {
		return None;
	}
	let font = dlg.get_font()?;
	let chosen_color = if let Some(fd) = dlg.get_font_data() {
		let c = fd.get_chosen_colour();
		// Prevent double-free: this FontData pointer is owned by the dialog, not by us
		mem::forget(fd);
		c.map(|col| ((col.r as i32) << 16) | ((col.g as i32) << 8) | col.b as i32).unwrap_or(-1)
	} else {
		-1
	};
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
