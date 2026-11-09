//! The General tab: how the app starts and behaves.

use std::{cell::RefCell, rc::Rc};

use paperback_core::config::{ConfigManager, KeyChord, ShortcutsConfig};
use patois::{t, wx::populate_language_choice};
use wxdragon::prelude::*;

use super::{OPTION_PADDING, labeled_row};
use crate::{
	translation_manager::TranslationManager,
	ui::dialogs::{Keymap, prompt_for_shortcuts},
};

const MAX_RECENT_DOCUMENTS: i32 = 100;

#[derive(Clone, Debug)]
pub struct GeneralSettings {
	pub restore_previous_documents: bool,
	pub auto_reload_documents: bool,
	pub recent_documents_to_show: i32,
	pub start_maximized: bool,
	pub minimize_to_tray: bool,
	pub check_for_updates_on_startup: bool,
	/// A language code, or empty for the system's language.
	pub language: String,
	pub hotkey: Option<KeyChord>,
	pub shortcuts: ShortcutsConfig,
}

impl GeneralSettings {
	pub fn load(config: &ConfigManager) -> Self {
		Self {
			restore_previous_documents: config.get_app_bool("restore_previous_documents", true),
			auto_reload_documents: config.get_app_bool("auto_reload_documents", true),
			recent_documents_to_show: config.get_app_int("recent_documents_to_show", 25).clamp(0, MAX_RECENT_DOCUMENTS),
			start_maximized: config.get_app_bool("start_maximized", false),
			minimize_to_tray: config.get_app_bool("minimize_to_tray", false),
			check_for_updates_on_startup: config.get_app_bool("check_for_updates_on_startup", true),
			language: config.get_app_string("language", ""),
			hotkey: config.get_hotkey(),
			shortcuts: config.get_shortcuts(),
		}
	}

	pub fn save(&self, config: &ConfigManager) {
		config.set_app_bool("restore_previous_documents", self.restore_previous_documents);
		config.set_app_bool("auto_reload_documents", self.auto_reload_documents);
		config.set_app_int("recent_documents_to_show", self.recent_documents_to_show);
		config.set_app_bool("start_maximized", self.start_maximized);
		config.set_app_bool("minimize_to_tray", self.minimize_to_tray);
		config.set_app_bool("check_for_updates_on_startup", self.check_for_updates_on_startup);
		config.set_app_string("language", &self.language);
		config.set_hotkey(self.hotkey.as_ref());
		config.set_shortcuts(&self.shortcuts);
	}
}

pub(super) struct GeneralTab {
	restore_docs: CheckBox,
	auto_reload: CheckBox,
	recent_docs: SpinCtrl,
	start_maximized: CheckBox,
	minimize_to_tray: CheckBox,
	check_for_updates: CheckBox,
	language: Choice,
	language_codes: Vec<String>,
	current_language: String,
	hotkey: Rc<RefCell<Option<KeyChord>>>,
	shortcuts: Rc<RefCell<ShortcutsConfig>>,
}

impl GeneralTab {
	/// `dialog` is the parent of the shortcut editor this tab opens.
	pub(super) fn new(panel: Panel, dialog: Dialog, settings: &GeneralSettings, padding: i32) -> Self {
		let restore_docs =
			// TRANSLATORS: Option to restore documents that were open when the app was last closed
			CheckBox::builder(&panel).with_label(&t("&Restore previously opened documents on startup")).build();
		let auto_reload =
			// TRANSLATORS: Option to automatically reload an open document when its file changes on disk
			CheckBox::builder(&panel).with_label(&t("&Automatically reload changed documents")).build();
		let recent_docs_label =
			// TRANSLATORS: Label for the number of recently opened documents to keep in history
			StaticText::builder(&panel).with_label(&t("Number of &recent documents to show:")).build();
		let recent_docs = SpinCtrl::builder(&panel).with_range(0, MAX_RECENT_DOCUMENTS).build();
		// TRANSLATORS: Option to start the app maximized
		let start_maximized = CheckBox::builder(&panel).with_label(&t("&Start maximized")).build();
		// TRANSLATORS: Option to minimize the app window to the system tray instead of the taskbar
		let minimize_to_tray = CheckBox::builder(&panel).with_label(&t("&Minimize to system tray")).build();
		let check_for_updates =
			// TRANSLATORS: Option to check for app updates automatically on startup
			CheckBox::builder(&panel).with_label(&t("Check for &updates on startup")).build();
		// TRANSLATORS: Label for the language selection dropdown
		let language_label_text = t("&Language:");
		let language_label = StaticText::builder(&panel).with_label(&language_label_text).build();
		let language = Choice::builder(&panel).build();
		let languages = TranslationManager::instance().lock().unwrap().available_languages();
		let language_codes = populate_language_choice(&language, &languages);
		#[cfg(target_os = "macos")]
		language.set_accessibility_label(language_label_text.replace('&', "").trim_end_matches(':').trim());
		// TRANSLATORS: Button label to open the Keyboard Shortcuts customization dialog
		let shortcuts_button = Button::builder(&panel).with_label(&t("Customize &Keyboard Shortcuts...")).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&restore_docs, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&auto_reload, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(recent_docs_label, &recent_docs, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&start_maximized, 0, SizerFlag::All, OPTION_PADDING);
		#[cfg(target_os = "windows")]
		sizer.add(&minimize_to_tray, 0, SizerFlag::All, OPTION_PADDING);
		#[cfg(not(target_os = "windows"))]
		minimize_to_tray.show(false);
		sizer.add(&check_for_updates, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(language_label, &language, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&shortcuts_button, 0, SizerFlag::All, OPTION_PADDING);
		panel.set_sizer(sizer, true);
		restore_docs.set_value(settings.restore_previous_documents);
		auto_reload.set_value(settings.auto_reload_documents);
		recent_docs.set_value(settings.recent_documents_to_show);
		start_maximized.set_value(settings.start_maximized);
		minimize_to_tray.set_value(settings.minimize_to_tray);
		check_for_updates.set_value(settings.check_for_updates_on_startup);
		let current_language = if settings.language.is_empty() {
			TranslationManager::instance().lock().unwrap().current_language()
		} else {
			settings.language.clone()
		};
		if let Some(index) = language_codes.iter().position(|code| code == &current_language) {
			language.set_selection(u32::try_from(index).unwrap_or(0));
		}
		let hotkey = Rc::new(RefCell::new(settings.hotkey.clone()));
		let shortcuts = Rc::new(RefCell::new(settings.shortcuts.clone()));
		let (hotkey_state, shortcuts_state) = (Rc::clone(&hotkey), Rc::clone(&shortcuts));
		shortcuts_button.on_click(move |_| {
			let initial =
				Keymap { shortcuts: shortcuts_state.borrow().clone(), window_hotkey: hotkey_state.borrow().clone() };
			if let Some(updated) = prompt_for_shortcuts(&dialog, &initial) {
				*shortcuts_state.borrow_mut() = updated.shortcuts;
				*hotkey_state.borrow_mut() = updated.window_hotkey;
			}
		});
		Self {
			restore_docs,
			auto_reload,
			recent_docs,
			start_maximized,
			minimize_to_tray,
			check_for_updates,
			language,
			language_codes,
			current_language,
			hotkey,
			shortcuts,
		}
	}

	pub(super) fn values(&self) -> GeneralSettings {
		GeneralSettings {
			restore_previous_documents: self.restore_docs.is_checked(),
			auto_reload_documents: self.auto_reload.is_checked(),
			recent_documents_to_show: self.recent_docs.value(),
			start_maximized: self.start_maximized.is_checked(),
			minimize_to_tray: self.minimize_to_tray.is_checked(),
			check_for_updates_on_startup: self.check_for_updates.is_checked(),
			language: patois::wx::resolve_language_choice(&self.language, &self.language_codes)
				.unwrap_or_else(|| self.current_language.clone()),
			hotkey: self.hotkey.borrow().clone(),
			shortcuts: self.shortcuts.borrow().clone(),
		}
	}
}
