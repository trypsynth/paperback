use std::{
	cell::{Cell, RefCell},
	ffi::CString,
	fmt::Write,
	path::Path,
	rc::Rc,
	sync::Mutex,
};

use bitflags::bitflags;
use wxdragon::{event::WebViewEvents, ffi, prelude::*, translations::translate as t, widgets::WebView};

#[cfg(target_os = "linux")]
mod accessible_tree;
#[cfg(target_os = "linux")]
mod elements_gtk;
#[cfg(target_os = "linux")]
mod toc_gtk;

use crate::{
	config::{ConfigManager, ReadabilityFont},
	document::{DocumentStats, TocItem},
	reader_core,
	session::DocumentSession,
	translation_manager::TranslationManager,
	types::{BookmarkDisplayEntry, BookmarkFilterType, DocumentListStatus},
};

const DIALOG_PADDING: i32 = 10;
const RECENT_DOCS_LIST_WIDTH: i32 = 800;
const RECENT_DOCS_LIST_HEIGHT: i32 = 600;
const RECENT_DOCS_FILENAME_WIDTH: i32 = 250;
const RECENT_DOCS_STATUS_WIDTH: i32 = 100;
const RECENT_DOCS_PATH_WIDTH: i32 = 450;
const DOC_INFO_WIDTH: i32 = 600;
const DOC_INFO_HEIGHT: i32 = 400;
const KEY_DELETE: i32 = 127;
const KEY_NUMPAD_DELETE: i32 = 330;
#[cfg(not(target_os = "linux"))]
const KEY_SPACE: i32 = 32;
const KEY_ESCAPE: i32 = 27;
const KEY_RETURN: i32 = 13;
const KEY_NUMPAD_ENTER: i32 = 370;
const WXK_END: i32 = 312;
const WXK_HOME: i32 = 313;
const WXK_LEFT: i32 = 314;
const WXK_UP: i32 = 315;
const WXK_RIGHT: i32 = 316;
const WXK_DOWN: i32 = 317;
const WXK_PAGEUP: i32 = 366;
const WXK_PAGEDOWN: i32 = 367;

type NavigationHandler = Box<dyn Fn(&str) -> bool>;

#[derive(Clone, Debug)]
pub struct OptionsDialogResult {
	pub flags: OptionsDialogFlags,
	pub recent_documents_to_show: i32,
	pub reading_speed_wpm: i32,
	pub language: String,
	pub update_channel: crate::config::UpdateChannel,
	pub readability_font: ReadabilityFont,
	pub line_spacing: i32,
	pub bg_color: i32,
	pub text_alignment: i32,
	pub letter_spacing: i32,
	pub paragraph_spacing: i32,
}

bitflags! {
	#[derive(Clone, Copy, Debug)]
	pub struct OptionsDialogFlags: u16 {
		const RESTORE_PREVIOUS_DOCUMENTS = 1 << 0;
		const WORD_WRAP = 1 << 1;
		const MINIMIZE_TO_TRAY = 1 << 2;
		const START_MAXIMIZED = 1 << 3;
		const COMPACT_GO_MENU = 1 << 4;
		const NAVIGATION_WRAP = 1 << 5;
		const CHECK_FOR_UPDATES_ON_STARTUP = 1 << 6;
		const BOOKMARK_SOUNDS = 1 << 7;
	}
}

pub struct BookmarkDialogResult {
	pub start: i64,
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
	let flags = build_options_dialog_flags(&ui);
	let update_channel = match ui.update_channel_combo.get_selection() {
		Some(1) => crate::config::UpdateChannel::Dev,
		_ => crate::config::UpdateChannel::Stable,
	};
	let readability_font = ui.readability_font.borrow().clone();
	let line_spacing = ui.line_spacing_ctrl.get_selection().unwrap_or(0) as i32;
	let bg_color = ui.bg_color.get();
	let text_alignment = ui.text_alignment_ctrl.get_selection().unwrap_or(0) as i32;
	let letter_spacing = ui.letter_spacing_ctrl.get_selection().unwrap_or(0) as i32;
	let paragraph_spacing = ui.paragraph_spacing_ctrl.get_selection().unwrap_or(0) as i32;
	Some(OptionsDialogResult {
		flags,
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

	// Readability tab — Font group
	let font_group_box = StaticBox::builder(&readability_panel).with_label(&t("Font")).build();
	let font_group_sizer = StaticBoxSizerBuilder::new_with_box(&font_group_box, Orientation::Vertical).build();
	let font_preview_label = StaticText::builder(&readability_panel).with_label("").build();
	let choose_font_button = Button::builder(&readability_panel).with_label(&t("Choose &Font...")).build();
	let reset_font_button = Button::builder(&readability_panel).with_label(&t("&Reset to Default Font")).build();
	font_group_sizer.add(&font_preview_label, 0, SizerFlag::All, option_padding);
	font_group_sizer.add(&choose_font_button, 0, SizerFlag::All, option_padding);
	font_group_sizer.add(&reset_font_button, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&font_group_sizer, 0, SizerFlag::Expand | SizerFlag::All, option_padding);

	// Background color
	let bg_group_box = StaticBox::builder(&readability_panel).with_label(&t("Background Color")).build();
	let bg_group_sizer = StaticBoxSizerBuilder::new_with_box(&bg_group_box, Orientation::Vertical).build();
	let bg_color_label = StaticText::builder(&readability_panel).with_label("").build();
	let choose_bg_button = Button::builder(&readability_panel).with_label(&t("Choose &Background Color...")).build();
	let reset_bg_button = Button::builder(&readability_panel).with_label(&t("Reset to &Default Background")).build();
	bg_group_sizer.add(&bg_color_label, 0, SizerFlag::All, option_padding);
	bg_group_sizer.add(&choose_bg_button, 0, SizerFlag::All, option_padding);
	bg_group_sizer.add(&reset_bg_button, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&bg_group_sizer, 0, SizerFlag::Expand | SizerFlag::All, option_padding);

	// Layout choices
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
		crate::config::UpdateChannel::Stable => 0,
		crate::config::UpdateChannel::Dev => 1,
	};
	update_channel_combo.set_selection(channel_index);

	// Initialize readability font state from config
	let initial_font = config.get_readability_font();
	font_preview_label.set_label(&font_description(&initial_font));
	let readability_font = Rc::new(RefCell::new(initial_font));
	let stored_line_spacing = config.get_line_spacing().clamp(0, 2) as u32;
	line_spacing_ctrl.set_selection(stored_line_spacing);
	paragraph_spacing_ctrl.set_selection(config.get_paragraph_spacing().clamp(0, 2) as u32);
	letter_spacing_ctrl.set_selection(config.get_letter_spacing().clamp(0, 2) as u32);
	text_alignment_ctrl.set_selection(config.get_text_alignment().clamp(0, 3) as u32);

	// Background color state
	let stored_bg = config.get_bg_color();
	bg_color_label.set_label(&color_description(stored_bg));
	let bg_color = Rc::new(Cell::new(stored_bg));

	// "Choose Font..." button handler
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

	// "Reset to Default Font" button handler
	let font_state_reset = Rc::clone(&readability_font);
	let preview_label_reset = preview_label;
	reset_font_button.on_click(move |_| {
		let default_font = ReadabilityFont::default();
		preview_label_reset.set_label(&font_description(&default_font));
		*font_state_reset.borrow_mut() = default_font;
	});

	// "Choose Background Color..." button handler
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

	// "Reset to Default Background" button handler
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

fn build_options_dialog_flags(ui: &OptionsDialogUi) -> OptionsDialogFlags {
	let mut flags = OptionsDialogFlags::empty();
	if ui.restore_docs_check.is_checked() {
		flags.insert(OptionsDialogFlags::RESTORE_PREVIOUS_DOCUMENTS);
	}
	if ui.word_wrap_check.is_checked() {
		flags.insert(OptionsDialogFlags::WORD_WRAP);
	}
	if ui.minimize_to_tray_check.is_checked() {
		flags.insert(OptionsDialogFlags::MINIMIZE_TO_TRAY);
	}
	if ui.start_maximized_check.is_checked() {
		flags.insert(OptionsDialogFlags::START_MAXIMIZED);
	}
	if ui.compact_go_menu_check.is_checked() {
		flags.insert(OptionsDialogFlags::COMPACT_GO_MENU);
	}
	if ui.navigation_wrap_check.is_checked() {
		flags.insert(OptionsDialogFlags::NAVIGATION_WRAP);
	}
	if ui.check_for_updates_check.is_checked() {
		flags.insert(OptionsDialogFlags::CHECK_FOR_UPDATES_ON_STARTUP);
	}
	if ui.bookmark_sounds_check.is_checked() {
		flags.insert(OptionsDialogFlags::BOOKMARK_SOUNDS);
	}
	flags
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
		std::mem::forget(fd);
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

pub fn show_bookmark_dialog(
	parent: &Frame,
	session: &DocumentSession,
	config: &Rc<Mutex<ConfigManager>>,
	current_pos: i64,
	initial_filter: BookmarkFilterType,
) -> Option<BookmarkDialogResult> {
	let file_path = session.file_path().to_string();
	let content = Rc::new(session.content());
	let dialog = Dialog::builder(parent, &t("Jump to Bookmark")).build();
	let BookmarkDialogUi {
		filter_choice,
		filter_sizer,
		bookmark_list,
		edit_button,
		delete_button,
		jump_button,
		cancel_button,
	} = build_bookmark_dialog_ui(dialog, initial_filter);
	let state = build_bookmark_dialog_state(jump_button, delete_button, edit_button);
	let repopulate = build_bookmark_repopulate(BookmarkRepopulateParams {
		list: bookmark_list,
		config: Rc::clone(config),
		file_path: file_path.clone(),
		content: Rc::clone(&content),
		entries: Rc::clone(&state.entries),
		selected_start: Rc::clone(&state.selected_start),
		selected_end: Rc::clone(&state.selected_end),
		filter_choice,
		set_buttons_enabled: Rc::clone(&state.set_buttons_enabled),
	});
	repopulate(current_pos);
	bind_bookmark_selection(BookmarkSelectionParams {
		list: bookmark_list,
		entries: Rc::clone(&state.entries),
		selected_start: Rc::clone(&state.selected_start),
		selected_end: Rc::clone(&state.selected_end),
		set_buttons_enabled: Rc::clone(&state.set_buttons_enabled),
	});
	bind_bookmark_jump(dialog, jump_button, &state.selected_start);
	bind_bookmark_actions(BookmarkDialogActions {
		dialog,
		filter_choice,
		bookmark_list,
		edit_button,
		delete_button,
		cancel_button,
		repopulate: Rc::clone(&repopulate),
		selected_start: Rc::clone(&state.selected_start),
		selected_end: Rc::clone(&state.selected_end),
		config: Rc::clone(config),
		file_path,
		current_pos,
	});
	finalize_bookmark_dialog_layout(
		dialog,
		filter_sizer,
		bookmark_list,
		edit_button,
		delete_button,
		jump_button,
		cancel_button,
	);
	if dialog.show_modal() != wxdragon::id::ID_OK {
		return None;
	}
	let start = state.selected_start.get();
	if start >= 0 { Some(BookmarkDialogResult { start }) } else { None }
}

struct BookmarkDialogUi {
	filter_choice: ComboBox,
	filter_sizer: BoxSizer,
	bookmark_list: ListBox,
	edit_button: Button,
	delete_button: Button,
	jump_button: Button,
	cancel_button: Button,
}

struct BookmarkDialogState {
	entries: Rc<RefCell<Vec<BookmarkDisplayEntry>>>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	set_buttons_enabled: Rc<dyn Fn(bool)>,
}

struct BookmarkRepopulateParams {
	list: ListBox,
	config: Rc<Mutex<ConfigManager>>,
	file_path: String,
	content: Rc<String>,
	entries: Rc<RefCell<Vec<BookmarkDisplayEntry>>>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	filter_choice: ComboBox,
	set_buttons_enabled: Rc<dyn Fn(bool)>,
}

struct BookmarkSelectionParams {
	list: ListBox,
	entries: Rc<RefCell<Vec<BookmarkDisplayEntry>>>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	set_buttons_enabled: Rc<dyn Fn(bool)>,
}

struct BookmarkDialogActions {
	dialog: Dialog,
	filter_choice: ComboBox,
	bookmark_list: ListBox,
	edit_button: Button,
	delete_button: Button,
	cancel_button: Button,
	repopulate: Rc<dyn Fn(i64)>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	config: Rc<Mutex<ConfigManager>>,
	file_path: String,
	current_pos: i64,
}

fn build_bookmark_dialog_ui(dialog: Dialog, initial_filter: BookmarkFilterType) -> BookmarkDialogUi {
	let filter_label = StaticText::builder(&dialog).with_label(&t("&Filter:")).build();
	let filter_choice = ComboBox::builder(&dialog).with_style(ComboBoxStyle::ReadOnly).build();
	filter_choice.append(&t("All"));
	filter_choice.append(&t("Bookmarks"));
	filter_choice.append(&t("Notes"));
	let initial_index = match initial_filter {
		BookmarkFilterType::BookmarksOnly => 1,
		BookmarkFilterType::NotesOnly => 2,
		BookmarkFilterType::All => 0,
	};
	filter_choice.set_selection(initial_index);
	let filter_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	filter_sizer.add(&filter_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 6);
	filter_sizer.add(&filter_choice, 1, SizerFlag::Expand, 0);
	let bookmark_list = ListBox::builder(&dialog).build();
	let edit_button = Button::builder(&dialog).with_label(&t("&Edit Note")).build();
	let delete_button = Button::builder(&dialog).with_label(&t("&Delete")).build();
	let jump_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("&Jump")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("&Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	jump_button.set_default();
	BookmarkDialogUi {
		filter_choice,
		filter_sizer,
		bookmark_list,
		edit_button,
		delete_button,
		jump_button,
		cancel_button,
	}
}

fn build_bookmark_dialog_state(jump_button: Button, delete_button: Button, edit_button: Button) -> BookmarkDialogState {
	let entries: Rc<RefCell<Vec<BookmarkDisplayEntry>>> = Rc::new(RefCell::new(Vec::new()));
	let selected_start = Rc::new(Cell::new(-1i64));
	let selected_end = Rc::new(Cell::new(-1i64));
	let jump_button_for_state = jump_button;
	let delete_button_for_state = delete_button;
	let edit_button_for_state = edit_button;
	let set_buttons_enabled = Rc::new(move |enabled: bool| {
		jump_button_for_state.enable(enabled);
		delete_button_for_state.enable(enabled);
		edit_button_for_state.enable(enabled);
	});
	set_buttons_enabled(false);
	BookmarkDialogState { entries, selected_start, selected_end, set_buttons_enabled }
}

fn build_bookmark_repopulate(params: BookmarkRepopulateParams) -> Rc<dyn Fn(i64)> {
	let BookmarkRepopulateParams {
		list,
		config,
		file_path,
		content,
		entries,
		selected_start,
		selected_end,
		filter_choice,
		set_buttons_enabled,
	} = params;
	Rc::new(move |pos: i64| {
		let filter_index = filter_choice.get_selection().unwrap_or(0);
		let filter = match filter_index {
			1 => BookmarkFilterType::BookmarksOnly,
			2 => BookmarkFilterType::NotesOnly,
			_ => BookmarkFilterType::All,
		};
		let content_for_snippet = Rc::clone(&content);
		let get_text_range = move |start: i64, end: i64| -> String {
			let content = content_for_snippet.as_str();
			let total_chars = content.chars().count();
			let start_pos = usize::try_from(start.max(0)).unwrap_or(0).min(total_chars);
			let end_pos = usize::try_from(end.max(0)).unwrap_or(0).min(total_chars);
			if start_pos >= end_pos {
				return String::new();
			}
			content.chars().skip(start_pos).take(end_pos - start_pos).collect()
		};
		let content_for_line = Rc::clone(&content);
		let get_line_text = move |position: i64| -> String {
			let content = content_for_line.as_str();
			let total_chars = content.chars().count();
			let pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
			let line_start =
				content.chars().take(pos).collect::<Vec<_>>().iter().rposition(|&c| c == '\n').map_or(0, |idx| idx + 1);
			let chars_after_start: String = content.chars().skip(line_start).collect();
			let line_end = chars_after_start.find('\n').map_or(chars_after_start.len(), |idx| idx);
			chars_after_start.chars().take(line_end).collect()
		};
		let previous_selected = selected_start.get();
		list.clear();
		entries.borrow_mut().clear();
		let filtered = {
			let cfg = config.lock().unwrap();
			reader_core::get_filtered_bookmarks(&cfg, &file_path, pos, filter)
		};
		for item in filtered.items {
			let snippet =
				if item.is_whole_line { get_line_text(item.start) } else { get_text_range(item.start, item.end) };
			let mut snippet = snippet.trim().to_string();
			if snippet.is_empty() {
				snippet = t("blank");
			}
			let display = if item.note.is_empty() { snippet.clone() } else { format!("{} - {}", item.note, snippet) };
			entries.borrow_mut().push(BookmarkDisplayEntry { start: item.start, end: item.end });
			list.append(&display);
		}
		selected_start.set(-1);
		selected_end.set(-1);
		set_buttons_enabled(false);
		let entries_ref = entries.borrow();
		if previous_selected >= 0 {
			if let Some((idx, entry)) =
				entries_ref.iter().enumerate().find(|(_, entry)| entry.start == previous_selected)
			{
				if let Ok(idx_u32) = u32::try_from(idx) {
					list.set_selection(idx_u32, true);
				}
				selected_start.set(entry.start);
				selected_end.set(entry.end);
				set_buttons_enabled(true);
				return;
			}
		}
		if filtered.closest_index >= 0 {
			if let Ok(idx) = usize::try_from(filtered.closest_index) {
				if let Some(entry) = entries_ref.get(idx) {
					if let Ok(idx_u32) = u32::try_from(idx) {
						list.set_selection(idx_u32, true);
					}
					selected_start.set(entry.start);
					selected_end.set(entry.end);
					set_buttons_enabled(true);
				}
			}
		}
	})
}

fn bind_bookmark_selection(params: BookmarkSelectionParams) {
	let BookmarkSelectionParams { list, entries, selected_start, selected_end, set_buttons_enabled } = params;
	list.on_selection_changed(move |event| {
		let selection = event.get_selection().unwrap_or(-1);
		if selection >= 0 {
			let entries_ref = entries.borrow();
			if let Ok(index) = usize::try_from(selection) {
				if let Some(entry) = entries_ref.get(index) {
					selected_start.set(entry.start);
					selected_end.set(entry.end);
					set_buttons_enabled(true);
					return;
				}
			}
		}
		selected_start.set(-1);
		selected_end.set(-1);
		set_buttons_enabled(false);
	});
}

fn bind_bookmark_jump(dialog: Dialog, jump_button: Button, selected_start: &Rc<Cell<i64>>) {
	let dialog_for_jump = dialog;
	let selected_start_for_jump = Rc::clone(selected_start);
	jump_button.on_click(move |_| {
		if selected_start_for_jump.get() >= 0 {
			dialog_for_jump.end_modal(wxdragon::id::ID_OK);
		} else {
			MessageDialog::builder(&dialog_for_jump, &t("Please select a bookmark to jump to."), &t("Error"))
				.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
				.build()
				.show_modal();
		}
	});
}

fn bind_bookmark_actions(actions: BookmarkDialogActions) {
	let BookmarkDialogActions {
		dialog,
		filter_choice,
		bookmark_list,
		edit_button,
		delete_button,
		cancel_button,
		repopulate,
		selected_start,
		selected_end,
		config,
		file_path,
		current_pos,
	} = actions;
	bind_bookmark_cancel(dialog, cancel_button);
	bind_bookmark_filter(filter_choice, Rc::clone(&repopulate), current_pos);
	bind_bookmark_delete(
		delete_button,
		Rc::clone(&repopulate),
		Rc::clone(&selected_start),
		Rc::clone(&selected_end),
		Rc::clone(&config),
		file_path.clone(),
		current_pos,
	);
	bind_bookmark_edit(BookmarkEditParams {
		dialog,
		edit_button,
		repopulate: Rc::clone(&repopulate),
		selected_start: Rc::clone(&selected_start),
		selected_end: Rc::clone(&selected_end),
		config: Rc::clone(&config),
		file_path: file_path.clone(),
		current_pos,
	});
	bind_bookmark_key_actions(
		bookmark_list,
		Rc::clone(&repopulate),
		Rc::clone(&selected_start),
		Rc::clone(&selected_end),
		Rc::clone(&config),
		file_path,
		current_pos,
	);
	bind_bookmark_double_click(bookmark_list, dialog, selected_start);
}

fn bind_bookmark_cancel(dialog: Dialog, cancel_button: Button) {
	cancel_button.on_click(move |_| {
		dialog.end_modal(wxdragon::id::ID_CANCEL);
	});
}

fn bind_bookmark_filter(filter_choice: ComboBox, repopulate: Rc<dyn Fn(i64)>, current_pos: i64) {
	filter_choice.on_selection_changed(move |_event| {
		repopulate(current_pos);
	});
}

fn bind_bookmark_delete(
	delete_button: Button,
	repopulate: Rc<dyn Fn(i64)>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	config: Rc<Mutex<ConfigManager>>,
	file_path: String,
	current_pos: i64,
) {
	delete_button.on_click(move |_| {
		let start = selected_start.get();
		let end = selected_end.get();
		if start < 0 {
			return;
		}
		{
			let cfg = config.lock().unwrap();
			cfg.remove_bookmark(&file_path, start, end);
			cfg.flush();
		}
		repopulate(current_pos);
	});
}

struct BookmarkEditParams {
	dialog: Dialog,
	edit_button: Button,
	repopulate: Rc<dyn Fn(i64)>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	config: Rc<Mutex<ConfigManager>>,
	file_path: String,
	current_pos: i64,
}

fn bind_bookmark_edit(params: BookmarkEditParams) {
	let BookmarkEditParams {
		dialog,
		edit_button,
		repopulate,
		selected_start,
		selected_end,
		config,
		file_path,
		current_pos,
	} = params;
	edit_button.on_click(move |_| {
		let start = selected_start.get();
		let end = selected_end.get();
		if start < 0 {
			return;
		}
		let existing_note = {
			let cfg = config.lock().unwrap();
			cfg.get_bookmarks(&file_path)
				.into_iter()
				.find(|bm| bm.start == start && bm.end == end)
				.map(|bm| bm.note)
				.unwrap_or_default()
		};
		let Some(note) =
			show_note_entry_dialog(&dialog, &t("Bookmark Note"), &t("Edit bookmark note:"), &existing_note)
		else {
			return;
		};
		{
			let cfg = config.lock().unwrap();
			cfg.update_bookmark_note(&file_path, start, end, &note);
			cfg.flush();
		}
		repopulate(current_pos);
	});
}

fn bind_bookmark_key_actions(
	bookmark_list: ListBox,
	repopulate: Rc<dyn Fn(i64)>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	config: Rc<Mutex<ConfigManager>>,
	file_path: String,
	current_pos: i64,
) {
	bookmark_list.bind_internal(EventType::KEY_DOWN, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		if key == KEY_DELETE || key == KEY_NUMPAD_DELETE {
			let start = selected_start.get();
			let end = selected_end.get();
			if start >= 0 {
				{
					let cfg = config.lock().unwrap();
					cfg.remove_bookmark(&file_path, start, end);
					cfg.flush();
				}
				repopulate(current_pos);
			}
			event.skip(false);
			return;
		}
		event.skip(true);
	});
}

fn bind_bookmark_double_click(bookmark_list: ListBox, dialog: Dialog, selected_start: Rc<Cell<i64>>) {
	bookmark_list.on_item_double_clicked(move |_| {
		if selected_start.get() >= 0 {
			dialog.end_modal(wxdragon::id::ID_OK);
		}
	});
}

fn finalize_bookmark_dialog_layout(
	dialog: Dialog,
	filter_sizer: BoxSizer,
	bookmark_list: ListBox,
	edit_button: Button,
	delete_button: Button,
	jump_button: Button,
	cancel_button: Button,
) {
	let action_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	action_sizer.add(&edit_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&delete_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&jump_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&cancel_button, 0, SizerFlag::Right, DIALOG_PADDING);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&filter_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(
		&bookmark_list,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	content_sizer.add_sizer(&action_sizer, 0, SizerFlag::AlignRight | SizerFlag::All, DIALOG_PADDING);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	bookmark_list.set_focus();
}

pub fn show_note_entry_dialog(
	parent: &dyn WxWidget,
	title: &str,
	message: &str,
	existing_note: &str,
) -> Option<String> {
	let dialog = Dialog::builder(parent, title).build();
	let message_label = StaticText::builder(&dialog).with_label(message).build();
	let note_ctrl = TextCtrl::builder(&dialog)
		.with_value(existing_note)
		.with_style(TextCtrlStyle::MultiLine)
		.with_size(Size::new(400, 200))
		.build();
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		dialog_for_ok.end_modal(wxdragon::id::ID_OK);
	});
	let dialog_for_cancel = dialog;
	cancel_button.on_click(move |_| {
		dialog_for_cancel.end_modal(wxdragon::id::ID_CANCEL);
	});
	let dialog_for_key = dialog;
	note_ctrl.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_RETURN {
				if event.shift_down() {
					event.skip(true);
				} else {
					dialog_for_key.end_modal(wxdragon::id::ID_OK);
					event.skip(false);
				}
				return;
			}
		}
		event.skip(true);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&message_label, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(
		&note_ctrl,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	note_ctrl.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK { Some(note_ctrl.get_value()) } else { None }
}

pub fn show_view_note_dialog(parent: &dyn WxWidget, note_text: &str) {
	let dialog = Dialog::builder(parent, &t("View Note")).build();
	let note_ctrl = TextCtrl::builder(&dialog)
		.with_value(note_text)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2)
		.with_size(Size::new(400, 200))
		.build();
	let close_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("Close")).build();
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let dialog_for_close = dialog;
	close_button.on_click(move |_| {
		dialog_for_close.end_modal(wxdragon::id::ID_OK);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&note_ctrl, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&close_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	note_ctrl.set_focus();
	dialog.show_modal();
}

pub fn show_toc_dialog(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	#[cfg(target_os = "linux")]
	return toc_gtk::show_toc_dialog(parent, toc_items, current_offset);
	#[cfg(not(target_os = "linux"))]
	return show_toc_dialog_wx(parent, toc_items, current_offset);
}

#[cfg(not(target_os = "linux"))]
fn show_toc_dialog_wx(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	let dialog_title = t("Table of Contents");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let selected_offset = Rc::new(Cell::new(-1));
	let (tree, _root) = build_toc_tree(dialog, toc_items, current_offset);
	bind_toc_selection(tree, Rc::clone(&selected_offset));
	bind_toc_activation(dialog, tree, Rc::clone(&selected_offset));
	bind_toc_search(tree);
	let (ok_button, cancel_button) = build_toc_buttons(dialog);
	bind_toc_ok(dialog, ok_button, Rc::clone(&selected_offset));
	bind_toc_layout(dialog, tree, ok_button, cancel_button);
	tree.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

#[cfg(not(target_os = "linux"))]
fn build_toc_tree(dialog: Dialog, toc_items: &[TocItem], current_offset: i32) -> (TreeCtrl, TreeItemId) {
	let tree = TreeCtrl::builder(&dialog)
		.with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HideRoot)
		.with_size(Size::new(400, 500))
		.build();
	let root = tree.add_root(&t("Root"), None, None).unwrap();
	populate_toc_tree(tree, &root, toc_items);
	if current_offset != -1 {
		find_and_select_item(tree, &root, current_offset);
	}
	(tree, root)
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_selection(tree: TreeCtrl, selected_offset: Rc<Cell<i32>>) {
	let tree_for_sel = tree;
	tree.on_selection_changed(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_sel.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i32>() {
					selected_offset.set(*offset);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_activation(dialog: Dialog, tree: TreeCtrl, selected_offset: Rc<Cell<i32>>) {
	let dialog_for_activate = dialog;
	let tree_for_activate = tree;
	tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_activate.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i32>() {
					selected_offset.set(*offset);
					dialog_for_activate.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_search(tree: TreeCtrl) {
	// Native Win32 first-letter navigation is used as-is. The only tweak needed is
	// preventing space from activating the OK button (space fires item_activated on
	// the tree, which our activation handler maps to OK).
	tree.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_SPACE {
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
}

#[cfg(not(target_os = "linux"))]
fn build_toc_buttons(dialog: Dialog) -> (Button, Button) {
	let ok_button = Button::builder(&dialog).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	(ok_button, cancel_button)
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_ok(dialog: Dialog, ok_button: Button, selected_offset: Rc<Cell<i32>>) {
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		if selected_offset.get() >= 0 {
			dialog_for_ok.end_modal(wxdragon::id::ID_OK);
		} else {
			MessageDialog::builder(
				&dialog_for_ok,
				&t("Please select a section from the table of contents."),
				&t("No Selection"),
			)
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
			.build()
			.show_modal();
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_layout(dialog: Dialog, tree: TreeCtrl, ok_button: Button, cancel_button: Button) {
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&tree, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::Bottom | SizerFlag::Right, DIALOG_PADDING);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}

#[cfg(not(target_os = "linux"))]
fn populate_toc_tree(tree: TreeCtrl, parent: &TreeItemId, items: &[TocItem]) {
	for item in items {
		let display_text = if item.name.is_empty() { t("Untitled") } else { item.name.clone() };
		let offset = i32::try_from(item.offset).unwrap_or(i32::MAX);
		if let Some(id) = tree.append_item_with_data(parent, &display_text, offset, None, None) {
			if !item.children.is_empty() {
				populate_toc_tree(tree, &id, &item.children);
			}
		}
	}
}

#[cfg(not(target_os = "linux"))]
fn find_and_select_item(tree: TreeCtrl, parent: &TreeItemId, offset: i32) -> bool {
	if let Some((child, mut cookie)) = tree.get_first_child(parent) {
		let mut current_child = Some(child);
		while let Some(item) = current_child {
			if let Some(data) = tree.get_custom_data(&item) {
				if let Some(item_offset) = data.downcast_ref::<i32>() {
					if *item_offset == offset {
						tree.select_item(&item);
						tree.set_focused_item(&item);
						tree.ensure_visible(&item);
						return true;
					}
				}
			}
			if find_and_select_item(tree, &item, offset) {
				return true;
			}
			current_child = tree.get_next_child(parent, &mut cookie);
		}
	}
	false
}

fn format_reading_time(word_count: usize, wpm: i32) -> String {
	if wpm <= 0 {
		return String::new();
	}
	let total_seconds = (word_count as f64 / wpm as f64 * 60.0).round() as u64;
	let hours = total_seconds / 3600;
	let minutes = (total_seconds % 3600) / 60;
	let seconds = total_seconds % 60;
	let mut parts: Vec<String> = Vec::new();
	if hours == 1 {
		parts.push(t("1 hour"));
	} else if hours > 1 {
		parts.push(format!("{} {}", hours, t("hours")));
	}
	if minutes == 1 {
		parts.push(t("1 minute"));
	} else if minutes > 1 {
		parts.push(format!("{} {}", minutes, t("minutes")));
	}
	if seconds == 1 {
		parts.push(t("1 second"));
	} else if seconds > 1 || total_seconds == 0 {
		parts.push(format!("{} {}", seconds, t("seconds")));
	}
	let time_str = parts.join(", ");
	let template = t("Estimated reading time: {}");
	template.replace("{}", &time_str)
}

pub fn show_word_count_dialog(parent: &Frame, word_count: usize, reading_speed_wpm: i32) {
	let words_template = t("This document contains {} words.");
	let mut msg = words_template.replace("{}", &word_count.to_string());
	let reading_time = format_reading_time(word_count, reading_speed_wpm);
	if !reading_time.is_empty() {
		msg.push('\n');
		msg.push_str(&reading_time);
	}
	let title = t("Word Count");
	let dialog = MessageDialog::builder(parent, &msg, &title).with_style(MessageDialogStyle::OK).build();
	dialog.show_modal();
}

pub fn show_document_info_dialog(parent: &Frame, path: &Path, title: &str, author: &str, stats: &DocumentStats) {
	let dialog_title = t("Document Info");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let info_ctrl = TextCtrl::builder(&dialog)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
		.with_size(Size::new(DOC_INFO_WIDTH, DOC_INFO_HEIGHT))
		.build();
	let path_label = t("Path:");
	let title_label = t("Title:");
	let author_label = t("Author:");
	let words_label = t("Words:");
	let lines_label = t("Lines:");
	let characters_label = t("Characters:");
	let characters_no_spaces_label = t("Characters (excluding spaces):");
	let mut info = String::new();
	let _ = writeln!(info, "{path_label} {}", path.display());
	if !title.is_empty() {
		let _ = writeln!(info, "{title_label} {title}");
	}
	if !author.is_empty() {
		let _ = writeln!(info, "{author_label} {author}");
	}
	let _ = writeln!(info, "{} {}", words_label, stats.word_count);
	let _ = writeln!(info, "{lines_label} {}", stats.line_count);
	let _ = writeln!(info, "{characters_label} {}", stats.char_count);
	let _ = writeln!(info, "{characters_no_spaces_label} {}", stats.char_count_no_whitespace);
	info_ctrl.set_value(&info);
	bind_escape_to_close(&dialog, dialog);
	bind_escape_to_close(&info_ctrl, dialog);
	let ok_label = t("OK");
	let ok_button = Button::builder(&dialog).with_label(&ok_label).build();
	bind_escape_to_close(&ok_button, dialog);
	let dialog_copy = dialog;
	ok_button.on_click(move |_| {
		dialog_copy.end_modal(wxdragon::id::ID_OK);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&info_ctrl, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	dialog.show_modal();
}

pub fn show_go_to_line_dialog(parent: &Frame, current_line: i32, max_lines: i32) -> Option<i32> {
	let dialog_title = t("Go to Line");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let label_text = t("&Line number:");
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	let max_lines = max_lines.max(1);
	let current_line = current_line.clamp(1, max_lines);
	let line_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, max_lines)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	line_ctrl.set_value(current_line);
	let dialog_for_enter = dialog;
	line_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(wxdragon::id::ID_OK);
	});
	let line_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	line_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	line_sizer.add(&line_ctrl, 1, SizerFlag::Expand, 0);
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	ok_button.set_default();
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&line_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	line_ctrl.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK { Some(line_ctrl.value().clamp(1, max_lines)) } else { None }
}

pub fn show_go_to_page_dialog(parent: &Frame, current_page: i32, max_page: i32) -> Option<i32> {
	let max_page = max_page.max(1);
	let dialog_title = t("Go to page");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let label_template = t("Go to page (%d/%d):");
	let label_text = label_template.replacen("%d", &current_page.clamp(1, max_page).to_string(), 1).replacen(
		"%d",
		&max_page.to_string(),
		1,
	);
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	let current = current_page.clamp(1, max_page);
	let page_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, max_page)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	page_ctrl.set_value(current);
	let dialog_for_enter = dialog;
	page_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(wxdragon::id::ID_OK);
	});
	let label_for_update = label;
	let label_template_for_update = label_template;
	page_ctrl.on_value_changed(move |event| {
		let text = label_template_for_update.replacen("%d", &event.get_value().to_string(), 1).replacen(
			"%d",
			&max_page.to_string(),
			1,
		);
		label_for_update.set_label(&text);
	});
	let page_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	page_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	page_sizer.add(&page_ctrl, 1, SizerFlag::Expand, 0);
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&page_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	page_ctrl.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK { Some(page_ctrl.value().clamp(1, max_page)) } else { None }
}

pub fn show_go_to_percent_dialog(parent: &Frame, current_percent: i32) -> Option<i32> {
	let dialog_title = t("Go to Percent");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let current_percent = current_percent.clamp(0, 100);
	let slider_label = StaticText::builder(&dialog).with_label(&t("&Percent")).build();
	let percent_slider =
		Slider::builder(&dialog).with_value(current_percent).with_min_value(0).with_max_value(100).build();
	let input_label = StaticText::builder(&dialog).with_label(&t("P&ercent:")).build();
	let input_ctrl = SpinCtrl::builder(&dialog)
		.with_range(0, 100)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	input_ctrl.set_value(current_percent);
	let input_ctrl_for_slider = input_ctrl;
	percent_slider.on_slider(move |event| {
		input_ctrl_for_slider.set_value(event.get_value());
	});
	let percent_slider_for_spin = percent_slider;
	input_ctrl.on_value_changed(move |event| {
		percent_slider_for_spin.set_value(event.get_value());
	});
	let dialog_for_enter = dialog;
	input_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(wxdragon::id::ID_OK);
	});
	let dialog_for_slider_enter = dialog;
	percent_slider.bind_internal(EventType::KEY_DOWN, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		if key == KEY_RETURN || key == KEY_NUMPAD_ENTER {
			event.skip(false);
			dialog_for_slider_enter.end_modal(wxdragon::id::ID_OK);
			return;
		}
		event.skip(true);
	});
	let percent_slider_for_keys = percent_slider;
	let input_ctrl_for_keys = input_ctrl;
	percent_slider.bind_internal(EventType::CHAR, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		let current = percent_slider_for_keys.value();
		let min_val = percent_slider_for_keys.min();
		let max_val = percent_slider_for_keys.max();
		let new_value = match key {
			WXK_UP | WXK_RIGHT => Some((current + 1).min(max_val)),
			WXK_DOWN | WXK_LEFT => Some((current - 1).max(min_val)),
			WXK_PAGEUP => Some((current + 10).min(max_val)),
			WXK_PAGEDOWN => Some((current - 10).max(min_val)),
			WXK_HOME => Some(min_val),
			WXK_END => Some(max_val),
			_ => None,
		};
		if let Some(val) = new_value {
			percent_slider_for_keys.set_value(val);
			input_ctrl_for_keys.set_value(val);
			event.skip(false);
		} else {
			event.skip(true);
		}
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&slider_label, 0, SizerFlag::Left | SizerFlag::Top, 5);
	content_sizer.add(&percent_slider, 0, SizerFlag::Expand | SizerFlag::Bottom, 5);
	content_sizer.add(&input_label, 0, SizerFlag::Left, 5);
	content_sizer.add(&input_ctrl, 0, SizerFlag::Expand, 0);
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	ok_button.set_default();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
	main_sizer.add_sizer(&content_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	main_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(main_sizer, true);
	dialog.centre();
	percent_slider.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK { Some(input_ctrl.value().clamp(0, 100)) } else { None }
}

fn show_yes_no_dialog(parent: &dyn WxWidget, message: &str, title: &str) -> bool {
	let dialog = Dialog::builder(parent, title).build();
	let panel = Panel::builder(&dialog).build();
	let message_label = StaticText::builder(&panel).with_label(message).build();
	let yes_button = Button::builder(&panel).with_id(wxdragon::id::ID_OK).with_label(&t("&Yes")).build();
	let no_button = Button::builder(&panel).with_id(wxdragon::id::ID_CANCEL).with_label(&t("&No")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&message_label, 0, SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&yes_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&no_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::All, 0);
	panel.set_sizer(content_sizer, true);
	let dialog_sizer = BoxSizer::builder(Orientation::Vertical).build();
	dialog_sizer.add(&panel, 1, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(dialog_sizer, true);
	dialog.centre();
	dialog.show_modal() == wxdragon::id::ID_OK
}

pub fn show_update_dialog(parent: &dyn WxWidget, new_version: &str, changelog: &str) -> bool {
	let dialog_title = t("Update to %s").replace("%s", new_version);
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let panel = Panel::builder(&dialog).build();
	let message = StaticText::builder(&panel)
		.with_label(&t("A new version of Paperback is available. Here's what's new:"))
		.build();
	let changelog_ctrl = TextCtrl::builder(&panel)
		.with_value(changelog)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2)
		.with_size(Size::new(500, 300))
		.build();
	let yes_button = Button::builder(&panel).with_id(wxdragon::id::ID_OK).with_label(&t("&Yes")).build();
	let no_button = Button::builder(&panel).with_id(wxdragon::id::ID_CANCEL).with_label(&t("&No")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&message, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(
		&changelog_ctrl,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&yes_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&no_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::All, 0);
	panel.set_sizer(content_sizer, true);
	let dialog_sizer = BoxSizer::builder(Orientation::Vertical).build();
	dialog_sizer.add(&panel, 1, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(dialog_sizer, true);
	dialog.centre();
	dialog.raise();
	changelog_ctrl.set_focus();
	dialog.show_modal() == wxdragon::id::ID_OK
}

pub struct AllDocumentsResult {
	pub open: Option<String>,
	pub paths_to_close: Vec<String>,
}

pub fn show_all_documents_dialog(
	parent: &Frame,
	config: &Rc<Mutex<ConfigManager>>,
	open_paths: Vec<String>,
) -> AllDocumentsResult {
	let open_paths = Rc::new(open_paths);
	let dialog_title = t("All Documents");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let selected_path = Rc::new(Mutex::new(None));
	let paths_to_close: Rc<Mutex<Vec<String>>> = Rc::new(Mutex::new(Vec::new()));
	let search_label = StaticText::builder(&dialog).with_label(&t("&search")).build();
	let search_ctrl = TextCtrl::builder(&dialog).with_size(Size::new(300, -1)).build();
	let doc_list = build_all_documents_list(dialog);
	let (open_button, remove_button, clear_all_button, ok_button) = build_all_documents_buttons(dialog);

	bind_escape_to_close(&open_button, dialog);
	bind_escape_to_close(&remove_button, dialog);
	bind_escape_to_close(&clear_all_button, dialog);
	bind_escape_to_close(&ok_button, dialog);
	bind_escape_to_close(&dialog, dialog);
	bind_escape_to_close(&search_ctrl, dialog);
	bind_escape_to_close(&doc_list, dialog);

	populate_document_list(&DocumentListParams {
		list: doc_list,
		open_button,
		remove_button,
		clear_all_button,
		config,
		open_paths: open_paths.as_ref(),
		filter: "",
		selection: None,
	});

	bind_all_documents_selection(doc_list, open_button);
	let open_action = make_all_documents_open_action(dialog, doc_list, Rc::clone(&selected_path));
	bind_all_documents_open(doc_list, open_button, &open_action);

	let remove_action = make_all_documents_remove_action(
		dialog,
		doc_list,
		search_ctrl,
		open_button,
		remove_button,
		clear_all_button,
		Rc::clone(config),
		Rc::clone(&open_paths),
		Rc::clone(&paths_to_close),
	);
	remove_button.on_click({
		let remove_action = Rc::clone(&remove_action);
		move |_| remove_action()
	});

	bind_all_documents_clear(
		dialog,
		doc_list,
		search_ctrl,
		open_button,
		remove_button,
		clear_all_button,
		Rc::clone(config),
		Rc::clone(&open_paths),
		Rc::clone(&paths_to_close),
	);
	bind_all_documents_search(
		search_ctrl,
		doc_list,
		open_button,
		remove_button,
		clear_all_button,
		Rc::clone(config),
		Rc::clone(&open_paths),
	);
	bind_all_documents_keys(doc_list, &open_action, &remove_action);
	bind_all_documents_layout(
		dialog,
		AllDocumentsLayout {
			search_label,
			search_ctrl,
			doc_list,
			open_button,
			remove_button,
			clear_all_button,
			ok_button,
		},
	);

	dialog.show_modal();
	AllDocumentsResult {
		open: selected_path.lock().unwrap().clone(),
		paths_to_close: paths_to_close.lock().unwrap().clone(),
	}
}

fn build_all_documents_list(dialog: Dialog) -> ListCtrl {
	let doc_list = ListCtrl::builder(&dialog)
		.with_style(ListCtrlStyle::Report)
		.with_size(Size::new(RECENT_DOCS_LIST_WIDTH, RECENT_DOCS_LIST_HEIGHT))
		.build();
	doc_list.insert_column(0, &t("File Name"), ListColumnFormat::Left, RECENT_DOCS_FILENAME_WIDTH);
	doc_list.insert_column(1, &t("Status"), ListColumnFormat::Left, RECENT_DOCS_STATUS_WIDTH);
	doc_list.insert_column(2, &t("Path"), ListColumnFormat::Left, RECENT_DOCS_PATH_WIDTH);
	doc_list
}

fn build_all_documents_buttons(dialog: Dialog) -> (Button, Button, Button, Button) {
	let open_button = Button::builder(&dialog).with_label(&t("&Open")).build();
	let remove_button = Button::builder(&dialog).with_label(&t("&Remove")).build();
	let clear_all_button = Button::builder(&dialog).with_label(&t("&Clear All")).build();
	let ok_button = Button::builder(&dialog).with_label(&t("OK")).build();
	(open_button, remove_button, clear_all_button, ok_button)
}

fn bind_all_documents_selection(list: ListCtrl, open_button: Button) {
	let list_for_select = list;
	let open_button_for_select = open_button;
	list.on_item_selected(move |event| {
		let index = event.get_item_index();
		update_open_button_for_index(list_for_select, open_button_for_select, index);
	});
	let list_for_focus = list;
	let open_button_for_focus = open_button;
	list.on_item_focused(move |event| {
		let index = event.get_item_index();
		if index >= 0 {
			update_open_button_for_index(list_for_focus, open_button_for_focus, index);
		}
	});
}

fn make_all_documents_open_action(
	dialog: Dialog,
	list: ListCtrl,
	selected_path: Rc<Mutex<Option<String>>>,
) -> Rc<dyn Fn()> {
	Rc::new(move || {
		if let Some(path) = get_selected_path(list) {
			if Path::new(&path).exists() {
				*selected_path.lock().unwrap() = Some(path);
				dialog.end_modal(wxdragon::id::ID_OK);
			}
		}
	})
}

fn bind_all_documents_open(list: ListCtrl, open_button: Button, open_action: &Rc<dyn Fn()>) {
	let open_action_for_button = Rc::clone(open_action);
	open_button.on_click(move |_| {
		open_action_for_button();
	});
	let open_action_for_activate = Rc::clone(open_action);
	list.on_item_activated(move |event| {
		if event.get_item_index() >= 0 {
			open_action_for_activate();
		}
	});
}

fn make_all_documents_remove_action(
	dialog: Dialog,
	list: ListCtrl,
	search_ctrl: TextCtrl,
	open_button: Button,
	remove_button: Button,
	clear_button: Button,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
	paths_to_close: Rc<Mutex<Vec<String>>>,
) -> Rc<dyn Fn()> {
	Rc::new(move || {
		let indices = get_selected_indices(list);
		if indices.is_empty() {
			return;
		}
		let confirm_message = if indices.len() == 1 {
			t(
				"Are you sure you want to remove this document from the list? This will also remove its reading position.",
			)
		} else {
			let template = t(
				"Are you sure you want to remove these {} documents from the list? This will also remove their reading positions.",
			);
			template.replace("{}", &indices.len().to_string())
		};
		if !show_yes_no_dialog(&dialog, &confirm_message, &t("Confirm")) {
			return;
		}
		let paths_to_remove: Vec<String> = indices.iter().filter_map(|&i| get_path_for_index(list, i)).collect();
		{
			let cfg = config.lock().unwrap();
			for path in &paths_to_remove {
				cfg.remove_document_history(path);
			}
			cfg.flush();
		}
		{
			let mut to_close = paths_to_close.lock().unwrap();
			for path in &paths_to_remove {
				if open_paths.contains(path) && !to_close.contains(path) {
					to_close.push(path.clone());
				}
			}
		}
		let new_selection = indices.iter().copied().max();
		let filter = search_ctrl.get_value();
		populate_document_list(&DocumentListParams {
			list,
			open_button,
			remove_button,
			clear_all_button: clear_button,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: &filter,
			selection: new_selection,
		});
	})
}

fn bind_all_documents_clear(
	dialog: Dialog,
	list: ListCtrl,
	search_ctrl: TextCtrl,
	open_button: Button,
	remove_button: Button,
	clear_button: Button,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
	paths_to_close: Rc<Mutex<Vec<String>>>,
) {
	clear_button.on_click(move |_| {
		if list.get_item_count() == 0 {
			return;
		}
		if !show_yes_no_dialog(
			&dialog,
			&t("Are you sure you want to remove all documents from the list? This will also remove all reading positions and bookmarks."),
			&t("Confirm"),
		) {
			return;
		}
		{
			let cfg = config.lock().unwrap();
			let all_docs = cfg.get_all_documents();
			{
				let mut to_close = paths_to_close.lock().unwrap();
				for path in &all_docs {
					if open_paths.contains(path) && !to_close.contains(path) {
						to_close.push(path.clone());
					}
				}
			}
			for path in &all_docs {
				cfg.remove_document_history(path);
			}
			cfg.flush();
		}
		search_ctrl.set_value("");
		populate_document_list(&DocumentListParams {
			list,
			open_button,
			remove_button,
			clear_all_button: clear_button,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: "",
			selection: None,
		});
	});
}

fn bind_all_documents_search(
	search_ctrl: TextCtrl,
	list: ListCtrl,
	open_button: Button,
	remove_button: Button,
	clear_button: Button,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
) {
	search_ctrl.on_text_updated(move |_event| {
		let filter = search_ctrl.get_value();
		populate_document_list(&DocumentListParams {
			list,
			open_button,
			remove_button,
			clear_all_button: clear_button,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: &filter,
			selection: None,
		});
	});
}

fn bind_all_documents_keys(list: ListCtrl, open_action: &Rc<dyn Fn()>, remove_action: &Rc<dyn Fn()>) {
	let remove_action_for_keys = Rc::clone(remove_action);
	let open_action_for_keys = Rc::clone(open_action);
	let list_for_keys = list;
	list.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_DELETE || key == KEY_NUMPAD_DELETE {
				remove_action_for_keys();
				event.skip(false);
				return;
			}
			if key == KEY_RETURN || key == KEY_NUMPAD_ENTER {
				open_action_for_keys();
				event.skip(false);
				return;
			}
			if key == i32::from(b'A') && event.control_down() {
				list_for_keys.set_item_state(-1, ListItemState::Selected, ListItemState::Selected);
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
	let open_action_for_char = Rc::clone(open_action);
	list.bind_internal(EventType::CHAR, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_RETURN || key == KEY_NUMPAD_ENTER {
				open_action_for_char();
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
}

#[derive(Copy, Clone)]
struct AllDocumentsLayout {
	search_label: StaticText,
	search_ctrl: TextCtrl,
	doc_list: ListCtrl,
	open_button: Button,
	remove_button: Button,
	clear_all_button: Button,
	ok_button: Button,
}

fn bind_all_documents_layout(dialog: Dialog, layout: AllDocumentsLayout) {
	let AllDocumentsLayout {
		search_label,
		search_ctrl,
		doc_list,
		open_button,
		remove_button,
		clear_all_button,
		ok_button,
	} = layout;
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		dialog_for_ok.end_modal(wxdragon::id::ID_OK);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let search_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	search_sizer.add(&search_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	search_sizer.add(&search_ctrl, 1, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING / 2);
	content_sizer.add_sizer(&search_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(&doc_list, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	doc_list.set_focus();
	let action_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	action_sizer.add(&open_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&remove_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&clear_all_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(
		&action_sizer,
		0,
		SizerFlag::AlignLeft | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	let ok_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	ok_sizer.add_stretch_spacer(1);
	ok_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&ok_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}

pub fn show_open_as_dialog(parent: &Frame, path: &Path) -> Option<String> {
	let title = t("Open As");
	let dialog = Dialog::builder(parent, &title).build();
	let message_template = t("No suitable parser was found for {}.\nHow would you like to open this file?");
	let message = message_template.replace("{}", &path.display().to_string());
	let label = StaticText::builder(&dialog).with_label(&message).build();
	let format_label_text = t("Open &as:");
	let format_label = StaticText::builder(&dialog).with_label(&format_label_text).build();
	let format_combo = ComboBox::builder(&dialog).with_style(ComboBoxStyle::ReadOnly).build();
	format_combo.append(&t("Plain Text"));
	format_combo.append(&t("HTML"));
	format_combo.append(&t("Markdown"));
	format_combo.set_selection(0);
	let ok_label = t("OK");
	let ok_button = Button::builder(&dialog).with_label(&ok_label).build();
	let cancel_label = t("Cancel");
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&cancel_label).build();
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		dialog_for_ok.end_modal(wxdragon::id::ID_OK);
	});
	let dialog_for_cancel = dialog;
	cancel_button.on_click(move |_| {
		dialog_for_cancel.end_modal(wxdragon::id::ID_CANCEL);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&label, 0, SizerFlag::All, DIALOG_PADDING / 2);
	let format_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	format_sizer.add(&format_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	format_sizer.add(&format_combo, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&format_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING / 2);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	format_combo.set_focus();
	if dialog.show_modal() != wxdragon::id::ID_OK {
		return None;
	}
	let selection = format_combo.get_selection();
	let format = match selection {
		Some(1) => "html",
		Some(2) => "md",
		_ => "txt",
	};
	Some(format.to_string())
}

struct DocumentListParams<'a> {
	list: ListCtrl,
	open_button: Button,
	remove_button: Button,
	clear_all_button: Button,
	config: &'a Rc<Mutex<ConfigManager>>,
	open_paths: &'a [String],
	filter: &'a str,
	selection: Option<i32>,
}

fn populate_document_list(params: &DocumentListParams<'_>) {
	let DocumentListParams {
		list,
		open_button,
		remove_button,
		clear_all_button,
		config,
		open_paths,
		filter,
		selection,
	} = *params;
	list.cleanup_all_custom_data();
	list.delete_all_items();

	let items = {
		let cfg = config.lock().unwrap();
		crate::config::get_sorted_document_list(&cfg, open_paths, filter)
	};

	for item in items {
		let index = i64::from(list.get_item_count());
		list.insert_item(index, &item.filename, None);
		if let Ok(index_u64) = u64::try_from(index) {
			list.set_custom_data(index_u64, item.path.clone());
		}
		let status = match item.status {
			DocumentListStatus::Open => t("Open"),
			DocumentListStatus::Closed => t("Closed"),
			DocumentListStatus::Missing => t("Missing"),
		};
		list.set_item_text_by_column(index, 1, &status);
		list.set_item_text_by_column(index, 2, &item.path);
	}

	if list.get_item_count() > 0 {
		let mut select_index = selection.unwrap_or(0);
		if select_index >= list.get_item_count() {
			select_index = list.get_item_count() - 1;
		}
		list.set_item_state(
			i64::from(select_index),
			ListItemState::Selected | ListItemState::Focused,
			ListItemState::Selected | ListItemState::Focused,
		);
		list.ensure_visible(i64::from(select_index));
		update_open_button_for_index(list, open_button, select_index);
		remove_button.enable(true);
		clear_all_button.enable(true);
	} else {
		open_button.enable(false);
		remove_button.enable(false);
		clear_all_button.enable(false);
	}
}

fn update_open_button_for_index(list: ListCtrl, open_button: Button, index: i32) {
	if index < 0 {
		open_button.enable(false);
		return;
	}
	let status = list.get_item_text(i64::from(index), 1);
	open_button.enable(status != t("Missing"));
}

fn bind_escape_to_close(handler: &impl WxEvtHandler, dialog: Dialog) {
	let dialog_for_escape = dialog;
	handler.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_ESCAPE {
				dialog_for_escape.end_modal(wxdragon::id::ID_CANCEL);
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
	let dialog_for_escape = dialog;
	handler.bind_internal(EventType::CHAR, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_ESCAPE {
				dialog_for_escape.end_modal(wxdragon::id::ID_CANCEL);
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
}

fn get_selected_index(list: ListCtrl) -> i32 {
	let selected = list.get_first_selected_item();
	if selected >= 0 {
		return selected;
	}
	list.get_next_item(-1, ListNextItemFlag::All, ListItemState::Focused)
}

fn get_selected_indices(list: ListCtrl) -> Vec<i32> {
	let mut indices = Vec::new();
	let mut next = list.get_first_selected_item();
	while next >= 0 {
		indices.push(next);
		next = list.get_next_item(i64::from(next), ListNextItemFlag::All, ListItemState::Selected);
	}
	indices
}

fn get_path_for_index(list: ListCtrl, index: i32) -> Option<String> {
	if index < 0 {
		return None;
	}
	if let Ok(index_u64) = u64::try_from(index) {
		if let Some(data) = list.get_custom_data(index_u64) {
			if let Some(path) = data.as_ref().downcast_ref::<String>() {
				return Some(path.clone());
			}
		}
	}
	let path = list.get_item_text(i64::from(index), 2);
	if path.is_empty() { None } else { Some(path) }
}

fn get_selected_path(list: ListCtrl) -> Option<String> {
	let index = get_selected_index(list);
	get_path_for_index(list, index)
}

pub fn show_sleep_timer_dialog(parent: &Frame, initial_duration: i32) -> Option<i32> {
	let dialog = Dialog::builder(parent, &t("Sleep Timer")).build();
	let label = StaticText::builder(&dialog).with_label(&t("&Minutes:")).build();
	let input_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, 999)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	input_ctrl.set_value(initial_duration.clamp(1, 999));
	let dialog_for_enter = dialog;
	input_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(wxdragon::id::ID_OK);
	});
	let input_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	input_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	input_sizer.add(&input_ctrl, 1, SizerFlag::Expand, 0);
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	ok_button.set_default();
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&input_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	input_ctrl.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK { Some(input_ctrl.value()) } else { None }
}

pub fn show_web_view_dialog(
	parent: &Frame,
	title: &str,
	url_or_content: &str,
	is_url: bool,
	navigation_handler: Option<NavigationHandler>,
) {
	let dialog = Dialog::builder(parent, title)
		.with_style(DialogStyle::DefaultDialogStyle | DialogStyle::ResizeBorder)
		.with_size(800, 600)
		.build();
	let web_view = WebView::builder(&dialog).build();
	web_view.add_script_message_handler("wx");
	let dialog_for_close = dialog;
	web_view.on_script_message_received(move |event| {
		if event.get_string() == Some("close_dialog".to_string()) {
			dialog_for_close.end_modal(wxdragon::id::ID_CANCEL);
		}
	});
	if let Some(handler) = navigation_handler {
		web_view.on_navigating(move |event| {
			if let Some(url) = event.get_string() {
				let url_str: String = url;
				if !handler(&url_str) {
					event.event.event.veto();
				}
			}
		});
	}
	if is_url {
		web_view.load_url(url_or_content);
	} else {
		let full_html = if url_or_content.to_lowercase().contains("<html") {
			url_or_content.to_string()
		} else {
			format!("<html><head><title>{title}</title></head><body>{url_or_content}</body></html>")
		};
		web_view.set_page(&full_html, "");
	}
	let web_view_for_load = web_view;
	web_view.on_loaded(move |_| {
		web_view_for_load.run_script(
			"document.addEventListener('keydown', function(event) { \
             if (event.key === 'Escape' || event.keyCode === 27) { \
             window.wx.postMessage('close_dialog'); \
             } \
             });",
		);
	});
	let close_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Close")).build();
	let dialog_for_ok = dialog;
	close_button.on_click(move |_| {
		dialog_for_ok.end_modal(wxdragon::id::ID_OK);
	});
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	let sizer = BoxSizer::builder(Orientation::Vertical).build();
	sizer.add(&web_view, 1, SizerFlag::Expand | SizerFlag::All, 5);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&close_button, 0, SizerFlag::All, 5);
	sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer(sizer, true);
	dialog.centre();
	dialog.show_modal();
}

pub fn show_elements_dialog(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	#[cfg(target_os = "linux")]
	return elements_gtk::show_elements_dialog(parent, session, current_pos);
	#[cfg(not(target_os = "linux"))]
	return show_elements_dialog_wx(parent, session, current_pos);
}

#[cfg(not(target_os = "linux"))]
fn show_elements_dialog_wx(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	let dialog = Dialog::builder(parent, &t("Elements")).build();
	let ElementsDialogUi { content_sizer, view_choice, headings_tree, links_list } = build_elements_dialog_ui(dialog);
	let (selected_offset, link_offsets) = populate_elements_dialog(session, current_pos, headings_tree, links_list);
	bind_elements_view_toggle(view_choice, headings_tree, links_list, dialog);
	bind_elements_activation(dialog, headings_tree, links_list, &selected_offset, &link_offsets);
	let (ok_button, cancel_button) = build_elements_buttons(dialog);
	bind_elements_ok_action(dialog, view_choice, headings_tree, links_list, &link_offsets, &selected_offset, ok_button);
	finalize_elements_layout(dialog, content_sizer, ok_button, cancel_button);
	if view_choice.get_selection().unwrap_or(0) == 0 {
		headings_tree.set_focus();
	} else {
		links_list.set_focus();
	}
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

#[cfg(not(target_os = "linux"))]
struct ElementsDialogUi {
	content_sizer: BoxSizer,
	view_choice: ComboBox,
	headings_tree: TreeCtrl,
	links_list: ListBox,
}

#[cfg(not(target_os = "linux"))]
fn build_elements_dialog_ui(dialog: Dialog) -> ElementsDialogUi {
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let choice_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	let choice_label = StaticText::builder(&dialog).with_label(&t("&View:")).build();
	let view_choice = ComboBox::builder(&dialog).with_style(ComboBoxStyle::ReadOnly).build();
	view_choice.append(&t("Headings"));
	view_choice.append(&t("Links"));
	view_choice.set_selection(0);
	choice_sizer.add(&choice_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	choice_sizer.add(&view_choice, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&choice_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let headings_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let headings_tree = TreeCtrl::builder(&dialog)
		.with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HideRoot)
		.with_size(Size::new(400, 500))
		.build();
	headings_sizer.add(&headings_tree, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(
		&headings_sizer,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	let links_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let links_list = ListBox::builder(&dialog).build();
	links_sizer.add(&links_list, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(
		&links_sizer,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	links_list.show(false);
	ElementsDialogUi { content_sizer, view_choice, headings_tree, links_list }
}

#[cfg(not(target_os = "linux"))]
fn populate_elements_dialog(
	session: &DocumentSession,
	current_pos: i64,
	headings_tree: TreeCtrl,
	links_list: ListBox,
) -> (Rc<Cell<i64>>, Rc<Vec<i64>>) {
	let selected_offset = Rc::new(Cell::new(-1i64));
	let root = headings_tree.add_root(&t("Root"), None, None).unwrap();
	let tree_data = session.heading_tree(current_pos);
	let mut item_ids: Vec<wxdragon::widgets::treectrl::TreeItemId> = Vec::new();
	if !tree_data.items.is_empty() {
		item_ids.reserve(tree_data.items.len());
	}
	for item in &tree_data.items {
		let parent_id = if item.parent_index >= 0 {
			usize::try_from(item.parent_index)
				.ok()
				.and_then(|idx| item_ids.get(idx).cloned())
				.unwrap_or_else(|| root.clone())
		} else {
			root.clone()
		};
		let display_text = if item.text.is_empty() { t("Untitled") } else { item.text.clone() };
		let offset = i64::try_from(item.offset).unwrap_or(i64::MAX);
		if let Some(id) = headings_tree.append_item_with_data(&parent_id, &display_text, offset, None, None) {
			item_ids.push(id);
		} else if let Some(root_child) = headings_tree.append_item_with_data(&root, &display_text, offset, None, None) {
			item_ids.push(root_child);
		}
	}
	headings_tree.expand_all();
	if tree_data.closest_index >= 0 {
		if let Ok(index) = usize::try_from(tree_data.closest_index) {
			if let Some(item) = item_ids.get(index) {
				headings_tree.select_item(item);
				headings_tree.ensure_visible(item);
			}
		}
	} else if let Some((first_child, _)) = headings_tree.get_first_child(&root) {
		headings_tree.select_item(&first_child);
		headings_tree.ensure_visible(&first_child);
	}
	let link_data = session.link_list(current_pos);
	let mut link_offsets = Vec::new();
	for item in link_data.items {
		links_list.append(&item.text);
		link_offsets.push(i64::try_from(item.offset).unwrap_or(i64::MAX));
	}
	if !link_offsets.is_empty() {
		let idx = if link_data.closest_index >= 0 { link_data.closest_index } else { 0 };
		if let Ok(idx_u32) = u32::try_from(idx) {
			links_list.set_selection(idx_u32, true);
		}
	}
	(selected_offset, Rc::new(link_offsets))
}

#[cfg(not(target_os = "linux"))]
fn bind_elements_view_toggle(view_choice: ComboBox, headings_tree: TreeCtrl, links_list: ListBox, dialog: Dialog) {
	let headings_tree_for_choice = headings_tree;
	let links_list_for_choice = links_list;
	let dialog_for_layout = dialog;
	view_choice.on_selection_changed(move |_| {
		let selection = view_choice.get_selection().unwrap_or(0);
		if selection == 0 {
			headings_tree_for_choice.show(true);
			links_list_for_choice.show(false);
			headings_tree_for_choice.set_focus();
		} else {
			headings_tree_for_choice.show(false);
			links_list_for_choice.show(true);
			links_list_for_choice.set_focus();
		}
		dialog_for_layout.layout();
	});
}

#[cfg(not(target_os = "linux"))]
fn bind_elements_activation(
	dialog: Dialog,
	headings_tree: TreeCtrl,
	links_list: ListBox,
	selected_offset: &Rc<Cell<i64>>,
	link_offsets: &Rc<Vec<i64>>,
) {
	let selected_offset_for_tree = Rc::clone(selected_offset);
	let tree_for_activate = headings_tree;
	let dialog_for_tree = dialog;
	headings_tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_activate.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i64>() {
					selected_offset_for_tree.set(*offset);
					dialog_for_tree.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
	let selected_offset_for_list = Rc::clone(selected_offset);
	let offsets_for_list = Rc::clone(link_offsets);
	let dialog_for_list = dialog;
	links_list.on_item_double_clicked(move |event| {
		let selection = event.get_selection().unwrap_or(-1);
		if selection >= 0 {
			if let Ok(index) = usize::try_from(selection) {
				if let Some(offset) = offsets_for_list.get(index) {
					selected_offset_for_list.set(*offset);
					dialog_for_list.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn build_elements_buttons(dialog: Dialog) -> (Button, Button) {
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	ok_button.set_default();
	(ok_button, cancel_button)
}

#[cfg(not(target_os = "linux"))]
fn bind_elements_ok_action(
	dialog: Dialog,
	view_choice: ComboBox,
	headings_tree: TreeCtrl,
	links_list: ListBox,
	link_offsets: &Rc<Vec<i64>>,
	selected_offset: &Rc<Cell<i64>>,
	ok_button: Button,
) {
	let offsets_for_ok = Rc::clone(link_offsets);
	let selected_offset_for_ok = Rc::clone(selected_offset);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		let selection = view_choice.get_selection().unwrap_or(0);
		if selection == 0 {
			if let Some(item) = headings_tree.get_selection() {
				if let Some(data) = headings_tree.get_custom_data(&item) {
					if let Some(offset) = data.downcast_ref::<i64>() {
						selected_offset_for_ok.set(*offset);
						dialog_for_ok.end_modal(wxdragon::id::ID_OK);
					}
				}
			}
		} else if let Some(idx) = links_list.get_selection() {
			if let Ok(index) = usize::try_from(idx) {
				if let Some(offset) = offsets_for_ok.get(index) {
					selected_offset_for_ok.set(*offset);
					dialog_for_ok.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn finalize_elements_layout(dialog: Dialog, content_sizer: BoxSizer, ok_button: Button, cancel_button: Button) {
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}

pub fn show_about_dialog(parent: &Frame) {
	let name = CString::new("Paperback").unwrap_or_else(|_| CString::new("").unwrap());
	let version = CString::new(env!("CARGO_PKG_VERSION")).unwrap_or_else(|_| CString::new("").unwrap());
	let description = CString::new(t("An accessible, lightweight, fast ebook and document reader"))
		.unwrap_or_else(|_| CString::new("").unwrap());
	let copyright = CString::new("Copyright (C) 2025-2026 Quin Gillespie. All rights reserved.")
		.unwrap_or_else(|_| CString::new("").unwrap());
	let website = CString::new("https://paperback.dev").unwrap_or_else(|_| CString::new("").unwrap());
	unsafe {
		let info = ffi::wxd_AboutDialogInfo_Create();
		if info.is_null() {
			return;
		}
		ffi::wxd_AboutDialogInfo_SetName(info, name.as_ptr());
		ffi::wxd_AboutDialogInfo_SetVersion(info, version.as_ptr());
		ffi::wxd_AboutDialogInfo_SetDescription(info, description.as_ptr());
		ffi::wxd_AboutDialogInfo_SetCopyright(info, copyright.as_ptr());
		ffi::wxd_AboutDialogInfo_SetWebSite(info, website.as_ptr());
		ffi::wxd_AboutBox(info, parent.handle_ptr());
		ffi::wxd_AboutDialogInfo_Destroy(info);
	}
}
