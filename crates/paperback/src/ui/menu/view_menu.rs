use paperback_core::config::{ActionId, ConfigManager};
use patois::t;

use super::builder::{MenuEntry, check, format_menu_label, item_with_help, radio, submenu};
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
	let page_layout = |id: i32, label: String, help: String| radio(id, label, help, id == menu_ids::PAGE_LAYOUT_SHARED);
	let page_layouts = vec![
		page_layout(
			menu_ids::PAGE_LAYOUT_SHARED,
			// TRANSLATORS: Item in the View menu's Page Layout submenu: the open PDF or XPS document follows the page layout chosen for all documents in Settings
			t("&Same as Other Documents"),
			// TRANSLATORS: Status-bar help text for the Same as Other Documents page layout item
			t("Read this document with the page layout set in Settings"),
		),
		page_layout(
			menu_ids::PAGE_LAYOUT_AUTOMATIC,
			// TRANSLATORS: Item in the View menu's Page Layout submenu: the open document's pages are read in the order the document gives, from its tags where it has them, with wrapped lines joined into paragraphs
			t("&Automatic"),
			// TRANSLATORS: Status-bar help text for the Automatic page layout item
			t("Read this document in its own order, joining wrapped lines"),
		),
		page_layout(
			menu_ids::PAGE_LAYOUT_COLUMNS,
			// TRANSLATORS: Item in the View menu's Page Layout submenu: the open document's own reading order is ignored and the columns of each page are found from where the text sits, for documents read out of order
			t("By &Columns"),
			// TRANSLATORS: Status-bar help text for the By Columns page layout item
			t("Find the columns of each page instead of following the document's own order"),
		),
		page_layout(
			menu_ids::PAGE_LAYOUT_LINES,
			// TRANSLATORS: Item in the View menu's Page Layout submenu: like By Columns, but every line is kept apart instead of joined into paragraphs, for code, poetry and lists of clauses
			t("By Columns, &Every Line Apart"),
			// TRANSLATORS: Status-bar help text for the By Columns, Every Line Apart page layout item
			t("Find the columns of each page and keep every line separate"),
		),
	];
	let mut entries = commands::menu_entries(&[ActionId::Reload], config);
	entries.extend([
		MenuEntry::Separator,
		check(menu_ids::TOGGLE_WORD_WRAP, word_wrap_label, word_wrap_help, config.get_app_bool("word_wrap", false)),
		check(menu_ids::TOGGLE_FULL_SCREEN, full_screen_label, full_screen_help, false),
		// TRANSLATORS: Submenu of the View menu choosing how the open PDF or XPS document's pages are put in reading order and paragraphs
		submenu(t("Page &Layout"), t("How this document's pages are read"), page_layouts),
		MenuEntry::Separator,
		item_with_help(menu_ids::OPEN_IN_WEB_VIEW, web_view_label, web_view_help),
		item_with_help(menu_ids::VIEW_SOURCE, view_source_label, view_source_help),
	]);
	entries
}
