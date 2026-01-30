use std::{rc::Rc, sync::Mutex};

use wxdragon::{prelude::*, translations::translate as t};

use super::{document_manager::DocumentManager, menu_ids};
use crate::config::ConfigManager;

pub(crate) struct TrayState {
	pub(crate) _icon: TaskBarIcon,
	pub(crate) _menu: Menu,
}

pub(crate) fn bind_tray_events(
	frame: Frame,
	doc_manager: Rc<Mutex<DocumentManager>>,
	config: Rc<Mutex<ConfigManager>>,
	tray_state: Rc<Mutex<Option<TrayState>>>,
) {
	let frame_for_size = frame;
	let tray_state_for_size = Rc::clone(&tray_state);
	let config_for_size = Rc::clone(&config);
	let doc_manager_for_size = Rc::clone(&doc_manager);
	frame.on_size(move |_event| {
		if !frame_for_size.is_iconized() {
			return;
		}
		let minimize_to_tray = config_for_size.lock().unwrap().get_app_bool("minimize_to_tray", false);
		if !minimize_to_tray {
			return;
		}
		let mut tray_state_guard = tray_state_for_size.lock().unwrap();
		if tray_state_guard.is_none() {
			if let Some(state) =
				create_tray_state(frame_for_size, Rc::clone(&doc_manager_for_size), Rc::clone(&tray_state_for_size))
			{
				*tray_state_guard = Some(state);
			}
		}
		frame_for_size.show(false);
	});

	let frame_for_idle = frame;
	let tray_state_for_idle = Rc::clone(&tray_state);
	let config_for_idle = Rc::clone(&config);
	let doc_manager_for_idle = Rc::clone(&doc_manager);
	frame.on_idle(move |_event| {
		if !frame_for_idle.is_iconized() {
			return;
		}
		let minimize_to_tray = config_for_idle.lock().unwrap().get_app_bool("minimize_to_tray", false);
		if !minimize_to_tray {
			return;
		}
		let mut tray_state_guard = tray_state_for_idle.lock().unwrap();
		if tray_state_guard.is_none() {
			if let Some(state) =
				create_tray_state(frame_for_idle, Rc::clone(&doc_manager_for_idle), Rc::clone(&tray_state_for_idle))
			{
				*tray_state_guard = Some(state);
			}
		}
		frame_for_idle.show(false);
	});
}

fn create_tray_state(
	frame: Frame,
	doc_manager: Rc<Mutex<DocumentManager>>,
	tray_state: Rc<Mutex<Option<TrayState>>>,
) -> Option<TrayState> {
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

	let frame_for_menu = frame;
	let doc_manager_for_menu = Rc::clone(&doc_manager);
	let tray_state_for_menu = Rc::clone(&tray_state);
	icon.on_menu(move |event| match event.get_id() {
		menu_ids::RESTORE => restore_from_tray(&frame_for_menu, &doc_manager_for_menu, &tray_state_for_menu),
		menu_ids::EXIT => frame_for_menu.close(true),
		_ => {}
	});

	let frame_for_click = frame;
	let doc_manager_for_click = Rc::clone(&doc_manager);
	let tray_state_for_click = Rc::clone(&tray_state);
	#[cfg(any(target_os = "windows", target_os = "linux"))]
	{
		icon.on_left_up(move |_event| {
			restore_from_tray(&frame_for_click, &doc_manager_for_click, &tray_state_for_click);
		});
	}

	let frame_for_dclick = frame;
	let doc_manager_for_dclick = Rc::clone(&doc_manager);
	let tray_state_for_dclick = Rc::clone(&tray_state);
	#[cfg(any(target_os = "windows", target_os = "linux"))]
	{
		icon.on_left_double_click(move |_event| {
			restore_from_tray(&frame_for_dclick, &doc_manager_for_dclick, &tray_state_for_dclick);
		});
	}

	Some(TrayState { _icon: icon, _menu: menu })
}

fn restore_from_tray(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	tray_state: &Rc<Mutex<Option<TrayState>>>,
) {
	if frame.is_iconized() {
		frame.maximize(false);
	}
	frame.show(true);
	frame.raise();
	let dm = doc_manager.lock().unwrap();
	dm.restore_focus();
	let mut tray_state_guard = tray_state.lock().unwrap();
	if let Some(state) = tray_state_guard.take() {
		state._icon.remove_icon();
	}
}
