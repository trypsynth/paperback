use paperback_core::config::{ActionId, ConfigManager};
use patois::t;

use super::builder::{MenuEntry, check, format_menu_label, item, item_with_help, submenu};
use crate::ui::{commands, menu_ids};

pub fn entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in Tools > Import/Export to import bookmarks and reading position from a file.
	let import_label = format_menu_label(&t("&Import Document Data..."), ActionId::ImportDocumentData, config);
	// TRANSLATORS: Status-bar help text for the Import Document Data menu item.
	let import_help = t("Import bookmarks and position");
	// TRANSLATORS: Menu item in Tools > Import/Export to export bookmarks and reading position to a file.
	let export_label = format_menu_label(&t("&Export Document Data..."), ActionId::ExportDocumentData, config);
	// TRANSLATORS: Status-bar help text for the Export Document Data menu item.
	let export_help = t("Export bookmarks and position");
	// TRANSLATORS: Menu item in Tools > Import/Export to export the document as a plain text file.
	let export_text_label = format_menu_label(&t("Export to &Plain Text..."), ActionId::ExportToPlainText, config);
	// TRANSLATORS: Status-bar help text for the Export to Plain Text menu item.
	let export_text_help = t("Export document as plain text");
	// TRANSLATORS: Menu item in Tools > Import/Export to export the document as an HTML file.
	let export_html_label = format_menu_label(&t("Export to &HTML..."), ActionId::ExportToHtml, config);
	// TRANSLATORS: Status-bar help text for the Export to HTML menu item.
	let export_html_help = t("Export document as HTML");
	// TRANSLATORS: Menu item in Tools > Import/Export to export the document as a Markdown file.
	let export_markdown_label = format_menu_label(&t("Export to &Markdown..."), ActionId::ExportToMarkdown, config);
	// TRANSLATORS: Status-bar help text for the Export to Markdown menu item.
	let export_markdown_help = t("Export document as Markdown");
	let import_export = vec![
		item_with_help(menu_ids::IMPORT_DOCUMENT_DATA, import_label, import_help),
		item_with_help(menu_ids::EXPORT_DOCUMENT_DATA, export_label, export_help),
		MenuEntry::Separator,
		item_with_help(menu_ids::EXPORT_TO_PLAIN_TEXT, export_text_label, export_text_help),
		item_with_help(menu_ids::EXPORT_TO_HTML, export_html_label, export_html_help),
		item_with_help(menu_ids::EXPORT_TO_MARKDOWN, export_markdown_label, export_markdown_help),
	];
	// TRANSLATORS: Menu item in the Tools menu to show the document's word count.
	let word_count_label = format_menu_label(&t("&Word Count"), ActionId::WordCount, config);
	// TRANSLATORS: Status-bar help text for the Word Count menu item.
	let word_count_help = t("Show word count");
	// TRANSLATORS: Menu item in the Tools menu to show information about the document.
	let doc_info_label = format_menu_label(&t("Document &Info"), ActionId::DocumentInfo, config);
	// TRANSLATORS: Status-bar help text for the Document Info menu item.
	let doc_info_help = t("Show document information");
	// TRANSLATORS: Menu item in the Tools menu to show the document's table of contents.
	let toc_label = format_menu_label(&t("&Table of Contents"), ActionId::TableOfContents, config);
	// TRANSLATORS: Status-bar help text for the Table of Contents menu item.
	let toc_help = t("Show table of contents");
	// TRANSLATORS: Menu item in the Tools menu to show a list of the document's structural elements.
	let elements_label = format_menu_label(&t("&Elements List..."), ActionId::ElementsList, config);
	// TRANSLATORS: Status-bar help text for the Elements List menu item.
	let elements_help = t("Show elements list");
	// TRANSLATORS: Menu item in the Tools menu to reveal the document's file in the system file manager.
	let open_folder_label = format_menu_label(&t("Reveal &File in Folder"), ActionId::RevealFileInFolder, config);
	// TRANSLATORS: Status-bar help text for the Reveal File in Folder menu item.
	let open_folder_help = t("Reveal document in the file manager");
	// TRANSLATORS: Menu item in the Tools menu to open the document in a web view.
	let web_view_label = format_menu_label(&t("Open in &Web View"), ActionId::OpenInWebView, config);
	// TRANSLATORS: Status-bar help text for the Open in Web View menu item.
	let web_view_help = t("Open document in web view");
	// TRANSLATORS: Menu item in the Tools menu to open the document's underlying source markup in a new tab.
	let view_source_label = format_menu_label(&t("View &Source"), ActionId::ViewSource, config);
	// TRANSLATORS: Status-bar help text for the View Source menu item.
	let view_source_help = t("Open the document source in a new tab");
	// TRANSLATORS: Label for the Import/Export submenu in the Tools menu.
	let import_export_label = t("I&mport/Export");
	// TRANSLATORS: Status-bar help text for the Tools > Import/Export submenu.
	let import_export_help = t("Import and export options");
	let mut entries = vec![
		item_with_help(menu_ids::WORD_COUNT, word_count_label, word_count_help),
		item_with_help(menu_ids::DOCUMENT_INFO, doc_info_label, doc_info_help),
		MenuEntry::Separator,
		item_with_help(menu_ids::TABLE_OF_CONTENTS, toc_label, toc_help),
		item_with_help(menu_ids::ELEMENTS_LIST, elements_label, elements_help),
		MenuEntry::Separator,
		item_with_help(menu_ids::REVEAL_FILE_IN_FOLDER, open_folder_label, open_folder_help),
		item_with_help(menu_ids::OPEN_IN_WEB_VIEW, web_view_label, web_view_help),
		item_with_help(menu_ids::VIEW_SOURCE, view_source_label, view_source_help),
		MenuEntry::Separator,
		submenu(import_export_label, import_export_help, import_export),
		MenuEntry::Separator,
	];
	entries.extend(commands::menu_entries(&[ActionId::ToggleBookmark, ActionId::BookmarkWithNote], config));
	entries.push(MenuEntry::Separator);
	// TRANSLATORS: Label for the Select and copy submenu in the Tools menu.
	let select_copy_label = t("Select and &copy");
	// TRANSLATORS: Status-bar help text for the Tools > Select and copy submenu.
	let select_copy_help = t("Mark a selection and copy from it");
	let select_copy = commands::menu_entries(
		&[ActionId::SetSelectionStart, ActionId::CopyFromSelectionStart, ActionId::JumpToSelectionStart],
		config,
	);
	entries.push(submenu(select_copy_label, select_copy_help, select_copy));
	entries.push(MenuEntry::Separator);
	// TRANSLATORS: Checkable menu item in the Tools menu that toggles whether word wrap is enabled.
	let word_wrap_label = format_menu_label(&t("Word w&rap"), ActionId::ToggleWordWrap, config);
	// TRANSLATORS: Status-bar help text for the Word Wrap menu item.
	let word_wrap_help = t("Toggle word wrap");
	entries.push(check(
		menu_ids::TOGGLE_WORD_WRAP,
		word_wrap_label,
		word_wrap_help,
		config.get_app_bool("word_wrap", false),
	));
	entries.push(MenuEntry::Separator);
	entries.extend(commands::menu_entries(
		&[
			ActionId::PlayPauseAudio,
			ActionId::SeekAudioForward,
			ActionId::SeekAudioBackward,
			ActionId::IncreaseAudioSeekAmount,
			ActionId::DecreaseAudioSeekAmount,
			ActionId::IncreaseAudioSpeed,
			ActionId::DecreaseAudioSpeed,
		],
		config,
	));
	// TRANSLATORS: Checkable menu item in the Tools menu that toggles full screen mode.
	let full_screen_label = format_menu_label(&t("&Full Screen"), ActionId::ToggleFullScreen, config);
	// TRANSLATORS: Status-bar help text for the Full Screen menu item.
	let full_screen_help = t("Toggle full screen");
	entries.push(check(menu_ids::TOGGLE_FULL_SCREEN, full_screen_label, full_screen_help, false));
	entries.push(MenuEntry::Separator);
	// TRANSLATORS: Menu item in the Tools menu to open the application's settings dialog.
	let options_label = format_menu_label(&t("&Settings"), ActionId::Options, config);
	// TRANSLATORS: Menu item in the Tools menu to open the dialog for customizing keyboard shortcuts.
	let shortcuts_label =
		format_menu_label(&t("Customize &Keyboard Shortcuts..."), ActionId::CustomizeShortcuts, config);
	// TRANSLATORS: Menu item in the Tools menu to open the sleep timer dialog.
	let sleep_label = format_menu_label(&t("&Sleep Timer..."), ActionId::SleepTimer, config);
	let options_id = if cfg!(target_os = "macos") { menu_ids::PREFERENCES } else { menu_ids::OPTIONS };
	entries.push(item(options_id, options_label));
	entries.push(item(menu_ids::CUSTOMIZE_SHORTCUTS, shortcuts_label));
	entries.push(item(menu_ids::SLEEP_TIMER, sleep_label));
	// No OCR engine outside Windows and macOS, so there is nothing for the item to do there.
	#[cfg(any(target_os = "windows", target_os = "macos"))]
	{
		entries.push(MenuEntry::Separator);
		// TRANSLATORS: Tools menu item that opens the Batch OCR dialog for image-only PDF pages
		let batch_ocr_label = format_menu_label(&t("&Batch OCR..."), ActionId::BatchOcr, config);
		entries.push(item(menu_ids::BATCH_OCR, batch_ocr_label));
	}
	entries
}
