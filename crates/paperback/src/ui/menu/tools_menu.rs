use paperback_core::config::{ActionId, ConfigManager};
use patois::t;

use super::builder::{MenuEntry, format_menu_label, item, item_with_help};
use crate::ui::menu_ids;

pub fn entries(config: &ConfigManager) -> Vec<MenuEntry> {
	// TRANSLATORS: Menu item in the Tools menu to show the document's word count.
	let word_count_label = format_menu_label(&t("&Word Count"), ActionId::WordCount, config);
	// TRANSLATORS: Status-bar help text for the Word Count menu item.
	let word_count_help = t("Show word count");
	// TRANSLATORS: Menu item in the Tools menu to show information about the document.
	let doc_info_label = format_menu_label(&t("Document &Info"), ActionId::DocumentInfo, config);
	// TRANSLATORS: Status-bar help text for the Document Info menu item.
	let doc_info_help = t("Show document information");
	// TRANSLATORS: Menu item in the Tools menu to open the sleep timer dialog.
	let sleep_label = format_menu_label(&t("&Sleep Timer..."), ActionId::SleepTimer, config);
	// TRANSLATORS: Menu item in the Tools menu to open the dialog for customizing keyboard shortcuts.
	let shortcuts_label =
		format_menu_label(&t("Customize &Keyboard Shortcuts..."), ActionId::CustomizeShortcuts, config);
	// TRANSLATORS: Menu item in the Tools menu to open the application's settings dialog.
	let options_label = format_menu_label(&t("&Settings"), ActionId::Options, config);
	let options_id = if cfg!(target_os = "macos") { menu_ids::PREFERENCES } else { menu_ids::OPTIONS };
	let mut entries = vec![
		item_with_help(menu_ids::WORD_COUNT, word_count_label, word_count_help),
		item_with_help(menu_ids::DOCUMENT_INFO, doc_info_label, doc_info_help),
		MenuEntry::Separator,
	];
	// No OCR engine outside Windows and macOS, so there is nothing for the item to do there.
	#[cfg(any(target_os = "windows", target_os = "macos"))]
	{
		// TRANSLATORS: Tools menu item that opens the Batch OCR dialog for image-only PDF pages
		let batch_ocr_label = format_menu_label(&t("&Batch OCR..."), ActionId::BatchOcr, config);
		entries.push(item(menu_ids::BATCH_OCR, batch_ocr_label));
	}
	entries.extend([
		item(menu_ids::SLEEP_TIMER, sleep_label),
		MenuEntry::Separator,
		item(menu_ids::CUSTOMIZE_SHORTCUTS, shortcuts_label),
		item(options_id, options_label),
	]);
	entries
}
