use std::path::{Path, PathBuf};

use paperback::session::DocumentSession;
use wxdragon::prelude::*;

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
}

impl DocumentManager {
	/// Create a new document manager
	pub fn new(notebook: Notebook) -> Self {
		Self { notebook, tabs: Vec::new() }
	}

	/// Open a file and create a new tab for it
	/// Returns true if successful
	pub fn open_file(&mut self, path: &Path) -> bool {
		// Check if file is already open
		if let Some(index) = self.find_tab_by_path(path) {
			// Switch to existing tab
			self.notebook.set_selection(index);
			return true;
		}

		// Try to create a document session
		let session = match DocumentSession::new(path.to_string_lossy().as_ref(), "", "") {
			Ok(s) => s,
			Err(e) => {
				eprintln!("Failed to open document: {e}");
				return false;
			}
		};

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
		let text_ctrl = TextCtrl::builder(&panel)
			.with_style(
				TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2 | TextCtrlStyle::WordWrap,
			)
			.build();

		// Set up layout
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 0);
		panel.set_sizer(sizer, true);

		// Get document content and set it
		let content = session.content();
		text_ctrl.set_value(&content);

		// Add the page to the notebook
		let tab_index = self.tabs.len();
		self.notebook.add_page(&panel, &title, true, None);

		// Store tab data
		self.tabs.push(DocumentTab { panel, text_ctrl, session, file_path: path.to_path_buf() });

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
			// TODO: Save position to config
			println!("Saving position {} for {}", position, tab.file_path.display());
		}

		// Remove the page from notebook
		self.notebook.remove_page(index);

		// Remove tab data
		self.tabs.remove(index);

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
	fn find_tab_by_path(&self, path: &Path) -> Option<usize> {
		self.tabs.iter().position(|tab| tab.file_path == path)
	}

	/// Save the current tab's position
	pub fn save_current_tab_position(&self) {
		if let Some(tab) = self.active_tab() {
			let position = tab.text_ctrl.get_insertion_point();
			// TODO: Save to config via session
			println!("Position saved: {} for {}", position, tab.file_path.display());
		}
	}

	/// Restore focus to the text control of the active tab
	pub fn restore_focus(&self) {
		if let Some(tab) = self.active_tab() {
			tab.text_ctrl.set_focus();
		}
	}

	/// Get the notebook
	pub fn notebook(&self) -> &Notebook {
		&self.notebook
	}
}
