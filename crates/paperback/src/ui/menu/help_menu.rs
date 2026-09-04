use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::builder::format_menu_label;
use crate::ui::menu_ids;

pub fn create_help_menu(config: &ConfigManager) -> Menu {
	// TRANSLATORS: Menu item in the Help menu to show information about the application.
	let about_label = format_menu_label(&t("&About Paperback"), ActionId::About, config);
	// TRANSLATORS: Status-bar help text for the Help > About Paperback menu item.
	let about_help = t("About this application");
	// TRANSLATORS: Menu item in the Help menu to open the help documentation in the user's web browser.
	let help_browser_label = format_menu_label(&t("View Help in &Browser"), ActionId::ViewHelpBrowser, config);
	// TRANSLATORS: Status-bar help text for the Help > View Help in Browser menu item.
	let help_browser_help = t("View help in default browser");
	// TRANSLATORS: Menu item in the Help menu to open the help documentation inside Paperback itself.
	let help_paperback_label = format_menu_label(&t("View Help in &Paperback"), ActionId::ViewHelpPaperback, config);
	// TRANSLATORS: Status-bar help text for the Help > View Help in Paperback menu item.
	let help_paperback_help = t("View help in Paperback");
	// TRANSLATORS: Menu item in the Help menu to check for application updates.
	let updates_label = format_menu_label(&t("Check for &Updates"), ActionId::CheckForUpdates, config);
	// TRANSLATORS: Status-bar help text for the Help > Check for Updates menu item.
	let updates_help = t("Check for updates");
	// TRANSLATORS: Menu item in the Help menu to open the donation page for the application.
	let donate_label = format_menu_label(&t("&Donate"), ActionId::Donate, config);
	// TRANSLATORS: Status-bar help text for the Help > Donate menu item.
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
