use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::builder::{MenuEntry, append_menu_entries, build_menu, format_menu_label, item};
use crate::ui::{commands, menu_ids};

fn sections_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	commands::menu_entries(&[ActionId::PreviousSection, ActionId::NextSection], config)
}

fn pages_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Go menu to jump to a specific page number.
	let goto_page_label = format_menu_label(&t("Go to &Page"), ActionId::GoToPage, config);
	let mut entries = vec![item(menu_ids::GO_TO_PAGE, goto_page_label)];
	entries.extend(commands::menu_entries(&[ActionId::PreviousPage, ActionId::NextPage], config));
	entries
}

fn links_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	commands::menu_entries(&[ActionId::PreviousLink, ActionId::NextLink], config)
}

fn images_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	commands::menu_entries(&[ActionId::PreviousImage, ActionId::NextImage], config)
}

fn figures_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	commands::menu_entries(&[ActionId::PreviousFigure, ActionId::NextFigure], config)
}

fn tables_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	commands::menu_entries(&[ActionId::PreviousTable, ActionId::NextTable], config)
}

fn separators_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	commands::menu_entries(&[ActionId::PreviousSeparator, ActionId::NextSeparator], config)
}

fn lists_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	commands::menu_entries(
		&[ActionId::PreviousList, ActionId::NextList, ActionId::PreviousListItem, ActionId::NextListItem],
		config,
	)
}

fn containers_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	commands::menu_entries(&[ActionId::ContainerStart, ActionId::ContainerEnd], config)
}

fn headings_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	commands::menu_entries(
		&[
			ActionId::PreviousHeading,
			ActionId::NextHeading,
			ActionId::PreviousHeading1,
			ActionId::NextHeading1,
			ActionId::PreviousHeading2,
			ActionId::NextHeading2,
			ActionId::PreviousHeading3,
			ActionId::NextHeading3,
			ActionId::PreviousHeading4,
			ActionId::NextHeading4,
			ActionId::PreviousHeading5,
			ActionId::NextHeading5,
			ActionId::PreviousHeading6,
			ActionId::NextHeading6,
		],
		config,
	)
}

fn bookmarks_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let mut entries = commands::menu_entries(
		&[ActionId::PreviousBookmark, ActionId::NextBookmark, ActionId::PreviousNote, ActionId::NextNote],
		config,
	);
	entries.push(MenuEntry::Separator);
	entries.extend(commands::menu_entries(
		&[
			ActionId::JumpToAllBookmarks,
			ActionId::JumpToBookmarksOnly,
			ActionId::JumpToNotesOnly,
			ActionId::ViewNoteText,
		],
		config,
	));
	entries
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

#[cfg(test)]
mod tests {
	use super::*;

	/// Every action these builders name has to be in the command table: `menu_entries` panics
	/// otherwise, which would take the menu bar down while it was being built at startup.
	#[test]
	fn every_go_menu_item_is_a_known_command() {
		let actions = [
			ActionId::PreviousSection,
			ActionId::NextSection,
			ActionId::PreviousPage,
			ActionId::NextPage,
			ActionId::PreviousLink,
			ActionId::NextLink,
			ActionId::PreviousImage,
			ActionId::NextImage,
			ActionId::PreviousFigure,
			ActionId::NextFigure,
			ActionId::PreviousTable,
			ActionId::NextTable,
			ActionId::PreviousSeparator,
			ActionId::NextSeparator,
			ActionId::PreviousList,
			ActionId::NextList,
			ActionId::PreviousListItem,
			ActionId::NextListItem,
			ActionId::PreviousHeading,
			ActionId::NextHeading,
			ActionId::PreviousHeading1,
			ActionId::NextHeading1,
			ActionId::PreviousHeading2,
			ActionId::NextHeading2,
			ActionId::PreviousHeading3,
			ActionId::NextHeading3,
			ActionId::PreviousHeading4,
			ActionId::NextHeading4,
			ActionId::PreviousHeading5,
			ActionId::NextHeading5,
			ActionId::PreviousHeading6,
			ActionId::NextHeading6,
			ActionId::ContainerStart,
			ActionId::ContainerEnd,
			ActionId::PreviousBookmark,
			ActionId::NextBookmark,
			ActionId::PreviousNote,
			ActionId::NextNote,
			ActionId::JumpToAllBookmarks,
			ActionId::JumpToBookmarksOnly,
			ActionId::JumpToNotesOnly,
			ActionId::ViewNoteText,
		];
		for action in actions {
			assert!(commands::for_action(action).is_some(), "{action:?} is in the Go menu but not in COMMANDS");
		}
	}
}
