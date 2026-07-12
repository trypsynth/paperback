use std::path::Path;

use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::prelude::*;

use super::menu_ids;

/// IDs of menu items that require an open document.
const DOCUMENT_DEPENDENT_IDS: &[i32] = &[
	// File
	menu_ids::CLOSE,
	menu_ids::CLOSE_ALL,
	// Find
	menu_ids::FIND,
	menu_ids::FIND_NEXT,
	menu_ids::FIND_PREVIOUS,
	// Navigation
	menu_ids::GO_TO_LINE,
	menu_ids::GO_TO_PERCENT,
	menu_ids::GO_TO_PAGE,
	menu_ids::GO_BACK,
	menu_ids::GO_FORWARD,
	// Sections
	menu_ids::PREVIOUS_SECTION,
	menu_ids::NEXT_SECTION,
	// Headings
	menu_ids::PREVIOUS_HEADING,
	menu_ids::NEXT_HEADING,
	menu_ids::PREVIOUS_HEADING_1,
	menu_ids::NEXT_HEADING_1,
	menu_ids::PREVIOUS_HEADING_2,
	menu_ids::NEXT_HEADING_2,
	menu_ids::PREVIOUS_HEADING_3,
	menu_ids::NEXT_HEADING_3,
	menu_ids::PREVIOUS_HEADING_4,
	menu_ids::NEXT_HEADING_4,
	menu_ids::PREVIOUS_HEADING_5,
	menu_ids::NEXT_HEADING_5,
	menu_ids::PREVIOUS_HEADING_6,
	menu_ids::NEXT_HEADING_6,
	// Pages
	menu_ids::PREVIOUS_PAGE,
	menu_ids::NEXT_PAGE,
	// Bookmarks / Notes
	menu_ids::PREVIOUS_BOOKMARK,
	menu_ids::NEXT_BOOKMARK,
	menu_ids::PREVIOUS_NOTE,
	menu_ids::NEXT_NOTE,
	menu_ids::JUMP_TO_ALL_BOOKMARKS,
	menu_ids::JUMP_TO_BOOKMARKS_ONLY,
	menu_ids::JUMP_TO_NOTES_ONLY,
	menu_ids::VIEW_NOTE_TEXT,
	// Links
	menu_ids::PREVIOUS_LINK,
	menu_ids::NEXT_LINK,
	// Images
	menu_ids::PREVIOUS_IMAGE,
	menu_ids::NEXT_IMAGE,
	// Figures
	menu_ids::PREVIOUS_FIGURE,
	menu_ids::NEXT_FIGURE,
	// Tables
	menu_ids::PREVIOUS_TABLE,
	menu_ids::NEXT_TABLE,
	// Separators
	menu_ids::PREVIOUS_SEPARATOR,
	menu_ids::NEXT_SEPARATOR,
	// Lists
	menu_ids::PREVIOUS_LIST,
	menu_ids::NEXT_LIST,
	menu_ids::PREVIOUS_LIST_ITEM,
	menu_ids::NEXT_LIST_ITEM,
	// Containers
	menu_ids::CONTAINER_START,
	menu_ids::CONTAINER_END,
	// Tools
	menu_ids::WORD_COUNT,
	menu_ids::DOCUMENT_INFO,
	menu_ids::TABLE_OF_CONTENTS,
	menu_ids::ELEMENTS_LIST,
	menu_ids::REVEAL_FILE_IN_FOLDER,
	menu_ids::OPEN_IN_WEB_VIEW,
	menu_ids::VIEW_SOURCE,
	// Import/Export
	menu_ids::IMPORT_DOCUMENT_DATA,
	menu_ids::EXPORT_DOCUMENT_DATA,
	menu_ids::EXPORT_TO_PLAIN_TEXT,
	// Bookmark tools
	menu_ids::TOGGLE_BOOKMARK,
	menu_ids::BOOKMARK_WITH_NOTE,
];

/// Enable or disable all document-dependent menu items.
pub fn update_menu_item_states(frame: &Frame, has_document: bool) {
	let Some(menu_bar) = frame.get_menu_bar() else {
		return;
	};
	for &id in DOCUMENT_DEPENDENT_IDS {
		menu_bar.enable_item(id, has_document);
	}
}

/// Enable or disable the "Reopen Last Closed" menu item.
pub fn update_reopen_state(frame: &Frame, has_recently_closed: bool) {
	let Some(menu_bar) = frame.get_menu_bar() else {
		return;
	};
	menu_bar.enable_item(menu_ids::REOPEN_LAST_CLOSED, has_recently_closed);
}

pub struct MenuItemSpec {
	pub id: i32,
	pub label: String,
	pub help: Option<String>,
}

pub enum MenuEntry {
	Item(MenuItemSpec),
	Separator,
}

pub const fn item(id: i32, label: String) -> MenuEntry {
	MenuEntry::Item(MenuItemSpec { id, label, help: None })
}

pub const fn item_with_help(id: i32, label: String, help: String) -> MenuEntry {
	MenuEntry::Item(MenuItemSpec { id, label, help: Some(help) })
}

pub fn build_menu(entries: &[MenuEntry]) -> Menu {
	let mut builder = Menu::builder();
	for entry in entries {
		builder = match entry {
			MenuEntry::Item(spec) => builder.append_item(spec.id, &spec.label, spec.help.as_deref().unwrap_or("")),
			MenuEntry::Separator => builder.append_separator(),
		};
	}
	builder.build()
}

pub fn append_menu_entries(menu: &Menu, entries: &[MenuEntry]) {
	for entry in entries {
		match entry {
			MenuEntry::Item(spec) => {
				let _ = menu.append(spec.id, &spec.label, "", ItemKind::Normal);
			}
			MenuEntry::Separator => menu.append_separator(),
		}
	}
}

pub fn sections_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to go to the previous section
	let prev_section_label = t("Previous Section\t[");
	// TRANSLATORS: Status bar help text for the "Previous Section" menu item
	let prev_section_help = t("Go to previous section");
	// TRANSLATORS: Menu item label to go to the next section
	let next_section_label = t("Next Section\t]");
	// TRANSLATORS: Status bar help text for the "Next Section" menu item
	let next_section_help = t("Go to next section");
	vec![
		item_with_help(menu_ids::PREVIOUS_SECTION, prev_section_label, prev_section_help),
		item_with_help(menu_ids::NEXT_SECTION, next_section_label, next_section_help),
	]
}

pub fn pages_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to open the "go to page" dialog
	let goto_page_label = t("Go to &Page\tCtrl+P");
	// TRANSLATORS: Menu item label to go to the previous page
	let prev_page_label = t("Previous Pa&ge\tShift+P");
	// TRANSLATORS: Menu item label to go to the next page
	let next_page_label = t("Next Pag&e\tP");
	vec![
		item(menu_ids::GO_TO_PAGE, goto_page_label),
		item(menu_ids::PREVIOUS_PAGE, prev_page_label),
		item(menu_ids::NEXT_PAGE, next_page_label),
	]
}

pub fn links_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to go to the previous link
	let prev_link_label = t("Previous Lin&k\tShift+K");
	// TRANSLATORS: Menu item label to go to the next link
	let next_link_label = t("Next Lin&k\tK");
	vec![item(menu_ids::PREVIOUS_LINK, prev_link_label), item(menu_ids::NEXT_LINK, next_link_label)]
}

pub fn images_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to go to the previous image
	let prev_image_label = t("Previous Ima&ge\tShift+G");
	// TRANSLATORS: Menu item label to go to the next image
	let next_image_label = t("Next Ima&ge\tG");
	vec![item(menu_ids::PREVIOUS_IMAGE, prev_image_label), item(menu_ids::NEXT_IMAGE, next_image_label)]
}

pub fn figures_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to go to the previous figure
	let prev_figure_label = t("Previous Figu&re\tShift+F");
	// TRANSLATORS: Menu item label to go to the next figure
	let next_figure_label = t("Next Figu&re\tF");
	vec![item(menu_ids::PREVIOUS_FIGURE, prev_figure_label), item(menu_ids::NEXT_FIGURE, next_figure_label)]
}

pub fn tables_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to go to the previous table
	let prev_table_label = t("Previous &Table\tShift+T");
	// TRANSLATORS: Menu item label to go to the next table
	let next_table_label = t("Next &Table\tT");
	vec![item(menu_ids::PREVIOUS_TABLE, prev_table_label), item(menu_ids::NEXT_TABLE, next_table_label)]
}

pub fn separators_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to go to the previous separator element
	let prev_separator_label = t("Previous Se&parator\tShift+S");
	// TRANSLATORS: Menu item label to go to the next separator element
	let next_separator_label = t("Next Se&parator\tS");
	vec![item(menu_ids::PREVIOUS_SEPARATOR, prev_separator_label), item(menu_ids::NEXT_SEPARATOR, next_separator_label)]
}

pub fn lists_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to go to the previous list
	let prev_list_label = t("Previous L&ist\tShift+L");
	// TRANSLATORS: Menu item label to go to the next list
	let next_list_label = t("Next L&ist\tL");
	// TRANSLATORS: Menu item label to go to the previous list item
	let prev_list_item_label = t("Previous List &Item\tShift+I");
	// TRANSLATORS: Menu item label to go to the next list item
	let next_list_item_label = t("Next List I&tem\tI");
	vec![
		item(menu_ids::PREVIOUS_LIST, prev_list_label),
		item(menu_ids::NEXT_LIST, next_list_label),
		item(menu_ids::PREVIOUS_LIST_ITEM, prev_list_item_label),
		item(menu_ids::NEXT_LIST_ITEM, next_list_item_label),
	]
}

pub fn containers_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to go to the start of the enclosing list/table
	let container_start_label = t("Container &Start\tShift+,");
	// TRANSLATORS: Status bar help text for the "Container Start" menu item
	let container_start_help = t("Go to the start of the current list or table");
	// TRANSLATORS: Menu item label to go past the end of the enclosing list/table
	let container_end_label = t("Past Container &End\t,");
	// TRANSLATORS: Status bar help text for the "Past Container End" menu item
	let container_end_help = t("Go past the end of the current list or table");
	vec![
		item_with_help(menu_ids::CONTAINER_START, container_start_label, container_start_help),
		item_with_help(menu_ids::CONTAINER_END, container_end_label, container_end_help),
	]
}

pub fn headings_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to go to the next level-1 heading
	let next_heading1_label = t("Next Heading Level 1\t1");
	// TRANSLATORS: Menu item label to go to the previous level-2 heading
	let prev_heading2_label = t("Previous Heading Level &2\tShift+2");
	// TRANSLATORS: Menu item label to go to the next level-2 heading
	let next_heading2_label = t("Next Heading Level 2\t2");
	// TRANSLATORS: Menu item label to go to the previous level-3 heading
	let prev_heading3_label = t("Previous Heading Level &3\tShift+3");
	// TRANSLATORS: Menu item label to go to the next level-3 heading
	let next_heading3_label = t("Next Heading Level 3\t3");
	// TRANSLATORS: Menu item label to go to the previous level-4 heading
	let prev_heading4_label = t("Previous Heading Level &4\tShift+4");
	// TRANSLATORS: Menu item label to go to the next level-4 heading
	let next_heading4_label = t("Next Heading Level 4\t4");
	// TRANSLATORS: Menu item label to go to the previous level-5 heading
	let prev_heading5_label = t("Previous Heading Level &5\tShift+5");
	// TRANSLATORS: Menu item label to go to the next level-5 heading
	let next_heading5_label = t("Next Heading Level 5\t5");
	// TRANSLATORS: Menu item label to go to the previous level-6 heading
	let prev_heading6_label = t("Previous Heading Level &6\tShift+6");
	// TRANSLATORS: Menu item label to go to the next level-6 heading
	let next_heading6_label = t("Next Heading Level 6\t6");
	vec![
		item_with_help(
			menu_ids::PREVIOUS_HEADING,
			// TRANSLATORS: Menu item label to go to the previous heading of any level
			t("&Previous Heading\tShift+H"),
			// TRANSLATORS: Status bar help text for the "Previous Heading" menu item
			t("Go to previous heading"),
		),
		item_with_help(
			menu_ids::NEXT_HEADING,
			// TRANSLATORS: Menu item label to go to the next heading of any level
			t("&Next Heading\tH"),
			// TRANSLATORS: Status bar help text for the "Next Heading" menu item
			t("Go to next heading"),
		),
		MenuEntry::Separator,
		item(
			menu_ids::PREVIOUS_HEADING_1,
			// TRANSLATORS: Menu item label to go to the previous level-1 heading
			t("Previous Heading Level &1\tShift+1"),
		),
		item(menu_ids::NEXT_HEADING_1, next_heading1_label),
		item(menu_ids::PREVIOUS_HEADING_2, prev_heading2_label),
		item(menu_ids::NEXT_HEADING_2, next_heading2_label),
		item(menu_ids::PREVIOUS_HEADING_3, prev_heading3_label),
		item(menu_ids::NEXT_HEADING_3, next_heading3_label),
		item(menu_ids::PREVIOUS_HEADING_4, prev_heading4_label),
		item(menu_ids::NEXT_HEADING_4, next_heading4_label),
		item(menu_ids::PREVIOUS_HEADING_5, prev_heading5_label),
		item(menu_ids::NEXT_HEADING_5, next_heading5_label),
		item(menu_ids::PREVIOUS_HEADING_6, prev_heading6_label),
		item(menu_ids::NEXT_HEADING_6, next_heading6_label),
	]
}

pub fn bookmarks_entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item label to go to the previous bookmark
	let prev_bookmark_label = t("&Previous Bookmark\tShift+B");
	// TRANSLATORS: Status bar help text for the "Previous Bookmark" menu item
	let prev_bookmark_help = t("Go to previous bookmark");
	// TRANSLATORS: Menu item label to go to the next bookmark
	let next_bookmark_label = t("&Next Bookmark\tB");
	// TRANSLATORS: Status bar help text for the "Next Bookmark" menu item
	let next_bookmark_help = t("Go to next bookmark");
	// TRANSLATORS: Menu item label to go to the previous note
	let prev_note_label = t("Previous &Note\tShift+N");
	// TRANSLATORS: Status bar help text for the "Previous Note" menu item
	let prev_note_help = t("Go to previous note");
	// TRANSLATORS: Menu item label to go to the next note
	let next_note_label = t("Next N&ote\tN");
	// TRANSLATORS: Status bar help text for the "Next Note" menu item
	let next_note_help = t("Go to next note");
	// TRANSLATORS: Menu item label to open the bookmark/note picker showing all entries
	let all_bookmarks_label = t("Jump to &All...\tCtrl+B");
	// TRANSLATORS: Status bar help text for the "Jump to All" menu item
	let all_bookmarks_help = t("Show all bookmarks and notes");
	// TRANSLATORS: Menu item label to open the bookmark/note picker filtered to bookmarks only
	let bookmarks_only_label = t("Jump to &Bookmarks Only...\tCtrl+Alt+B");
	// TRANSLATORS: Status bar help text for the "Jump to Bookmarks Only" menu item
	let bookmarks_only_help = t("Show bookmarks only");
	// TRANSLATORS: Menu item label to open the bookmark/note picker filtered to notes only
	let notes_only_label = t("Jump to Notes &Only...\tCtrl+Alt+M");
	// TRANSLATORS: Status bar help text for the "Jump to Notes Only" menu item
	let notes_only_help = t("Show notes only");
	let view_note_label = if cfg!(target_os = "macos") {
		// TRANSLATORS: Menu item label to view the note at the cursor position (macOS variant)
		t("&View Note Text\tRawCtrl+Shift+W")
	} else {
		// TRANSLATORS: Menu item label to view the note at the cursor position
		t("&View Note Text\tCtrl+Shift+W")
	};
	// TRANSLATORS: Status bar help text for the "View Note Text" menu item
	let view_note_help = t("View the note at current position");
	vec![
		item_with_help(menu_ids::PREVIOUS_BOOKMARK, prev_bookmark_label, prev_bookmark_help),
		item_with_help(menu_ids::NEXT_BOOKMARK, next_bookmark_label, next_bookmark_help),
		item_with_help(menu_ids::PREVIOUS_NOTE, prev_note_label, prev_note_help),
		item_with_help(menu_ids::NEXT_NOTE, next_note_label, next_note_help),
		MenuEntry::Separator,
		item_with_help(menu_ids::JUMP_TO_ALL_BOOKMARKS, all_bookmarks_label, all_bookmarks_help),
		item_with_help(menu_ids::JUMP_TO_BOOKMARKS_ONLY, bookmarks_only_label, bookmarks_only_help),
		item_with_help(menu_ids::JUMP_TO_NOTES_ONLY, notes_only_label, notes_only_help),
		item_with_help(menu_ids::VIEW_NOTE_TEXT, view_note_label, view_note_help),
	]
}

pub fn create_sections_submenu() -> Menu {
	let entries = sections_entries();
	build_menu(&entries)
}

pub fn append_sections_items(menu: &Menu) {
	let entries = sections_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_pages_submenu() -> Menu {
	let entries = pages_entries();
	build_menu(&entries)
}

pub fn append_pages_items(menu: &Menu) {
	let entries = pages_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_links_submenu() -> Menu {
	let entries = links_entries();
	build_menu(&entries)
}

pub fn append_links_items(menu: &Menu) {
	let entries = links_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_images_submenu() -> Menu {
	let entries = images_entries();
	build_menu(&entries)
}

pub fn append_images_items(menu: &Menu) {
	let entries = images_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_figures_submenu() -> Menu {
	let entries = figures_entries();
	build_menu(&entries)
}

pub fn append_figures_items(menu: &Menu) {
	let entries = figures_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_tables_submenu() -> Menu {
	let entries = tables_entries();
	build_menu(&entries)
}

pub fn append_tables_items(menu: &Menu) {
	let entries = tables_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_separators_submenu() -> Menu {
	let entries = separators_entries();
	build_menu(&entries)
}

pub fn append_separators_items(menu: &Menu) {
	let entries = separators_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_lists_submenu() -> Menu {
	let entries = lists_entries();
	build_menu(&entries)
}

pub fn append_lists_items(menu: &Menu) {
	let entries = lists_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_containers_submenu() -> Menu {
	let entries = containers_entries();
	build_menu(&entries)
}

pub fn append_containers_items(menu: &Menu) {
	let entries = containers_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_headings_submenu() -> Menu {
	let entries = headings_entries();
	build_menu(&entries)
}

pub fn append_headings_items(menu: &Menu) {
	let entries = headings_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_bookmarks_submenu() -> Menu {
	let entries = bookmarks_entries();
	build_menu(&entries)
}

pub fn append_bookmarks_items(menu: &Menu) {
	let entries = bookmarks_entries();
	append_menu_entries(menu, &entries);
}

pub fn create_menu_bar(config: &ConfigManager) -> MenuBar {
	let file_menu = create_file_menu(config);
	let compact_go_menu = config.get_app_bool("compact_go_menu", true);
	let go_menu = create_go_menu(compact_go_menu);
	let tools_menu = create_tools_menu(config);
	let help_menu = create_help_menu();
	// TRANSLATORS: Top-level "File" menu bar label
	let file_label = t("&File");
	// TRANSLATORS: Top-level "Go" menu bar label
	let go_label = t("&Go");
	// TRANSLATORS: Top-level "Tools" menu bar label
	let tools_label = t("&Tools");
	// TRANSLATORS: Top-level "Help" menu bar label
	let help_label = t("&Help");
	#[allow(unused_mut)]
	let mut builder = MenuBar::builder().append(file_menu, &file_label);

	// All MacOS apps need an Edit menu.
	// This is where the OS places some items that each app should have, like "Start dictation" or "Emoji and Symbols."
	#[cfg(target_os = "macos")]
	{
		// TRANSLATORS: Top-level "Edit" menu bar label (macOS only)
		let edit_label = t("&Edit");
		builder = builder.append(create_edit_menu(), &edit_label);
	}
	builder.append(go_menu, &go_label).append(tools_menu, &tools_label).append(help_menu, &help_label).build()
}

pub fn create_file_menu(config: &ConfigManager) -> Menu {
	// TRANSLATORS: Menu item label to open a document
	let open_label = t("&Open...\tCtrl+O");
	// TRANSLATORS: Status bar help text for the "Open" menu item
	let open_help = t("Open a document");
	// On macOS, Ctrl+ maps to Cmd+, so use Cmd+W / Cmd+Shift+W for close.
	// On Windows/Linux, keep Ctrl+F4 / Ctrl+Shift+F4.
	// TRANSLATORS: Menu item label to close the current document
	let close_label = if cfg!(target_os = "macos") { t("&Close\tCtrl+W") } else { t("&Close\tCtrl+F4") };
	// TRANSLATORS: Status bar help text for the "Close" menu item
	let close_help = t("Close the current document");
	// TRANSLATORS: Menu item label to close all open documents
	let close_all_label =
		if cfg!(target_os = "macos") { t("Close &All\tCtrl+Shift+W") } else { t("Close &All\tCtrl+Shift+F4") };
	// TRANSLATORS: Status bar help text for the "Close All" menu item
	let close_all_help = t("Close all documents");
	// TRANSLATORS: Menu item label to reopen the most recently closed document
	let reopen_label = t("Reopen &Last Closed\tCtrl+Shift+T");
	// TRANSLATORS: Status bar help text for the "Reopen Last Closed" menu item
	let reopen_help = t("Reopen the last closed document");
	let file_menu = Menu::builder()
		.append_item(menu_ids::OPEN, &open_label, &open_help)
		.append_item(menu_ids::CLOSE, &close_label, &close_help)
		.append_item(menu_ids::CLOSE_ALL, &close_all_label, &close_all_help)
		.append_item(menu_ids::REOPEN_LAST_CLOSED, &reopen_label, &reopen_help)
		.build();
	let recent_menu = Menu::builder().build();
	populate_recent_documents_menu(&recent_menu, config);
	// TRANSLATORS: Submenu label listing recently opened documents
	let recent_label = t("&Recent Documents");
	// TRANSLATORS: Status bar help text for the "Recent Documents" submenu
	let recent_help = t("Open a recent document");
	let _ = file_menu.append_submenu(recent_menu, &recent_label, &recent_help);
	// On macOS, wxWidgets auto-moves wxID_EXIT to the app menu, so skip the
	// explicit Exit item to avoid a duplicate.
	if !cfg!(target_os = "macos") {
		file_menu.append_separator();
		// TRANSLATORS: Menu item label to exit the application
		let exit_label = t("E&xit\tCtrl+Q");
		// TRANSLATORS: Status bar help text for the "Exit" menu item
		let exit_help = t("Exit the application");
		let _ = file_menu.append(menu_ids::EXIT, &exit_label, &exit_help, ItemKind::Normal);
	}
	file_menu
}

/// The standard macOS Edit menu.
///
/// Each entry uses a real wxWidgets edit ID, so wxWidgets wires it to the matching
/// native macOS selector. AppKit handles enabling/disabling and routing to the
/// focused control.
///
/// Because a `copy:` item is present, AppKit  appends its own
/// items, like "Emoji & Symbols" and "Start Dictation".
#[cfg(target_os = "macos")]
pub fn create_edit_menu() -> Menu {
	// TRANSLATORS: Standard "Undo" edit menu item label
	let undo_label = t("&Undo\tCtrl+Z");
	// TRANSLATORS: Standard "Redo" edit menu item label
	let redo_label = t("&Redo\tCtrl+Shift+Z");
	// TRANSLATORS: Standard "Cut" edit menu item label
	let cut_label = t("Cu&t\tCtrl+X");
	// TRANSLATORS: Standard "Copy" edit menu item label
	let copy_label = t("&Copy\tCtrl+C");
	// TRANSLATORS: Standard "Paste" edit menu item label
	let paste_label = t("&Paste\tCtrl+V");
	// TRANSLATORS: Standard "Delete" edit menu item label
	let delete_label = t("&Delete");
	// TRANSLATORS: Standard "Select All" edit menu item label
	let select_all_label = t("Select &All\tCtrl+A");
	Menu::builder()
		.append_item(menu_ids::UNDO, &undo_label, "")
		.append_item(menu_ids::REDO, &redo_label, "")
		.append_separator()
		.append_item(menu_ids::CUT, &cut_label, "")
		.append_item(menu_ids::COPY, &copy_label, "")
		.append_item(menu_ids::PASTE, &paste_label, "")
		.append_item(menu_ids::DELETE, &delete_label, "")
		.append_separator()
		.append_item(menu_ids::SELECT_ALL, &select_all_label, "")
		.build()
}

pub fn create_go_menu(compact: bool) -> Menu {
	let headings_menu = create_headings_submenu();
	let bookmarks_menu = create_bookmarks_submenu();
	// TRANSLATORS: Menu item label to open the find dialog
	let find_label = t("&Find...\tCtrl+F");
	// TRANSLATORS: Status bar help text for the "Find" menu item
	let find_help = t("Find text in the document");
	// On macOS Cmd+G / Cmd+Shift+G are the standard find-next / find-previous shortcuts.
	// F3 is buried behind Fn on Mac keyboards, so swap the accelerators there.
	// TRANSLATORS: Menu item label to find the next match
	let find_next_label = if cfg!(target_os = "macos") { t("Find &Next\tCtrl+G") } else { t("Find &Next\tF3") };
	// TRANSLATORS: Status bar help text for the "Find Next" menu item
	let find_next_help = t("Find next occurrence");
	// TRANSLATORS: Menu item label to find the previous match
	let find_prev_label =
		if cfg!(target_os = "macos") { t("Find &Previous\tCtrl+Shift+G") } else { t("Find &Previous\tShift+F3") };
	// TRANSLATORS: Status bar help text for the "Find Previous" menu item
	let find_prev_help = t("Find previous occurrence");
	// On macOS Cmd+G is taken by Find Next, so use Cmd+L (Go to Line) and
	// Cmd+Shift+L (Go to Percent) — standard in most Mac editors.
	// TRANSLATORS: Menu item label to open the "go to line" dialog
	let goto_line_label =
		if cfg!(target_os = "macos") { t("Go to &line...\tCtrl+L") } else { t("Go to &line...\tCtrl+G") };
	// TRANSLATORS: Status bar help text for the "Go to line" menu item
	let goto_line_help = t("Go to a specific line");
	// TRANSLATORS: Menu item label to open the "go to percent" dialog
	let goto_percent_label = if cfg!(target_os = "macos") {
		t("Go to &percent...\tCtrl+Shift+L")
	} else {
		t("Go to &percent...\tCtrl+Shift+G")
	};
	// TRANSLATORS: Status bar help text for the "Go to percent" menu item
	let goto_percent_help = t("Go to a percentage of the document");
	// On macOS, Alt+Left/Right are reserved by AppKit for word-by-word caret
	// movement in text fields; binding history navigation there would shadow
	// them. Cmd+[ / Cmd+] are the system-standard back/forward (Safari, Finder,
	// Xcode, Preview), so use those instead.
	// TRANSLATORS: Menu item label to go back in navigation history
	let go_back_label = if cfg!(target_os = "macos") { t("Go &Back\tCtrl+[") } else { t("Go &Back\tAlt+Left") };
	// TRANSLATORS: Status bar help text for the "Go Back" menu item
	let go_back_help = t("Go back in history");
	// TRANSLATORS: Menu item label to go forward in navigation history
	let go_forward_label =
		if cfg!(target_os = "macos") { t("Go &Forward\tCtrl+]") } else { t("Go &Forward\tAlt+Right") };
	// TRANSLATORS: Status bar help text for the "Go Forward" menu item
	let go_forward_help = t("Go forward in history");
	let menu = Menu::builder()
		.append_item(menu_ids::FIND, &find_label, &find_help)
		.append_item(menu_ids::FIND_NEXT, &find_next_label, &find_next_help)
		.append_item(menu_ids::FIND_PREVIOUS, &find_prev_label, &find_prev_help)
		.append_separator()
		.append_item(menu_ids::GO_TO_LINE, &goto_line_label, &goto_line_help)
		.append_item(menu_ids::GO_TO_PERCENT, &goto_percent_label, &goto_percent_help)
		.append_separator()
		.append_item(menu_ids::GO_BACK, &go_back_label, &go_back_help)
		.append_item(menu_ids::GO_FORWARD, &go_forward_label, &go_forward_help)
		.append_separator()
		.build();
	if compact {
		// TRANSLATORS: Submenu label containing section navigation commands
		let sections_label = t("&Sections");
		// TRANSLATORS: Status bar help text for the "Sections" submenu
		let sections_help = t("Navigate by sections");
		menu.append_submenu(create_sections_submenu(), &sections_label, &sections_help);
		// TRANSLATORS: Submenu label containing heading navigation commands
		let headings_label = t("&Headings");
		// TRANSLATORS: Status bar help text for the "Headings" submenu
		let headings_help = t("Navigate by headings");
		menu.append_submenu(headings_menu, &headings_label, &headings_help);
		// TRANSLATORS: Submenu label containing page navigation commands
		let pages_label = t("&Pages");
		// TRANSLATORS: Status bar help text for the "Pages" submenu
		let pages_help = t("Navigate by pages");
		menu.append_submenu(create_pages_submenu(), &pages_label, &pages_help);
		// TRANSLATORS: Submenu label containing bookmark/note navigation commands
		let bookmarks_label = t("&Bookmarks");
		// TRANSLATORS: Status bar help text for the "Bookmarks" submenu
		let bookmarks_help = t("Navigate by bookmarks");
		menu.append_submenu(bookmarks_menu, &bookmarks_label, &bookmarks_help);
		// TRANSLATORS: Submenu label containing link navigation commands
		let links_label = t("&Links");
		// TRANSLATORS: Status bar help text for the "Links" submenu
		let links_help = t("Navigate by links");
		menu.append_submenu(create_links_submenu(), &links_label, &links_help);
		// TRANSLATORS: Submenu label containing image navigation commands
		let images_label = t("&Images");
		// TRANSLATORS: Status bar help text for the "Images" submenu
		let images_help = t("Navigate by images");
		menu.append_submenu(create_images_submenu(), &images_label, &images_help);
		// TRANSLATORS: Submenu label containing figure navigation commands
		let figures_label = t("&Figures");
		// TRANSLATORS: Status bar help text for the "Figures" submenu
		let figures_help = t("Navigate by figures");
		menu.append_submenu(create_figures_submenu(), &figures_label, &figures_help);
		// TRANSLATORS: Submenu label containing table navigation commands
		let tables_label = t("&Tables");
		// TRANSLATORS: Status bar help text for the "Tables" submenu
		let tables_help = t("Navigate by tables");
		menu.append_submenu(create_tables_submenu(), &tables_label, &tables_help);
		// TRANSLATORS: Submenu label containing separator-element navigation commands
		let separators_label = t("&Separators");
		// TRANSLATORS: Status bar help text for the "Separators" submenu
		let separators_help = t("Navigate by separators");
		menu.append_submenu(create_separators_submenu(), &separators_label, &separators_help);
		// TRANSLATORS: Submenu label containing list navigation commands
		let lists_label = t("&Lists");
		// TRANSLATORS: Status bar help text for the "Lists" submenu
		let lists_help = t("Navigate by lists");
		menu.append_submenu(create_lists_submenu(), &lists_label, &lists_help);
		// TRANSLATORS: Submenu label containing container (list/table) navigation commands
		let containers_label = t("&Containers");
		// TRANSLATORS: Status bar help text for the "Containers" submenu
		let containers_help = t("Navigate by containers");
		menu.append_submenu(create_containers_submenu(), &containers_label, &containers_help);
	} else {
		append_sections_items(&menu);
		menu.append_separator();
		append_headings_items(&menu);
		menu.append_separator();
		append_pages_items(&menu);
		menu.append_separator();
		append_bookmarks_items(&menu);
		menu.append_separator();
		append_links_items(&menu);
		menu.append_separator();
		append_images_items(&menu);
		menu.append_separator();
		append_figures_items(&menu);
		menu.append_separator();
		append_tables_items(&menu);
		menu.append_separator();
		append_separators_items(&menu);
		menu.append_separator();
		append_lists_items(&menu);
		menu.append_separator();
		append_containers_items(&menu);
	}
	menu
}

pub fn create_tools_menu(config: &ConfigManager) -> Menu {
	// TRANSLATORS: Menu item label to import bookmark/position data for the document
	let import_label = t("&Import Document Data...\tCtrl+Shift+I");
	// TRANSLATORS: Status bar help text for the "Import Document Data" menu item
	let import_help = t("Import bookmarks and position");
	// TRANSLATORS: Menu item label to export bookmark/position data for the document
	let export_label = t("&Export Document Data...\tCtrl+Shift+E");
	// TRANSLATORS: Status bar help text for the "Export Document Data" menu item
	let export_help = t("Export bookmarks and position");
	// TRANSLATORS: Menu item label to export the document as plain text
	let export_text_label = t("Export to &Plain Text...\tCtrl+E");
	// TRANSLATORS: Status bar help text for the "Export to Plain Text" menu item
	let export_text_help = t("Export document as plain text");
	// TRANSLATORS: Menu item label to export the document as HTML
	let export_html_label = t("Export to &HTML...");
	// TRANSLATORS: Status bar help text for the "Export to HTML" menu item
	let export_html_help = t("Export document as HTML");
	// TRANSLATORS: Menu item label to export the document as Markdown
	let export_markdown_label = t("Export to &Markdown...");
	// TRANSLATORS: Status bar help text for the "Export to Markdown" menu item
	let export_markdown_help = t("Export document as Markdown");
	let import_export_menu = Menu::builder()
		.append_item(menu_ids::IMPORT_DOCUMENT_DATA, &import_label, &import_help)
		.append_item(menu_ids::EXPORT_DOCUMENT_DATA, &export_label, &export_help)
		.append_separator()
		.append_item(menu_ids::EXPORT_TO_PLAIN_TEXT, &export_text_label, &export_text_help)
		.append_item(menu_ids::EXPORT_TO_HTML, &export_html_label, &export_html_help)
		.append_item(menu_ids::EXPORT_TO_MARKDOWN, &export_markdown_label, &export_markdown_help)
		.build();
	// On macOS, Cmd+W is close, so use Ctrl+W (raw Control key) for word count.
	// TRANSLATORS: Menu item label to show the word count dialog
	let word_count_label =
		if cfg!(target_os = "macos") { t("&Word Count\tRawCtrl+W") } else { t("&Word Count\tCtrl+W") };
	// TRANSLATORS: Status bar help text for the "Word Count" menu item
	let word_count_help = t("Show word count");
	// TRANSLATORS: Menu item label to show the document info dialog
	let doc_info_label = t("Document &Info\tCtrl+I");
	// TRANSLATORS: Status bar help text for the "Document Info" menu item
	let doc_info_help = t("Show document information");
	// TRANSLATORS: Menu item label to show the table of contents dialog
	let toc_label = t("&Table of Contents\tCtrl+T");
	// TRANSLATORS: Status bar help text for the "Table of Contents" menu item
	let toc_help = t("Show table of contents");
	// TRANSLATORS: Menu item label to show the elements list dialog
	let elements_label = t("&Elements List...\tF7");
	// TRANSLATORS: Status bar help text for the "Elements List" menu item
	let elements_help = t("Show elements list");
	// TRANSLATORS: Menu item label to reveal the document file in the system file manager
	let open_folder_label = t("Reveal &File in Folder\tCtrl+Shift+C");
	// TRANSLATORS: Status bar help text for the "Reveal File in Folder" menu item
	let open_folder_help = t("Reveal document in the file manager");
	// TRANSLATORS: Menu item label to open the document in the web view
	let web_view_label = t("Open in &Web View\tCtrl+Shift+V");
	// TRANSLATORS: Status bar help text for the "Open in Web View" menu item
	let web_view_help = t("Open document in web view");
	// TRANSLATORS: Menu item label to open the document's raw source in a new tab
	let view_source_label = t("View &Source\tCtrl+U");
	// TRANSLATORS: Status bar help text for the "View Source" menu item
	let view_source_help = t("Open the document source in a new tab");
	let menu = Menu::builder()
		.append_item(menu_ids::WORD_COUNT, &word_count_label, &word_count_help)
		.append_item(menu_ids::DOCUMENT_INFO, &doc_info_label, &doc_info_help)
		.append_separator()
		.append_item(menu_ids::TABLE_OF_CONTENTS, &toc_label, &toc_help)
		.append_item(menu_ids::ELEMENTS_LIST, &elements_label, &elements_help)
		.append_separator()
		.append_item(menu_ids::REVEAL_FILE_IN_FOLDER, &open_folder_label, &open_folder_help)
		.append_item(menu_ids::OPEN_IN_WEB_VIEW, &web_view_label, &web_view_help)
		.append_item(menu_ids::VIEW_SOURCE, &view_source_label, &view_source_help)
		.append_separator()
		.build();
	// TRANSLATORS: Submenu label containing import/export commands
	let import_export_label = t("I&mport/Export");
	// TRANSLATORS: Status bar help text for the "Import/Export" submenu
	let import_export_help = t("Import and export options");
	menu.append_submenu(import_export_menu, &import_export_label, &import_export_help);
	menu.append_separator();
	// TRANSLATORS: Menu item label to toggle a bookmark at the current position
	let toggle_bookmark_label = t("Toggle &Bookmark\tCtrl+Shift+B");
	// TRANSLATORS: Menu item label to add a bookmark with an attached note
	let bookmark_note_label = t("Bookmark with &Note\tCtrl+Shift+N");
	menu.append(menu_ids::TOGGLE_BOOKMARK, &toggle_bookmark_label, "", ItemKind::Normal);
	menu.append(menu_ids::BOOKMARK_WITH_NOTE, &bookmark_note_label, "", ItemKind::Normal);
	menu.append_separator();
	// TRANSLATORS: Checkable menu item label to toggle word wrap
	let word_wrap_label = t("Word w&rap\tCtrl+Alt+W");
	// TRANSLATORS: Status bar help text for the "Word wrap" menu item
	let word_wrap_help = t("Toggle word wrap");
	menu.append(menu_ids::TOGGLE_WORD_WRAP, &word_wrap_label, &word_wrap_help, ItemKind::Check);
	menu.check_item(menu_ids::TOGGLE_WORD_WRAP, config.get_app_bool("word_wrap", false));
	menu.append_separator();
	// TRANSLATORS: Menu item label to open the application options/preferences dialog
	let options_label = t("&Options\tCtrl+,");
	// TRANSLATORS: Menu item label to open the sleep timer dialog
	let sleep_label = t("&Sleep Timer...\tCtrl+Shift+S");
	// On macOS, use wxID_PREFERENCES so wxWidgets puts it in the app menu.
	let options_id = if cfg!(target_os = "macos") { menu_ids::PREFERENCES } else { menu_ids::OPTIONS };
	menu.append(options_id, &options_label, "", ItemKind::Normal);
	menu.append(menu_ids::SLEEP_TIMER, &sleep_label, "", ItemKind::Normal);
	menu
}

pub fn create_help_menu() -> Menu {
	// TRANSLATORS: Menu item label to open the About dialog
	let about_label = t("&About Paperback\tCtrl+F1");
	// TRANSLATORS: Status bar help text for the "About" menu item
	let about_help = t("About this application");
	// TRANSLATORS: Menu item label to open the help documentation in a web browser
	let help_browser_label = t("View Help in &Browser\tF1");
	// TRANSLATORS: Status bar help text for the "View Help in Browser" menu item
	let help_browser_help = t("View help in default browser");
	// TRANSLATORS: Menu item label to open the help documentation inside Paperback itself
	let help_paperback_label = t("View Help in &Paperback\tShift+F1");
	// TRANSLATORS: Status bar help text for the "View Help in Paperback" menu item
	let help_paperback_help = t("View help in Paperback");
	// TRANSLATORS: Menu item label to check for application updates
	let updates_label = t("Check for &Updates\tCtrl+Shift+U");
	// TRANSLATORS: Status bar help text for the "Check for Updates" menu item
	let updates_help = t("Check for updates");
	// TRANSLATORS: Menu item label to open the donation page
	let donate_label = t("&Donate\tCtrl+D");
	// TRANSLATORS: Status bar help text for the "Donate" menu item
	let donate_help = t("Support Paperback development");
	Menu::builder()
		.append_item(menu_ids::ABOUT, &about_label, &about_help)
		.append_item(menu_ids::VIEW_HELP_BROWSER, &help_browser_label, &help_browser_help)
		.append_item(menu_ids::VIEW_HELP_PAPERBACK, &help_paperback_label, &help_paperback_help)
		.append_separator()
		.append_item(menu_ids::CHECK_FOR_UPDATES, &updates_label, &updates_help)
		.append_separator()
		.append_item(menu_ids::DONATE, &donate_label, &donate_help)
		.build()
}

pub fn populate_recent_documents_menu(menu: &Menu, config: &ConfigManager) {
	let recent_docs = recent_documents_for_menu(config);
	if recent_docs.is_empty() {
		// TRANSLATORS: Placeholder menu item shown in the Recent Documents submenu when there are none
		let empty_label = t("(No recent documents)");
		if let Some(item) = menu.append(wxdragon::id::ID_ANY.try_into().unwrap(), &empty_label, "", ItemKind::Normal) {
			item.enable(false);
		}
	} else {
		for (index, path) in recent_docs.iter().enumerate() {
			let filename =
				Path::new(path).file_name().map_or_else(|| path.clone(), |s| s.to_string_lossy().to_string());
			let label = format!("&{} {}", index + 1, filename);
			if let Ok(offset) = i32::try_from(index) {
				let id = menu_ids::RECENT_DOCUMENT_BASE + offset;
				let _ = menu.append(id, &label, path, ItemKind::Normal);
			}
		}
	}
	menu.append_separator();
	// TRANSLATORS: Menu item label to open the full recent-documents list dialog
	let show_all_label = t("Show All...\tCtrl+R");
	let _ = menu.append(menu_ids::SHOW_ALL_DOCUMENTS, &show_all_label, "", ItemKind::Normal);
}

pub fn recent_documents_for_menu(config: &ConfigManager) -> Vec<String> {
	let limit = usize::try_from(config.get_app_int("recent_documents_to_show", 25).max(0)).unwrap_or(0);
	let mut docs = config.get_recent_documents();
	if docs.len() > limit {
		docs.truncate(limit);
	}
	docs
}
