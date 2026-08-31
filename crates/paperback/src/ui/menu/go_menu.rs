use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::builder::{MenuEntry, append_menu_entries, build_menu, format_menu_label, item, item_with_help};
use crate::ui::menu_ids;

fn sections_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to move to the previous section of the document.
	let prev_section_label = format_menu_label(&t("Previous Section"), ActionId::PreviousSection, config);
	// TRANSLATORS: Status-bar help text for the Go > Previous Section menu item.
	let prev_section_help = t("Go to previous section");
	// TRANSLATORS: Menu item in the Go menu to move to the next section of the document.
	let next_section_label = format_menu_label(&t("Next Section"), ActionId::NextSection, config);
	// TRANSLATORS: Status-bar help text for the Go > Next Section menu item.
	let next_section_help = t("Go to next section");
	vec![
		item_with_help(menu_ids::PREVIOUS_SECTION, prev_section_label, prev_section_help),
		item_with_help(menu_ids::NEXT_SECTION, next_section_label, next_section_help),
	]
}

fn pages_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to jump to a specific page number.
	let goto_page_label = format_menu_label(&t("Go to &Page"), ActionId::GoToPage, config);
	// TRANSLATORS: Menu item in the Go menu to move to the previous page.
	let prev_page_label = format_menu_label(&t("Previous Pa&ge"), ActionId::PreviousPage, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next page.
	let next_page_label = format_menu_label(&t("Next Pag&e"), ActionId::NextPage, config);
	vec![
		item(menu_ids::GO_TO_PAGE, goto_page_label),
		item(menu_ids::PREVIOUS_PAGE, prev_page_label),
		item(menu_ids::NEXT_PAGE, next_page_label),
	]
}

fn links_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to move to the previous link in the document.
	let prev_link_label = format_menu_label(&t("Previous Lin&k"), ActionId::PreviousLink, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next link in the document.
	let next_link_label = format_menu_label(&t("Next Lin&k"), ActionId::NextLink, config);
	vec![item(menu_ids::PREVIOUS_LINK, prev_link_label), item(menu_ids::NEXT_LINK, next_link_label)]
}

fn images_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to move to the previous image in the document.
	let prev_image_label = format_menu_label(&t("Previous Ima&ge"), ActionId::PreviousImage, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next image in the document.
	let next_image_label = format_menu_label(&t("Next Ima&ge"), ActionId::NextImage, config);
	vec![item(menu_ids::PREVIOUS_IMAGE, prev_image_label), item(menu_ids::NEXT_IMAGE, next_image_label)]
}

fn figures_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to move to the previous figure in the document.
	let prev_figure_label = format_menu_label(&t("Previous Figu&re"), ActionId::PreviousFigure, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next figure in the document.
	let next_figure_label = format_menu_label(&t("Next Figu&re"), ActionId::NextFigure, config);
	vec![item(menu_ids::PREVIOUS_FIGURE, prev_figure_label), item(menu_ids::NEXT_FIGURE, next_figure_label)]
}

fn tables_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to move to the previous table in the document.
	let prev_table_label = format_menu_label(&t("Previous &Table"), ActionId::PreviousTable, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next table in the document.
	let next_table_label = format_menu_label(&t("Next &Table"), ActionId::NextTable, config);
	vec![item(menu_ids::PREVIOUS_TABLE, prev_table_label), item(menu_ids::NEXT_TABLE, next_table_label)]
}

fn separators_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to move to the previous separator (e.g. a horizontal rule) in the document.
	let prev_separator_label = format_menu_label(&t("Previous Se&parator"), ActionId::PreviousSeparator, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next separator (e.g. a horizontal rule) in the document.
	let next_separator_label = format_menu_label(&t("Next Se&parator"), ActionId::NextSeparator, config);
	vec![item(menu_ids::PREVIOUS_SEPARATOR, prev_separator_label), item(menu_ids::NEXT_SEPARATOR, next_separator_label)]
}

fn lists_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to move to the previous list in the document.
	let prev_list_label = format_menu_label(&t("Previous L&ist"), ActionId::PreviousList, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next list in the document.
	let next_list_label = format_menu_label(&t("Next L&ist"), ActionId::NextList, config);
	// TRANSLATORS: Menu item in the Go menu to move to the previous item within the current list.
	let prev_list_item_label = format_menu_label(&t("Previous List &Item"), ActionId::PreviousListItem, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next item within the current list.
	let next_list_item_label = format_menu_label(&t("Next List I&tem"), ActionId::NextListItem, config);
	vec![
		item(menu_ids::PREVIOUS_LIST, prev_list_label),
		item(menu_ids::NEXT_LIST, next_list_label),
		item(menu_ids::PREVIOUS_LIST_ITEM, prev_list_item_label),
		item(menu_ids::NEXT_LIST_ITEM, next_list_item_label),
	]
}

fn containers_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to move to the start of the current list or table.
	let container_start_label = format_menu_label(&t("Container &Start"), ActionId::ContainerStart, config);
	// TRANSLATORS: Status-bar help text for the Go > Container Start menu item.
	let container_start_help = t("Go to the start of the current list or table");
	// TRANSLATORS: Menu item in the Go menu to move past the end of the current list or table.
	let container_end_label = format_menu_label(&t("Past Container &End"), ActionId::ContainerEnd, config);
	// TRANSLATORS: Status-bar help text for the Go > Past Container End menu item.
	let container_end_help = t("Go past the end of the current list or table");
	vec![
		item_with_help(menu_ids::CONTAINER_START, container_start_label, container_start_help),
		item_with_help(menu_ids::CONTAINER_END, container_end_label, container_end_help),
	]
}

fn headings_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to move to the previous heading of any level in the document.
	let prev_heading_label = format_menu_label(&t("&Previous Heading"), ActionId::PreviousHeading, config);
	// TRANSLATORS: Status-bar help text for the Go > Previous Heading menu item.
	let prev_heading_help = t("Go to previous heading");
	// TRANSLATORS: Menu item in the Go menu to move to the next heading of any level in the document.
	let next_heading_label = format_menu_label(&t("&Next Heading"), ActionId::NextHeading, config);
	// TRANSLATORS: Status-bar help text for the Go > Next Heading menu item.
	let next_heading_help = t("Go to next heading");
	// TRANSLATORS: Menu item in the Go menu to move to the previous level-1 heading in the document.
	let prev_heading1_label = format_menu_label(&t("Previous Heading Level &1"), ActionId::PreviousHeading1, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next level-1 heading in the document.
	let next_heading1_label = format_menu_label(&t("Next Heading Level 1"), ActionId::NextHeading1, config);
	// TRANSLATORS: Menu item in the Go menu to move to the previous level-2 heading in the document.
	let prev_heading2_label = format_menu_label(&t("Previous Heading Level &2"), ActionId::PreviousHeading2, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next level-2 heading in the document.
	let next_heading2_label = format_menu_label(&t("Next Heading Level 2"), ActionId::NextHeading2, config);
	// TRANSLATORS: Menu item in the Go menu to move to the previous level-3 heading in the document.
	let prev_heading3_label = format_menu_label(&t("Previous Heading Level &3"), ActionId::PreviousHeading3, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next level-3 heading in the document.
	let next_heading3_label = format_menu_label(&t("Next Heading Level 3"), ActionId::NextHeading3, config);
	// TRANSLATORS: Menu item in the Go menu to move to the previous level-4 heading in the document.
	let prev_heading4_label = format_menu_label(&t("Previous Heading Level &4"), ActionId::PreviousHeading4, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next level-4 heading in the document.
	let next_heading4_label = format_menu_label(&t("Next Heading Level 4"), ActionId::NextHeading4, config);
	// TRANSLATORS: Menu item in the Go menu to move to the previous level-5 heading in the document.
	let prev_heading5_label = format_menu_label(&t("Previous Heading Level &5"), ActionId::PreviousHeading5, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next level-5 heading in the document.
	let next_heading5_label = format_menu_label(&t("Next Heading Level 5"), ActionId::NextHeading5, config);
	// TRANSLATORS: Menu item in the Go menu to move to the previous level-6 heading in the document.
	let prev_heading6_label = format_menu_label(&t("Previous Heading Level &6"), ActionId::PreviousHeading6, config);
	// TRANSLATORS: Menu item in the Go menu to move to the next level-6 heading in the document.
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
	// TRANSLATORS: Menu item in the Go menu to move to the previous bookmark in the document.
	let prev_bookmark_label = format_menu_label(&t("&Previous Bookmark"), ActionId::PreviousBookmark, config);
	// TRANSLATORS: Status-bar help text for the Go > Previous Bookmark menu item.
	let prev_bookmark_help = t("Go to previous bookmark");
	// TRANSLATORS: Menu item in the Go menu to move to the next bookmark in the document.
	let next_bookmark_label = format_menu_label(&t("&Next Bookmark"), ActionId::NextBookmark, config);
	// TRANSLATORS: Status-bar help text for the Go > Next Bookmark menu item.
	let next_bookmark_help = t("Go to next bookmark");
	// TRANSLATORS: Menu item in the Go menu to move to the previous note in the document.
	let prev_note_label = format_menu_label(&t("Previous &Note"), ActionId::PreviousNote, config);
	// TRANSLATORS: Status-bar help text for the Go > Previous Note menu item.
	let prev_note_help = t("Go to previous note");
	// TRANSLATORS: Menu item in the Go menu to move to the next note in the document.
	let next_note_label = format_menu_label(&t("Next N&ote"), ActionId::NextNote, config);
	// TRANSLATORS: Status-bar help text for the Go > Next Note menu item.
	let next_note_help = t("Go to next note");
	// TRANSLATORS: Menu item in the Go menu to open a dialog listing all bookmarks and notes.
	let all_bookmarks_label = format_menu_label(&t("Jump to &All..."), ActionId::JumpToAllBookmarks, config);
	// TRANSLATORS: Status-bar help text for the Go > Jump to All menu item.
	let all_bookmarks_help = t("Show all bookmarks and notes");
	// TRANSLATORS: Menu item in the Go menu to open a dialog listing only bookmarks.
	let bookmarks_only_label =
		format_menu_label(&t("Jump to &Bookmarks Only..."), ActionId::JumpToBookmarksOnly, config);
	// TRANSLATORS: Status-bar help text for the Go > Jump to Bookmarks Only menu item.
	let bookmarks_only_help = t("Show bookmarks only");
	// TRANSLATORS: Menu item in the Go menu to open a dialog listing only notes.
	let notes_only_label = format_menu_label(&t("Jump to Notes &Only..."), ActionId::JumpToNotesOnly, config);
	// TRANSLATORS: Status-bar help text for the Go > Jump to Notes Only menu item.
	let notes_only_help = t("Show notes only");
	// TRANSLATORS: Menu item in the Go menu to view the text of the note at the current reading position.
	let view_note_label = format_menu_label(&t("&View Note Text"), ActionId::ViewNoteText, config);
	// TRANSLATORS: Status-bar help text for the Go > View Note Text menu item.
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
	// TRANSLATORS: Menu item in the Go menu to open the find dialog.
	let find_label = format_menu_label(&t("&Find..."), ActionId::Find, config);
	// TRANSLATORS: Status-bar help text for the Go > Find menu item.
	let find_help = t("Find text in the document");
	// TRANSLATORS: Menu item in the Go menu to find the next occurrence of the current search term.
	let find_next_label = format_menu_label(&t("Find &Next"), ActionId::FindNext, config);
	// TRANSLATORS: Status-bar help text for the Go > Find Next menu item.
	let find_next_help = t("Find next occurrence");
	// TRANSLATORS: Menu item in the Go menu to find the previous occurrence of the current search term.
	let find_prev_label = format_menu_label(&t("Find &Previous"), ActionId::FindPrevious, config);
	// TRANSLATORS: Status-bar help text for the Go > Find Previous menu item.
	let find_prev_help = t("Find previous occurrence");
	// TRANSLATORS: Menu item in the Go menu to jump to a specific line number.
	let goto_line_label = format_menu_label(&t("Go to &line..."), ActionId::GoToLine, config);
	// TRANSLATORS: Status-bar help text for the Go > Go to Line menu item.
	let goto_line_help = t("Go to a specific line");
	// TRANSLATORS: Menu item in the Go menu to jump to a percentage position within the document.
	let goto_percent_label = format_menu_label(&t("Go to &percent..."), ActionId::GoToPercent, config);
	// TRANSLATORS: Status-bar help text for the Go > Go to Percent menu item.
	let goto_percent_help = t("Go to a percentage of the document");
	// TRANSLATORS: Menu item in the Go menu to move back to the previous position in navigation history.
	let go_back_label = format_menu_label(&t("Go &Back"), ActionId::GoBack, config);
	// TRANSLATORS: Status-bar help text for the Go > Go Back menu item.
	let go_back_help = t("Go back in history");
	// TRANSLATORS: Menu item in the Go menu to move forward to the next position in navigation history.
	let go_forward_label = format_menu_label(&t("Go &Forward"), ActionId::GoForward, config);
	// TRANSLATORS: Status-bar help text for the Go > Go Forward menu item.
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
		// TRANSLATORS: Label for the Sections submenu in the compact Go menu.
		let sections_label = t("&Sections");
		// TRANSLATORS: Status-bar help text for the Go > Sections submenu.
		let sections_help = t("Navigate by sections");
		menu.append_submenu(create_sections_submenu(config), &sections_label, &sections_help);
		// TRANSLATORS: Label for the Headings submenu in the compact Go menu.
		let headings_label = t("&Headings");
		// TRANSLATORS: Status-bar help text for the Go > Headings submenu.
		let headings_help = t("Navigate by headings");
		menu.append_submenu(headings_menu, &headings_label, &headings_help);
		// TRANSLATORS: Label for the Pages submenu in the compact Go menu.
		let pages_label = t("&Pages");
		// TRANSLATORS: Status-bar help text for the Go > Pages submenu.
		let pages_help = t("Navigate by pages");
		menu.append_submenu(create_pages_submenu(config), &pages_label, &pages_help);
		// TRANSLATORS: Label for the Bookmarks submenu in the compact Go menu.
		let bookmarks_label = t("&Bookmarks");
		// TRANSLATORS: Status-bar help text for the Go > Bookmarks submenu.
		let bookmarks_help = t("Navigate by bookmarks");
		menu.append_submenu(bookmarks_menu, &bookmarks_label, &bookmarks_help);
		// TRANSLATORS: Label for the Links submenu in the compact Go menu.
		let links_label = t("&Links");
		// TRANSLATORS: Status-bar help text for the Go > Links submenu.
		let links_help = t("Navigate by links");
		menu.append_submenu(create_links_submenu(config), &links_label, &links_help);
		// TRANSLATORS: Label for the Images submenu in the compact Go menu.
		let images_label = t("&Images");
		// TRANSLATORS: Status-bar help text for the Go > Images submenu.
		let images_help = t("Navigate by images");
		menu.append_submenu(create_images_submenu(config), &images_label, &images_help);
		// TRANSLATORS: Label for the Figures submenu in the compact Go menu.
		let figures_label = t("&Figures");
		// TRANSLATORS: Status-bar help text for the Go > Figures submenu.
		let figures_help = t("Navigate by figures");
		menu.append_submenu(create_figures_submenu(config), &figures_label, &figures_help);
		// TRANSLATORS: Label for the Tables submenu in the compact Go menu.
		let tables_label = t("&Tables");
		// TRANSLATORS: Status-bar help text for the Go > Tables submenu.
		let tables_help = t("Navigate by tables");
		menu.append_submenu(create_tables_submenu(config), &tables_label, &tables_help);
		// TRANSLATORS: Label for the Separators submenu in the compact Go menu.
		let separators_label = t("&Separators");
		// TRANSLATORS: Status-bar help text for the Go > Separators submenu.
		let separators_help = t("Navigate by separators");
		menu.append_submenu(create_separators_submenu(config), &separators_label, &separators_help);
		// TRANSLATORS: Label for the Lists submenu in the compact Go menu.
		let lists_label = t("&Lists");
		// TRANSLATORS: Status-bar help text for the Go > Lists submenu.
		let lists_help = t("Navigate by lists");
		menu.append_submenu(create_lists_submenu(config), &lists_label, &lists_help);
		// TRANSLATORS: Label for the Containers submenu in the compact Go menu.
		let containers_label = t("&Containers");
		// TRANSLATORS: Status-bar help text for the Go > Containers submenu.
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
