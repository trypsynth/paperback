use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::builder::{MenuEntry, append_menu_entries, build_menu, format_menu_label, item, item_with_help};
use crate::ui::menu_ids;

fn sections_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_section_label = format_menu_label(&t("Previous Section"), ActionId::PreviousSection, config);
	let prev_section_help = t("Go to previous section");
	let next_section_label = format_menu_label(&t("Next Section"), ActionId::NextSection, config);
	let next_section_help = t("Go to next section");
	vec![
		item_with_help(menu_ids::PREVIOUS_SECTION, prev_section_label, prev_section_help),
		item_with_help(menu_ids::NEXT_SECTION, next_section_label, next_section_help),
	]
}

fn pages_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let goto_page_label = format_menu_label(&t("Go to &Page"), ActionId::GoToPage, config);
	let prev_page_label = format_menu_label(&t("Previous Pa&ge"), ActionId::PreviousPage, config);
	let next_page_label = format_menu_label(&t("Next Pag&e"), ActionId::NextPage, config);
	vec![
		item(menu_ids::GO_TO_PAGE, goto_page_label),
		item(menu_ids::PREVIOUS_PAGE, prev_page_label),
		item(menu_ids::NEXT_PAGE, next_page_label),
	]
}

fn links_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_link_label = format_menu_label(&t("Previous Lin&k"), ActionId::PreviousLink, config);
	let next_link_label = format_menu_label(&t("Next Lin&k"), ActionId::NextLink, config);
	vec![item(menu_ids::PREVIOUS_LINK, prev_link_label), item(menu_ids::NEXT_LINK, next_link_label)]
}

fn images_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_image_label = format_menu_label(&t("Previous Ima&ge"), ActionId::PreviousImage, config);
	let next_image_label = format_menu_label(&t("Next Ima&ge"), ActionId::NextImage, config);
	vec![item(menu_ids::PREVIOUS_IMAGE, prev_image_label), item(menu_ids::NEXT_IMAGE, next_image_label)]
}

fn figures_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_figure_label = format_menu_label(&t("Previous Figu&re"), ActionId::PreviousFigure, config);
	let next_figure_label = format_menu_label(&t("Next Figu&re"), ActionId::NextFigure, config);
	vec![item(menu_ids::PREVIOUS_FIGURE, prev_figure_label), item(menu_ids::NEXT_FIGURE, next_figure_label)]
}

fn tables_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_table_label = format_menu_label(&t("Previous &Table"), ActionId::PreviousTable, config);
	let next_table_label = format_menu_label(&t("Next &Table"), ActionId::NextTable, config);
	vec![item(menu_ids::PREVIOUS_TABLE, prev_table_label), item(menu_ids::NEXT_TABLE, next_table_label)]
}

fn separators_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let prev_separator_label = format_menu_label(&t("Previous Se&parator"), ActionId::PreviousSeparator, config);
	let next_separator_label = format_menu_label(&t("Next Se&parator"), ActionId::NextSeparator, config);
	vec![item(menu_ids::PREVIOUS_SEPARATOR, prev_separator_label), item(menu_ids::NEXT_SEPARATOR, next_separator_label)]
}

fn lists_entries(config: &ConfigManager) -> Vec<MenuEntry> {
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

fn containers_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let container_start_label = format_menu_label(&t("Container &Start"), ActionId::ContainerStart, config);
	let container_start_help = t("Go to the start of the current list or table");
	let container_end_label = format_menu_label(&t("Past Container &End"), ActionId::ContainerEnd, config);
	let container_end_help = t("Go past the end of the current list or table");
	vec![
		item_with_help(menu_ids::CONTAINER_START, container_start_label, container_start_help),
		item_with_help(menu_ids::CONTAINER_END, container_end_label, container_end_help),
	]
}

fn headings_entries(config: &ConfigManager) -> Vec<MenuEntry> {
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

fn bookmarks_entries(config: &ConfigManager) -> Vec<MenuEntry> {
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

fn create_sections_submenu(config: &ConfigManager) -> Menu {
	let entries = sections_entries(config);
	build_menu(&entries)
}

fn append_sections_items(menu: &Menu, config: &ConfigManager) {
	let entries = sections_entries(config);
	append_menu_entries(menu, &entries);
}

fn create_pages_submenu(config: &ConfigManager) -> Menu {
	let entries = pages_entries(config);
	build_menu(&entries)
}

fn append_pages_items(menu: &Menu, config: &ConfigManager) {
	let entries = pages_entries(config);
	append_menu_entries(menu, &entries);
}

fn create_links_submenu(config: &ConfigManager) -> Menu {
	let entries = links_entries(config);
	build_menu(&entries)
}

fn append_links_items(menu: &Menu, config: &ConfigManager) {
	let entries = links_entries(config);
	append_menu_entries(menu, &entries);
}

fn create_images_submenu(config: &ConfigManager) -> Menu {
	let entries = images_entries(config);
	build_menu(&entries)
}

fn append_images_items(menu: &Menu, config: &ConfigManager) {
	let entries = images_entries(config);
	append_menu_entries(menu, &entries);
}

fn create_figures_submenu(config: &ConfigManager) -> Menu {
	let entries = figures_entries(config);
	build_menu(&entries)
}

fn append_figures_items(menu: &Menu, config: &ConfigManager) {
	let entries = figures_entries(config);
	append_menu_entries(menu, &entries);
}

fn create_tables_submenu(config: &ConfigManager) -> Menu {
	let entries = tables_entries(config);
	build_menu(&entries)
}

fn append_tables_items(menu: &Menu, config: &ConfigManager) {
	let entries = tables_entries(config);
	append_menu_entries(menu, &entries);
}

fn create_separators_submenu(config: &ConfigManager) -> Menu {
	let entries = separators_entries(config);
	build_menu(&entries)
}

fn append_separators_items(menu: &Menu, config: &ConfigManager) {
	let entries = separators_entries(config);
	append_menu_entries(menu, &entries);
}

fn create_lists_submenu(config: &ConfigManager) -> Menu {
	let entries = lists_entries(config);
	build_menu(&entries)
}

fn append_lists_items(menu: &Menu, config: &ConfigManager) {
	let entries = lists_entries(config);
	append_menu_entries(menu, &entries);
}

fn create_containers_submenu(config: &ConfigManager) -> Menu {
	let entries = containers_entries(config);
	build_menu(&entries)
}

fn append_containers_items(menu: &Menu, config: &ConfigManager) {
	let entries = containers_entries(config);
	append_menu_entries(menu, &entries);
}

fn create_headings_submenu(config: &ConfigManager) -> Menu {
	let entries = headings_entries(config);
	build_menu(&entries)
}

fn append_headings_items(menu: &Menu, config: &ConfigManager) {
	let entries = headings_entries(config);
	append_menu_entries(menu, &entries);
}

fn create_bookmarks_submenu(config: &ConfigManager) -> Menu {
	let entries = bookmarks_entries(config);
	build_menu(&entries)
}

fn append_bookmarks_items(menu: &Menu, config: &ConfigManager) {
	let entries = bookmarks_entries(config);
	append_menu_entries(menu, &entries);
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
