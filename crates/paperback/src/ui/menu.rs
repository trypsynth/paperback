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
	let file_label = t("&File");
	let go_label = t("&Go");
	let tools_label = t("&Tools");
	let help_label = t("&Help");
	#[allow(unused_mut)]
	let mut builder = MenuBar::builder().append(file_menu, &file_label);
	#[cfg(target_os = "macos")]
	{
		let edit_label = t("&Edit");
		builder = builder.append(edit_menu::create_edit_menu(config), &edit_label);
	}
	builder.append(go_menu, &go_label).append(tools_menu, &tools_label).append(help_menu, &help_label).build()
}
