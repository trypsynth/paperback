use std::path::Path;

use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::menu_ids;

const DOCUMENT_DEPENDENT_IDS: &[i32] = &[
	menu_ids::CLOSE,
	menu_ids::CLOSE_ALL,
	menu_ids::FIND,
	menu_ids::FIND_NEXT,
	menu_ids::FIND_PREVIOUS,
	menu_ids::GO_TO_LINE,
	menu_ids::GO_TO_PERCENT,
	menu_ids::GO_TO_PAGE,
	menu_ids::GO_BACK,
	menu_ids::GO_FORWARD,
	menu_ids::PREVIOUS_SECTION,
	menu_ids::NEXT_SECTION,
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
	menu_ids::PREVIOUS_PAGE,
	menu_ids::NEXT_PAGE,
	menu_ids::PREVIOUS_BOOKMARK,
	menu_ids::NEXT_BOOKMARK,
	menu_ids::PREVIOUS_NOTE,
	menu_ids::NEXT_NOTE,
	menu_ids::JUMP_TO_ALL_BOOKMARKS,
	menu_ids::JUMP_TO_BOOKMARKS_ONLY,
	menu_ids::JUMP_TO_NOTES_ONLY,
	menu_ids::VIEW_NOTE_TEXT,
	menu_ids::PREVIOUS_LINK,
	menu_ids::NEXT_LINK,
	menu_ids::PREVIOUS_IMAGE,
	menu_ids::NEXT_IMAGE,
	menu_ids::PREVIOUS_FIGURE,
	menu_ids::NEXT_FIGURE,
	menu_ids::PREVIOUS_TABLE,
	menu_ids::NEXT_TABLE,
	menu_ids::PREVIOUS_SEPARATOR,
	menu_ids::NEXT_SEPARATOR,
	menu_ids::PREVIOUS_LIST,
	menu_ids::NEXT_LIST,
	menu_ids::PREVIOUS_LIST_ITEM,
	menu_ids::NEXT_LIST_ITEM,
	menu_ids::CONTAINER_START,
	menu_ids::CONTAINER_END,
	menu_ids::WORD_COUNT,
	menu_ids::DOCUMENT_INFO,
	menu_ids::TABLE_OF_CONTENTS,
	menu_ids::ELEMENTS_LIST,
	menu_ids::REVEAL_FILE_IN_FOLDER,
	menu_ids::OPEN_IN_WEB_VIEW,
	menu_ids::VIEW_SOURCE,
	menu_ids::IMPORT_DOCUMENT_DATA,
	menu_ids::EXPORT_DOCUMENT_DATA,
	menu_ids::EXPORT_TO_PLAIN_TEXT,
	menu_ids::TOGGLE_BOOKMARK,
	menu_ids::BOOKMARK_WITH_NOTE,
];

pub fn update_menu_item_states(frame: &Frame, has_document: bool) {
	let Some(menu_bar) = frame.get_menu_bar() else {
		return;
	};
	for &id in DOCUMENT_DEPENDENT_IDS {
		menu_bar.enable_item(id, has_document);
	}
}

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

pub fn format_menu_label(base: &str, action: ActionId, config: &ConfigManager) -> String {
	let shortcut = config.get_shortcut_menu_str(action);
	if shortcut.is_empty() { base.to_string() } else { format!("{base}\t{shortcut}") }
}

pub fn sections_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_section_label = format_menu_label(&t("Previous Section"), ActionId::PreviousSection, config);
	let prev_section_help = t("Go to previous section");
	let next_section_label = format_menu_label(&t("Next Section"), ActionId::NextSection, config);
	let next_section_help = t("Go to next section");
	vec![
		item_with_help(menu_ids::PREVIOUS_SECTION, prev_section_label, prev_section_help),
		item_with_help(menu_ids::NEXT_SECTION, next_section_label, next_section_help),
	]
}

pub fn pages_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let goto_page_label = format_menu_label(&t("Go to &Page"), ActionId::GoToPage, config);
	let prev_page_label = format_menu_label(&t("Previous Pa&ge"), ActionId::PreviousPage, config);
	let next_page_label = format_menu_label(&t("Next Pag&e"), ActionId::NextPage, config);
	vec![
		item(menu_ids::GO_TO_PAGE, goto_page_label),
		item(menu_ids::PREVIOUS_PAGE, prev_page_label),
		item(menu_ids::NEXT_PAGE, next_page_label),
	]
}

pub fn links_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_link_label = format_menu_label(&t("Previous Lin&k"), ActionId::PreviousLink, config);
	let next_link_label = format_menu_label(&t("Next Lin&k"), ActionId::NextLink, config);
	vec![item(menu_ids::PREVIOUS_LINK, prev_link_label), item(menu_ids::NEXT_LINK, next_link_label)]
}

pub fn images_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_image_label = format_menu_label(&t("Previous Ima&ge"), ActionId::PreviousImage, config);
	let next_image_label = format_menu_label(&t("Next Ima&ge"), ActionId::NextImage, config);
	vec![item(menu_ids::PREVIOUS_IMAGE, prev_image_label), item(menu_ids::NEXT_IMAGE, next_image_label)]
}

pub fn figures_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_figure_label = format_menu_label(&t("Previous Figu&re"), ActionId::PreviousFigure, config);
	let next_figure_label = format_menu_label(&t("Next Figu&re"), ActionId::NextFigure, config);
	vec![item(menu_ids::PREVIOUS_FIGURE, prev_figure_label), item(menu_ids::NEXT_FIGURE, next_figure_label)]
}

pub fn tables_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_table_label = format_menu_label(&t("Previous &Table"), ActionId::PreviousTable, config);
	let next_table_label = format_menu_label(&t("Next &Table"), ActionId::NextTable, config);
	vec![item(menu_ids::PREVIOUS_TABLE, prev_table_label), item(menu_ids::NEXT_TABLE, next_table_label)]
}

pub fn separators_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_separator_label = format_menu_label(&t("Previous Se&parator"), ActionId::PreviousSeparator, config);
	let next_separator_label = format_menu_label(&t("Next Se&parator"), ActionId::NextSeparator, config);
	vec![item(menu_ids::PREVIOUS_SEPARATOR, prev_separator_label), item(menu_ids::NEXT_SEPARATOR, next_separator_label)]
}

pub fn lists_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_list_label = format_menu_label(&t("Previous L&ist"), ActionId::PreviousList, config);
	let next_list_label = format_menu_label(&t("Next L&ist"), ActionId::NextList, config);
	let prev_list_item_label = format_menu_label(&t("Previous List &Item"), ActionId::PreviousListItem, config);
	let next_list_item_label = format_menu_label(&t("Next List I&tem"), ActionId::NextListItem, config);
	vec![
		item(menu_ids::PREVIOUS_LIST, prev_list_label),
		item(menu_ids::NEXT_LIST, next_list_label),
		item(menu_ids::PREVIOUS_LIST_ITEM, prev_list_item_label),
		item(menu_ids::NEXT_LIST_ITEM, next_list_item_label),
	]
}

pub fn containers_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let container_start_label = format_menu_label(&t("Container &Start"), ActionId::ContainerStart, config);
	let container_start_help = t("Go to the start of the current list or table");
	let container_end_label = format_menu_label(&t("Past Container &End"), ActionId::ContainerEnd, config);
	let container_end_help = t("Go past the end of the current list or table");
	vec![
		item_with_help(menu_ids::CONTAINER_START, container_start_label, container_start_help),
		item_with_help(menu_ids::CONTAINER_END, container_end_label, container_end_help),
	]
}

pub fn headings_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_heading_label = format_menu_label(&t("&Previous Heading"), ActionId::PreviousHeading, config);
	let prev_heading_help = t("Go to previous heading");
	let next_heading_label = format_menu_label(&t("&Next Heading"), ActionId::NextHeading, config);
	let next_heading_help = t("Go to next heading");
	let prev_heading1_label = format_menu_label(&t("Previous Heading Level &1"), ActionId::PreviousHeading1, config);
	let next_heading1_label = format_menu_label(&t("Next Heading Level 1"), ActionId::NextHeading1, config);
	let prev_heading2_label = format_menu_label(&t("Previous Heading Level &2"), ActionId::PreviousHeading2, config);
	let next_heading2_label = format_menu_label(&t("Next Heading Level 2"), ActionId::NextHeading2, config);
	let prev_heading3_label = format_menu_label(&t("Previous Heading Level &3"), ActionId::PreviousHeading3, config);
	let next_heading3_label = format_menu_label(&t("Next Heading Level 3"), ActionId::NextHeading3, config);
	let prev_heading4_label = format_menu_label(&t("Previous Heading Level &4"), ActionId::PreviousHeading4, config);
	let next_heading4_label = format_menu_label(&t("Next Heading Level 4"), ActionId::NextHeading4, config);
	let prev_heading5_label = format_menu_label(&t("Previous Heading Level &5"), ActionId::PreviousHeading5, config);
	let next_heading5_label = format_menu_label(&t("Next Heading Level 5"), ActionId::NextHeading5, config);
	let prev_heading6_label = format_menu_label(&t("Previous Heading Level &6"), ActionId::PreviousHeading6, config);
	let next_heading6_label = format_menu_label(&t("Next Heading Level 6"), ActionId::NextHeading6, config);
	vec![
		item_with_help(menu_ids::PREVIOUS_HEADING, prev_heading_label, prev_heading_help),
		item_with_help(menu_ids::NEXT_HEADING, next_heading_label, next_heading_help),
		MenuEntry::Separator,
		item(menu_ids::PREVIOUS_HEADING_1, prev_heading1_label),
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

pub fn bookmarks_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_bookmark_label = format_menu_label(&t("&Previous Bookmark"), ActionId::PreviousBookmark, config);
	let prev_bookmark_help = t("Go to previous bookmark");
	let next_bookmark_label = format_menu_label(&t("&Next Bookmark"), ActionId::NextBookmark, config);
	let next_bookmark_help = t("Go to next bookmark");
	let prev_note_label = format_menu_label(&t("Previous &Note"), ActionId::PreviousNote, config);
	let prev_note_help = t("Go to previous note");
	let next_note_label = format_menu_label(&t("Next N&ote"), ActionId::NextNote, config);
	let next_note_help = t("Go to next note");
	let all_bookmarks_label = format_menu_label(&t("Jump to &All..."), ActionId::JumpToAllBookmarks, config);
	let all_bookmarks_help = t("Show all bookmarks and notes");
	let bookmarks_only_label =
		format_menu_label(&t("Jump to &Bookmarks Only..."), ActionId::JumpToBookmarksOnly, config);
	let bookmarks_only_help = t("Show bookmarks only");
	let notes_only_label = format_menu_label(&t("Jump to Notes &Only..."), ActionId::JumpToNotesOnly, config);
	let notes_only_help = t("Show notes only");
	let view_note_label = format_menu_label(&t("&View Note Text"), ActionId::ViewNoteText, config);
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

pub fn create_sections_submenu(config: &ConfigManager) -> Menu {
	let entries = sections_entries(config);
	build_menu(&entries)
}

pub fn append_sections_items(menu: &Menu, config: &ConfigManager) {
	let entries = sections_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_pages_submenu(config: &ConfigManager) -> Menu {
	let entries = pages_entries(config);
	build_menu(&entries)
}

pub fn append_pages_items(menu: &Menu, config: &ConfigManager) {
	let entries = pages_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_links_submenu(config: &ConfigManager) -> Menu {
	let entries = links_entries(config);
	build_menu(&entries)
}

pub fn append_links_items(menu: &Menu, config: &ConfigManager) {
	let entries = links_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_images_submenu(config: &ConfigManager) -> Menu {
	let entries = images_entries(config);
	build_menu(&entries)
}

pub fn append_images_items(menu: &Menu, config: &ConfigManager) {
	let entries = images_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_figures_submenu(config: &ConfigManager) -> Menu {
	let entries = figures_entries(config);
	build_menu(&entries)
}

pub fn append_figures_items(menu: &Menu, config: &ConfigManager) {
	let entries = figures_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_tables_submenu(config: &ConfigManager) -> Menu {
	let entries = tables_entries(config);
	build_menu(&entries)
}

pub fn append_tables_items(menu: &Menu, config: &ConfigManager) {
	let entries = tables_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_separators_submenu(config: &ConfigManager) -> Menu {
	let entries = separators_entries(config);
	build_menu(&entries)
}

pub fn append_separators_items(menu: &Menu, config: &ConfigManager) {
	let entries = separators_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_lists_submenu(config: &ConfigManager) -> Menu {
	let entries = lists_entries(config);
	build_menu(&entries)
}

pub fn append_lists_items(menu: &Menu, config: &ConfigManager) {
	let entries = lists_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_containers_submenu(config: &ConfigManager) -> Menu {
	let entries = containers_entries(config);
	build_menu(&entries)
}

pub fn append_containers_items(menu: &Menu, config: &ConfigManager) {
	let entries = containers_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_headings_submenu(config: &ConfigManager) -> Menu {
	let entries = headings_entries(config);
	build_menu(&entries)
}

pub fn append_headings_items(menu: &Menu, config: &ConfigManager) {
	let entries = headings_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_bookmarks_submenu(config: &ConfigManager) -> Menu {
	let entries = bookmarks_entries(config);
	build_menu(&entries)
}

pub fn append_bookmarks_items(menu: &Menu, config: &ConfigManager) {
	let entries = bookmarks_entries(config);
	append_menu_entries(menu, &entries);
}

pub fn create_menu_bar(config: &ConfigManager) -> MenuBar {
	let file_menu = create_file_menu(config);
	let compact_go_menu = config.get_app_bool("compact_go_menu", true);
	let go_menu = create_go_menu(config, compact_go_menu);
	let tools_menu = create_tools_menu(config);
	let help_menu = create_help_menu(config);
	let file_label = t("&File");
	let go_label = t("&Go");
	let tools_label = t("&Tools");
	let help_label = t("&Help");
	#[allow(unused_mut)]
	let mut builder = MenuBar::builder().append(file_menu, &file_label);

	#[cfg(target_os = "macos")]
	{
		let edit_label = t("&Edit");
		builder = builder.append(create_edit_menu(config), &edit_label);
	}
	builder.append(go_menu, &go_label).append(tools_menu, &tools_label).append(help_menu, &help_label).build()
}

pub fn create_file_menu(config: &ConfigManager) -> Menu {
	let open_label = format_menu_label(&t("&Open..."), ActionId::Open, config);
	let open_help = t("Open a document");
	let close_label = format_menu_label(&t("&Close"), ActionId::Close, config);
	let close_help = t("Close the current document");
	let close_all_label = format_menu_label(&t("Close &All"), ActionId::CloseAll, config);
	let close_all_help = t("Close all documents");
	let reopen_label = format_menu_label(&t("Reopen &Last Closed"), ActionId::ReopenLastClosed, config);
	let reopen_help = t("Reopen the last closed document");
	let file_menu = Menu::builder()
		.append_item(menu_ids::OPEN, &open_label, &open_help)
		.append_item(menu_ids::CLOSE, &close_label, &close_help)
		.append_item(menu_ids::CLOSE_ALL, &close_all_label, &close_all_help)
		.append_item(menu_ids::REOPEN_LAST_CLOSED, &reopen_label, &reopen_help)
		.build();
	let recent_menu = Menu::builder().build();
	populate_recent_documents_menu(&recent_menu, config);
	let recent_label = t("&Recent Documents");
	let recent_help = t("Open a recent document");
	let _ = file_menu.append_submenu(recent_menu, &recent_label, &recent_help);
	if !cfg!(target_os = "macos") {
		file_menu.append_separator();
		let exit_label = format_menu_label(&t("E&xit"), ActionId::Exit, config);
		let exit_help = t("Exit the application");
		let _ = file_menu.append(menu_ids::EXIT, &exit_label, &exit_help, ItemKind::Normal);
	}
	file_menu
}

#[cfg(target_os = "macos")]
pub fn create_edit_menu(_config: &ConfigManager) -> Menu {
	let undo_label = t("&Undo\tCtrl+Z");
	let redo_label = t("&Redo\tCtrl+Shift+Z");
	let cut_label = t("Cu&t\tCtrl+X");
	let copy_label = t("&Copy\tCtrl+C");
	let paste_label = t("&Paste\tCtrl+V");
	let delete_label = t("&Delete");
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

pub fn create_go_menu(config: &ConfigManager, compact: bool) -> Menu {
	let headings_menu = create_headings_submenu(config);
	let bookmarks_menu = create_bookmarks_submenu(config);
	let find_label = format_menu_label(&t("&Find..."), ActionId::Find, config);
	let find_help = t("Find text in the document");
	let find_next_label = format_menu_label(&t("Find &Next"), ActionId::FindNext, config);
	let find_next_help = t("Find next occurrence");
	let find_prev_label = format_menu_label(&t("Find &Previous"), ActionId::FindPrevious, config);
	let find_prev_help = t("Find previous occurrence");
	let goto_line_label = format_menu_label(&t("Go to &line..."), ActionId::GoToLine, config);
	let goto_line_help = t("Go to a specific line");
	let goto_percent_label = format_menu_label(&t("Go to &percent..."), ActionId::GoToPercent, config);
	let goto_percent_help = t("Go to a percentage of the document");
	let go_back_label = format_menu_label(&t("Go &Back"), ActionId::GoBack, config);
	let go_back_help = t("Go back in history");
	let go_forward_label = format_menu_label(&t("Go &Forward"), ActionId::GoForward, config);
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
		let sections_label = t("&Sections");
		let sections_help = t("Navigate by sections");
		menu.append_submenu(create_sections_submenu(config), &sections_label, &sections_help);
		let headings_label = t("&Headings");
		let headings_help = t("Navigate by headings");
		menu.append_submenu(headings_menu, &headings_label, &headings_help);
		let pages_label = t("&Pages");
		let pages_help = t("Navigate by pages");
		menu.append_submenu(create_pages_submenu(config), &pages_label, &pages_help);
		let bookmarks_label = t("&Bookmarks");
		let bookmarks_help = t("Navigate by bookmarks");
		menu.append_submenu(bookmarks_menu, &bookmarks_label, &bookmarks_help);
		let links_label = t("&Links");
		let links_help = t("Navigate by links");
		menu.append_submenu(create_links_submenu(config), &links_label, &links_help);
		let images_label = t("&Images");
		let images_help = t("Navigate by images");
		menu.append_submenu(create_images_submenu(config), &images_label, &images_help);
		let figures_label = t("&Figures");
		let figures_help = t("Navigate by figures");
		menu.append_submenu(create_figures_submenu(config), &figures_label, &figures_help);
		let tables_label = t("&Tables");
		let tables_help = t("Navigate by tables");
		menu.append_submenu(create_tables_submenu(config), &tables_label, &tables_help);
		let separators_label = t("&Separators");
		let separators_help = t("Navigate by separators");
		menu.append_submenu(create_separators_submenu(config), &separators_label, &separators_help);
		let lists_label = t("&Lists");
		let lists_help = t("Navigate by lists");
		menu.append_submenu(create_lists_submenu(config), &lists_label, &lists_help);
		let containers_label = t("&Containers");
		let containers_help = t("Navigate by containers");
		menu.append_submenu(create_containers_submenu(config), &containers_label, &containers_help);
	} else {
		append_sections_items(&menu, config);
		menu.append_separator();
		append_headings_items(&menu, config);
		menu.append_separator();
		append_pages_items(&menu, config);
		menu.append_separator();
		append_bookmarks_items(&menu, config);
		menu.append_separator();
		append_links_items(&menu, config);
		menu.append_separator();
		append_images_items(&menu, config);
		menu.append_separator();
		append_figures_items(&menu, config);
		menu.append_separator();
		append_tables_items(&menu, config);
		menu.append_separator();
		append_separators_items(&menu, config);
		menu.append_separator();
		append_lists_items(&menu, config);
		menu.append_separator();
		append_containers_items(&menu, config);
	}
	menu
}

pub fn create_tools_menu(config: &ConfigManager) -> Menu {
	let import_label = format_menu_label(&t("&Import Document Data..."), ActionId::ImportDocumentData, config);
	let import_help = t("Import bookmarks and position");
	let export_label = format_menu_label(&t("&Export Document Data..."), ActionId::ExportDocumentData, config);
	let export_help = t("Export bookmarks and position");
	let export_text_label = format_menu_label(&t("Export to &Plain Text..."), ActionId::ExportToPlainText, config);
	let export_text_help = t("Export document as plain text");
	let export_html_label = format_menu_label(&t("Export to &HTML..."), ActionId::ExportToHtml, config);
	let export_html_help = t("Export document as HTML");
	let export_markdown_label = format_menu_label(&t("Export to &Markdown..."), ActionId::ExportToMarkdown, config);
	let export_markdown_help = t("Export document as Markdown");
	let import_export_menu = Menu::builder()
		.append_item(menu_ids::IMPORT_DOCUMENT_DATA, &import_label, &import_help)
		.append_item(menu_ids::EXPORT_DOCUMENT_DATA, &export_label, &export_help)
		.append_separator()
		.append_item(menu_ids::EXPORT_TO_PLAIN_TEXT, &export_text_label, &export_text_help)
		.append_item(menu_ids::EXPORT_TO_HTML, &export_html_label, &export_html_help)
		.append_item(menu_ids::EXPORT_TO_MARKDOWN, &export_markdown_label, &export_markdown_help)
		.build();
	let word_count_label = format_menu_label(&t("&Word Count"), ActionId::WordCount, config);
	let word_count_help = t("Show word count");
	let doc_info_label = format_menu_label(&t("Document &Info"), ActionId::DocumentInfo, config);
	let doc_info_help = t("Show document information");
	let toc_label = format_menu_label(&t("&Table of Contents"), ActionId::TableOfContents, config);
	let toc_help = t("Show table of contents");
	let elements_label = format_menu_label(&t("&Elements List..."), ActionId::ElementsList, config);
	let elements_help = t("Show elements list");
	let open_folder_label = format_menu_label(&t("Reveal &File in Folder"), ActionId::RevealFileInFolder, config);
	let open_folder_help = t("Reveal document in the file manager");
	let web_view_label = format_menu_label(&t("Open in &Web View"), ActionId::OpenInWebView, config);
	let web_view_help = t("Open document in web view");
	let view_source_label = format_menu_label(&t("View &Source"), ActionId::ViewSource, config);
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
	let import_export_label = t("I&mport/Export");
	let import_export_help = t("Import and export options");
	menu.append_submenu(import_export_menu, &import_export_label, &import_export_help);
	menu.append_separator();
	let toggle_bookmark_label = format_menu_label(&t("Toggle &Bookmark"), ActionId::ToggleBookmark, config);
	let bookmark_note_label = format_menu_label(&t("Bookmark with &Note"), ActionId::BookmarkWithNote, config);
	menu.append(menu_ids::TOGGLE_BOOKMARK, &toggle_bookmark_label, "", ItemKind::Normal);
	menu.append(menu_ids::BOOKMARK_WITH_NOTE, &bookmark_note_label, "", ItemKind::Normal);
	menu.append_separator();
	let word_wrap_label = format_menu_label(&t("Word w&rap"), ActionId::ToggleWordWrap, config);
	let word_wrap_help = t("Toggle word wrap");
	menu.append(menu_ids::TOGGLE_WORD_WRAP, &word_wrap_label, &word_wrap_help, ItemKind::Check);
	menu.check_item(menu_ids::TOGGLE_WORD_WRAP, config.get_app_bool("word_wrap", false));
	let full_screen_label = format_menu_label(&t("&Full Screen"), ActionId::ToggleFullScreen, config);
	let full_screen_help = t("Toggle full screen");
	menu.append(menu_ids::TOGGLE_FULL_SCREEN, &full_screen_label, &full_screen_help, ItemKind::Check);
	menu.append_separator();
	let options_label = format_menu_label(&t("&Options"), ActionId::Options, config);
	let shortcuts_label =
		format_menu_label(&t("Customize &Keyboard Shortcuts..."), ActionId::CustomizeShortcuts, config);
	let sleep_label = format_menu_label(&t("&Sleep Timer..."), ActionId::SleepTimer, config);
	let options_id = if cfg!(target_os = "macos") { menu_ids::PREFERENCES } else { menu_ids::OPTIONS };
	menu.append(options_id, &options_label, "", ItemKind::Normal);
	menu.append(menu_ids::CUSTOMIZE_SHORTCUTS, &shortcuts_label, "", ItemKind::Normal);
	menu.append(menu_ids::SLEEP_TIMER, &sleep_label, "", ItemKind::Normal);
	menu
}

pub fn create_help_menu(config: &ConfigManager) -> Menu {
	let about_label = format_menu_label(&t("&About Paperback"), ActionId::About, config);
	let about_help = t("About this application");
	let help_browser_label = format_menu_label(&t("View Help in &Browser"), ActionId::ViewHelpBrowser, config);
	let help_browser_help = t("View help in default browser");
	let help_paperback_label = format_menu_label(&t("View Help in &Paperback"), ActionId::ViewHelpPaperback, config);
	let help_paperback_help = t("View help in Paperback");
	let updates_label = format_menu_label(&t("Check for &Updates"), ActionId::CheckForUpdates, config);
	let updates_help = t("Check for updates");
	let donate_label = format_menu_label(&t("&Donate"), ActionId::Donate, config);
	let donate_help = t("Support Paperback development");
	#[cfg_attr(target_os = "macos", allow(unused_mut))]
	let mut builder = Menu::builder()
		.append_item(menu_ids::ABOUT, &about_label, &about_help)
		.append_item(menu_ids::VIEW_HELP_BROWSER, &help_browser_label, &help_browser_help)
		.append_item(menu_ids::VIEW_HELP_PAPERBACK, &help_paperback_label, &help_paperback_help)
		.append_separator();
	#[cfg(not(target_os = "macos"))]
	{
		builder = builder.append_item(menu_ids::CHECK_FOR_UPDATES, &updates_label, &updates_help).append_separator();
	}
	builder.append_item(menu_ids::DONATE, &donate_label, &donate_help).build()
}

pub fn populate_recent_documents_menu(menu: &Menu, config: &ConfigManager) {
	let recent_docs = recent_documents_for_menu(config);
	if recent_docs.is_empty() {
		let empty_label = t("(No recent documents)");
		if let Some(item) = menu.append(ID_ANY.try_into().unwrap(), &empty_label, "", ItemKind::Normal) {
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
	let show_all_label = format_menu_label(&t("Show All..."), ActionId::ShowAllRecentDocuments, config);
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
