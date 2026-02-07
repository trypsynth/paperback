use wxdragon::{prelude::*, translations::translate as t};

use super::menu_ids;
use crate::config::ConfigManager;

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
	let prev_section_label = t("Previous Section\t[");
	let prev_section_help = t("Go to previous section");
	let next_section_label = t("Next Section\t]");
	let next_section_help = t("Go to next section");
	vec![
		item_with_help(menu_ids::PREVIOUS_SECTION, prev_section_label, prev_section_help),
		item_with_help(menu_ids::NEXT_SECTION, next_section_label, next_section_help),
	]
}

pub fn pages_entries() -> Vec<MenuEntry> {
	let goto_page_label = t("Go to &Page\tCtrl+P");
	let prev_page_label = t("Previous Pa&ge\tShift+P");
	let next_page_label = t("Next Pag&e\tP");
	vec![
		item(menu_ids::GO_TO_PAGE, goto_page_label),
		item(menu_ids::PREVIOUS_PAGE, prev_page_label),
		item(menu_ids::NEXT_PAGE, next_page_label),
	]
}

pub fn links_entries() -> Vec<MenuEntry> {
	let prev_link_label = t("Previous Lin&k\tShift+K");
	let next_link_label = t("Next Lin&k\tK");
	vec![item(menu_ids::PREVIOUS_LINK, prev_link_label), item(menu_ids::NEXT_LINK, next_link_label)]
}

pub fn tables_entries() -> Vec<MenuEntry> {
	let prev_table_label = t("Previous &Table\tShift+T");
	let next_table_label = t("Next &Table\tT");
	vec![item(menu_ids::PREVIOUS_TABLE, prev_table_label), item(menu_ids::NEXT_TABLE, next_table_label)]
}

pub fn separators_entries() -> Vec<MenuEntry> {
	let prev_separator_label = t("Previous Se&parator\tShift+S");
	let next_separator_label = t("Next Se&parator\tS");
	vec![item(menu_ids::PREVIOUS_SEPARATOR, prev_separator_label), item(menu_ids::NEXT_SEPARATOR, next_separator_label)]
}

pub fn lists_entries() -> Vec<MenuEntry> {
	let prev_list_label = t("Previous L&ist\tShift+L");
	let next_list_label = t("Next L&ist\tL");
	let prev_list_item_label = t("Previous List &Item\tShift+I");
	let next_list_item_label = t("Next List I&tem\tI");
	vec![
		item(menu_ids::PREVIOUS_LIST, prev_list_label),
		item(menu_ids::NEXT_LIST, next_list_label),
		item(menu_ids::PREVIOUS_LIST_ITEM, prev_list_item_label),
		item(menu_ids::NEXT_LIST_ITEM, next_list_item_label),
	]
}

pub fn headings_entries() -> Vec<MenuEntry> {
	let next_heading1_label = t("Next Heading Level 1\t1");
	let prev_heading2_label = t("Previous Heading Level &2\tShift+2");
	let next_heading2_label = t("Next Heading Level 2\t2");
	let prev_heading3_label = t("Previous Heading Level &3\tShift+3");
	let next_heading3_label = t("Next Heading Level 3\t3");
	let prev_heading4_label = t("Previous Heading Level &4\tShift+4");
	let next_heading4_label = t("Next Heading Level 4\t4");
	let prev_heading5_label = t("Previous Heading Level &5\tShift+5");
	let next_heading5_label = t("Next Heading Level 5\t5");
	let prev_heading6_label = t("Previous Heading Level &6\tShift+6");
	let next_heading6_label = t("Next Heading Level 6\t6");
	vec![
		item_with_help(menu_ids::PREVIOUS_HEADING, t("&Previous Heading\tShift+H"), t("Go to previous heading")),
		item_with_help(menu_ids::NEXT_HEADING, t("&Next Heading\tH"), t("Go to next heading")),
		MenuEntry::Separator,
		item(menu_ids::PREVIOUS_HEADING_1, t("Previous Heading Level &1\tShift+1")),
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
	let prev_bookmark_label = t("&Previous Bookmark\tShift+B");
	let prev_bookmark_help = t("Go to previous bookmark");
	let next_bookmark_label = t("&Next Bookmark\tB");
	let next_bookmark_help = t("Go to next bookmark");
	let prev_note_label = t("Previous &Note\tShift+N");
	let prev_note_help = t("Go to previous note");
	let next_note_label = t("Next N&ote\tN");
	let next_note_help = t("Go to next note");
	let all_bookmarks_label = t("Jump to &All...\tCtrl+B");
	let all_bookmarks_help = t("Show all bookmarks and notes");
	let bookmarks_only_label = t("Jump to &Bookmarks Only...\tCtrl+Alt+B");
	let bookmarks_only_help = t("Show bookmarks only");
	let notes_only_label = t("Jump to Notes &Only...\tCtrl+Alt+M");
	let notes_only_help = t("Show notes only");
	let view_note_label = if cfg!(target_os = "macos") {
		t("&View Note Text\tRawCtrl+Shift+W")
	} else {
		t("&View Note Text\tCtrl+Shift+W")
	};
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
	let tools_menu = create_tools_menu();
	let help_menu = create_help_menu();
	let file_label = t("&File");
	let go_label = t("&Go");
	let tools_label = t("&Tools");
	let help_label = t("&Help");
	MenuBar::builder()
		.append(file_menu, &file_label)
		.append(go_menu, &go_label)
		.append(tools_menu, &tools_label)
		.append(help_menu, &help_label)
		.build()
}

pub fn create_file_menu(config: &ConfigManager) -> Menu {
	let open_label = t("&Open...\tCtrl+O");
	let open_help = t("Open a document");
	// On macOS, Ctrl+ maps to Cmd+, so use Cmd+W / Cmd+Shift+W for close.
	// On Windows/Linux, keep Ctrl+F4 / Ctrl+Shift+F4.
	let close_label = if cfg!(target_os = "macos") {
		t("&Close\tCtrl+W")
	} else {
		t("&Close\tCtrl+F4")
	};
	let close_help = t("Close the current document");
	let close_all_label = if cfg!(target_os = "macos") {
		t("Close &All\tCtrl+Shift+W")
	} else {
		t("Close &All\tCtrl+Shift+F4")
	};
	let close_all_help = t("Close all documents");
	let file_menu = Menu::builder()
		.append_item(menu_ids::OPEN, &open_label, &open_help)
		.append_item(menu_ids::CLOSE, &close_label, &close_help)
		.append_item(menu_ids::CLOSE_ALL, &close_all_label, &close_all_help)
		.build();
	let recent_menu = Menu::builder().build();
	populate_recent_documents_menu(&recent_menu, config);
	let recent_label = t("&Recent Documents");
	let recent_help = t("Open a recent document");
	let _ = file_menu.append_submenu(recent_menu, &recent_label, &recent_help);
	// On macOS, wxWidgets auto-moves wxID_EXIT to the app menu, so skip the
	// explicit Exit item to avoid a duplicate.
	if !cfg!(target_os = "macos") {
		file_menu.append_separator();
		let exit_label = t("E&xit");
		let exit_help = t("Exit the application");
		let _ = file_menu.append(menu_ids::EXIT, &exit_label, &exit_help, ItemKind::Normal);
	}
	file_menu
}

pub fn create_go_menu(compact: bool) -> Menu {
	let headings_menu = create_headings_submenu();
	let bookmarks_menu = create_bookmarks_submenu();
	let find_label = t("&Find...\tCtrl+F");
	let find_help = t("Find text in the document");
	let find_next_label = t("Find &Next\tF3");
	let find_next_help = t("Find next occurrence");
	let find_prev_label = t("Find &Previous\tShift+F3");
	let find_prev_help = t("Find previous occurrence");
	let goto_line_label = t("Go to &line...\tCtrl+G");
	let goto_line_help = t("Go to a specific line");
	let goto_percent_label = t("Go to &percent...\tCtrl+Shift+G");
	let goto_percent_help = t("Go to a percentage of the document");
	let go_back_label = t("Go &Back\tAlt+Left");
	let go_back_help = t("Go back in history");
	let go_forward_label = t("Go &Forward\tAlt+Right");
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
		menu.append_submenu(create_sections_submenu(), &sections_label, &sections_help);
		let headings_label = t("&Headings");
		let headings_help = t("Navigate by headings");
		menu.append_submenu(headings_menu, &headings_label, &headings_help);
		let pages_label = t("&Pages");
		let pages_help = t("Navigate by pages");
		menu.append_submenu(create_pages_submenu(), &pages_label, &pages_help);
		let bookmarks_label = t("&Bookmarks");
		let bookmarks_help = t("Navigate by bookmarks");
		menu.append_submenu(bookmarks_menu, &bookmarks_label, &bookmarks_help);
		let links_label = t("&Links");
		let links_help = t("Navigate by links");
		menu.append_submenu(create_links_submenu(), &links_label, &links_help);
		let tables_label = t("&Tables");
		let tables_help = t("Navigate by tables");
		menu.append_submenu(create_tables_submenu(), &tables_label, &tables_help);
		let separators_label = t("&Separators");
		let separators_help = t("Navigate by separators");
		menu.append_submenu(create_separators_submenu(), &separators_label, &separators_help);
		let lists_label = t("&Lists");
		let lists_help = t("Navigate by lists");
		menu.append_submenu(create_lists_submenu(), &lists_label, &lists_help);
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
		append_tables_items(&menu);
		menu.append_separator();
		append_separators_items(&menu);
		menu.append_separator();
		append_lists_items(&menu);
	}
	menu
}

pub fn create_tools_menu() -> Menu {
	let import_label = t("&Import Document Data...\tCtrl+Shift+I");
	let import_help = t("Import bookmarks and position");
	let export_label = t("&Export Document Data...\tCtrl+Shift+E");
	let export_help = t("Export bookmarks and position");
	let export_text_label = t("Export to &Plain Text...\tCtrl+E");
	let export_text_help = t("Export document as plain text");
	let import_export_menu = Menu::builder()
		.append_item(menu_ids::IMPORT_DOCUMENT_DATA, &import_label, &import_help)
		.append_item(menu_ids::EXPORT_DOCUMENT_DATA, &export_label, &export_help)
		.append_item(menu_ids::EXPORT_TO_PLAIN_TEXT, &export_text_label, &export_text_help)
		.build();
	// On macOS, Cmd+W is close, so use Ctrl+W (raw Control key) for word count.
	let word_count_label = if cfg!(target_os = "macos") {
		t("&Word Count\tRawCtrl+W")
	} else {
		t("&Word Count\tCtrl+W")
	};
	let word_count_help = t("Show word count");
	let doc_info_label = t("Document &Info\tCtrl+I");
	let doc_info_help = t("Show document information");
	let toc_label = t("&Table of Contents\tCtrl+T");
	let toc_help = t("Show table of contents");
	let elements_label = t("&Elements List...\tF7");
	let elements_help = t("Show elements list");
	let open_folder_label = t("Open &Containing Folder\tCtrl+Shift+C");
	let open_folder_help = t("Open folder containing the document");
	let web_view_label = t("Open in &Web View\tCtrl+Shift+V");
	let web_view_help = t("Open document in web view");
	let menu = Menu::builder()
		.append_item(menu_ids::WORD_COUNT, &word_count_label, &word_count_help)
		.append_item(menu_ids::DOCUMENT_INFO, &doc_info_label, &doc_info_help)
		.append_separator()
		.append_item(menu_ids::TABLE_OF_CONTENTS, &toc_label, &toc_help)
		.append_item(menu_ids::ELEMENTS_LIST, &elements_label, &elements_help)
		.append_separator()
		.append_item(menu_ids::OPEN_CONTAINING_FOLDER, &open_folder_label, &open_folder_help)
		.append_item(menu_ids::OPEN_IN_WEB_VIEW, &web_view_label, &web_view_help)
		.append_separator()
		.build();
	let import_export_label = t("I&mport/Export");
	let import_export_help = t("Import and export options");
	menu.append_submenu(import_export_menu, &import_export_label, &import_export_help);
	menu.append_separator();
	let toggle_bookmark_label = t("Toggle &Bookmark\tCtrl+Shift+B");
	let bookmark_note_label = t("Bookmark with &Note\tCtrl+Shift+N");
	menu.append(menu_ids::TOGGLE_BOOKMARK, &toggle_bookmark_label, "", ItemKind::Normal);
	menu.append(menu_ids::BOOKMARK_WITH_NOTE, &bookmark_note_label, "", ItemKind::Normal);
	menu.append_separator();
	let options_label = t("&Options\tCtrl+,");
	let sleep_label = t("&Sleep Timer...\tCtrl+Shift+S");
	// On macOS, use wxID_PREFERENCES so wxWidgets puts it in the app menu.
	let options_id = if cfg!(target_os = "macos") { menu_ids::PREFERENCES } else { menu_ids::OPTIONS };
	menu.append(options_id, &options_label, "", ItemKind::Normal);
	menu.append(menu_ids::SLEEP_TIMER, &sleep_label, "", ItemKind::Normal);
	menu
}

pub fn create_help_menu() -> Menu {
	let about_label = t("&About Paperback\tCtrl+F1");
	let about_help = t("About this application");
	let help_browser_label = t("View Help in &Browser\tF1");
	let help_browser_help = t("View help in default browser");
	let help_paperback_label = t("View Help in &Paperback\tShift+F1");
	let help_paperback_help = t("View help in Paperback");
	let updates_label = t("Check for &Updates\tCtrl+Shift+U");
	let updates_help = t("Check for updates");
	let donate_label = t("&Donate\tCtrl+D");
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
		let empty_label = t("(No recent documents)");
		if let Some(item) = menu.append(wxdragon::id::ID_ANY.try_into().unwrap(), &empty_label, "", ItemKind::Normal) {
			item.enable(false);
		}
	} else {
		for (index, path) in recent_docs.iter().enumerate() {
			let filename = std::path::Path::new(path)
				.file_name()
				.map_or_else(|| path.clone(), |s| s.to_string_lossy().to_string());
			let label = format!("&{} {}", index + 1, filename);
			if let Ok(offset) = i32::try_from(index) {
				let id = menu_ids::RECENT_DOCUMENT_BASE + offset;
				let _ = menu.append(id, &label, path, ItemKind::Normal);
			}
		}
	}
	menu.append_separator();
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
