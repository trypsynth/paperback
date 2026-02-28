use std::{process, rc::Rc, sync::Mutex};

use wxdragon::{prelude::*, translations::translate as t};

use super::{document_manager::DocumentManager, menu_ids};
use crate::config::ConfigManager;

pub struct TrayState {
	pub icon: TaskBarIcon,
	#[allow(dead_code)] // Menu must stay alive for the tray popup
	menu: Menu,
}

pub fn bind_tray_events(
	frame: Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	tray_state: &Rc<Mutex<Option<TrayState>>>,
) {
	// Both on_size and on_idle need the same minimize-to-tray logic to handle different timing scenarios across platforms.
	macro_rules! bind_minimize_handler {
		($method:ident) => {{
			let config = Rc::clone(config);
			let doc_manager = Rc::clone(doc_manager);
			let tray_state = Rc::clone(tray_state);
			frame.$method(move |event| {
				handle_minimize_to_tray(frame, &config, &doc_manager, &tray_state);
				event.skip(true);
			});
		}};
	}
	bind_minimize_handler!(on_size);
	bind_minimize_handler!(on_idle);
}

fn handle_minimize_to_tray(
	frame: Frame,
	config: &Rc<Mutex<ConfigManager>>,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	tray_state: &Rc<Mutex<Option<TrayState>>>,
) {
	if !frame.is_iconized() {
		return;
	}
	if !config.lock().unwrap().get_app_bool("minimize_to_tray", false) {
		return;
	}
	let mut tray_state_guard = tray_state.lock().unwrap();
	if tray_state_guard.is_none() {
		let state = create_tray_state(frame, Rc::clone(doc_manager), Rc::clone(tray_state), Rc::clone(config));
		*tray_state_guard = Some(state);
	} else {
		let state = tray_state_guard.as_mut().unwrap();
		if let Some(bundle) =
			ArtProvider::get_bitmap_bundle(ArtId::Information, ArtClient::MessageBox, Some(Size::new(32, 32)))
		{
			state.icon.set_icon_bundle(&bundle, "Paperback");
		} else if let Some(bitmap) =
			ArtProvider::get_bitmap(ArtId::Information, ArtClient::MessageBox, Some(Size::new(32, 32)))
		{
			state.icon.set_icon(&bitmap, "Paperback");
		}
	}
	drop(tray_state_guard);
	frame.show(false);
}

fn create_tray_state(
	frame: Frame,
	doc_manager: Rc<Mutex<DocumentManager>>,
	tray_state: Rc<Mutex<Option<TrayState>>>,
	config: Rc<Mutex<ConfigManager>>,
) -> TrayState {
	let restore_label = t("&Restore");
	let restore_help = t("Restore Paperback");
	let exit_label = t("E&xit");
	let exit_help = t("Exit Paperback");
	let mut menu = Menu::builder()
		.append_item(menu_ids::RESTORE, &restore_label, &restore_help)
		.append_separator()
		.append_item(menu_ids::EXIT, &exit_label, &exit_help)
		.build();

	let icon = TaskBarIcon::builder().build();
	if let Some(bundle) =
		ArtProvider::get_bitmap_bundle(ArtId::Information, ArtClient::MessageBox, Some(Size::new(32, 32)))
	{
		icon.set_icon_bundle(&bundle, "Paperback");
	} else if let Some(bitmap) =
		ArtProvider::get_bitmap(ArtId::Information, ArtClient::MessageBox, Some(Size::new(32, 32)))
	{
		icon.set_icon(&bitmap, "Paperback");
	}
	icon.set_popup_menu(&mut menu);
	{
		let doc_manager = Rc::clone(&doc_manager);
		let tray_state = Rc::clone(&tray_state);
		let config = Rc::clone(&config);
		icon.on_menu(move |event| match event.get_id() {
			menu_ids::RESTORE => restore_from_tray(frame, &doc_manager, &tray_state),
			menu_ids::EXIT => {
				let dm = doc_manager.lock().unwrap();
				if let Some(tab) = dm.active_tab() {
					let path = tab.file_path.to_string_lossy();
					let cfg = config.lock().unwrap();
					cfg.set_app_string("active_document", &path);
					cfg.flush();
				}
				dm.save_all_positions();
				process::exit(0);
			}
			_ => {}
		});
	}
	// On Windows/Linux, also restore on click and double-click
	#[cfg(any(target_os = "windows", target_os = "linux"))]
	{
		let doc_manager_click = Rc::clone(&doc_manager);
		let tray_state_click = Rc::clone(&tray_state);
		icon.on_left_up(move |_event| {
			restore_from_tray(frame, &doc_manager_click, &tray_state_click);
		});
		icon.on_left_double_click(move |_event| {
			restore_from_tray(frame, &doc_manager, &tray_state);
		});
	}
	TrayState { icon, menu }
}

fn restore_from_tray(
	frame: Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	tray_state: &Rc<Mutex<Option<TrayState>>>,
) {
	if frame.is_iconized() {
		frame.iconize(false);
	}
	frame.show(true);
	frame.raise();
	doc_manager.lock().unwrap().restore_focus();
	let mut state_guard = tray_state.lock().unwrap();
	if let Some(state) = state_guard.as_mut() {
		state.icon.remove_icon();
	}
}
