use std::{
	path::{Path, PathBuf},
	rc::Rc,
	sync::Mutex,
};

use wxdragon::prelude::*;

use crate::{config::ConfigManager, parser::PASSWORD_REQUIRED_ERROR_PREFIX, session::DocumentSession};

/// Data associated with each document tab
pub struct DocumentTab {
	/// The panel containing the text control
	pub panel: Panel,
	/// The text control displaying document content
	pub text_ctrl: TextCtrl,
	/// The document session (parser + navigation state)
	pub session: DocumentSession,
	/// Full path to the document file
	pub file_path: PathBuf,
}

/// Manages document tabs in the notebook
pub struct DocumentManager {
	notebook: Notebook,
	tabs: Vec<DocumentTab>,
	config: Rc<Mutex<ConfigManager>>,
}

impl DocumentManager {
	/// Create a new document manager
	pub fn new(notebook: Notebook, config: Rc<Mutex<ConfigManager>>) -> Self {
		Self { notebook, tabs: Vec::new(), config }
	}

	/// Open a file and create a new tab for it
	/// Returns true if successful
	pub fn open_file(&mut self, path: &Path) -> bool {
		if !path.exists() {
			eprintln!("File not found: {}", path.display());
			return false;
		}
		if let Some(index) = self.find_tab_by_path(path) {
			self.notebook.set_selection(index);
			return true;
		}
		let (password, forced_extension) = {
			let mut config = self.config.lock().unwrap();
			let path_str = path.to_string_lossy();
			config.import_document_settings(&path_str);
			let forced_extension = config.get_document_format(&path_str);
			let password = config.get_document_password(&path_str);
			(password, forced_extension)
		};
		eprintln!("Loading document session: {}", path.display());
		let session = match DocumentSession::new(path.to_string_lossy().as_ref(), &password, &forced_extension) {
			Ok(s) => s,
			Err(e) => {
				if e.starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
					eprintln!("Password required for {}", path.display());
				} else {
					eprintln!("Failed to open document: {e}");
				}
				return false;
			}
		};
		eprintln!("Session loaded: {}", path.display());
		self.add_session_tab(path, session, &password)
	}

	/// Add a tab for an already loaded session
	/// Returns true if successful
	pub fn add_session_tab(&mut self, path: &Path, session: DocumentSession, password: &str) -> bool {
		if let Some(index) = self.find_tab_by_path(path) {
			self.notebook.set_selection(index);
			return true;
		}

		// Get document title for tab
		let title = session.title();
		let title = if title.is_empty() {
			path.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "Untitled".to_string())
		} else {
			title
		};

		// Create panel for the tab
		let panel = Panel::builder(&self.notebook).build();

		// Create text control with multiline, readonly, rich text, word wrap
		let mut config = self.config.lock().unwrap();
		let word_wrap = config.get_app_bool("word_wrap", false);
		let style = TextCtrlStyle::MultiLine
			| TextCtrlStyle::ReadOnly
			| TextCtrlStyle::Rich2
			| if word_wrap { TextCtrlStyle::WordWrap } else { TextCtrlStyle::DontWrap };
		let text_ctrl = TextCtrl::builder(&panel).with_style(style).build();

		// Set up layout
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 0);
		panel.set_sizer(sizer, true);

		// Get document content and set it
		let content = session.content();
		eprintln!("Populating text control ({} chars)", content.chars().count());
		fill_text_ctrl(&text_ctrl, &content);

		// Add the page to the notebook
		self.notebook.add_page(&panel, &title, true, None);

		// Store tab data
		self.tabs.push(DocumentTab { panel, text_ctrl, session, file_path: path.to_path_buf() });

		let path_str = path.to_string_lossy();
		if !password.is_empty() {
			config.set_document_password(&path_str, password);
		}
		let tab_index = self.tabs.len() - 1;
		let max_pos = self.tabs[tab_index].text_ctrl.get_last_position();
		let saved_pos = config.get_validated_document_position(&path_str, max_pos);
		if saved_pos >= 0 {
			self.tabs[tab_index].text_ctrl.set_insertion_point(saved_pos);
			self.tabs[tab_index].text_ctrl.show_position(saved_pos);
		} else {
			self.tabs[tab_index].text_ctrl.set_insertion_point(0);
			self.tabs[tab_index].text_ctrl.show_position(0);
		}
		config.add_recent_document(&path_str);
		config.set_document_opened(&path_str, true);
		config.add_opened_document(&path_str);
		config.flush();

		println!("Opened document: {} ({} chars)", title, content.len());
		true
	}

	/// Close the document at the given index
	pub fn close_document(&mut self, index: usize) -> bool {
		if index >= self.tabs.len() {
			return false;
		}

		// Save position before closing
		if let Some(tab) = self.tabs.get(index) {
			let position = tab.text_ctrl.get_insertion_point();
			let path_str = tab.file_path.to_string_lossy();
			let mut config = self.config.lock().unwrap();
			config.set_document_position(&path_str, position);
			config.set_document_opened(&path_str, false);
			config.remove_opened_document(&path_str);
			config.flush();
			println!("Saving position {} for {}", position, tab.file_path.display());
		}

		// Remove the page from notebook and destroy the window.
		let _page = self.notebook.get_page(index);
		self.notebook.remove_page(index);

		// Remove tab data
		self.tabs.remove(index);
		let count = self.tabs.len();
		if count > 0 {
			let new_index = index.min(count - 1);
			self.notebook.set_selection(new_index);
		}

		true
	}

	/// Close all documents
	pub fn close_all_documents(&mut self) {
		while !self.tabs.is_empty() {
			self.close_document(0);
		}
	}

	/// Get the currently active tab index
	pub fn active_tab_index(&self) -> Option<usize> {
		let selection = self.notebook.selection();
		if selection >= 0 { Some(selection as usize) } else { None }
	}

	/// Get the currently active tab
	pub fn active_tab(&self) -> Option<&DocumentTab> {
		self.active_tab_index().and_then(|i| self.tabs.get(i))
	}

	/// Get the currently active tab mutably
	pub fn active_tab_mut(&mut self) -> Option<&mut DocumentTab> {
		self.active_tab_index().and_then(|i| self.tabs.get_mut(i))
	}

	/// Get a tab by index
	pub fn get_tab(&self, index: usize) -> Option<&DocumentTab> {
		self.tabs.get(index)
	}

	/// Get tab count
	pub fn tab_count(&self) -> usize {
		self.tabs.len()
	}

	/// Find tab index by file path
	pub fn find_tab_by_path(&self, path: &Path) -> Option<usize> {
		self.tabs.iter().position(|tab| tab.file_path == path)
	}

	/// Save the current tab's position
	pub fn save_current_tab_position(&self) {
		if let Some(tab) = self.active_tab() {
			let position = tab.text_ctrl.get_insertion_point();
			let path_str = tab.file_path.to_string_lossy();
			let mut config = self.config.lock().unwrap();
			config.set_document_position(&path_str, position);
			config.flush();
			println!("Position saved: {} for {}", position, tab.file_path.display());
		}
	}

	/// Restore focus to the text control of the active tab
	pub fn restore_focus(&self) {
		if let Some(tab) = self.active_tab() {
			tab.text_ctrl.set_focus();
		} else {
			self.notebook.set_focus();
		}
	}

	/// Get the notebook
	pub fn notebook(&self) -> &Notebook {
		&self.notebook
	}
}

fn fill_text_ctrl(text_ctrl: &TextCtrl, content: &str) {
	text_ctrl.clear();
	const CHUNK_SIZE: usize = 32 * 1024;
	let mut buf = String::new();
	for ch in content.chars() {
		buf.push(ch);
		if buf.len() >= CHUNK_SIZE {
			text_ctrl.append_text(&buf);
			buf.clear();
		}
	}
	if !buf.is_empty() {
		text_ctrl.append_text(&buf);
	}
}
