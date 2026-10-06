use paperback_core::config::{ActionId, ConfigManager};
use patois::t;

use super::builder::{MenuEntry, check, format_menu_label, item_with_help};
use crate::ui::{commands, menu_ids};

pub fn entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Checkable menu item in the View menu that toggles whether word wrap is enabled.
	let word_wrap_label = format_menu_label(&t("W&ord wrap"), ActionId::ToggleWordWrap, config);
	// TRANSLATORS: Status-bar help text for the Word Wrap menu item.
	let word_wrap_help = t("Toggle word wrap");
	// TRANSLATORS: Checkable menu item in the View menu that toggles full screen mode.
	let full_screen_label = format_menu_label(&t("&Full Screen"), ActionId::ToggleFullScreen, config);
	// TRANSLATORS: Status-bar help text for the Full Screen menu item.
	let full_screen_help = t("Toggle full screen");
	// TRANSLATORS: Menu item in the View menu to open the document in a web view.
	let web_view_label = format_menu_label(&t("Open in &Web View"), ActionId::OpenInWebView, config);
	// TRANSLATORS: Status-bar help text for the Open in Web View menu item.
	let web_view_help = t("Open document in web view");
	// TRANSLATORS: Menu item in the View menu to open the document's underlying source markup in a new tab.
	let view_source_label = format_menu_label(&t("View &Source"), ActionId::ViewSource, config);
	// TRANSLATORS: Status-bar help text for the View Source menu item.
	let view_source_help = t("Open the document source in a new tab");
	let mut entries = commands::menu_entries(&[ActionId::Reload], config);
	entries.extend([
		MenuEntry::Separator,
		check(menu_ids::TOGGLE_WORD_WRAP, word_wrap_label, word_wrap_help, config.get_app_bool("word_wrap", false)),
		check(menu_ids::TOGGLE_FULL_SCREEN, full_screen_label, full_screen_help, false),
		MenuEntry::Separator,
		item_with_help(menu_ids::OPEN_IN_WEB_VIEW, web_view_label, web_view_help),
		item_with_help(menu_ids::VIEW_SOURCE, view_source_label, view_source_help),
	]);
	entries
}
