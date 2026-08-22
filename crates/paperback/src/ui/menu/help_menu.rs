use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::builder::format_menu_label;
use crate::ui::menu_ids;

pub fn create_help_menu(config: &ConfigManager) -> Menu {
	let about_label = format_menu_label(&t("&About Paperback"), ActionId::About, config);
	let about_help = t("About this application");
	let help_browser_label = format_menu_label(&t("View Help in &Browser"), ActionId::ViewHelpBrowser, config);
	let help_browser_help = t("View help in default browser");
	let help_paperback_label = format_menu_label(&t("View Help in &Paperback"), ActionId::ViewHelpPaperback, config);
	let help_paperback_help = t("View help in Paperback");
	let updates_label = format_menu_label(&t("Check for &Updates"), ActionId::CheckForUpdates, config);
	let updates_help = t("Check for updates");
	let donate_label = format_menu_label(&t("&Donate"), ActionId::Donate, config);
	let donate_help = t("Support Paperback development");
	#[cfg_attr(target_os = "macos", allow(unused_mut))]
	let mut builder = Menu::builder()
		.append_item(menu_ids::ABOUT, &about_label, &about_help)
		.append_item(menu_ids::VIEW_HELP_BROWSER, &help_browser_label, &help_browser_help)
		.append_item(menu_ids::VIEW_HELP_PAPERBACK, &help_paperback_label, &help_paperback_help)
		.append_separator();
	#[cfg(not(target_os = "macos"))]
	{
		builder = builder.append_item(menu_ids::CHECK_FOR_UPDATES, &updates_label, &updates_help).append_separator();
	}
	builder.append_item(menu_ids::DONATE, &donate_label, &donate_help).build()
}
