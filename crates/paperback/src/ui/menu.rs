use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::prelude::*;

mod builder;
#[cfg(target_os = "macos")]
mod edit_menu;
mod file_menu;
mod go_menu;
mod help_menu;
mod state;
mod tools_menu;

pub use file_menu::recent_documents_for_menu;
pub use state::{update_menu_item_states, update_reopen_state};

pub fn create_menu_bar(config: &ConfigManager) -> MenuBar {
	let file_menu = file_menu::create_file_menu(config);
	let compact_go_menu = config.get_app_bool("compact_go_menu", true);
	let go_menu = go_menu::create_go_menu(config, compact_go_menu);
	let tools_menu = tools_menu::create_tools_menu(config);
	let help_menu = help_menu::create_help_menu(config);
	// TRANSLATORS: Top-level "File" menu label in the menu bar
	let file_label = t("&File");
	// TRANSLATORS: Top-level "Go" menu label in the menu bar
	let go_label = t("&Go");
	// TRANSLATORS: Top-level "Tools" menu label in the menu bar
	let tools_label = t("&Tools");
	// TRANSLATORS: Top-level "Help" menu label in the menu bar
	let help_label = t("&Help");
	#[allow(unused_mut)]
	let mut builder = MenuBar::builder().append(file_menu, &file_label);
	#[cfg(target_os = "macos")]
	{
		// TRANSLATORS: Top-level "Edit" menu label in the menu bar (macOS only)
		let edit_label = t("&Edit");
		builder = builder.append(edit_menu::create_edit_menu(config), &edit_label);
	}
	builder.append(go_menu, &go_label).append(tools_menu, &tools_label).append(help_menu, &help_label).build()
}
