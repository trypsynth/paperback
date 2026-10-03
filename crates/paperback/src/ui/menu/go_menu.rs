use paperback_core::config::{ActionId, ConfigManager};
use patois::t;

use super::builder::{MenuEntry, format_menu_label, item, item_with_help, submenu};
use crate::ui::{commands, menu_ids};

/// The navigation groups, each as its label, status-bar help text and items: a submenu apiece in
/// the compact Go menu, or runs of items between separators in the flat one.
fn groups(config: &ConfigManager) -> Vec<(String, String, Vec<MenuEntry>)> {
	// TRANSLATORS: Menu item in the Go menu to jump to a specific page number.
	let goto_page_label = format_menu_label(&t("Go to &Page"), ActionId::GoToPage, config);
	let mut pages = vec![item(menu_ids::GO_TO_PAGE, goto_page_label)];
	pages.extend(commands::menu_entries(&[ActionId::PreviousPage, ActionId::NextPage], config));
	vec![
		(
			// TRANSLATORS: Label for the Sections submenu in the compact Go menu.
			t("&Sections"),
			// TRANSLATORS: Status-bar help text for the Go > Sections submenu.
			t("Navigate by sections"),
			commands::menu_entries(&[ActionId::PreviousSection, ActionId::NextSection], config),
		),
		(
			// TRANSLATORS: Label for the Headings submenu in the compact Go menu.
			t("&Headings"),
			// TRANSLATORS: Status-bar help text for the Go > Headings submenu.
			t("Navigate by headings"),
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
			),
		),
		(
			// TRANSLATORS: Label for the Pages submenu in the compact Go menu.
			t("&Pages"),
			// TRANSLATORS: Status-bar help text for the Go > Pages submenu.
			t("Navigate by pages"),
			pages,
		),
		(
			// TRANSLATORS: Label for the Links submenu in the compact Go menu.
			t("&Links"),
			// TRANSLATORS: Status-bar help text for the Go > Links submenu.
			t("Navigate by links"),
			commands::menu_entries(&[ActionId::PreviousLink, ActionId::NextLink], config),
		),
		(
			// TRANSLATORS: Label for the Images submenu in the compact Go menu.
			t("&Images"),
			// TRANSLATORS: Status-bar help text for the Go > Images submenu.
			t("Navigate by images"),
			commands::menu_entries(&[ActionId::PreviousImage, ActionId::NextImage], config),
		),
		(
			// TRANSLATORS: Label for the Figures submenu in the compact Go menu.
			t("&Figures"),
			// TRANSLATORS: Status-bar help text for the Go > Figures submenu.
			t("Navigate by figures"),
			commands::menu_entries(&[ActionId::PreviousFigure, ActionId::NextFigure], config),
		),
		(
			// TRANSLATORS: Label for the Tables submenu in the compact Go menu.
			t("&Tables"),
			// TRANSLATORS: Status-bar help text for the Go > Tables submenu.
			t("Navigate by tables"),
			commands::menu_entries(&[ActionId::PreviousTable, ActionId::NextTable], config),
		),
		(
			// TRANSLATORS: Label for the Formulas submenu in the compact Go menu.
			t("For&mulas"),
			// TRANSLATORS: Status-bar help text for the Go > Formulas submenu.
			t("Navigate by formulas"),
			commands::menu_entries(&[ActionId::PreviousFormula, ActionId::NextFormula], config),
		),
		(
			// TRANSLATORS: Label for the Separators submenu in the compact Go menu.
			t("&Separators"),
			// TRANSLATORS: Status-bar help text for the Go > Separators submenu.
			t("Navigate by separators"),
			commands::menu_entries(&[ActionId::PreviousSeparator, ActionId::NextSeparator], config),
		),
		(
			// TRANSLATORS: Label for the Lists submenu in the compact Go menu.
			t("&Lists"),
			// TRANSLATORS: Status-bar help text for the Go > Lists submenu.
			t("Navigate by lists"),
			commands::menu_entries(
				&[ActionId::PreviousList, ActionId::NextList, ActionId::PreviousListItem, ActionId::NextListItem],
				config,
			),
		),
		(
			// TRANSLATORS: Label for the Containers submenu in the compact Go menu.
			t("&Containers"),
			// TRANSLATORS: Status-bar help text for the Go > Containers submenu.
			t("Navigate by containers"),
			commands::menu_entries(&[ActionId::ContainerStart, ActionId::ContainerEnd], config),
		),
	]
}

pub fn entries(config: &ConfigManager, compact: bool) -> Vec<MenuEntry> {
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
	let mut entries = vec![
		item_with_help(menu_ids::FIND, find_label, find_help),
		item_with_help(menu_ids::FIND_NEXT, find_next_label, find_next_help),
		item_with_help(menu_ids::FIND_PREVIOUS, find_prev_label, find_prev_help),
		MenuEntry::Separator,
		item_with_help(menu_ids::GO_TO_LINE, goto_line_label, goto_line_help),
		item_with_help(menu_ids::GO_TO_PERCENT, goto_percent_label, goto_percent_help),
		MenuEntry::Separator,
		item_with_help(menu_ids::GO_BACK, go_back_label, go_back_help),
		item_with_help(menu_ids::GO_FORWARD, go_forward_label, go_forward_help),
		MenuEntry::Separator,
	];
	for (index, (label, help, group)) in groups(config).into_iter().enumerate() {
		if compact {
			entries.push(submenu(label, help, group));
		} else {
			if index > 0 {
				entries.push(MenuEntry::Separator);
			}
			entries.extend(group);
		}
	}
	entries
}
