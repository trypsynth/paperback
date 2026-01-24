use std::{
	cell::Cell,
	path::Path,
	rc::Rc,
	sync::{
		Mutex,
		atomic::{AtomicI32, AtomicI64, AtomicUsize, Ordering},
	},
	thread,
	time::{self, SystemTime},
};

use wxdragon::{prelude::*, scrollable::WxScrollable, timer::Timer, translations::translate as t};
use wxdragon_sys as ffi;

use super::{
	dialogs,
	document_manager::{DocumentManager, DocumentTab},
	menu_ids, utils,
};
use crate::{
	config::ConfigManager,
	live_region::{self, LiveRegionMode},
	parser::parser_supports_extension,
	translation_manager::TranslationManager,
	ui_types::BookmarkFilterType,
	update::{self, UpdateCheckOutcome, UpdateError},
	utils::text::{display_len, markdown_to_text},
};

const KEY_DELETE: i32 = 127;
const KEY_NUMPAD_DELETE: i32 = 330;
const DIALOG_PADDING: i32 = 10;
const MAX_FIND_HISTORY_SIZE: usize = 10;
static MAIN_WINDOW_PTR: AtomicUsize = AtomicUsize::new(0);
pub static SLEEP_TIMER_START_MS: AtomicI64 = AtomicI64::new(0);
pub static SLEEP_TIMER_DURATION_MINUTES: AtomicI32 = AtomicI32::new(0);

/// Main application window
pub struct MainWindow {
	frame: Frame,
	doc_manager: Rc<Mutex<DocumentManager>>,
	_config: Rc<Mutex<ConfigManager>>,
	tray_state: Rc<Mutex<Option<TrayState>>>,
	live_region_label: StaticText,
	find_dialog: Rc<Mutex<Option<FindDialogState>>>,
}

impl MainWindow {
	/// Create a new main window
	pub fn new(config: Rc<Mutex<ConfigManager>>) -> Self {
		let app_title = t("Paperback");
		let frame = Frame::builder().with_title(&app_title).with_size(Size::new(800, 600)).build();
		MAIN_WINDOW_PTR.store(frame.handle_ptr() as usize, Ordering::SeqCst);

		// Create status bar
		frame.create_status_bar(1, 0, -1, "statusbar");
		frame.set_status_text(&t("Ready"), 0);

		// Create menu bar
		let menu_bar = Self::create_menu_bar(&config.lock().unwrap());
		frame.set_menu_bar(menu_bar);

		// Create main panel and sizer
		let panel = Panel::builder(&frame).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();

		let live_region_label = StaticText::builder(&panel).with_label("").with_size(Size::new(0, 0)).build();
		live_region_label.show(false);
		let _ = live_region::set_live_region(&live_region_label, LiveRegionMode::Polite);

		// Create notebook for document tabs
		let notebook = Notebook::builder(&panel).with_style(NotebookStyle::Top).build();

		sizer.add(&notebook, 1, SizerFlag::Expand | SizerFlag::All, 0);
		panel.set_sizer(sizer, true);

		// Create document manager
		let doc_manager =
			Rc::new(Mutex::new(DocumentManager::new(frame, notebook, Rc::clone(&config), live_region_label)));

		let find_dialog = Rc::new(Mutex::new(None));
		Self::bind_menu_events(
			&frame,
			Rc::clone(&doc_manager),
			Rc::clone(&config),
			Rc::clone(&find_dialog),
			live_region_label,
		);
		let dm = Rc::clone(&doc_manager);
		let frame_copy = frame;
		let notebook = *doc_manager.lock().unwrap().notebook();
		notebook.on_page_changed(move |_event| {
			// Update title bar with document name
			let dm_ref = match dm.try_lock() {
				Ok(dm_ref) => dm_ref,
				Err(_) => return,
			};
			update_title_from_manager(&frame_copy, &dm_ref);
		});
		let dm = Rc::clone(&doc_manager);
		let frame_copy = frame;
		notebook.on_key_down(move |event| {
			if let wxdragon::event::WindowEventData::Keyboard(key_event) = &event {
				if let Some(key) = key_event.get_key_code() {
					if key == KEY_DELETE || key == KEY_NUMPAD_DELETE {
						let mut dm = dm.lock().unwrap();
						if let Some(index) = dm.active_tab_index() {
							dm.close_document(index);
						}
						update_title_from_manager(&frame_copy, &dm);
						if dm.tab_count() == 0 {
							dm.notebook().set_focus();
						} else {
							dm.restore_focus();
						}
						event.skip(false);
						return;
					}
				}
			}
			event.skip(true);
		});

		let tray_state = Rc::new(Mutex::new(None));
		Self::bind_tray_events(frame, Rc::clone(&doc_manager), Rc::clone(&config), Rc::clone(&tray_state));
		Self::schedule_restore_documents(frame, Rc::clone(&doc_manager), Rc::clone(&config));
		Self { frame, doc_manager, _config: config, tray_state, live_region_label, find_dialog }
	}

	pub fn show(&self) {
		self.frame.show(true);
		self.frame.centre();
	}

	pub fn check_for_updates(&self, silent: bool) {
		run_update_check(silent);
	}

	pub fn open_file(&self, path: &Path) -> bool {
		if !self.ensure_parser_ready(path) {
			return false;
		}
		let result = self.doc_manager.lock().unwrap().open_file(Rc::clone(&self.doc_manager), path);
		if result {
			self.update_title();
			self.update_recent_documents_menu();
			self.doc_manager.lock().unwrap().restore_focus();
		}
		result
	}

	fn update_title(&self) {
		let dm = match self.doc_manager.try_lock() {
			Ok(dm) => dm,
			Err(_) => return,
		};
		if dm.tab_count() == 0 {
			self.frame.set_title(&t("Paperback"));
			self.frame.set_status_text(&t("Ready"), 0);
			return;
		}
		if let Some(tab) = dm.active_tab() {
			let title = tab.session.title();
			let display_title = if title.is_empty() {
				tab.file_path.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| t("Untitled"))
			} else {
				title
			};
			let template = t("Paperback - {}");
			self.frame.set_title(&template.replace("{}", &display_title));
			let chars_label = t("{} chars");
			self.frame.set_status_text(&chars_label.replace("{}", &tab.session.content().len().to_string()), 0);
		}
	}

	/// Create the menu bar with all menus
	fn create_menu_bar(config: &ConfigManager) -> MenuBar {
		let file_menu = Self::create_file_menu(config);
		let compact_go_menu = config.get_app_bool("compact_go_menu", true);
		let go_menu = Self::create_go_menu(compact_go_menu);
		let tools_menu = Self::create_tools_menu();
		let help_menu = Self::create_help_menu();
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

	/// Create the File menu
	fn create_file_menu(config: &ConfigManager) -> Menu {
		let open_label = t("&Open...\tCtrl+O");
		let open_help = t("Open a document");
		let close_label = t("&Close\tCtrl+F4");
		let close_help = t("Close the current document");
		let close_all_label = t("Close &All\tCtrl+Shift+F4");
		let close_all_help = t("Close all documents");
		let exit_label = t("E&xit");
		let exit_help = t("Exit the application");
		let file_menu = Menu::builder()
			.append_item(menu_ids::OPEN, &open_label, &open_help)
			.append_item(menu_ids::CLOSE, &close_label, &close_help)
			.append_item(menu_ids::CLOSE_ALL, &close_all_label, &close_all_help)
			.append_separator()
			.append_item(menu_ids::EXIT, &exit_label, &exit_help)
			.build();

		let recent_menu = Menu::builder().build();
		Self::populate_recent_documents_menu(&recent_menu, config);
		let recent_label = t("&Recent Documents");
		let recent_help = t("Open a recent document");
		let _ = file_menu.append_submenu(recent_menu, &recent_label, &recent_help);

		file_menu
	}

	/// Create the Go menu
	fn create_go_menu(compact: bool) -> Menu {
		let headings_menu = Self::create_headings_submenu();
		let bookmarks_menu = Self::create_bookmarks_submenu();

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
			menu.append_submenu(Self::create_sections_submenu(), &sections_label, &sections_help);
			let headings_label = t("&Headings");
			let headings_help = t("Navigate by headings");
			menu.append_submenu(headings_menu, &headings_label, &headings_help);
			let pages_label = t("&Pages");
			let pages_help = t("Navigate by pages");
			menu.append_submenu(Self::create_pages_submenu(), &pages_label, &pages_help);
			let bookmarks_label = t("&Bookmarks");
			let bookmarks_help = t("Navigate by bookmarks");
			menu.append_submenu(bookmarks_menu, &bookmarks_label, &bookmarks_help);
			let links_label = t("&Links");
			let links_help = t("Navigate by links");
			menu.append_submenu(Self::create_links_submenu(), &links_label, &links_help);
			let tables_label = t("&Tables");
			let tables_help = t("Navigate by tables");
			menu.append_submenu(Self::create_tables_submenu(), &tables_label, &tables_help);
			let lists_label = t("&Lists");
			let lists_help = t("Navigate by lists");
			menu.append_submenu(Self::create_lists_submenu(), &lists_label, &lists_help);
		} else {
			Self::append_sections_items(&menu);
			menu.append_separator();
			Self::append_headings_items(&menu);
			menu.append_separator();
			Self::append_pages_items(&menu);
			menu.append_separator();
			Self::append_bookmarks_items(&menu);
			menu.append_separator();
			Self::append_links_items(&menu);
			menu.append_separator();
			Self::append_tables_items(&menu);
			menu.append_separator();
			Self::append_lists_items(&menu);
		}

		menu
	}

	fn create_sections_submenu() -> Menu {
		let prev_section_label = t("Previous Section\t[");
		let prev_section_help = t("Go to previous section");
		let next_section_label = t("Next Section\t]");
		let next_section_help = t("Go to next section");
		Menu::builder()
			.append_item(menu_ids::PREVIOUS_SECTION, &prev_section_label, &prev_section_help)
			.append_item(menu_ids::NEXT_SECTION, &next_section_label, &next_section_help)
			.build()
	}

	fn append_sections_items(menu: &Menu) {
		let prev_section_label = t("Previous Section\t[");
		let next_section_label = t("Next Section\t]");
		menu.append(menu_ids::PREVIOUS_SECTION, &prev_section_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_SECTION, &next_section_label, "", ItemKind::Normal);
	}

	fn create_pages_submenu() -> Menu {
		let goto_page_label = t("Go to &Page\tCtrl+P");
		let prev_page_label = t("Previous Pa&ge\tShift+P");
		let next_page_label = t("Next Pag&e\tP");
		Menu::builder()
			.append_item(menu_ids::GO_TO_PAGE, &goto_page_label, "")
			.append_item(menu_ids::PREVIOUS_PAGE, &prev_page_label, "")
			.append_item(menu_ids::NEXT_PAGE, &next_page_label, "")
			.build()
	}

	fn append_pages_items(menu: &Menu) {
		let goto_page_label = t("Go to &Page\tCtrl+P");
		let prev_page_label = t("Previous Pa&ge\tShift+P");
		let next_page_label = t("Next Pag&e\tP");
		menu.append(menu_ids::GO_TO_PAGE, &goto_page_label, "", ItemKind::Normal);
		menu.append(menu_ids::PREVIOUS_PAGE, &prev_page_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_PAGE, &next_page_label, "", ItemKind::Normal);
	}

	fn create_links_submenu() -> Menu {
		let prev_link_label = t("Previous Lin&k\tShift+K");
		let next_link_label = t("Next Lin&k\tK");
		Menu::builder()
			.append_item(menu_ids::PREVIOUS_LINK, &prev_link_label, "")
			.append_item(menu_ids::NEXT_LINK, &next_link_label, "")
			.build()
	}

	fn append_links_items(menu: &Menu) {
		let prev_link_label = t("Previous Lin&k\tShift+K");
		let next_link_label = t("Next Lin&k\tK");
		menu.append(menu_ids::PREVIOUS_LINK, &prev_link_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_LINK, &next_link_label, "", ItemKind::Normal);
	}

	fn create_tables_submenu() -> Menu {
		let prev_table_label = t("Previous &Table\tShift+T");
		let next_table_label = t("Next &Table\tT");
		Menu::builder()
			.append_item(menu_ids::PREVIOUS_TABLE, &prev_table_label, "")
			.append_item(menu_ids::NEXT_TABLE, &next_table_label, "")
			.build()
	}

	fn append_tables_items(menu: &Menu) {
		let prev_table_label = t("Previous &Table\tShift+T");
		let next_table_label = t("Next &Table\tT");
		menu.append(menu_ids::PREVIOUS_TABLE, &prev_table_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_TABLE, &next_table_label, "", ItemKind::Normal);
	}

	fn create_lists_submenu() -> Menu {
		let prev_list_label = t("Previous L&ist\tShift+L");
		let next_list_label = t("Next L&ist\tL");
		let prev_list_item_label = t("Previous List &Item\tShift+I");
		let next_list_item_label = t("Next List I&tem\tI");
		Menu::builder()
			.append_item(menu_ids::PREVIOUS_LIST, &prev_list_label, "")
			.append_item(menu_ids::NEXT_LIST, &next_list_label, "")
			.append_item(menu_ids::PREVIOUS_LIST_ITEM, &prev_list_item_label, "")
			.append_item(menu_ids::NEXT_LIST_ITEM, &next_list_item_label, "")
			.build()
	}

	fn append_lists_items(menu: &Menu) {
		let prev_list_label = t("Previous L&ist\tShift+L");
		let next_list_label = t("Next L&ist\tL");
		let prev_list_item_label = t("Previous List &Item\tShift+I");
		let next_list_item_label = t("Next List I&tem\tI");
		menu.append(menu_ids::PREVIOUS_LIST, &prev_list_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_LIST, &next_list_label, "", ItemKind::Normal);
		menu.append(menu_ids::PREVIOUS_LIST_ITEM, &prev_list_item_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_LIST_ITEM, &next_list_item_label, "", ItemKind::Normal);
	}

	fn create_headings_submenu() -> Menu {
		let prev_heading_label = t("&Previous Heading\tShift+H");
		let prev_heading_help = t("Go to previous heading");
		let next_heading_label = t("&Next Heading\tH");
		let next_heading_help = t("Go to next heading");
		let prev_heading1_label = t("Previous Heading &1\tShift+1");
		let next_heading1_label = t("Next Heading 1\t1");
		let prev_heading2_label = t("Previous Heading &2\tShift+2");
		let next_heading2_label = t("Next Heading 2\t2");
		let prev_heading3_label = t("Previous Heading &3\tShift+3");
		let next_heading3_label = t("Next Heading 3\t3");
		let prev_heading4_label = t("Previous Heading &4\tShift+4");
		let next_heading4_label = t("Next Heading 4\t4");
		let prev_heading5_label = t("Previous Heading &5\tShift+5");
		let next_heading5_label = t("Next Heading 5\t5");
		let prev_heading6_label = t("Previous Heading &6\tShift+6");
		let next_heading6_label = t("Next Heading 6\t6");
		Menu::builder()
			.append_item(menu_ids::PREVIOUS_HEADING, &prev_heading_label, &prev_heading_help)
			.append_item(menu_ids::NEXT_HEADING, &next_heading_label, &next_heading_help)
			.append_separator()
			.append_item(menu_ids::PREVIOUS_HEADING_1, &prev_heading1_label, "")
			.append_item(menu_ids::NEXT_HEADING_1, &next_heading1_label, "")
			.append_item(menu_ids::PREVIOUS_HEADING_2, &prev_heading2_label, "")
			.append_item(menu_ids::NEXT_HEADING_2, &next_heading2_label, "")
			.append_item(menu_ids::PREVIOUS_HEADING_3, &prev_heading3_label, "")
			.append_item(menu_ids::NEXT_HEADING_3, &next_heading3_label, "")
			.append_item(menu_ids::PREVIOUS_HEADING_4, &prev_heading4_label, "")
			.append_item(menu_ids::NEXT_HEADING_4, &next_heading4_label, "")
			.append_item(menu_ids::PREVIOUS_HEADING_5, &prev_heading5_label, "")
			.append_item(menu_ids::NEXT_HEADING_5, &next_heading5_label, "")
			.append_item(menu_ids::PREVIOUS_HEADING_6, &prev_heading6_label, "")
			.append_item(menu_ids::NEXT_HEADING_6, &next_heading6_label, "")
			.build()
	}

	fn append_headings_items(menu: &Menu) {
		let prev_heading_label = t("&Previous Heading\tShift+H");
		let next_heading_label = t("&Next Heading\tH");
		let prev_heading1_label = t("Previous Heading &1\tShift+1");
		let next_heading1_label = t("Next Heading 1\t1");
		let prev_heading2_label = t("Previous Heading &2\tShift+2");
		let next_heading2_label = t("Next Heading 2\t2");
		let prev_heading3_label = t("Previous Heading &3\tShift+3");
		let next_heading3_label = t("Next Heading 3\t3");
		let prev_heading4_label = t("Previous Heading &4\tShift+4");
		let next_heading4_label = t("Next Heading 4\t4");
		let prev_heading5_label = t("Previous Heading &5\tShift+5");
		let next_heading5_label = t("Next Heading 5\t5");
		let prev_heading6_label = t("Previous Heading &6\tShift+6");
		let next_heading6_label = t("Next Heading 6\t6");
		menu.append(menu_ids::PREVIOUS_HEADING, &prev_heading_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_HEADING, &next_heading_label, "", ItemKind::Normal);
		menu.append_separator();
		menu.append(menu_ids::PREVIOUS_HEADING_1, &prev_heading1_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_HEADING_1, &next_heading1_label, "", ItemKind::Normal);
		menu.append(menu_ids::PREVIOUS_HEADING_2, &prev_heading2_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_HEADING_2, &next_heading2_label, "", ItemKind::Normal);
		menu.append(menu_ids::PREVIOUS_HEADING_3, &prev_heading3_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_HEADING_3, &next_heading3_label, "", ItemKind::Normal);
		menu.append(menu_ids::PREVIOUS_HEADING_4, &prev_heading4_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_HEADING_4, &next_heading4_label, "", ItemKind::Normal);
		menu.append(menu_ids::PREVIOUS_HEADING_5, &prev_heading5_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_HEADING_5, &next_heading5_label, "", ItemKind::Normal);
		menu.append(menu_ids::PREVIOUS_HEADING_6, &prev_heading6_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_HEADING_6, &next_heading6_label, "", ItemKind::Normal);
	}

	fn create_bookmarks_submenu() -> Menu {
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
		let view_note_label = t("&View Note Text\tCtrl+Shift+W");
		let view_note_help = t("View the note at current position");
		Menu::builder()
			.append_item(menu_ids::PREVIOUS_BOOKMARK, &prev_bookmark_label, &prev_bookmark_help)
			.append_item(menu_ids::NEXT_BOOKMARK, &next_bookmark_label, &next_bookmark_help)
			.append_item(menu_ids::PREVIOUS_NOTE, &prev_note_label, &prev_note_help)
			.append_item(menu_ids::NEXT_NOTE, &next_note_label, &next_note_help)
			.append_separator()
			.append_item(menu_ids::JUMP_TO_ALL_BOOKMARKS, &all_bookmarks_label, &all_bookmarks_help)
			.append_item(menu_ids::JUMP_TO_BOOKMARKS_ONLY, &bookmarks_only_label, &bookmarks_only_help)
			.append_item(menu_ids::JUMP_TO_NOTES_ONLY, &notes_only_label, &notes_only_help)
			.append_item(menu_ids::VIEW_NOTE_TEXT, &view_note_label, &view_note_help)
			.build()
	}

	fn append_bookmarks_items(menu: &Menu) {
		let prev_bookmark_label = t("&Previous Bookmark\tShift+B");
		let next_bookmark_label = t("&Next Bookmark\tB");
		let prev_note_label = t("Previous &Note\tShift+N");
		let next_note_label = t("Next N&ote\tN");
		let all_bookmarks_label = t("Jump to &All...\tCtrl+B");
		let bookmarks_only_label = t("Jump to &Bookmarks Only...\tCtrl+Alt+B");
		let notes_only_label = t("Jump to Notes &Only...\tCtrl+Alt+M");
		let view_note_label = t("&View Note Text\tCtrl+Shift+W");
		menu.append(menu_ids::PREVIOUS_BOOKMARK, &prev_bookmark_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_BOOKMARK, &next_bookmark_label, "", ItemKind::Normal);
		menu.append(menu_ids::PREVIOUS_NOTE, &prev_note_label, "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_NOTE, &next_note_label, "", ItemKind::Normal);
		menu.append_separator();
		menu.append(menu_ids::JUMP_TO_ALL_BOOKMARKS, &all_bookmarks_label, "", ItemKind::Normal);
		menu.append(menu_ids::JUMP_TO_BOOKMARKS_ONLY, &bookmarks_only_label, "", ItemKind::Normal);
		menu.append(menu_ids::JUMP_TO_NOTES_ONLY, &notes_only_label, "", ItemKind::Normal);
		menu.append(menu_ids::VIEW_NOTE_TEXT, &view_note_label, "", ItemKind::Normal);
	}

	fn create_tools_menu() -> Menu {
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

		let word_count_label = t("&Word Count\tCtrl+W");
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
		menu.append(menu_ids::OPTIONS, &options_label, "", ItemKind::Normal);
		menu.append(menu_ids::SLEEP_TIMER, &sleep_label, "", ItemKind::Normal);
		menu
	}

	fn create_help_menu() -> Menu {
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

	/// Bind menu event handlers
	fn bind_menu_events(
		frame: &Frame,
		doc_manager: Rc<Mutex<DocumentManager>>,
		config: Rc<Mutex<ConfigManager>>,
		find_dialog: Rc<Mutex<Option<FindDialogState>>>,
		live_region_label: StaticText,
	) {
		let frame_copy = *frame;
		let dm = Rc::clone(&doc_manager);
		let config = Rc::clone(&config);
		let find_dialog = Rc::clone(&find_dialog);
		let live_region_label = live_region_label;
		let sleep_timer = Rc::new(Timer::new(frame));
		let sleep_timer_running = Rc::new(Cell::new(false));
		let sleep_timer_start_time = Rc::new(Cell::new(0i64));
		let sleep_timer_duration_minutes = Rc::new(Cell::new(0i32));
		let sleep_timer_for_tick = Rc::clone(&sleep_timer);
		let sleep_timer_running_for_tick = Rc::clone(&sleep_timer_running);
		let frame_for_timer = *frame;
		let dm_for_timer = Rc::clone(&doc_manager);
		let config_for_timer = Rc::clone(&config);
		sleep_timer.on_tick(move |_| {
			sleep_timer_running_for_tick.set(false);
			sleep_timer_for_tick.stop();
			SLEEP_TIMER_START_MS.store(0, Ordering::SeqCst);
			SLEEP_TIMER_DURATION_MINUTES.store(0, Ordering::SeqCst);
			{
				let dm = dm_for_timer.lock().unwrap();
				let cfg = config_for_timer.lock().unwrap();
				for i in 0..dm.tab_count() {
					if let Some(tab) = dm.get_tab(i) {
						let current_pos = tab.text_ctrl.get_insertion_point();
						let path_str = tab.file_path.to_string_lossy();
						cfg.set_document_position(&path_str, current_pos);
					}
				}
				cfg.flush();
			}
			frame_for_timer.close(true);
		});

		// Create a status update timer for sleep countdown display
		let status_update_timer = Rc::new(Timer::new(frame));
		let sleep_timer_running_for_status = Rc::clone(&sleep_timer_running);
		let sleep_timer_start_for_status = Rc::clone(&sleep_timer_start_time);
		let sleep_timer_duration_for_status = Rc::clone(&sleep_timer_duration_minutes);
		let dm_for_status = Rc::clone(&doc_manager);
		let frame_for_status = *frame;
		status_update_timer.on_tick(move |_| {
			if !sleep_timer_running_for_status.get() {
				return;
			}
			let dm = match dm_for_status.try_lock() {
				Ok(dm) => dm,
				Err(_) => return,
			};
			update_status_bar_with_sleep_timer(
				&frame_for_status,
				&dm,
				sleep_timer_start_for_status.get(),
				sleep_timer_duration_for_status.get(),
			);
		});
		// Start the status update timer (runs every second)
		status_update_timer.start(1000, false);

		let sleep_timer_for_menu = Rc::clone(&sleep_timer);
		let sleep_timer_running_for_menu = Rc::clone(&sleep_timer_running);
		let sleep_timer_start_for_menu = Rc::clone(&sleep_timer_start_time);
		let sleep_timer_duration_for_menu = Rc::clone(&sleep_timer_duration_minutes);
		frame.on_menu(move |event| {
			let id = event.get_id();
			match id {
				menu_ids::OPEN => {
					Self::handle_open(&frame_copy, &dm, &config);
				}
				menu_ids::CLOSE => {
					let mut dm = dm.lock().unwrap();
					if let Some(index) = dm.active_tab_index() {
						dm.close_document(index);
					}
					update_title_from_manager(&frame_copy, &dm);
					if dm.tab_count() == 0 {
						dm.notebook().set_focus();
					} else {
						dm.restore_focus();
					}
				}
				menu_ids::CLOSE_ALL => {
					let mut dm = dm.lock().unwrap();
					dm.close_all_documents();
					update_title_from_manager(&frame_copy, &dm);
					dm.notebook().set_focus();
				}
				menu_ids::EXIT => {
					std::process::exit(0);
				}

				// Navigation commands would go here
				menu_ids::FIND => {
					show_find_dialog(&frame_copy, &dm, &config, &find_dialog, live_region_label);
				}
				menu_ids::FIND_NEXT => {
					handle_find_action(&frame_copy, &dm, &config, &find_dialog, live_region_label, true);
				}
				menu_ids::FIND_PREVIOUS => {
					handle_find_action(&frame_copy, &dm, &config, &find_dialog, live_region_label, false);
				}
				menu_ids::GO_TO_LINE => {
					let mut dm_guard = dm.lock().unwrap();
					let Some(tab) = dm_guard.active_tab_mut() else {
						return;
					};
					let current_pos = tab.text_ctrl.get_insertion_point();
					if let Some(line) = dialogs::show_go_to_line_dialog(&frame_copy, &tab.session, current_pos) {
						let target_pos = tab.session.position_from_line(line);
						tab.session.record_position(current_pos);
						tab.text_ctrl.set_focus();
						tab.text_ctrl.set_insertion_point(target_pos);
						tab.text_ctrl.show_position(target_pos);
						let (history, history_index) = tab.session.get_history();
						let path_str = tab.file_path.to_string_lossy();
						let cfg = config.lock().unwrap();
						cfg.set_navigation_history(&path_str, history, history_index);
					}
				}
				menu_ids::GO_TO_PAGE => {
					let mut dm_guard = dm.lock().unwrap();
					let Some(tab) = dm_guard.active_tab_mut() else {
						return;
					};
					if tab.session.page_count() == 0 {
						live_region::announce(&live_region_label, &t("No pages."));
						return;
					}
					let current_pos = tab.text_ctrl.get_insertion_point();
					let current_page = tab.session.current_page(current_pos);
					if let Some(target_pos) = dialogs::show_go_to_page_dialog(&frame_copy, &tab.session, current_page) {
						tab.session.record_position(current_pos);
						tab.text_ctrl.set_focus();
						tab.text_ctrl.set_insertion_point(target_pos);
						tab.text_ctrl.show_position(target_pos);
						let (history, history_index) = tab.session.get_history();
						let path_str = tab.file_path.to_string_lossy();
						let cfg = config.lock().unwrap();
						cfg.set_navigation_history(&path_str, history, history_index);
					}
				}
				menu_ids::GO_TO_PERCENT => {
					let mut dm_guard = dm.lock().unwrap();
					let Some(tab) = dm_guard.active_tab_mut() else {
						return;
					};
					let current_pos = tab.text_ctrl.get_insertion_point();
					if let Some(target_pos) = dialogs::show_go_to_percent_dialog(&frame_copy, &tab.session, current_pos)
					{
						tab.session.record_position(current_pos);
						tab.text_ctrl.set_focus();
						tab.text_ctrl.set_insertion_point(target_pos);
						tab.text_ctrl.show_position(target_pos);
						let (history, history_index) = tab.session.get_history();
						let path_str = tab.file_path.to_string_lossy();
						let cfg = config.lock().unwrap();
						cfg.set_navigation_history(&path_str, history, history_index);
					}
				}
				menu_ids::GO_BACK => {
					handle_history_navigation(&dm, &config, live_region_label, false);
				}
				menu_ids::GO_FORWARD => {
					handle_history_navigation(&dm, &config, live_region_label, true);
				}
				menu_ids::PREVIOUS_SECTION => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Section, false);
				}
				menu_ids::NEXT_SECTION => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Section, true);
				}
				menu_ids::PREVIOUS_HEADING => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(0), false);
				}
				menu_ids::NEXT_HEADING => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(0), true);
				}
				menu_ids::PREVIOUS_HEADING_1 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(1), false);
				}
				menu_ids::NEXT_HEADING_1 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(1), true);
				}
				menu_ids::PREVIOUS_HEADING_2 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(2), false);
				}
				menu_ids::NEXT_HEADING_2 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(2), true);
				}
				menu_ids::PREVIOUS_HEADING_3 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(3), false);
				}
				menu_ids::NEXT_HEADING_3 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(3), true);
				}
				menu_ids::PREVIOUS_HEADING_4 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(4), false);
				}
				menu_ids::NEXT_HEADING_4 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(4), true);
				}
				menu_ids::PREVIOUS_HEADING_5 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(5), false);
				}
				menu_ids::NEXT_HEADING_5 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(5), true);
				}
				menu_ids::PREVIOUS_HEADING_6 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(6), false);
				}
				menu_ids::NEXT_HEADING_6 => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Heading(6), true);
				}
				menu_ids::PREVIOUS_PAGE => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Page, false);
				}
				menu_ids::NEXT_PAGE => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Page, true);
				}
				menu_ids::PREVIOUS_BOOKMARK => {
					handle_bookmark_navigation(&dm, &config, live_region_label, false, false);
				}
				menu_ids::NEXT_BOOKMARK => {
					handle_bookmark_navigation(&dm, &config, live_region_label, true, false);
				}
				menu_ids::PREVIOUS_NOTE => {
					handle_bookmark_navigation(&dm, &config, live_region_label, false, true);
				}
				menu_ids::NEXT_NOTE => {
					handle_bookmark_navigation(&dm, &config, live_region_label, true, true);
				}
				menu_ids::JUMP_TO_ALL_BOOKMARKS => {
					handle_bookmark_dialog(&frame_copy, &dm, &config, live_region_label, BookmarkFilterType::All);
				}
				menu_ids::JUMP_TO_BOOKMARKS_ONLY => {
					handle_bookmark_dialog(
						&frame_copy,
						&dm,
						&config,
						live_region_label,
						BookmarkFilterType::BookmarksOnly,
					);
				}
				menu_ids::JUMP_TO_NOTES_ONLY => {
					handle_bookmark_dialog(&frame_copy, &dm, &config, live_region_label, BookmarkFilterType::NotesOnly);
				}
				menu_ids::TOGGLE_BOOKMARK => {
					handle_toggle_bookmark(&dm, &config, live_region_label);
				}
				menu_ids::BOOKMARK_WITH_NOTE => {
					handle_bookmark_with_note(&frame_copy, &dm, &config, live_region_label);
				}
				menu_ids::VIEW_NOTE_TEXT => {
					handle_view_note_text(&frame_copy, &dm, &config);
				}
				menu_ids::PREVIOUS_LINK => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Link, false);
				}
				menu_ids::NEXT_LINK => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Link, true);
				}
				menu_ids::PREVIOUS_TABLE => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Table, false);
				}
				menu_ids::NEXT_TABLE => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Table, true);
				}
				menu_ids::PREVIOUS_LIST => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::List, false);
				}
				menu_ids::NEXT_LIST => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::List, true);
				}
				menu_ids::PREVIOUS_LIST_ITEM => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::ListItem, false);
				}
				menu_ids::NEXT_LIST_ITEM => {
					handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::ListItem, true);
				}
				menu_ids::EXPORT_TO_PLAIN_TEXT => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					let Some(tab) = dm_ref.active_tab() else {
						return;
					};
					let default_name = tab
						.file_path
						.file_stem()
						.map(|s| s.to_string_lossy().to_string())
						.unwrap_or_else(|| t("document"));
					let default_file = format!("{default_name}.txt");
					let wildcard = t("Plain text files (*.txt)|*.txt|All files (*.*)|*.*");
					let dialog = FileDialog::builder(&frame_copy)
						.with_message(&t("Export document to plain text"))
						.with_default_file(&default_file)
						.with_wildcard(&wildcard)
						.with_style(FileDialogStyle::Save | FileDialogStyle::OverwritePrompt)
						.build();
					if dialog.show_modal() == wxdragon::id::ID_OK {
						if let Some(path) = dialog.get_path() {
							if tab.session.export_content(&path).is_err() {
								let dialog =
									MessageDialog::builder(&frame_copy, &t("Failed to export document."), &t("Error"))
										.with_style(
											MessageDialogStyle::OK
												| MessageDialogStyle::IconError | MessageDialogStyle::Centre,
										)
										.build();
								dialog.show_modal();
							}
						}
					}
				}
				menu_ids::EXPORT_DOCUMENT_DATA => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					let Some(tab) = dm_ref.active_tab() else {
						return;
					};
					let default_name = tab
						.file_path
						.file_stem()
						.map(|s| s.to_string_lossy().to_string())
						.unwrap_or_else(|| t("document"));
					let default_file = format!("{default_name}.paperback");
					let wildcard = t("Paperback files (*.paperback)|*.paperback");
					let dialog = FileDialog::builder(&frame_copy)
						.with_message(&t("Export notes and bookmarks"))
						.with_default_file(&default_file)
						.with_wildcard(&wildcard)
						.with_style(FileDialogStyle::Save | FileDialogStyle::OverwritePrompt)
						.build();
					if dialog.show_modal() == wxdragon::id::ID_OK {
						if let Some(path) = dialog.get_path() {
							let path_str = tab.file_path.to_string_lossy();
							config.lock().unwrap().export_document_settings(&path_str, &path);
							let dialog = MessageDialog::builder(
								&frame_copy,
								&t("Notes and bookmarks exported successfully."),
								&t("Export Successful"),
							)
							.with_style(
								MessageDialogStyle::OK
									| MessageDialogStyle::IconInformation
									| MessageDialogStyle::Centre,
							)
							.build();
							dialog.show_modal();
						}
					}
				}
				menu_ids::IMPORT_DOCUMENT_DATA => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					let Some(tab) = dm_ref.active_tab() else {
						return;
					};
					let wildcard = t("Paperback files (*.paperback)|*.paperback");
					let dialog = FileDialog::builder(&frame_copy)
						.with_message(&t("Import notes and bookmarks"))
						.with_wildcard(&wildcard)
						.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
						.build();
					if dialog.show_modal() == wxdragon::id::ID_OK {
						if let Some(path) = dialog.get_path() {
							let path_str = tab.file_path.to_string_lossy();
							{
								let config = config.lock().unwrap();
								config.import_settings_from_file(&path_str, &path);
								let max_pos = tab.text_ctrl.get_last_position();
								let pos = config.get_validated_document_position(&path_str, max_pos);
								if pos >= 0 {
									tab.text_ctrl.set_insertion_point(pos);
									tab.text_ctrl.show_position(pos);
								}
							}
							let dialog = MessageDialog::builder(
								&frame_copy,
								&t("Notes and bookmarks imported successfully."),
								&t("Import Successful"),
							)
							.with_style(
								MessageDialogStyle::OK
									| MessageDialogStyle::IconInformation
									| MessageDialogStyle::Centre,
							)
							.build();
							dialog.show_modal();
						}
					}
				}
				menu_ids::WORD_COUNT => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					if let Some(tab) = dm_ref.active_tab() {
						let stats = tab.session.stats();
						let msg_template = t("The document contains {} words.");
						let msg = msg_template.replace("{}", &stats.word_count.to_string());
						let title = t("Word count");
						let dialog = MessageDialog::builder(&frame_copy, &msg, &title)
							.with_style(MessageDialogStyle::OK)
							.build();
						dialog.show_modal();
					}
				}
				menu_ids::DOCUMENT_INFO => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					if let Some(tab) = dm_ref.active_tab() {
						let stats = tab.session.stats();
						let title = tab.session.title();
						let author = tab.session.author();
						dialogs::show_document_info_dialog(&frame_copy, &tab.file_path, &title, &author, stats);
					}
				}
				menu_ids::TABLE_OF_CONTENTS => {
					let mut dm_guard = dm.lock().unwrap();
					if let Some(tab) = dm_guard.active_tab_mut() {
						let current_pos = tab.text_ctrl.get_insertion_point();
						let current_toc_offset = tab.session.handle().find_closest_toc_offset(current_pos as usize);
						if let Some(offset) = dialogs::show_toc_dialog(
							&frame_copy,
							&tab.session.handle().document().toc_items,
							current_toc_offset as i32,
						) {
							tab.session.record_position(current_pos);
							tab.text_ctrl.set_focus();
							tab.text_ctrl.set_insertion_point(offset as i64);
							tab.text_ctrl.show_position(offset as i64);
							let (history, history_index) = tab.session.get_history();
							let path_str = tab.file_path.to_string_lossy();
							let cfg = config.lock().unwrap();
							cfg.set_navigation_history(&path_str, history, history_index);
						}
					}
				}
				menu_ids::ELEMENTS_LIST => {
					let mut dm_guard = dm.lock().unwrap();
					if let Some(tab) = dm_guard.active_tab_mut() {
						let current_pos = tab.text_ctrl.get_insertion_point();
						if let Some(offset) = dialogs::show_elements_dialog(&frame_copy, &tab.session, current_pos) {
							tab.session.record_position(current_pos);
							tab.text_ctrl.set_focus();
							tab.text_ctrl.set_insertion_point(offset);
							tab.text_ctrl.show_position(offset);
							let (history, history_index) = tab.session.get_history();
							let path_str = tab.file_path.to_string_lossy();
							let cfg = config.lock().unwrap();
							cfg.set_navigation_history(&path_str, history, history_index);
						}
					}
				}
				menu_ids::OPEN_IN_WEB_VIEW => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					let Some(tab) = dm_ref.active_tab() else {
						return;
					};
					let current_pos = tab.text_ctrl.get_insertion_point();
					let temp_dir = std::env::temp_dir().to_string_lossy().to_string();
					if let Some(target_path) = tab.session.webview_target_path(current_pos, &temp_dir) {
						let url = format!("file:///{}", target_path.replace("\\", "/"));
						dialogs::show_web_view_dialog(
							&frame_copy,
							&t("Web View"),
							&url,
							true,
							Some(Box::new(|url| {
								if url.to_lowercase().starts_with("http://")
									|| url.to_lowercase().starts_with("https://")
									|| url.to_lowercase().starts_with("mailto:")
								{
									wxdragon::utils::launch_default_browser(
										url,
										wxdragon::utils::BrowserLaunchFlags::Default,
									);
									false
								} else {
									true
								}
							})),
						);
					} else {
						let dialog = MessageDialog::builder(
							&frame_copy,
							&t("Could not determine content to display in Web View."),
							&t("Error"),
						)
						.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
						.build();
						dialog.show_modal();
					}
				}
				menu_ids::OPTIONS => {
					let current_language = TranslationManager::instance().lock().unwrap().current_language();
					let options = {
						let cfg = config.lock().unwrap();
						dialogs::show_options_dialog(&frame_copy, &cfg)
					};
					let Some(options) = options else {
						return;
					};
					let (old_word_wrap, old_compact_menu) = {
						let cfg = config.lock().unwrap();
						(cfg.get_app_bool("word_wrap", false), cfg.get_app_bool("compact_go_menu", true))
					};
					let cfg = config.lock().unwrap();
					cfg.set_app_bool("restore_previous_documents", options.restore_previous_documents);
					cfg.set_app_bool("word_wrap", options.word_wrap);
					cfg.set_app_bool("minimize_to_tray", options.minimize_to_tray);
					cfg.set_app_bool("start_maximized", options.start_maximized);
					cfg.set_app_bool("compact_go_menu", options.compact_go_menu);
					cfg.set_app_bool("navigation_wrap", options.navigation_wrap);
					cfg.set_app_bool("check_for_updates_on_startup", options.check_for_updates_on_startup);
					cfg.set_app_int("recent_documents_to_show", options.recent_documents_to_show);
					cfg.set_app_string("language", &options.language);
					cfg.flush();
					drop(cfg);
					if old_word_wrap != options.word_wrap {
						let mut dm_ref = dm.lock().unwrap();
						dm_ref.apply_word_wrap(options.word_wrap);
						dm_ref.restore_focus();
					}
					if current_language != options.language || old_compact_menu != options.compact_go_menu {
						if current_language != options.language {
							let _ = TranslationManager::instance().lock().unwrap().set_language(&options.language);
						}
						let dm_ref = dm.lock().unwrap();
						update_title_from_manager(&frame_copy, &dm_ref);
					}
					let menu_bar = Self::create_menu_bar(&config.lock().unwrap());
					frame_copy.set_menu_bar(menu_bar);
				}
				menu_ids::SLEEP_TIMER => {
					if sleep_timer_running_for_menu.get() {
						sleep_timer_for_menu.stop();
						sleep_timer_running_for_menu.set(false);
						sleep_timer_start_for_menu.set(0);
						sleep_timer_duration_for_menu.set(0);
						SLEEP_TIMER_START_MS.store(0, Ordering::SeqCst);
						SLEEP_TIMER_DURATION_MINUTES.store(0, Ordering::SeqCst);
						let dm_ref = dm.lock().unwrap();
						update_title_from_manager(&frame_copy, &dm_ref);
						live_region::announce(&live_region_label, &t("Sleep timer cancelled."));
						return;
					}
					let initial_duration = config.lock().unwrap().get_app_int("sleep_timer_duration", 30);
					if let Some(duration) = dialogs::show_sleep_timer_dialog(&frame_copy, initial_duration) {
						{
							let cfg = config.lock().unwrap();
							cfg.set_app_int("sleep_timer_duration", duration);
							cfg.flush();
						}
						let duration_ms = duration as u64 * 60 * 1000;
						sleep_timer_for_menu.start(duration_ms as i32, true);
						sleep_timer_running_for_menu.set(true);
						// Track start time and duration for countdown display
						let now = std::time::SystemTime::now()
							.duration_since(std::time::UNIX_EPOCH)
							.map(|d| d.as_millis() as i64)
							.unwrap_or(0);
						sleep_timer_start_for_menu.set(now);
						sleep_timer_duration_for_menu.set(duration);
						SLEEP_TIMER_START_MS.store(now, Ordering::SeqCst);
						SLEEP_TIMER_DURATION_MINUTES.store(duration, Ordering::SeqCst);
						let msg = if duration == 1 {
							t("Sleep timer set for 1 minute.")
						} else {
							t("Sleep timer set for %d minutes.").replace("%d", &duration.to_string())
						};
						live_region::announce(&live_region_label, &msg);
					}
				}
				menu_ids::ABOUT => {
					dialogs::show_about_dialog(&frame_copy);
				}
				menu_ids::CHECK_FOR_UPDATES => {
					run_update_check(false);
				}

				_ => {
					if id >= menu_ids::RECENT_DOCUMENT_BASE && id <= menu_ids::RECENT_DOCUMENT_MAX {
						let doc_index = id - menu_ids::RECENT_DOCUMENT_BASE;
						let recent_docs = {
							let config_guard = config.lock().unwrap();
							Self::recent_documents_for_menu_static(&config_guard)
						};
						if let Some(path) = recent_docs.get(doc_index as usize) {
							let path = Path::new(path);
							if !ensure_parser_ready_for_path(&frame_copy, path, &config) {
								return;
							}
							if dm.lock().unwrap().open_file(Rc::clone(&dm), path) {
								let dm_ref = dm.lock().unwrap();
								update_title_from_manager(&frame_copy, &dm_ref);
								dm_ref.restore_focus();
								let menu_bar = Self::create_menu_bar(&config.lock().unwrap());
								frame_copy.set_menu_bar(menu_bar);
							}
						}
					} else if id == menu_ids::SHOW_ALL_DOCUMENTS {
						let has_documents = {
							let config_guard = config.lock().unwrap();
							!config_guard.get_all_documents().is_empty()
						};
						if !has_documents {
							live_region::announce(&live_region_label, &t("No recent documents."));
							return;
						}
						let open_paths = dm.lock().unwrap().open_paths();
						let config_for_dialog = Rc::clone(&config);
						let selection = dialogs::show_all_documents_dialog(&frame_copy, config_for_dialog, open_paths);
						if let Some(path) = selection {
							let path = Path::new(&path);
							if !ensure_parser_ready_for_path(&frame_copy, path, &config) {
								return;
							}
							if dm.lock().unwrap().open_file(Rc::clone(&dm), path) {
								let dm_ref = dm.lock().unwrap();
								update_title_from_manager(&frame_copy, &dm_ref);
								dm_ref.restore_focus();
								let menu_bar = Self::create_menu_bar(&config.lock().unwrap());
								frame_copy.set_menu_bar(menu_bar);
							}
						} else {
							let menu_bar = Self::create_menu_bar(&config.lock().unwrap());
							frame_copy.set_menu_bar(menu_bar);
						}
					}
				}
			}
		});
	}

	/// Handle the Open menu command
	fn handle_open(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>, config: &Rc<Mutex<ConfigManager>>) {
		let wildcard = "All supported files|*.epub;*.pdf;*.txt;*.md;*.html;*.htm;*.docx;*.odt;*.fb2;*.chm;*.pptx;*.odp|\
                        EPUB files (*.epub)|*.epub|\
                        PDF files (*.pdf)|*.pdf|\
                        Text files (*.txt)|*.txt|\
                        Markdown files (*.md)|*.md|\
                        HTML files (*.html;*.htm)|*.html;*.htm|\
                        Word documents (*.docx)|*.docx|\
                        OpenDocument Text (*.odt)|*.odt|\
                        FictionBook2 (*.fb2)|*.fb2|\
                        CHM files (*.chm)|*.chm|\
                        PowerPoint (*.pptx)|*.pptx|\
                        OpenDocument Presentation (*.odp)|*.odp|\
                        All files (*.*)|*.*";

		let dialog_title = t("Open Document");
		let dialog = FileDialog::builder(frame)
			.with_message(&dialog_title)
			.with_wildcard(wildcard)
			.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
			.build();

		if dialog.show_modal() == wxdragon::id::ID_OK {
			if let Some(path) = dialog.get_path() {
				let path = std::path::Path::new(&path);
				if !ensure_parser_ready_for_path(frame, path, config) {
					return;
				}
				if doc_manager.lock().unwrap().open_file(Rc::clone(doc_manager), path) {
					let dm_ref = match doc_manager.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					update_title_from_manager(frame, &dm_ref);
					dm_ref.restore_focus();
				}
			}
		}
	}

	/// Get the frame
	#[allow(dead_code)]
	pub fn frame(&self) -> &Frame {
		&self.frame
	}

	/// Get the document manager
	#[allow(dead_code)]
	pub fn doc_manager(&self) -> &Rc<Mutex<DocumentManager>> {
		&self.doc_manager
	}

	pub fn live_region_label(&self) -> StaticText {
		self.live_region_label
	}

	fn ensure_parser_ready(&self, path: &Path) -> bool {
		ensure_parser_ready_for_path(&self.frame, path, &self._config)
	}

	fn update_recent_documents_menu(&self) {
		let menu_bar = Self::create_menu_bar(&self._config.lock().unwrap());
		self.frame.set_menu_bar(menu_bar);
	}

	fn populate_recent_documents_menu(menu: &Menu, config: &ConfigManager) {
		let recent_docs = Self::recent_documents_for_menu_static(config);
		if recent_docs.is_empty() {
			let empty_label = t("(No recent documents)");
			if let Some(item) =
				menu.append(wxdragon::id::ID_ANY.try_into().unwrap(), &empty_label, "", ItemKind::Normal)
			{
				item.enable(false);
			}
		} else {
			for (index, path) in recent_docs.iter().enumerate() {
				let filename = Path::new(path)
					.file_name()
					.map(|s| s.to_string_lossy().to_string())
					.unwrap_or_else(|| path.clone());
				let label = format!("&{} {}", index + 1, filename);
				let id = menu_ids::RECENT_DOCUMENT_BASE + index as i32;
				let _ = menu.append(id, &label, path, ItemKind::Normal);
			}
		}
		menu.append_separator();
		let show_all_label = t("Show All...\tCtrl+R");
		let _ = menu.append(menu_ids::SHOW_ALL_DOCUMENTS, &show_all_label, "", ItemKind::Normal);
	}

	fn recent_documents_for_menu_static(config: &ConfigManager) -> Vec<String> {
		let limit = config.get_app_int("recent_documents_to_show", 25).max(0) as usize;
		let mut docs = config.get_recent_documents();
		if docs.len() > limit {
			docs.truncate(limit);
		}
		docs
	}

	fn schedule_restore_documents(
		frame: Frame,
		doc_manager: Rc<Mutex<DocumentManager>>,
		config: Rc<Mutex<ConfigManager>>,
	) {
		let restore = config.lock().unwrap().get_app_bool("restore_previous_documents", true);
		if !restore {
			return;
		}
		#[derive(Default)]
		struct RestoreState {
			restored: bool,
			closing: bool,
		}
		let state = Rc::new(Mutex::new(RestoreState::default()));
		let state_for_close = Rc::clone(&state);
		frame.on_close(move |_event| {
			state_for_close.lock().unwrap().closing = true;
		});
		let state_for_destroy = Rc::clone(&state);
		frame.on_destroy(move |_event| {
			state_for_destroy.lock().unwrap().closing = true;
		});
		let state_for_idle = Rc::clone(&state);
		frame.on_idle(move |_event| {
			let mut state = state_for_idle.lock().unwrap();
			if state.restored || state.closing {
				return;
			}
			state.restored = true;
			drop(state);
			let paths = config.lock().unwrap().get_opened_documents_existing();
			for path in paths {
				let path = Path::new(&path);
				if !ensure_parser_ready_for_path(&frame, path, &config) {
					continue;
				}
				let _ = doc_manager.lock().unwrap().open_file(Rc::clone(&doc_manager), path);
			}
			let dm_ref = doc_manager.lock().unwrap();
			update_title_from_manager(&frame, &dm_ref);
			let menu_bar = Self::create_menu_bar(&config.lock().unwrap());
			frame.set_menu_bar(menu_bar);
			dm_ref.restore_focus();
		});
	}

	fn bind_tray_events(
		frame: Frame,
		doc_manager: Rc<Mutex<DocumentManager>>,
		config: Rc<Mutex<ConfigManager>>,
		tray_state: Rc<Mutex<Option<TrayState>>>,
	) {
		let frame_for_size = frame;
		let tray_state_for_size = Rc::clone(&tray_state);
		let config_for_size = Rc::clone(&config);
		let doc_manager_for_size = Rc::clone(&doc_manager);
		frame.on_size(move |_event| {
			if !frame_for_size.is_iconized() {
				return;
			}
			let minimize_to_tray = config_for_size.lock().unwrap().get_app_bool("minimize_to_tray", false);
			if !minimize_to_tray {
				return;
			}
			let mut tray_state_guard = tray_state_for_size.lock().unwrap();
			if tray_state_guard.is_none() {
				if let Some(state) =
					create_tray_state(frame_for_size, Rc::clone(&doc_manager_for_size), Rc::clone(&tray_state_for_size))
				{
					*tray_state_guard = Some(state);
				}
			}
			frame_for_size.show(false);
		});

		let frame_for_idle = frame;
		let tray_state_for_idle = Rc::clone(&tray_state);
		let config_for_idle = Rc::clone(&config);
		let doc_manager_for_idle = Rc::clone(&doc_manager);
		frame.on_idle(move |_event| {
			if !frame_for_idle.is_iconized() {
				return;
			}
			let minimize_to_tray = config_for_idle.lock().unwrap().get_app_bool("minimize_to_tray", false);
			if !minimize_to_tray {
				return;
			}
			let mut tray_state_guard = tray_state_for_idle.lock().unwrap();
			if tray_state_guard.is_none() {
				if let Some(state) =
					create_tray_state(frame_for_idle, Rc::clone(&doc_manager_for_idle), Rc::clone(&tray_state_for_idle))
				{
					*tray_state_guard = Some(state);
				}
			}
			frame_for_idle.show(false);
		});
	}
}

fn ensure_parser_ready_for_path(frame: &Frame, path: &Path, config: &Rc<Mutex<ConfigManager>>) -> bool {
	let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();
	if extension.is_empty() || parser_supports_extension(extension) {
		return true;
	}
	let mut cfg = config.lock().unwrap();
	utils::ensure_parser_for_unknown_file(frame, path, &mut cfg)
}

#[derive(Clone)]
struct FindDialogState {
	dialog: Dialog,
	find_combo: ComboBox,
	match_case: CheckBox,
	whole_word: CheckBox,
	use_regex: CheckBox,
	in_progress: Rc<Cell<bool>>,
}

impl FindDialogState {
	fn new(
		frame: &Frame,
		config: &Rc<Mutex<ConfigManager>>,
		doc_manager: &Rc<Mutex<DocumentManager>>,
		find_dialog: &Rc<Mutex<Option<FindDialogState>>>,
		live_region_label: StaticText,
	) -> Self {
		let dialog = Dialog::builder(frame, &t("Find")).build();
		let combo_width = 250;
		let option_padding = 2;
		let button_spacing = 5;

		let find_label = StaticText::builder(&dialog).with_label(&t("Find &what:")).build();
		let find_combo = ComboBox::builder(&dialog)
			.with_style(ComboBoxStyle::ProcessEnter)
			.with_size(Size::new(combo_width, -1))
			.build();

		let options_box = StaticBoxSizerBuilder::new_with_label(Orientation::Vertical, &dialog, &t("Options")).build();
		let match_case = CheckBox::builder(&dialog).with_label(&t("&Match case")).build();
		let whole_word = CheckBox::builder(&dialog).with_label(&t("Match &whole word")).build();
		let use_regex = CheckBox::builder(&dialog).with_label(&t("Use &regular expressions")).build();
		options_box.add(&match_case, 0, SizerFlag::All, option_padding);
		options_box.add(&whole_word, 0, SizerFlag::All, option_padding);
		options_box.add(&use_regex, 0, SizerFlag::All, option_padding);

		let find_prev_btn = Button::builder(&dialog).with_label(&t("Find &Previous")).build();
		let find_next_btn = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("Find &Next")).build();
		let cancel_btn = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
		dialog.set_escape_id(wxdragon::id::ID_CANCEL);
		dialog.set_affirmative_id(wxdragon::id::ID_OK);

		let find_sizer = BoxSizer::builder(Orientation::Horizontal).build();
		find_sizer.add(&find_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
		find_sizer.add(&find_combo, 1, SizerFlag::Expand, 0);

		let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
		button_sizer.add(&find_prev_btn, 0, SizerFlag::Right, button_spacing);
		button_sizer.add(&find_next_btn, 0, SizerFlag::Right, button_spacing);
		button_sizer.add_stretch_spacer(1);
		button_sizer.add(&cancel_btn, 0, SizerFlag::All, 0);

		let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
		main_sizer.add_sizer(&find_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
		main_sizer.add_sizer(
			&options_box,
			0,
			SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
			DIALOG_PADDING,
		);
		main_sizer.add_sizer(
			&button_sizer,
			0,
			SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
			DIALOG_PADDING,
		);

		dialog.set_sizer_and_fit(main_sizer, true);
		dialog.centre();

		let frame_for_next = *frame;
		let find_dialog_for_next = Rc::clone(find_dialog);
		let doc_manager_for_next = Rc::clone(doc_manager);
		let config_for_next = Rc::clone(config);
		find_next_btn.on_click(move |_| {
			handle_find_action(
				&frame_for_next,
				&doc_manager_for_next,
				&config_for_next,
				&find_dialog_for_next,
				live_region_label,
				true,
			);
		});

		let frame_for_prev = *frame;
		let find_dialog_for_prev = Rc::clone(find_dialog);
		let doc_manager_for_prev = Rc::clone(doc_manager);
		let config_for_prev = Rc::clone(config);
		find_prev_btn.on_click(move |_| {
			handle_find_action(
				&frame_for_prev,
				&doc_manager_for_prev,
				&config_for_prev,
				&find_dialog_for_prev,
				live_region_label,
				false,
			);
		});

		let dialog_for_cancel = dialog;
		let find_dialog_for_cancel = Rc::clone(find_dialog);
		let config_for_cancel = Rc::clone(config);
		cancel_btn.on_click(move |_| {
			if let Some(state) = find_dialog_for_cancel.lock().unwrap().as_ref() {
				state.save_settings(&config_for_cancel);
				dialog_for_cancel.show(false);
			}
		});

		let frame_for_enter = *frame;
		let find_dialog_for_enter = Rc::clone(find_dialog);
		let doc_manager_for_enter = Rc::clone(doc_manager);
		let config_for_enter = Rc::clone(config);
		find_combo.bind_internal(EventType::TEXT_ENTER, move |event| {
			handle_find_action(
				&frame_for_enter,
				&doc_manager_for_enter,
				&config_for_enter,
				&find_dialog_for_enter,
				live_region_label,
				true,
			);
			event.skip(false);
		});

		let dialog_for_close = dialog;
		let find_dialog_for_close = Rc::clone(find_dialog);
		let config_for_close = Rc::clone(config);
		dialog.on_close(move |event| {
			if let Some(state) = find_dialog_for_close.lock().unwrap().as_ref() {
				state.save_settings(&config_for_close);
			}
			dialog_for_close.show(false);
			event.skip(false);
		});

		let state = FindDialogState {
			dialog,
			find_combo,
			match_case,
			whole_word,
			use_regex,
			in_progress: Rc::new(Cell::new(false)),
		};
		state.reload_history(config);
		state.save_settings(config);
		state
	}

	fn reload_history(&self, config: &Rc<Mutex<ConfigManager>>) {
		self.find_combo.clear();
		let settings = {
			let cfg = config.lock().unwrap();
			for entry in cfg.get_find_history() {
				self.find_combo.append(&entry);
			}
			cfg.get_find_settings()
		};
		self.match_case.set_value(settings.match_case);
		self.whole_word.set_value(settings.whole_word);
		self.use_regex.set_value(settings.use_regex);
	}

	fn save_settings(&self, config: &Rc<Mutex<ConfigManager>>) {
		let settings = crate::config::FindSettings {
			match_case: self.match_case.is_checked(),
			whole_word: self.whole_word.is_checked(),
			use_regex: self.use_regex.is_checked(),
		};
		config.lock().unwrap().set_find_settings(settings);
	}

	fn add_to_history(&self, config: &Rc<Mutex<ConfigManager>>, text: &str) {
		config.lock().unwrap().add_find_history(text, MAX_FIND_HISTORY_SIZE);
		self.reload_history(config);
		self.find_combo.set_value(text);
	}

	fn find_text(&self) -> String {
		self.find_combo.get_value()
	}

	fn set_find_text(&self, text: &str) {
		self.find_combo.set_value(text);
		let len = self.find_combo.get_last_position();
		self.find_combo.set_text_selection(0, len);
	}

	fn focus_find_text(&self) {
		self.find_combo.set_focus();
		let len = self.find_combo.get_last_position();
		self.find_combo.set_text_selection(0, len);
	}

	fn try_begin_find(&self) -> Option<FindInProgressGuard> {
		if self.in_progress.replace(true) {
			return None;
		}
		Some(FindInProgressGuard { flag: Rc::clone(&self.in_progress) })
	}
}

struct FindInProgressGuard {
	flag: Rc<std::cell::Cell<bool>>,
}

impl Drop for FindInProgressGuard {
	fn drop(&mut self) {
		self.flag.set(false);
	}
}

fn ensure_find_dialog(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	find_dialog: &Rc<Mutex<Option<FindDialogState>>>,
	live_region_label: StaticText,
) {
	let mut dialog_guard = find_dialog.lock().unwrap();
	if dialog_guard.is_some() {
		return;
	}
	let state = FindDialogState::new(frame, config, doc_manager, find_dialog, live_region_label);
	*dialog_guard = Some(state);
}

fn show_find_dialog(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	find_dialog: &Rc<Mutex<Option<FindDialogState>>>,
	live_region_label: StaticText,
) {
	ensure_find_dialog(frame, doc_manager, config, find_dialog, live_region_label);
	let state = {
		let dialog_state = find_dialog.lock().unwrap();
		dialog_state.as_ref().cloned()
	};
	let Some(state) = state else {
		return;
	};
	let text_ctrl = {
		let dm = doc_manager.lock().unwrap();
		dm.active_tab().map(|tab| tab.text_ctrl)
	};
	if let Some(text_ctrl) = text_ctrl {
		let (start, end) = text_ctrl.get_selection();
		if start != end {
			let selection = text_ctrl.get_string_selection();
			state.set_find_text(&selection);
		}
	}
	state.dialog.show(true);
	state.dialog.raise();
	state.focus_find_text();
}

fn handle_find_action(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	find_dialog: &Rc<Mutex<Option<FindDialogState>>>,
	live_region_label: StaticText,
	forward: bool,
) {
	ensure_find_dialog(frame, doc_manager, config, find_dialog, live_region_label);
	let state = {
		let dialog_state = find_dialog.lock().unwrap();
		dialog_state.as_ref().cloned()
	};
	let Some(state) = state else {
		return;
	};
	if state.find_text().trim().is_empty() {
		let text_ctrl = {
			let dm = doc_manager.lock().unwrap();
			dm.active_tab().map(|tab| tab.text_ctrl)
		};
		if let Some(text_ctrl) = text_ctrl {
			let (start, end) = text_ctrl.get_selection();
			if start != end {
				let selection = text_ctrl.get_string_selection();
				state.set_find_text(&selection);
			}
		}
	}
	if state.find_text().trim().is_empty() {
		show_find_dialog(frame, doc_manager, config, find_dialog, live_region_label);
		return;
	}
	do_find(forward, &state, doc_manager, config, live_region_label);
}

fn do_find(
	forward: bool,
	state: &FindDialogState,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let text_ctrl = {
		let dm = doc_manager.lock().unwrap();
		match dm.active_tab() {
			Some(tab) => tab.text_ctrl,
			None => return,
		}
	};
	if !text_ctrl.is_valid() {
		return;
	}
	let query = state.find_text();
	if query.trim().is_empty() {
		return;
	}
	let _find_guard = match state.try_begin_find() {
		Some(guard) => guard,
		None => return,
	};
	state.save_settings(config);
	state.add_to_history(config, &query);
	let mut options = utils::FindOptions::default();
	if forward {
		options |= utils::FindOptions::FORWARD;
	}
	if state.match_case.is_checked() {
		options |= utils::FindOptions::MATCH_CASE;
	}
	if state.whole_word.is_checked() {
		options |= utils::FindOptions::MATCH_WHOLE_WORD;
	}
	if state.use_regex.is_checked() {
		options |= utils::FindOptions::USE_REGEX;
	}
	let (sel_start, sel_end) = text_ctrl.get_selection();
	let start_pos = if forward { sel_end } else { sel_start };
	let result = utils::find_text_with_wrap(&text_ctrl.get_value(), &query, start_pos, options);
	if !result.found {
		live_region::announce(&live_region_label, &t("Not found."));
		state.dialog.show(true);
		state.dialog.raise();
		state.focus_find_text();
		return;
	}
	if result.wrapped {
		live_region::announce(&live_region_label, &t("No more results. Wrapping search."));
	}
	if result.position < 0 {
		return;
	}
	let len = display_len(&query) as i64;
	let last_pos = text_ctrl.get_last_position();
	if last_pos <= 0 {
		return;
	}
	let start = result.position.clamp(0, last_pos);
	let end = (start + len).min(last_pos);
	text_ctrl.set_focus();
	text_ctrl.set_selection(start, end);
	text_ctrl.show_position(start);
	state.dialog.show(false);
}

#[derive(Clone, Copy)]
enum MarkerNavTarget {
	Section,
	Page,
	Heading(i32),
	Link,
	Table,
	List,
	ListItem,
}

enum NavFoundFormat {
	TextOnly,
	TextWithLevel,
	PageFormat,
	LinkFormat,
}

struct NavAnnouncements {
	not_supported: String,
	not_found_next: String,
	not_found_prev: String,
	format: NavFoundFormat,
}

fn nav_announcements(target: MarkerNavTarget, level_filter: i32) -> NavAnnouncements {
	match target {
		MarkerNavTarget::Section => NavAnnouncements {
			not_supported: t("No sections."),
			not_found_next: t("No next section"),
			not_found_prev: t("No previous section"),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Heading(level) => {
			if level_filter > 0 {
				let no_headings = t("No headings at level %d.");
				let no_next = t("No next heading at level %d.");
				let no_prev = t("No previous heading at level %d.");
				NavAnnouncements {
					not_supported: no_headings.replacen("%d", &level.to_string(), 1),
					not_found_next: no_next.replacen("%d", &level.to_string(), 1),
					not_found_prev: no_prev.replacen("%d", &level.to_string(), 1),
					format: NavFoundFormat::TextWithLevel,
				}
			} else {
				NavAnnouncements {
					not_supported: t("No headings."),
					not_found_next: t("No next heading."),
					not_found_prev: t("No previous heading."),
					format: NavFoundFormat::TextWithLevel,
				}
			}
		}
		MarkerNavTarget::Page => NavAnnouncements {
			not_supported: t("No pages."),
			not_found_next: t("No next page."),
			not_found_prev: t("No previous page."),
			format: NavFoundFormat::PageFormat,
		},
		MarkerNavTarget::Link => NavAnnouncements {
			not_supported: t("No links."),
			not_found_next: t("No next link."),
			not_found_prev: t("No previous link."),
			format: NavFoundFormat::LinkFormat,
		},
		MarkerNavTarget::List => NavAnnouncements {
			not_supported: t("No lists."),
			not_found_next: t("No next list."),
			not_found_prev: t("No previous list."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::ListItem => NavAnnouncements {
			not_supported: t("No list items."),
			not_found_next: t("No next list item."),
			not_found_prev: t("No previous list item."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Table => NavAnnouncements {
			not_supported: t("No tables."),
			not_found_next: t("No next table."),
			not_found_prev: t("No previous table."),
			format: NavFoundFormat::TextOnly,
		},
	}
}

fn format_nav_found_message(
	ann: &NavAnnouncements,
	context_text: &str,
	context_index: i32,
	wrapped: bool,
	next: bool,
) -> String {
	let wrap_prefix =
		if wrapped { if next { t("Wrapping to start. ") } else { t("Wrapping to end. ") } } else { String::new() };
	match ann.format {
		NavFoundFormat::TextOnly => format!("{wrap_prefix}{context_text}"),
		NavFoundFormat::TextWithLevel => {
			let template = t("%s Heading level %d");
			let message = template.replacen("%s", context_text, 1).replacen("%d", &context_index.to_string(), 1);
			format!("{wrap_prefix}{message}")
		}
		NavFoundFormat::PageFormat => {
			let template = t("Page %d: %s");
			let page_text = (context_index + 1).to_string();
			let message = template.replacen("%d", &page_text, 1).replacen("%s", context_text, 1);
			format!("{wrap_prefix}{message}")
		}
		NavFoundFormat::LinkFormat => {
			let message = format!("{context_text}{}", t(" link"));
			format!("{wrap_prefix}{message}")
		}
	}
}

fn apply_navigation_result(
	tab: &mut DocumentTab,
	result: crate::session::NavigationResult,
	target: MarkerNavTarget,
	next: bool,
	live_region_label: StaticText,
) -> bool {
	let level_filter = match target {
		MarkerNavTarget::Heading(level) => level,
		_ => 0,
	};
	let ann = nav_announcements(target, level_filter);
	if result.not_supported {
		live_region::announce(&live_region_label, &ann.not_supported);
		return false;
	}
	if !result.found {
		let message = if next { &ann.not_found_next } else { &ann.not_found_prev };
		live_region::announce(&live_region_label, message);
		return false;
	}
	let mut context_text = result.marker_text.clone();
	if context_text.is_empty() {
		context_text = tab.session.get_line_text(result.offset);
	}
	let context_index = match target {
		MarkerNavTarget::Heading(_) => result.marker_level,
		MarkerNavTarget::Page => result.marker_index,
		_ => 0,
	};
	let message = format_nav_found_message(&ann, &context_text, context_index, result.wrapped, next);
	live_region::announce(&live_region_label, &message);
	let offset = result.offset;
	tab.text_ctrl.set_focus();
	tab.text_ctrl.set_insertion_point(offset);
	tab.text_ctrl.show_position(offset);
	true
}

fn handle_history_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	forward: bool,
) {
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else {
		return;
	};
	let current_pos = tab.text_ctrl.get_insertion_point();
	let result =
		if forward { tab.session.history_go_forward(current_pos) } else { tab.session.history_go_back(current_pos) };
	if result.found {
		let message = if forward { t("Navigated to next position.") } else { t("Navigated to previous position.") };
		live_region::announce(&live_region_label, &message);
		tab.text_ctrl.set_focus();
		tab.text_ctrl.set_insertion_point(result.offset);
		tab.text_ctrl.show_position(result.offset);
		let (history, history_index) = tab.session.get_history();
		let path_str = tab.file_path.to_string_lossy();
		let cfg = config.lock().unwrap();
		cfg.set_navigation_history(&path_str, history, history_index);
	} else {
		let message = if forward { t("No next position.") } else { t("No previous position.") };
		live_region::announce(&live_region_label, &message);
	}
}

fn handle_marker_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	target: MarkerNavTarget,
	next: bool,
) {
	let wrap = config.lock().unwrap().get_app_bool("navigation_wrap", false);
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else {
		return;
	};
	let current_pos = tab.text_ctrl.get_insertion_point();
	let result = match target {
		MarkerNavTarget::Section => tab.session.navigate_section(current_pos, wrap, next),
		MarkerNavTarget::Page => tab.session.navigate_page(current_pos, wrap, next),
		MarkerNavTarget::Heading(level) => tab.session.navigate_heading(current_pos, wrap, next, level),
		MarkerNavTarget::Link => tab.session.navigate_link(current_pos, wrap, next),
		MarkerNavTarget::Table => tab.session.navigate_table(current_pos, wrap, next),
		MarkerNavTarget::List => tab.session.navigate_list(current_pos, wrap, next),
		MarkerNavTarget::ListItem => tab.session.navigate_list_item(current_pos, wrap, next),
	};
	if result.found && !result.not_supported {
		tab.session.record_position(current_pos);
	}
	if apply_navigation_result(tab, result, target, next, live_region_label) {
		let (history, history_index) = tab.session.get_history();
		let path_str = tab.file_path.to_string_lossy();
		let cfg = config.lock().unwrap();
		cfg.set_navigation_history(&path_str, history, history_index);
	}
}

fn selected_range(text_ctrl: &TextCtrl) -> (i64, i64) {
	let (start, end) = text_ctrl.get_selection();
	if start == end {
		let pos = text_ctrl.get_insertion_point();
		(pos, pos)
	} else if start <= end {
		(start, end)
	} else {
		(end, start)
	}
}

fn handle_bookmark_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	next: bool,
	notes_only: bool,
) {
	let wrap = config.lock().unwrap().get_app_bool("navigation_wrap", false);
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else {
		return;
	};
	let current_pos = tab.text_ctrl.get_insertion_point();
	let path_str = tab.file_path.to_string_lossy().to_string();
	let (result, has_items) = {
		let cfg = config.lock().unwrap();
		let bookmarks = cfg.get_bookmarks(&path_str);
		let has_items = if notes_only { bookmarks.iter().any(|bm| !bm.note.is_empty()) } else { !bookmarks.is_empty() };
		let result = if notes_only {
			tab.session.navigate_note(&cfg, current_pos, wrap, next)
		} else {
			tab.session.navigate_bookmark(&cfg, current_pos, wrap, next)
		};
		(result, has_items)
	};
	if !result.found {
		let message = if !has_items {
			if notes_only { t("No notes.") } else { t("No bookmarks.") }
		} else if next {
			if notes_only { t("No next note.") } else { t("No next bookmark.") }
		} else if notes_only {
			t("No previous note.")
		} else {
			t("No previous bookmark.")
		};
		live_region::announce(&live_region_label, &message);
		return;
	}
	tab.session.record_position(current_pos);
	tab.text_ctrl.set_focus();
	tab.text_ctrl.set_insertion_point(result.offset);
	tab.text_ctrl.show_position(result.offset);
	let mut context_text = result.marker_text;
	if context_text.is_empty() {
		context_text = tab.session.get_line_text(result.offset);
	}
	let wrap_prefix = if result.wrapped {
		if next { t("Wrapping to start. ") } else { t("Wrapping to end. ") }
	} else {
		String::new()
	};
	let message = format!("{wrap_prefix}{context_text}");
	live_region::announce(&live_region_label, &message);
	let (history, history_index) = tab.session.get_history();
	let cfg = config.lock().unwrap();
	cfg.set_navigation_history(&path_str, history, history_index);
}

fn handle_bookmark_dialog(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	filter: BookmarkFilterType,
) {
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else {
		return;
	};
	let current_pos = tab.text_ctrl.get_insertion_point();
	let selection = dialogs::show_bookmark_dialog(frame, &tab.session, Rc::clone(config), current_pos, filter);
	let Some(selection) = selection else {
		return;
	};
	tab.session.record_position(current_pos);
	tab.text_ctrl.set_focus();
	tab.text_ctrl.set_insertion_point(selection.start);
	tab.text_ctrl.show_position(selection.start);
	let message = {
		let cfg = config.lock().unwrap();
		let info = tab.session.bookmark_display_at_position(&cfg, selection.start);
		if info.found {
			let mut text = info.note;
			if text.is_empty() {
				text = info.snippet;
			}
			if text.is_empty() { t("Bookmark.") } else { text }
		} else {
			t("Bookmark.")
		}
	};
	live_region::announce(&live_region_label, &message);
	let (history, history_index) = tab.session.get_history();
	let path_str = tab.file_path.to_string_lossy();
	let cfg = config.lock().unwrap();
	cfg.set_navigation_history(&path_str, history, history_index);
}

fn handle_toggle_bookmark(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else {
		return;
	};
	let (start, end) = selected_range(&tab.text_ctrl);
	let path_str = tab.file_path.to_string_lossy().to_string();
	let cfg = config.lock().unwrap();
	let existed = cfg.get_bookmarks(&path_str).iter().any(|bm| bm.start == start && bm.end == end);
	cfg.toggle_bookmark(&path_str, start, end, "");
	cfg.flush();
	let message = if existed { t("Bookmark removed.") } else { t("Bookmark added.") };
	live_region::announce(&live_region_label, &message);
}

fn handle_bookmark_with_note(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else {
		return;
	};
	let (start, end) = selected_range(&tab.text_ctrl);
	let path_str = tab.file_path.to_string_lossy().to_string();
	let existing = {
		let cfg = config.lock().unwrap();
		cfg.get_bookmarks(&path_str).into_iter().find(|bm| bm.start == start && bm.end == end)
	};
	let existing_note = existing.as_ref().map(|bm| bm.note.clone()).unwrap_or_default();
	let Some(note) =
		dialogs::show_note_entry_dialog(frame, &t("Bookmark Note"), &t("Enter bookmark note:"), &existing_note)
	else {
		return;
	};
	let cfg = config.lock().unwrap();
	if existing.is_some() {
		cfg.update_bookmark_note(&path_str, start, end, &note);
	} else {
		cfg.add_bookmark(&path_str, start, end, &note);
	}
	cfg.flush();
	live_region::announce(&live_region_label, &t("Bookmark saved."));
}

fn handle_view_note_text(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>, config: &Rc<Mutex<ConfigManager>>) {
	let dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab() else {
		return;
	};
	let current_pos = tab.text_ctrl.get_insertion_point();
	let path_str = tab.file_path.to_string_lossy();
	let note = {
		let cfg = config.lock().unwrap();
		crate::reader_core::bookmark_note_at_position(&cfg, &path_str, current_pos)
	};
	if note.is_empty() {
		let dialog = MessageDialog::builder(frame, &t("No note at the current position."), &t("View Note"))
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
			.build();
		dialog.show_modal();
		return;
	}
	dialogs::show_view_note_dialog(frame, &note);
}

fn run_update_check(silent: bool) {
	let current_version = env!("CARGO_PKG_VERSION").to_string();
	let is_installer = is_installer_distribution();
	thread::spawn(move || {
		let outcome = update::check_for_updates(&current_version, is_installer);
		wxdragon::call_after(Box::new(move || {
			present_update_result(outcome, silent, &current_version);
		}));
	});
}

fn is_installer_distribution() -> bool {
	let Ok(exe_path) = std::env::current_exe() else {
		return false;
	};
	let Some(exe_dir) = exe_path.parent() else {
		return false;
	};
	exe_dir.join("unins000.exe").exists()
}

fn present_update_result(outcome: Result<UpdateCheckOutcome, UpdateError>, silent: bool, current_version: &str) {
	let parent_window = main_window_parent();
	match outcome {
		Ok(UpdateCheckOutcome::UpdateAvailable(result)) => {
			let latest_version =
				if result.latest_version.is_empty() { current_version.to_string() } else { result.latest_version };
			let plain_notes = markdown_to_text(&result.release_notes);
			let release_notes =
				if plain_notes.trim().is_empty() { t("No release notes were provided.") } else { plain_notes };
			if let Some(parent) = parent_window.as_ref() {
				if dialogs::show_update_dialog(parent, &latest_version, &release_notes)
					&& !result.download_url.is_empty()
				{
					wxdragon::utils::launch_default_browser(
						&result.download_url,
						wxdragon::utils::BrowserLaunchFlags::Default,
					);
				}
			}
		}
		Ok(UpdateCheckOutcome::UpToDate(_)) => {
			if silent {
				return;
			}
			let message = t("No updates available.");
			let title = t("Info");
			if let Some(parent) = parent_window.as_ref() {
				let dialog = MessageDialog::builder(parent, &message, &title)
					.with_style(
						MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre,
					)
					.build();
				dialog.show_modal();
			}
		}
		Err(err) => {
			if silent {
				return;
			}
			let (message, title) = match err {
				UpdateError::HttpError(code) if code > 0 => {
					let template = t("Failed to check for updates. HTTP status: %d");
					(template.replacen("%d", &code.to_string(), 1), t("Error"))
				}
				_ => {
					let msg = err.to_string();
					let fallback = t("Error checking for updates.");
					(if msg.is_empty() { fallback } else { msg }, t("Error"))
				}
			};
			if let Some(parent) = parent_window.as_ref() {
				let dialog = MessageDialog::builder(parent, &message, &title)
					.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
					.build();
				dialog.show_modal();
			}
		}
	}
}

struct ParentWindow {
	handle: *mut ffi::wxd_Window_t,
}

impl wxdragon::window::WxWidget for ParentWindow {
	fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
		self.handle
	}
}

fn main_window_parent() -> Option<ParentWindow> {
	let ptr = MAIN_WINDOW_PTR.load(Ordering::SeqCst);
	if ptr == 0 {
		return None;
	}
	let handle = ptr as *mut ffi::wxd_Window_t;
	if handle.is_null() {
		return None;
	}
	Some(ParentWindow { handle })
}

fn update_title_from_manager(frame: &Frame, dm: &DocumentManager) {
	let sleep_start = SLEEP_TIMER_START_MS.load(Ordering::SeqCst);
	let sleep_duration = SLEEP_TIMER_DURATION_MINUTES.load(Ordering::SeqCst);
	if dm.tab_count() == 0 {
		frame.set_title(&t("Paperback"));
		let mut status_text = t("Ready");
		if sleep_start > 0 {
			let remaining = calculate_sleep_timer_remaining(sleep_start, sleep_duration);
			if remaining > 0 {
				status_text = format_sleep_timer_status(&status_text, remaining);
			}
		}
		frame.set_status_text(&status_text, 0);
		return;
	}
	if let Some(tab) = dm.active_tab() {
		let title = tab.session.title();
		let display_title = if title.is_empty() {
			tab.file_path.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| t("Untitled"))
		} else {
			title
		};
		let template = t("Paperback - {}");
		frame.set_title(&template.replace("{}", &display_title));
		let position = tab.text_ctrl.get_insertion_point();
		let status_info = tab.session.get_status_info(position);
		let mut status_text = format_status_text(&status_info);
		if sleep_start > 0 {
			let remaining = calculate_sleep_timer_remaining(sleep_start, sleep_duration);
			if remaining > 0 {
				status_text = format_sleep_timer_status(&status_text, remaining);
			}
		}
		frame.set_status_text(&status_text, 0);
	}
}

fn format_status_text(info: &crate::session::StatusInfo) -> String {
	let line_label = t("Line");
	let char_label = t("Character");
	let reading_label = t("Reading");
	format!(
		"{} {}, {} {}, {} {}%",
		line_label, info.line_number, char_label, info.character_number, reading_label, info.percentage
	)
}

fn update_status_bar_with_sleep_timer(
	frame: &Frame,
	dm: &DocumentManager,
	sleep_timer_start_ms: i64,
	sleep_timer_duration_minutes: i32,
) {
	if dm.tab_count() == 0 {
		if sleep_timer_start_ms > 0 {
			let remaining = calculate_sleep_timer_remaining(sleep_timer_start_ms, sleep_timer_duration_minutes);
			if remaining > 0 {
				let status_text = format_sleep_timer_status(&t("Ready"), remaining);
				frame.set_status_text(&status_text, 0);
				return;
			}
		}
		frame.set_status_text(&t("Ready"), 0);
		return;
	}
	if let Some(tab) = dm.active_tab() {
		let position = tab.text_ctrl.get_insertion_point();
		let status_info = tab.session.get_status_info(position);
		let mut status_text = format_status_text(&status_info);
		if sleep_timer_start_ms > 0 {
			let remaining = calculate_sleep_timer_remaining(sleep_timer_start_ms, sleep_timer_duration_minutes);
			if remaining > 0 {
				status_text = format_sleep_timer_status(&status_text, remaining);
			}
		}
		frame.set_status_text(&status_text, 0);
	}
}

fn calculate_sleep_timer_remaining(start_ms: i64, duration_minutes: i32) -> i32 {
	let now = SystemTime::now().duration_since(time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0);
	let elapsed_ms = now - start_ms;
	let duration_ms = i64::from(duration_minutes) * 60 * 1000;
	let remaining_ms = duration_ms - elapsed_ms;
	if remaining_ms < 0 { 0 } else { (remaining_ms / 1000) as i32 }
}

fn format_sleep_timer_status(base_status: &str, remaining_seconds: i32) -> String {
	let minutes = remaining_seconds / 60;
	let seconds = remaining_seconds % 60;
	let sleep_label = t("Sleep timer");
	format!("{} | {}: {:02}:{:02}", base_status, sleep_label, minutes, seconds)
}

struct TrayState {
	_icon: TaskBarIcon,
	_menu: Menu,
}

fn create_tray_state(
	frame: Frame,
	doc_manager: Rc<Mutex<DocumentManager>>,
	tray_state: Rc<Mutex<Option<TrayState>>>,
) -> Option<TrayState> {
	let restore_label = t("&Restore");
	let restore_help = t("Restore Paperback");
	let exit_label = t("E&xit");
	let exit_help = t("Exit Paperback");
	let mut menu = Menu::builder()
		.append_item(menu_ids::RESTORE, &restore_label, &restore_help)
		.append_separator()
		.append_item(menu_ids::EXIT, &exit_label, &exit_help)
		.build();

	let icon = TaskBarIcon::builder().build();
	if let Some(bundle) =
		ArtProvider::get_bitmap_bundle(ArtId::Information, ArtClient::MessageBox, Some(Size::new(32, 32)))
	{
		icon.set_icon_bundle(&bundle, "Paperback");
	} else if let Some(bitmap) =
		ArtProvider::get_bitmap(ArtId::Information, ArtClient::MessageBox, Some(Size::new(32, 32)))
	{
		icon.set_icon(&bitmap, "Paperback");
	}
	icon.set_popup_menu(&mut menu);

	let frame_for_menu = frame;
	let doc_manager_for_menu = Rc::clone(&doc_manager);
	let tray_state_for_menu = Rc::clone(&tray_state);
	icon.on_menu(move |event| match event.get_id() {
		menu_ids::RESTORE => restore_from_tray(&frame_for_menu, &doc_manager_for_menu, &tray_state_for_menu),
		menu_ids::EXIT => frame_for_menu.close(true),
		_ => {}
	});

	let frame_for_click = frame;
	let doc_manager_for_click = Rc::clone(&doc_manager);
	let tray_state_for_click = Rc::clone(&tray_state);
	#[cfg(any(target_os = "windows", target_os = "linux"))]
	{
		icon.on_left_up(move |_event| {
			restore_from_tray(&frame_for_click, &doc_manager_for_click, &tray_state_for_click);
		});
	}

	let frame_for_dclick = frame;
	let doc_manager_for_dclick = Rc::clone(&doc_manager);
	let tray_state_for_dclick = Rc::clone(&tray_state);
	#[cfg(any(target_os = "windows", target_os = "linux"))]
	{
		icon.on_left_double_click(move |_event| {
			restore_from_tray(&frame_for_dclick, &doc_manager_for_dclick, &tray_state_for_dclick);
		});
	}

	Some(TrayState { _icon: icon, _menu: menu })
}

fn restore_from_tray(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	tray_state: &Rc<Mutex<Option<TrayState>>>,
) {
	frame.iconize(false);
	frame.show(true);
	frame.raise();
	let dm = doc_manager.lock().unwrap();
	dm.restore_focus();
	let mut tray_state_guard = tray_state.lock().unwrap();
	if let Some(state) = tray_state_guard.take() {
		state._icon.remove_icon();
	}
}
