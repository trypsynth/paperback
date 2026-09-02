use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::builder::format_menu_label;
use crate::ui::{commands, menu_ids};

pub fn create_tools_menu(config: &ConfigManager) -> Menu {
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
	let import_export_menu = Menu::builder()
		.append_item(menu_ids::IMPORT_DOCUMENT_DATA, &import_label, &import_help)
		.append_item(menu_ids::EXPORT_DOCUMENT_DATA, &export_label, &export_help)
		.append_separator()
		.append_item(menu_ids::EXPORT_TO_PLAIN_TEXT, &export_text_label, &export_text_help)
		.append_item(menu_ids::EXPORT_TO_HTML, &export_html_label, &export_html_help)
		.append_item(menu_ids::EXPORT_TO_MARKDOWN, &export_markdown_label, &export_markdown_help)
		.build();
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
	// TRANSLATORS: Label for the Import/Export submenu in the Tools menu.
	let import_export_label = t("I&mport/Export");
	// TRANSLATORS: Status-bar help text for the Tools > Import/Export submenu.
	let import_export_help = t("Import and export options");
	menu.append_submenu(import_export_menu, &import_export_label, &import_export_help);
	menu.append_separator();
	commands::append_item(&menu, ActionId::ToggleBookmark, config);
	commands::append_item(&menu, ActionId::BookmarkWithNote, config);
	menu.append_separator();
	// TRANSLATORS: Checkable menu item in the Tools menu that toggles whether word wrap is enabled.
	let word_wrap_label = format_menu_label(&t("Word w&rap"), ActionId::ToggleWordWrap, config);
	// TRANSLATORS: Status-bar help text for the Word Wrap menu item.
	let word_wrap_help = t("Toggle word wrap");
	menu.append(menu_ids::TOGGLE_WORD_WRAP, &word_wrap_label, &word_wrap_help, ItemKind::Check);
	menu.check_item(menu_ids::TOGGLE_WORD_WRAP, config.get_app_bool("word_wrap", false));
	menu.append_separator();
	for action in [
		ActionId::PlayPauseAudio,
		ActionId::SeekAudioForward,
		ActionId::SeekAudioBackward,
		ActionId::IncreaseAudioSeekAmount,
		ActionId::DecreaseAudioSeekAmount,
	] {
		commands::append_item(&menu, action, config);
	}
	// TRANSLATORS: Checkable menu item in the Tools menu that toggles full screen mode.
	let full_screen_label = format_menu_label(&t("&Full Screen"), ActionId::ToggleFullScreen, config);
	// TRANSLATORS: Status-bar help text for the Full Screen menu item.
	let full_screen_help = t("Toggle full screen");
	menu.append(menu_ids::TOGGLE_FULL_SCREEN, &full_screen_label, &full_screen_help, ItemKind::Check);
	menu.append_separator();
	// TRANSLATORS: Menu item in the Tools menu to open the application's settings dialog.
	let options_label = format_menu_label(&t("&Settings"), ActionId::Options, config);
	// TRANSLATORS: Menu item in the Tools menu to open the dialog for customizing keyboard shortcuts.
	let shortcuts_label =
		format_menu_label(&t("Customize &Keyboard Shortcuts..."), ActionId::CustomizeShortcuts, config);
	// TRANSLATORS: Menu item in the Tools menu to open the sleep timer dialog.
	let sleep_label = format_menu_label(&t("&Sleep Timer..."), ActionId::SleepTimer, config);
	let options_id = if cfg!(target_os = "macos") { menu_ids::PREFERENCES } else { menu_ids::OPTIONS };
	menu.append(options_id, &options_label, "", ItemKind::Normal);
	menu.append(menu_ids::CUSTOMIZE_SHORTCUTS, &shortcuts_label, "", ItemKind::Normal);
	menu.append(menu_ids::SLEEP_TIMER, &sleep_label, "", ItemKind::Normal);
	menu
}

#[cfg(test)]
mod tests {
	use super::*;

	/// `commands::append_item` panics on an action the table does not have, which would take the
	/// menu bar down as it was being built at startup.
	#[test]
	fn every_tools_menu_command_is_in_the_table() {
		let actions = [
			ActionId::ToggleBookmark,
			ActionId::BookmarkWithNote,
			ActionId::PlayPauseAudio,
			ActionId::SeekAudioForward,
			ActionId::SeekAudioBackward,
			ActionId::IncreaseAudioSeekAmount,
			ActionId::DecreaseAudioSeekAmount,
		];
		for action in actions {
			assert!(commands::for_action(action).is_some(), "{action:?} is in the Tools menu but not in COMMANDS");
		}
	}
}
