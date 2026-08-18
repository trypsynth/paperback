use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::builder::format_menu_label;
use crate::ui::menu_ids;

pub fn create_tools_menu(config: &ConfigManager) -> Menu {
	let import_label = format_menu_label(&t("&Import Document Data..."), ActionId::ImportDocumentData, config);
	let import_help = t("Import bookmarks and position");
	let export_label = format_menu_label(&t("&Export Document Data..."), ActionId::ExportDocumentData, config);
	let export_help = t("Export bookmarks and position");
	let export_text_label = format_menu_label(&t("Export to &Plain Text..."), ActionId::ExportToPlainText, config);
	let export_text_help = t("Export document as plain text");
	let export_html_label = format_menu_label(&t("Export to &HTML..."), ActionId::ExportToHtml, config);
	let export_html_help = t("Export document as HTML");
	let export_markdown_label = format_menu_label(&t("Export to &Markdown..."), ActionId::ExportToMarkdown, config);
	let export_markdown_help = t("Export document as Markdown");
	let import_export_menu = Menu::builder()
		.append_item(menu_ids::IMPORT_DOCUMENT_DATA, &import_label, &import_help)
		.append_item(menu_ids::EXPORT_DOCUMENT_DATA, &export_label, &export_help)
		.append_separator()
		.append_item(menu_ids::EXPORT_TO_PLAIN_TEXT, &export_text_label, &export_text_help)
		.append_item(menu_ids::EXPORT_TO_HTML, &export_html_label, &export_html_help)
		.append_item(menu_ids::EXPORT_TO_MARKDOWN, &export_markdown_label, &export_markdown_help)
		.build();
	let word_count_label = format_menu_label(&t("&Word Count"), ActionId::WordCount, config);
	let word_count_help = t("Show word count");
	let doc_info_label = format_menu_label(&t("Document &Info"), ActionId::DocumentInfo, config);
	let doc_info_help = t("Show document information");
	let toc_label = format_menu_label(&t("&Table of Contents"), ActionId::TableOfContents, config);
	let toc_help = t("Show table of contents");
	let elements_label = format_menu_label(&t("&Elements List..."), ActionId::ElementsList, config);
	let elements_help = t("Show elements list");
	let open_folder_label = format_menu_label(&t("Reveal &File in Folder"), ActionId::RevealFileInFolder, config);
	let open_folder_help = t("Reveal document in the file manager");
	let web_view_label = format_menu_label(&t("Open in &Web View"), ActionId::OpenInWebView, config);
	let web_view_help = t("Open document in web view");
	let view_source_label = format_menu_label(&t("View &Source"), ActionId::ViewSource, config);
	let view_source_help = t("Open the document source in a new tab");
	let menu = Menu::builder()
		.append_item(menu_ids::WORD_COUNT, &word_count_label, &word_count_help)
		.append_item(menu_ids::DOCUMENT_INFO, &doc_info_label, &doc_info_help)
		.append_separator()
		.append_item(menu_ids::TABLE_OF_CONTENTS, &toc_label, &toc_help)
		.append_item(menu_ids::ELEMENTS_LIST, &elements_label, &elements_help)
		.append_separator()
		.append_item(menu_ids::REVEAL_FILE_IN_FOLDER, &open_folder_label, &open_folder_help)
		.append_item(menu_ids::OPEN_IN_WEB_VIEW, &web_view_label, &web_view_help)
		.append_item(menu_ids::VIEW_SOURCE, &view_source_label, &view_source_help)
		.append_separator()
		.build();
	let import_export_label = t("I&mport/Export");
	let import_export_help = t("Import and export options");
	menu.append_submenu(import_export_menu, &import_export_label, &import_export_help);
	menu.append_separator();
	let toggle_bookmark_label = format_menu_label(&t("Toggle &Bookmark"), ActionId::ToggleBookmark, config);
	let bookmark_note_label = format_menu_label(&t("Bookmark with &Note"), ActionId::BookmarkWithNote, config);
	menu.append(menu_ids::TOGGLE_BOOKMARK, &toggle_bookmark_label, "", ItemKind::Normal);
	menu.append(menu_ids::BOOKMARK_WITH_NOTE, &bookmark_note_label, "", ItemKind::Normal);
	menu.append_separator();
	let word_wrap_label = format_menu_label(&t("Word w&rap"), ActionId::ToggleWordWrap, config);
	let word_wrap_help = t("Toggle word wrap");
	menu.append(menu_ids::TOGGLE_WORD_WRAP, &word_wrap_label, &word_wrap_help, ItemKind::Check);
	menu.check_item(menu_ids::TOGGLE_WORD_WRAP, config.get_app_bool("word_wrap", false));
	let full_screen_label = format_menu_label(&t("&Full Screen"), ActionId::ToggleFullScreen, config);
	let full_screen_help = t("Toggle full screen");
	menu.append(menu_ids::TOGGLE_FULL_SCREEN, &full_screen_label, &full_screen_help, ItemKind::Check);
	menu.append_separator();
	let options_label = format_menu_label(&t("&Options"), ActionId::Options, config);
	let shortcuts_label =
		format_menu_label(&t("Customize &Keyboard Shortcuts..."), ActionId::CustomizeShortcuts, config);
	let sleep_label = format_menu_label(&t("&Sleep Timer..."), ActionId::SleepTimer, config);
	let options_id = if cfg!(target_os = "macos") { menu_ids::PREFERENCES } else { menu_ids::OPTIONS };
	menu.append(options_id, &options_label, "", ItemKind::Normal);
	menu.append(menu_ids::CUSTOMIZE_SHORTCUTS, &shortcuts_label, "", ItemKind::Normal);
	menu.append(menu_ids::SLEEP_TIMER, &sleep_label, "", ItemKind::Normal);
	menu
}
