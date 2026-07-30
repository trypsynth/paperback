use std::{
	cell::{Cell, RefCell},
	fmt::Write,
	mem,
	rc::Rc,
};

use paperback_core::config::{ConfigManager, HotkeyConfig, ReadabilityFont};
use patois::{t, ui::populate_language_choice};
#[cfg(target_os = "windows")]
use wxdragon::accessible::AccRole;
use wxdragon::prelude::*;

use super::DIALOG_PADDING;
use crate::{
	config_ext::{UpdateChannel, get_update_channel},
	translation_manager::TranslationManager,
};

#[derive(Clone, Debug)]
pub struct OptionsDialogResult {
	pub restore_previous_documents: bool,
	pub word_wrap: bool,
	pub render_tables_inline: bool,
	pub minimize_to_tray: bool,
	pub start_maximized: bool,
	pub compact_go_menu: bool,
	pub navigation_wrap: bool,
	pub check_for_updates_on_startup: bool,
	pub bookmark_sounds: bool,
	pub recent_documents_to_show: i32,
	pub reading_speed_wpm: i32,
	pub language: String,
	pub update_channel: UpdateChannel,
	pub hotkey: HotkeyConfig,
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
	render_tables_inline_check: CheckBox,
	minimize_to_tray_check: CheckBox,
	start_maximized_check: CheckBox,
	compact_go_menu_check: CheckBox,
	navigation_wrap_check: CheckBox,
	check_for_updates_check: CheckBox,
	bookmark_sounds_check: CheckBox,
	recent_docs_ctrl: SpinCtrl,
	reading_speed_ctrl: SpinCtrl,
	language_combo: Choice,
	update_channel_combo: Choice,
	language_codes: Vec<String>,
	current_language: String,
	ok_button: Button,
	cancel_button: Button,
	hotkey: Rc<RefCell<HotkeyConfig>>,
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
	if ui.dialog.show_modal() != ID_OK {
		return None;
	}
	let language = resolve_options_language(&ui);
	let update_channel = match ui.update_channel_combo.get_selection() {
		Some(1) => UpdateChannel::Dev,
		_ => UpdateChannel::Stable,
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
		render_tables_inline: ui.render_tables_inline_check.is_checked(),
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
		hotkey: ui.hotkey.borrow().clone(),
		readability_font,
		line_spacing,
		bg_color,
		text_alignment,
		letter_spacing,
		paragraph_spacing,
	})
}

fn build_options_dialog_ui(parent: &Frame, config: &ConfigManager) -> OptionsDialogUi {
	// TRANSLATORS: Title of the Options dialog
	let dialog = Dialog::builder(parent, &t("Options")).build();
	let notebook = Notebook::builder(&dialog).with_style(NotebookStyle::Top).build();
	let general_panel = Panel::builder(&notebook).build();
	let reading_panel = Panel::builder(&notebook).build();
	let readability_panel = Panel::builder(&notebook).build();
	let general_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let reading_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let readability_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let restore_docs_check =
		// TRANSLATORS: Option to restore documents that were open when the app was last closed
		CheckBox::builder(&general_panel).with_label(&t("&Restore previously opened documents on startup")).build();
	// TRANSLATORS: Option to toggle word wrapping of text
	let word_wrap_check = CheckBox::builder(&readability_panel).with_label(&t("&Word wrap")).build();
	let render_tables_inline_check =
		// TRANSLATORS: Option to render tables inline rather than showing a placeholder link
		CheckBox::builder(&readability_panel).with_label(&t("Render tables &inline")).build();
	// TRANSLATORS: Option to minimize the app window to the system tray instead of the taskbar
	let minimize_to_tray_check = CheckBox::builder(&general_panel).with_label(&t("&Minimize to system tray")).build();
	// TRANSLATORS: Option to start the app maximized
	let start_maximized_check = CheckBox::builder(&general_panel).with_label(&t("&Start maximized")).build();
	// TRANSLATORS: Option to show a compact Go navigation menu in the menu bar
	let compact_go_menu_check = CheckBox::builder(&reading_panel).with_label(&t("Show compact &go menu")).build();
	// TRANSLATORS: Option to wrap navigation around to the beginning/end when navigating elements
	let navigation_wrap_check = CheckBox::builder(&reading_panel).with_label(&t("&Wrap navigation")).build();
	let bookmark_sounds_check =
		// TRANSLATORS: Option to play sound effects when bookmarks or notes are encountered
		CheckBox::builder(&reading_panel).with_label(&t("Play &sounds on bookmarks and notes")).build();
	let check_for_updates_check =
		// TRANSLATORS: Option to check for app updates automatically on startup
		CheckBox::builder(&general_panel).with_label(&t("Check for &updates on startup")).build();
	// TRANSLATORS: Button label to open the hotkey customization dialog
	let hotkey_button = Button::builder(&general_panel).with_label(&t("Customize &Window Hotkey...")).build();
	let option_padding = 5;
	general_sizer.add(&restore_docs_check, 0, SizerFlag::All, option_padding);
	general_sizer.add(&start_maximized_check, 0, SizerFlag::All, option_padding);
	#[cfg(not(target_os = "macos"))]
	general_sizer.add(&minimize_to_tray_check, 0, SizerFlag::All, option_padding);
	general_sizer.add(&check_for_updates_check, 0, SizerFlag::All, option_padding);
	general_sizer.add(&hotkey_button, 0, SizerFlag::All, option_padding);
	for check in [&navigation_wrap_check, &compact_go_menu_check, &bookmark_sounds_check] {
		reading_sizer.add(check, 0, SizerFlag::All, option_padding);
	}
	let reading_speed_label =
		// TRANSLATORS: Label for the reading speed input field (Words Per Minute)
		StaticText::builder(&reading_panel).with_label(&t("&Reading speed (words per minute):")).build();
	let reading_speed_ctrl = SpinCtrl::builder(&reading_panel).with_range(1, 2000).build();
	let reading_speed_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	reading_speed_sizer.add(&reading_speed_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	reading_speed_sizer.add(&reading_speed_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	reading_sizer.add_sizer(&reading_speed_sizer, 0, SizerFlag::All, option_padding);
	let max_recent_docs = 100;
	let recent_docs_label =
		// TRANSLATORS: Label for the number of recently opened documents to keep in history
		StaticText::builder(&general_panel).with_label(&t("Number of &recent documents to show:")).build();
	let recent_docs_ctrl = SpinCtrl::builder(&general_panel).with_range(0, max_recent_docs).build();
	let recent_docs_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	recent_docs_sizer.add(&recent_docs_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	recent_docs_sizer.add(&recent_docs_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	general_sizer.add_sizer(&recent_docs_sizer, 0, SizerFlag::All, option_padding);
	// TRANSLATORS: Label for the language selection dropdown
	let language_label_text = t("&Language:");
	let language_label = StaticText::builder(&general_panel).with_label(&language_label_text).build();
	let language_combo = Choice::builder(&general_panel).build();
	let languages = TranslationManager::instance().lock().unwrap().available_languages();
	let language_codes = populate_language_choice(&language_combo, &languages);
	#[cfg(target_os = "macos")]
	language_combo.set_accessibility_label(language_label_text.replace('&', "").trim_end_matches(':').trim());

	let language_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	language_sizer.add(&language_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	language_sizer.add(&language_combo, 0, SizerFlag::AlignCenterVertical, 0);
	general_sizer.add_sizer(&language_sizer, 0, SizerFlag::All, option_padding);
	// TRANSLATORS: Label for the update channel selection dropdown
	let channel_label_text = t("Update Channel:");
	let channel_label = StaticText::builder(&general_panel).with_label(&channel_label_text).build();
	let update_channel_combo = Choice::builder(&general_panel).build();
	// TRANSLATORS: Stable update channel option
	update_channel_combo.append(&t("Stable"));
	// TRANSLATORS: Developer/development update channel option
	update_channel_combo.append(&t("Dev"));
	#[cfg(target_os = "macos")]
	update_channel_combo.set_accessibility_label(channel_label_text.trim_end_matches(':').trim());

	let channel_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	channel_sizer.add(&channel_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	channel_sizer.add(&update_channel_combo, 0, SizerFlag::AlignCenterVertical, 0);
	general_sizer.add_sizer(&channel_sizer, 0, SizerFlag::All, option_padding);
	// TRANSLATORS: Label/header for the Font options section
	let font_group_box = StaticBox::builder(&readability_panel).with_label(&t("Font")).build();
	let font_group_sizer = StaticBoxSizerBuilder::new_with_box(&font_group_box, Orientation::Vertical).build();
	let font_preview_label = StaticText::builder(&readability_panel).with_label("").build();
	// TRANSLATORS: Button label to pick a font
	let choose_font_button = Button::builder(&readability_panel).with_label(&t("Choose &Font...")).build();
	// TRANSLATORS: Button label to restore font settings to default values
	let reset_font_button = Button::builder(&readability_panel).with_label(&t("&Reset to Default Font")).build();
	font_group_sizer.add(&font_preview_label, 0, SizerFlag::All, option_padding);
	font_group_sizer.add(&choose_font_button, 0, SizerFlag::All, option_padding);
	font_group_sizer.add(&reset_font_button, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&font_group_sizer, 0, SizerFlag::Expand | SizerFlag::All, option_padding);
	// TRANSLATORS: Label/header for the Background Color options section
	let bg_group_box = StaticBox::builder(&readability_panel).with_label(&t("Background Color")).build();
	let bg_group_sizer = StaticBoxSizerBuilder::new_with_box(&bg_group_box, Orientation::Vertical).build();
	let bg_color_label = StaticText::builder(&readability_panel).with_label("").build();
	// TRANSLATORS: Button label to pick a background color
	let choose_bg_button = Button::builder(&readability_panel).with_label(&t("Choose &Background Color...")).build();
	// TRANSLATORS: Button label to restore background color to default values
	let reset_bg_button = Button::builder(&readability_panel).with_label(&t("Reset to &Default Background")).build();
	bg_group_sizer.add(&bg_color_label, 0, SizerFlag::All, option_padding);
	bg_group_sizer.add(&choose_bg_button, 0, SizerFlag::All, option_padding);
	bg_group_sizer.add(&reset_bg_button, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&bg_group_sizer, 0, SizerFlag::Expand | SizerFlag::All, option_padding);
	// TRANSLATORS: Label for the line spacing dropdown
	let line_spacing_label_text = t("&Line spacing:");
	let line_spacing_label = StaticText::builder(&readability_panel).with_label(&line_spacing_label_text).build();
	let line_spacing_ctrl = Choice::builder(&readability_panel).build();
	// TRANSLATORS: Default spacing option (as opposed to relaxed/wide), shown in the line, paragraph, and letter spacing dropdowns
	line_spacing_ctrl.append(&t("Normal"));
	// TRANSLATORS: 1.5x line spacing option
	line_spacing_ctrl.append(&t("1.5\u{00d7}"));
	// TRANSLATORS: Double line spacing option
	line_spacing_ctrl.append(&t("Double"));
	#[cfg(target_os = "macos")]
	line_spacing_ctrl.set_accessibility_label(line_spacing_label_text.replace('&', "").trim_end_matches(':').trim());

	let line_spacing_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	line_spacing_sizer.add(&line_spacing_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	line_spacing_sizer.add(&line_spacing_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	// TRANSLATORS: Label for the paragraph spacing dropdown
	let paragraph_spacing_label_text = t("&Paragraph spacing:");
	let paragraph_spacing_label =
		StaticText::builder(&readability_panel).with_label(&paragraph_spacing_label_text).build();
	let paragraph_spacing_ctrl = Choice::builder(&readability_panel).build();
	// TRANSLATORS: Default spacing option (as opposed to relaxed/wide), shown in the line, paragraph, and letter spacing dropdowns
	paragraph_spacing_ctrl.append(&t("Normal"));
	// TRANSLATORS: Relaxed paragraph spacing option
	paragraph_spacing_ctrl.append(&t("Relaxed"));
	// TRANSLATORS: Wide spacing option, shown in the paragraph and letter spacing dropdowns
	paragraph_spacing_ctrl.append(&t("Wide"));
	#[cfg(target_os = "macos")]
	paragraph_spacing_ctrl
		.set_accessibility_label(paragraph_spacing_label_text.replace('&', "").trim_end_matches(':').trim());

	let paragraph_spacing_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	paragraph_spacing_sizer.add(
		&paragraph_spacing_label,
		0,
		SizerFlag::AlignCenterVertical | SizerFlag::Right,
		DIALOG_PADDING,
	);
	paragraph_spacing_sizer.add(&paragraph_spacing_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	// TRANSLATORS: Label for the letter spacing dropdown
	let letter_spacing_label_text = t("L&etter spacing:");
	let letter_spacing_label = StaticText::builder(&readability_panel).with_label(&letter_spacing_label_text).build();
	let letter_spacing_ctrl = Choice::builder(&readability_panel).build();
	// TRANSLATORS: Default spacing option (as opposed to relaxed/wide), shown in the line, paragraph, and letter spacing dropdowns
	letter_spacing_ctrl.append(&t("Normal"));
	// TRANSLATORS: Wide spacing option, shown in the paragraph and letter spacing dropdowns
	letter_spacing_ctrl.append(&t("Wide"));
	// TRANSLATORS: Very Wide letter spacing option
	letter_spacing_ctrl.append(&t("Very Wide"));
	#[cfg(target_os = "macos")]
	letter_spacing_ctrl
		.set_accessibility_label(letter_spacing_label_text.replace('&', "").trim_end_matches(':').trim());

	let letter_spacing_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	letter_spacing_sizer.add(
		&letter_spacing_label,
		0,
		SizerFlag::AlignCenterVertical | SizerFlag::Right,
		DIALOG_PADDING,
	);
	letter_spacing_sizer.add(&letter_spacing_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	// TRANSLATORS: Label for the text alignment dropdown
	let text_alignment_label_text = t("Text &alignment:");
	let text_alignment_label = StaticText::builder(&readability_panel).with_label(&text_alignment_label_text).build();
	let text_alignment_ctrl = Choice::builder(&readability_panel).build();
	// TRANSLATORS: Left text alignment option
	text_alignment_ctrl.append(&t("Left"));
	// TRANSLATORS: Center text alignment option
	text_alignment_ctrl.append(&t("Center"));
	// TRANSLATORS: Right text alignment option
	text_alignment_ctrl.append(&t("Right"));
	// TRANSLATORS: Justified text alignment option
	text_alignment_ctrl.append(&t("Justify"));
	#[cfg(target_os = "macos")]
	text_alignment_ctrl
		.set_accessibility_label(text_alignment_label_text.replace('&', "").trim_end_matches(':').trim());

	let text_alignment_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	text_alignment_sizer.add(
		&text_alignment_label,
		0,
		SizerFlag::AlignCenterVertical | SizerFlag::Right,
		DIALOG_PADDING,
	);
	text_alignment_sizer.add(&text_alignment_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	readability_sizer.add(&word_wrap_check, 0, SizerFlag::All, option_padding);
	readability_sizer.add(&render_tables_inline_check, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&line_spacing_sizer, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&paragraph_spacing_sizer, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&letter_spacing_sizer, 0, SizerFlag::All, option_padding);
	readability_sizer.add_sizer(&text_alignment_sizer, 0, SizerFlag::All, option_padding);
	readability_panel.set_sizer(readability_sizer, true);
	general_panel.set_sizer(general_sizer, true);
	reading_panel.set_sizer(reading_sizer, true);
	// TRANSLATORS: Tab label for the General options panel
	let general_label = t("General");
	// TRANSLATORS: Tab label for the Reading options panel
	let reading_label = t("Reading");
	// TRANSLATORS: Tab label for the Readability options panel
	let readability_label = t("Readability");
	general_panel.set_accessibility_label(&general_label);
	reading_panel.set_accessibility_label(&reading_label);
	readability_panel.set_accessibility_label(&readability_label);
	#[cfg(target_os = "windows")]
	{
		general_panel.set_accessibility_role(AccRole::PropertyPage);
		reading_panel.set_accessibility_role(AccRole::PropertyPage);
		readability_panel.set_accessibility_role(AccRole::PropertyPage);
	}
	notebook.add_page(&general_panel, &general_label, true, None);
	notebook.add_page(&reading_panel, &reading_label, false, None);
	notebook.add_page(&readability_panel, &readability_label, false, None);
	restore_docs_check.set_value(config.get_app_bool("restore_previous_documents", true));
	word_wrap_check.set_value(config.get_app_bool("word_wrap", false));
	render_tables_inline_check.set_value(config.get_app_bool("render_tables_inline", true));
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
	let current_channel = get_update_channel(config);
	let channel_index = match current_channel {
		UpdateChannel::Stable => 0,
		UpdateChannel::Dev => 1,
	};
	update_channel_combo.set_selection(channel_index);
	let current_hotkey = Rc::new(RefCell::new(config.get_hotkey()));
	let hotkey_state = Rc::clone(&current_hotkey);
	let hotkey_dialog_parent = dialog;
	hotkey_button.on_click(move |_| {
		let initial = hotkey_state.borrow().clone();
		if let Some(updated) = prompt_for_hotkey(&hotkey_dialog_parent, &initial) {
			*hotkey_state.borrow_mut() = updated;
		}
	});
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
		if dlg.show_modal() == ID_OK
			&& let Some(c) = dlg.get_colour()
		{
			let packed = (i32::from(c.r) << 16) | (i32::from(c.g) << 8) | i32::from(c.b);
			bg_state.set(packed);
			bg_label.set_label(&color_description(packed));
		}
	});
	let bg_state_reset = Rc::clone(&bg_color);
	let bg_label_reset = bg_label;
	reset_bg_button.on_click(move |_| {
		bg_state_reset.set(-1);
		bg_label_reset.set_label(&color_description(-1));
	});
	// TRANSLATORS: Label for the confirmation button
	let ok_button = Button::builder(&dialog_ref).with_id(ID_OK).with_label(&t("OK")).build();
	// TRANSLATORS: Label for the cancellation button
	let cancel_button = Button::builder(&dialog_ref).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	ok_button.set_default();
	OptionsDialogUi {
		dialog: dialog_ref,
		notebook,
		restore_docs_check,
		word_wrap_check,
		render_tables_inline_check,
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
		hotkey: current_hotkey,
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
	patois::ui::resolve_language_choice(&ui.language_combo, &ui.language_codes)
		.unwrap_or_else(|| ui.current_language.clone())
}

fn color_description(color: i32) -> String {
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

fn font_description(rf: &ReadabilityFont) -> String {
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
	if dlg.show_modal() != ID_OK {
		return None;
	}
	let font = dlg.get_font()?;
	let chosen_color = if let Some(fd) = dlg.get_font_data() {
		let c = fd.get_chosen_colour();
		// Prevent double-free: this FontData pointer is owned by the dialog, not by us
		mem::forget(fd);
		c.map_or(-1, |col| (i32::from(col.r) << 16) | (i32::from(col.g) << 8) | i32::from(col.b))
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

fn prompt_for_hotkey(parent: &dyn WxWidget, initial: &HotkeyConfig) -> Option<HotkeyConfig> {
	// TRANSLATORS: Title of the hotkey customization dialog
	let dialog = Dialog::builder(parent, &t("Window Hotkey")).with_size(300, 230).build();
	let panel = Panel::builder(&dialog).build();
	let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

	// TRANSLATORS: Checkbox label for Control modifier key
	let ctrl_cb = CheckBox::builder(&panel).with_label(&t("&Ctrl")).build();
	ctrl_cb.set_value(initial.ctrl);
	// TRANSLATORS: Checkbox label for Alt modifier key
	let alt_cb = CheckBox::builder(&panel).with_label(&t("&Alt")).build();
	alt_cb.set_value(initial.alt);
	// TRANSLATORS: Checkbox label for Shift modifier key
	let shift_cb = CheckBox::builder(&panel).with_label(&t("&Shift")).build();
	shift_cb.set_value(initial.shift);
	// TRANSLATORS: Checkbox label for Windows modifier key
	let win_cb = CheckBox::builder(&panel).with_label(&t("&Win")).build();
	win_cb.set_value(initial.win);

	main_sizer.add(&ctrl_cb, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Top, 10);
	main_sizer.add(&alt_cb, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right, 10);
	main_sizer.add(&shift_cb, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right, 10);
	main_sizer.add(&win_cb, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right, 10);

	// TRANSLATORS: Label for the hotkey key selection input field
	let key_label = StaticText::builder(&panel).with_label(&t("&Key:")).build();
	let key_text = TextCtrl::builder(&panel).build();
	key_text.set_value(&hotkey_key_display_name(initial.key));
	let key_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	key_sizer.add(&key_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 8);
	key_sizer.add(&key_text, 1, SizerFlag::Expand, 0);
	main_sizer.add_sizer(&key_sizer, 0, SizerFlag::Expand | SizerFlag::All, 10);

	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	// TRANSLATORS: Button label to clear the current hotkey selection
	let clear_button = Button::builder(&panel).with_label(&t("Clear")).build();
	// TRANSLATORS: Label for the confirmation button
	let ok_button = Button::builder(&panel).with_id(ID_OK).with_label(&t("OK")).build();
	ok_button.set_default();
	// TRANSLATORS: Label for the cancellation button
	let cancel_button = Button::builder(&panel).with_id(ID_CANCEL).with_label(&t("Cancel")).build();

	let key_text_clone = key_text;
	let ctrl_cb_clone = ctrl_cb;
	let alt_cb_clone = alt_cb;
	let shift_cb_clone = shift_cb;
	let win_cb_clone = win_cb;
	clear_button.on_click(move |_| {
		key_text_clone.set_value("");
		ctrl_cb_clone.set_value(false);
		alt_cb_clone.set_value(false);
		shift_cb_clone.set_value(false);
		win_cb_clone.set_value(false);
	});

	button_sizer.add(&clear_button, 0, SizerFlag::Right, 8);
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::Right, 8);
	button_sizer.add(&cancel_button, 0, SizerFlag::Right, 8);
	main_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::All, 10);

	panel.set_sizer(main_sizer, true);
	let dialog_sizer = BoxSizer::builder(Orientation::Vertical).build();
	dialog_sizer.add(&panel, 1, SizerFlag::Expand, 0);
	dialog.set_sizer(dialog_sizer, true);
	dialog.set_affirmative_id(ID_OK);
	dialog.set_escape_id(ID_CANCEL);
	dialog.centre();

	if dialog.show_modal() != ID_OK {
		return None;
	}

	let key_value = key_text.get_value();
	let key_char = if key_value.trim().is_empty() { '\0' } else { parse_hotkey_key(&key_value).unwrap_or(initial.key) };

	Some(HotkeyConfig {
		ctrl: ctrl_cb.is_checked(),
		alt: alt_cb.is_checked(),
		shift: shift_cb.is_checked(),
		win: win_cb.is_checked(),
		key: key_char,
	})
}

fn hotkey_key_display_name(key: char) -> String {
	match key {
		'\0' => String::new(),
		// TRANSLATORS: Representation of the Spacebar key
		' ' => t("Space"),
		c if c.is_ascii_alphanumeric() => c.to_ascii_uppercase().to_string(),
		c => c.to_string(),
	}
}

fn parse_hotkey_key(input: &str) -> Option<char> {
	let trimmed = input.trim();
	if trimmed.eq_ignore_ascii_case("space") {
		return Some(' ');
	}
	let ch = if trimmed.is_empty() { return None } else { trimmed.chars().last()? };
	if ch.is_ascii_alphanumeric() || ch.is_ascii_punctuation() || ch == ' ' {
		Some(ch.to_ascii_uppercase())
	} else {
		None
	}
}
