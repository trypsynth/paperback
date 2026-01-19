use std::{path::Path, rc::Rc, sync::Mutex};

use wxdragon::prelude::*;

use super::{dialogs, document_manager::DocumentManager, menu_ids};
use crate::config::ConfigManager;

const KEY_DELETE: i32 = 127;
const KEY_NUMPAD_DELETE: i32 = 330;

/// Main application window
pub struct MainWindow {
	frame: Frame,
	doc_manager: Rc<Mutex<DocumentManager>>,
	_config: Rc<Mutex<ConfigManager>>,
}

impl MainWindow {
	/// Create a new main window
	pub fn new(config: Rc<Mutex<ConfigManager>>) -> Self {
		let frame = Frame::builder().with_title("Paperback").with_size(Size::new(800, 600)).build();

		// Create status bar
		frame.create_status_bar(1, 0, -1, "statusbar");
		frame.set_status_text("Ready", 0);

		// Create menu bar
		let menu_bar = Self::create_menu_bar(&config.lock().unwrap());
		frame.set_menu_bar(menu_bar);

		// Create main panel and sizer
		let panel = Panel::builder(&frame).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();

		// Create notebook for document tabs
		let notebook = Notebook::builder(&panel).with_style(NotebookStyle::Top).build();

		sizer.add(&notebook, 1, SizerFlag::Expand | SizerFlag::All, 0);
		panel.set_sizer(sizer, true);

		// Create document manager
		let doc_manager = Rc::new(Mutex::new(DocumentManager::new(notebook, Rc::clone(&config))));

		// Bind menu events
		Self::bind_menu_events(&frame, Rc::clone(&doc_manager), Rc::clone(&config));

		// Bind notebook events
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

		Self { frame, doc_manager, _config: config }
	}

	/// Show the main window
	pub fn show(&self) {
		self.frame.show(true);
		self.frame.centre();
	}

	/// Open a file
	pub fn open_file(&self, path: &Path) -> bool {
		let result = self.doc_manager.lock().unwrap().open_file(path);
		if result {
			self.update_title();
			self.update_recent_documents_menu();
			self.doc_manager.lock().unwrap().restore_focus();
		}
		result
	}

	/// Update the title bar based on active document
	fn update_title(&self) {
		let dm = match self.doc_manager.try_lock() {
			Ok(dm) => dm,
			Err(_) => return,
		};
		if dm.tab_count() == 0 {
			self.frame.set_title("Paperback");
			self.frame.set_status_text("Ready", 0);
			return;
		}
		if let Some(tab) = dm.active_tab() {
			let title = tab.session.title();
			let display_title = if title.is_empty() {
				tab.file_path
					.file_name()
					.map(|s| s.to_string_lossy().to_string())
					.unwrap_or_else(|| "Untitled".to_string())
			} else {
				title
			};
			self.frame.set_title(&format!("Paperback - {display_title}"));
			self.frame.set_status_text(&format!("{} chars", tab.session.content().len()), 0);
		}
	}

	/// Create the menu bar with all menus
	fn create_menu_bar(config: &ConfigManager) -> MenuBar {
		let file_menu = Self::create_file_menu(config);
		let go_menu = Self::create_go_menu();
		let tools_menu = Self::create_tools_menu();
		let help_menu = Self::create_help_menu();

		MenuBar::builder()
			.append(file_menu, "&File")
			.append(go_menu, "&Go")
			.append(tools_menu, "&Tools")
			.append(help_menu, "&Help")
			.build()
	}

	/// Create the File menu
	fn create_file_menu(config: &ConfigManager) -> Menu {
		let file_menu = Menu::builder()
			.append_item(menu_ids::OPEN, "&Open...\tCtrl+O", "Open a document")
			.append_item(menu_ids::CLOSE, "&Close\tCtrl+F4", "Close the current document")
			.append_item(menu_ids::CLOSE_ALL, "Close &All\tCtrl+Shift+F4", "Close all documents")
			.append_separator()
			.append_item(menu_ids::EXIT, "E&xit", "Exit the application")
			.build();

		let recent_menu = Menu::builder().build();
		Self::populate_recent_documents_menu(&recent_menu, config);
		let _ = file_menu.append_submenu(recent_menu, "&Recent Documents", "Open a recent document");

		file_menu
	}

	/// Create the Go menu
	fn create_go_menu() -> Menu {
		let headings_menu = Self::create_headings_submenu();
		let bookmarks_menu = Self::create_bookmarks_submenu();

		let menu = Menu::builder()
			.append_item(menu_ids::FIND, "&Find...\tCtrl+F", "Find text in the document")
			.append_item(menu_ids::FIND_NEXT, "Find &Next\tF3", "Find next occurrence")
			.append_item(menu_ids::FIND_PREVIOUS, "Find &Previous\tShift+F3", "Find previous occurrence")
			.append_separator()
			.append_item(menu_ids::GO_TO_LINE, "Go to &line...\tCtrl+G", "Go to a specific line")
			.append_item(
				menu_ids::GO_TO_PERCENT,
				"Go to &percent...\tCtrl+Shift+G",
				"Go to a percentage of the document",
			)
			.append_separator()
			.append_item(menu_ids::GO_BACK, "Go &Back\tAlt+Left", "Go back in history")
			.append_item(menu_ids::GO_FORWARD, "Go &Forward\tAlt+Right", "Go forward in history")
			.append_separator()
			.append_item(menu_ids::PREVIOUS_SECTION, "Previous Section\t[", "Go to previous section")
			.append_item(menu_ids::NEXT_SECTION, "Next Section\t]", "Go to next section")
			.append_separator()
			.build();

		menu.append_submenu(headings_menu, "&Headings", "Navigate by headings");
		menu.append_submenu(bookmarks_menu, "&Bookmarks", "Navigate by bookmarks");

		menu.append_separator();
		menu.append(menu_ids::GO_TO_PAGE, "Go to &Page\tCtrl+P", "", ItemKind::Normal);
		menu.append(menu_ids::PREVIOUS_PAGE, "Previous Pa&ge\tShift+P", "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_PAGE, "Next Pag&e\tP", "", ItemKind::Normal);
		menu.append_separator();
		menu.append(menu_ids::PREVIOUS_LINK, "Previous Lin&k\tShift+K", "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_LINK, "Next Lin&k\tK", "", ItemKind::Normal);
		menu.append_separator();
		menu.append(menu_ids::PREVIOUS_TABLE, "Previous &Table\tShift+T", "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_TABLE, "Next &Table\tT", "", ItemKind::Normal);
		menu.append_separator();
		menu.append(menu_ids::PREVIOUS_LIST, "Previous L&ist\tShift+L", "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_LIST, "Next L&ist\tL", "", ItemKind::Normal);
		menu.append(menu_ids::PREVIOUS_LIST_ITEM, "Previous List &Item\tShift+I", "", ItemKind::Normal);
		menu.append(menu_ids::NEXT_LIST_ITEM, "Next List I&tem\tI", "", ItemKind::Normal);

		menu
	}

	fn create_headings_submenu() -> Menu {
		Menu::builder()
			.append_item(menu_ids::PREVIOUS_HEADING, "&Previous Heading\tShift+H", "Go to previous heading")
			.append_item(menu_ids::NEXT_HEADING, "&Next Heading\tH", "Go to next heading")
			.append_separator()
			.append_item(menu_ids::PREVIOUS_HEADING_1, "Previous Heading &1\tShift+1", "")
			.append_item(menu_ids::NEXT_HEADING_1, "Next Heading 1\t1", "")
			.append_item(menu_ids::PREVIOUS_HEADING_2, "Previous Heading &2\tShift+2", "")
			.append_item(menu_ids::NEXT_HEADING_2, "Next Heading 2\t2", "")
			.append_item(menu_ids::PREVIOUS_HEADING_3, "Previous Heading &3\tShift+3", "")
			.append_item(menu_ids::NEXT_HEADING_3, "Next Heading 3\t3", "")
			.append_item(menu_ids::PREVIOUS_HEADING_4, "Previous Heading &4\tShift+4", "")
			.append_item(menu_ids::NEXT_HEADING_4, "Next Heading 4\t4", "")
			.append_item(menu_ids::PREVIOUS_HEADING_5, "Previous Heading &5\tShift+5", "")
			.append_item(menu_ids::NEXT_HEADING_5, "Next Heading 5\t5", "")
			.append_item(menu_ids::PREVIOUS_HEADING_6, "Previous Heading &6\tShift+6", "")
			.append_item(menu_ids::NEXT_HEADING_6, "Next Heading 6\t6", "")
			.build()
	}

	fn create_bookmarks_submenu() -> Menu {
		Menu::builder()
			.append_item(menu_ids::PREVIOUS_BOOKMARK, "&Previous Bookmark\tShift+B", "Go to previous bookmark")
			.append_item(menu_ids::NEXT_BOOKMARK, "&Next Bookmark\tB", "Go to next bookmark")
			.append_item(menu_ids::PREVIOUS_NOTE, "Previous &Note\tShift+N", "Go to previous note")
			.append_item(menu_ids::NEXT_NOTE, "Next N&ote\tN", "Go to next note")
			.append_separator()
			.append_item(menu_ids::JUMP_TO_ALL_BOOKMARKS, "Jump to &All...\tCtrl+B", "Show all bookmarks and notes")
			.append_item(
				menu_ids::JUMP_TO_BOOKMARKS_ONLY,
				"Jump to &Bookmarks Only...\tCtrl+Alt+B",
				"Show bookmarks only",
			)
			.append_item(menu_ids::JUMP_TO_NOTES_ONLY, "Jump to Notes &Only...\tCtrl+Alt+M", "Show notes only")
			.append_item(menu_ids::VIEW_NOTE_TEXT, "&View Note Text\tCtrl+Shift+W", "View the note at current position")
			.build()
	}

	fn create_tools_menu() -> Menu {
		let import_export_menu = Menu::builder()
			.append_item(
				menu_ids::IMPORT_DOCUMENT_DATA,
				"&Import Document Data...\tCtrl+Shift+I",
				"Import bookmarks and position",
			)
			.append_item(
				menu_ids::EXPORT_DOCUMENT_DATA,
				"&Export Document Data...\tCtrl+Shift+E",
				"Export bookmarks and position",
			)
			.append_item(
				menu_ids::EXPORT_TO_PLAIN_TEXT,
				"Export to &Plain Text...\tCtrl+E",
				"Export document as plain text",
			)
			.build();

		let menu = Menu::builder()
			.append_item(menu_ids::WORD_COUNT, "&Word Count\tCtrl+W", "Show word count")
			.append_item(menu_ids::DOCUMENT_INFO, "Document &Info\tCtrl+I", "Show document information")
			.append_separator()
			.append_item(menu_ids::TABLE_OF_CONTENTS, "&Table of Contents\tCtrl+T", "Show table of contents")
			.append_item(menu_ids::ELEMENTS_LIST, "&Elements List...\tF7", "Show elements list")
			.append_separator()
			.append_item(
				menu_ids::OPEN_CONTAINING_FOLDER,
				"Open &Containing Folder\tCtrl+Shift+C",
				"Open folder containing the document",
			)
			.append_item(menu_ids::OPEN_IN_WEB_VIEW, "Open in &Web View\tCtrl+Shift+V", "Open document in web view")
			.append_separator()
			.build();

		menu.append_submenu(import_export_menu, "I&mport/Export", "Import and export options");

		menu.append_separator();
		menu.append(menu_ids::TOGGLE_BOOKMARK, "Toggle &Bookmark\tCtrl+Shift+B", "", ItemKind::Normal);
		menu.append(menu_ids::BOOKMARK_WITH_NOTE, "Bookmark with &Note\tCtrl+Shift+N", "", ItemKind::Normal);
		menu.append_separator();
		menu.append(menu_ids::OPTIONS, "&Options\tCtrl+,", "", ItemKind::Normal);
		menu.append(menu_ids::SLEEP_TIMER, "&Sleep Timer...\tCtrl+Shift+S", "", ItemKind::Normal);

		menu
	}

	fn create_help_menu() -> Menu {
		Menu::builder()
			.append_item(menu_ids::ABOUT, "&About Paperback\tCtrl+F1", "About this application")
			.append_item(menu_ids::VIEW_HELP_BROWSER, "View Help in &Browser\tF1", "View help in default browser")
			.append_item(menu_ids::VIEW_HELP_PAPERBACK, "View Help in &Paperback\tShift+F1", "View help in Paperback")
			.append_separator()
			.append_item(menu_ids::CHECK_FOR_UPDATES, "Check for &Updates\tCtrl+Shift+U", "Check for updates")
			.append_separator()
			.append_item(menu_ids::DONATE, "&Donate\tCtrl+D", "Support Paperback development")
			.build()
	}

	/// Bind menu event handlers
	fn bind_menu_events(frame: &Frame, doc_manager: Rc<Mutex<DocumentManager>>, config: Rc<Mutex<ConfigManager>>) {
		let frame_copy = *frame;
		let dm = Rc::clone(&doc_manager);
		let config = Rc::clone(&config);

		frame.on_menu(move |event| {
			let id = event.get_id();
			match id {
				menu_ids::OPEN => {
					Self::handle_open(&frame_copy, &dm);
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
				menu_ids::FIND => println!("Find requested"),
				menu_ids::FIND_NEXT => println!("Find next requested"),
				menu_ids::FIND_PREVIOUS => println!("Find previous requested"),
				menu_ids::GO_TO_LINE => println!("Go to line requested"),
				menu_ids::GO_TO_PERCENT => println!("Go to percent requested"),
				menu_ids::GO_BACK => println!("Go back requested"),
				menu_ids::GO_FORWARD => println!("Go forward requested"),

				// Tools
				menu_ids::WORD_COUNT => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					if let Some(tab) = dm_ref.active_tab() {
						let stats = tab.session.stats();
						let msg = format!("The document contains {} words.", stats.word_count);
						let dialog = MessageDialog::builder(&frame_copy, &msg, "Word count")
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
				menu_ids::TABLE_OF_CONTENTS => println!("TOC requested"),
				menu_ids::OPTIONS => println!("Options requested"),

				// Help
				menu_ids::ABOUT => {
					println!("Paperback 0.8.0 - An accessible ebook reader");
					// TODO: Show about dialog
				}
				menu_ids::CHECK_FOR_UPDATES => println!("Check for updates requested"),

				_ => {
					if id >= menu_ids::RECENT_DOCUMENT_BASE && id <= menu_ids::RECENT_DOCUMENT_MAX {
						let doc_index = id - menu_ids::RECENT_DOCUMENT_BASE;
						let recent_docs = {
							let config_guard = config.lock().unwrap();
							Self::recent_documents_for_menu_static(&config_guard)
						};
						if let Some(path) = recent_docs.get(doc_index as usize) {
							let path = Path::new(path);
							if dm.lock().unwrap().open_file(path) {
								let dm_ref = dm.lock().unwrap();
								update_title_from_manager(&frame_copy, &dm_ref);
								dm_ref.restore_focus();
								let menu_bar = Self::create_menu_bar(&config.lock().unwrap());
								frame_copy.set_menu_bar(menu_bar);
							}
						}
					} else if id == menu_ids::SHOW_ALL_DOCUMENTS {
						let open_paths = dm.lock().unwrap().open_paths();
						let config_for_dialog = Rc::clone(&config);
						let selection = dialogs::show_all_documents_dialog(&frame_copy, config_for_dialog, open_paths);
						if let Some(path) = selection {
							let path = Path::new(&path);
							if dm.lock().unwrap().open_file(path) {
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
	fn handle_open(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) {
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

		let dialog = FileDialog::builder(frame)
			.with_message("Open Document")
			.with_wildcard(wildcard)
			.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
			.build();

		if dialog.show_modal() == wxdragon::id::ID_OK {
			if let Some(path) = dialog.get_path() {
				let path = std::path::Path::new(&path);
				if doc_manager.lock().unwrap().open_file(path) {
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

	fn update_recent_documents_menu(&self) {
		let menu_bar = Self::create_menu_bar(&self._config.lock().unwrap());
		self.frame.set_menu_bar(menu_bar);
	}

	fn populate_recent_documents_menu(menu: &Menu, config: &ConfigManager) {
		let recent_docs = Self::recent_documents_for_menu_static(config);
		if recent_docs.is_empty() {
			if let Some(item) =
				menu.append(wxdragon::id::ID_ANY.try_into().unwrap(), "(No recent documents)", "", ItemKind::Normal)
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
		let _ = menu.append(menu_ids::SHOW_ALL_DOCUMENTS, "Show All...\tCtrl+R", "", ItemKind::Normal);
	}

	fn recent_documents_for_menu_static(config: &ConfigManager) -> Vec<String> {
		let limit = config.get_app_int("recent_documents_to_show", 25).max(0) as usize;
		let mut docs = config.get_recent_documents();
		if docs.len() > limit {
			docs.truncate(limit);
		}
		docs
	}
}

fn update_title_from_manager(frame: &Frame, dm: &DocumentManager) {
	if dm.tab_count() == 0 {
		frame.set_title("Paperback");
		frame.set_status_text("Ready", 0);
		return;
	}
	if let Some(tab) = dm.active_tab() {
		let title = tab.session.title();
		let display_title = if title.is_empty() {
			tab.file_path.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "Untitled".to_string())
		} else {
			title
		};
		frame.set_title(&format!("Paperback - {display_title}"));
		frame.set_status_text(&format!("{} chars", tab.session.content().len()), 0);
	}
}
