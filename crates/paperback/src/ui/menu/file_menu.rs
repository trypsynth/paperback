//! Assembles the File menu: the document commands from the command table, the export and
//! document data items, and the Recent Documents submenu, whose items are generated from the
//! recent list rather than being fixed commands.

use paperback_core::config::{ActionId, ConfigManager, document_name};
use patois::t;

use super::builder::{MenuEntry, format_menu_label, item, item_with_help, submenu};
use crate::ui::{commands, menu_ids};

/// Shown only on Windows and Linux; macOS puts Quit in the application menu.
const EXIT_ITEM: ActionId = ActionId::Exit;

/// Last item of the Recent Documents submenu, below the generated entries and Show All.
const CLEAR_RECENT_ITEM: ActionId = ActionId::ClearRecentDocuments;

pub fn entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Label for the Recent Documents submenu in the File menu.
	let recent_label = t("&Recent Documents");
	// TRANSLATORS: Status-bar help text for the File > Recent Documents submenu.
	let recent_help = t("Open a recent document");
	// TRANSLATORS: Label for the Export submenu in the File menu, which lists the formats a document can be exported to.
	let export_label = t("Ex&port");
	// TRANSLATORS: Menu item in the File > Export submenu to export the document as a plain text file.
	let export_text_label = format_menu_label(&t("Export to &Plain Text..."), ActionId::ExportToPlainText, config);
	// TRANSLATORS: Status-bar help text for the Export to Plain Text menu item.
	let export_text_help = t("Export document as plain text");
	// TRANSLATORS: Menu item in the File > Export submenu to export the document as an HTML file.
	let export_html_label = format_menu_label(&t("Export to &HTML..."), ActionId::ExportToHtml, config);
	// TRANSLATORS: Status-bar help text for the Export to HTML menu item.
	let export_html_help = t("Export document as HTML");
	// TRANSLATORS: Menu item in the File > Export submenu to export the document as a Markdown file.
	let export_markdown_label = format_menu_label(&t("Export to &Markdown..."), ActionId::ExportToMarkdown, config);
	// TRANSLATORS: Status-bar help text for the Export to Markdown menu item.
	let export_markdown_help = t("Export document as Markdown");
	// TRANSLATORS: Menu item in the File menu to import bookmarks and reading position from a file.
	let import_label = format_menu_label(&t("&Import Document Data..."), ActionId::ImportDocumentData, config);
	// TRANSLATORS: Status-bar help text for the Import Document Data menu item.
	let import_help = t("Import bookmarks and position");
	// TRANSLATORS: Menu item in the File menu to export bookmarks and reading position to a file.
	let export_data_label = format_menu_label(&t("&Export Document Data..."), ActionId::ExportDocumentData, config);
	// TRANSLATORS: Status-bar help text for the Export Document Data menu item.
	let export_data_help = t("Export bookmarks and position");
	// TRANSLATORS: Menu item in the File menu to reveal the document's file in the system file manager.
	let reveal_label = format_menu_label(&t("Reveal &File in Folder"), ActionId::RevealFileInFolder, config);
	// TRANSLATORS: Status-bar help text for the Reveal File in Folder menu item.
	let reveal_help = t("Reveal document in the file manager");
	let export_formats = vec![
		item_with_help(menu_ids::EXPORT_TO_PLAIN_TEXT, export_text_label, export_text_help),
		item_with_help(menu_ids::EXPORT_TO_HTML, export_html_label, export_html_help),
		item_with_help(menu_ids::EXPORT_TO_MARKDOWN, export_markdown_label, export_markdown_help),
	];
	let mut entries = commands::menu_entries(&[ActionId::Open], config);
	entries.push(submenu(recent_label, recent_help, recent_document_entries(config)));
	entries.push(commands::menu_entry(ActionId::ReopenLastClosed, config));
	entries.extend([
		MenuEntry::Separator,
		submenu(export_label, String::new(), export_formats),
		item_with_help(menu_ids::IMPORT_DOCUMENT_DATA, import_label, import_help),
		item_with_help(menu_ids::EXPORT_DOCUMENT_DATA, export_data_label, export_data_help),
		item_with_help(menu_ids::REVEAL_FILE_IN_FOLDER, reveal_label, reveal_help),
		MenuEntry::Separator,
	]);
	entries.extend(commands::menu_entries(&[ActionId::Close, ActionId::CloseAll], config));
	if !cfg!(target_os = "macos") {
		entries.push(MenuEntry::Separator);
		entries.push(commands::menu_entry(EXIT_ITEM, config));
	}
	entries
}

fn recent_document_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let recent_docs = recent_documents_for_menu(config);
	let mut entries = Vec::new();
	if recent_docs.is_empty() {
		// TRANSLATORS: Placeholder menu item shown in the Recent Documents submenu when there are no recent documents.
		entries.push(MenuEntry::Disabled(t("(No recent documents)")));
	} else {
		for (index, path) in recent_docs.iter().enumerate() {
			if let Ok(offset) = i32::try_from(index) {
				entries.push(item_with_help(
					menu_ids::RECENT_DOCUMENT_BASE + offset,
					recent_label(index, path),
					path.clone(),
				));
			}
		}
	}
	entries.push(MenuEntry::Separator);
	// TRANSLATORS: Menu item at the bottom of the Recent Documents submenu to open the full list of documents.
	let show_all_label = format_menu_label(&t("Show All..."), ActionId::ShowAllRecentDocuments, config);
	entries.push(item(menu_ids::SHOW_ALL_DOCUMENTS, show_all_label));
	entries.push(commands::menu_entry(CLEAR_RECENT_ITEM, config));
	entries
}

/// The Recent Documents item for the `index`th entry, numbered from 1 as its access key.
fn recent_label(index: usize, path: &str) -> String {
	format!("&{} {}", index + 1, document_name(path).unwrap_or_else(|| path.to_string()))
}

pub fn recent_documents_for_menu(config: &ConfigManager) -> Vec<String> {
	let mut docs = config.get_recent_documents();
	docs.truncate(config.recent_documents_limit());
	docs
}

#[cfg(test)]
mod tests {
	use super::recent_label;

	#[test]
	fn a_recent_link_is_named_after_its_file_name() {
		assert_eq!(recent_label(0, "https://example.org/books/Moby%20Dick.epub?from=list"), "&1 Moby Dick.epub");
	}

	#[test]
	fn a_recent_file_is_named_after_its_file_name() {
		assert_eq!(recent_label(1, "books/novel.epub"), "&2 novel.epub");
	}
}
