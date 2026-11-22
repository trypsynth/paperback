use std::{cell::RefCell, rc::Rc, sync::Mutex};

use paperback_core::config::ConfigManager;
use patois::t;
use wx_utils::tray::{Tray, Visibility};
use wxdragon::prelude::*;

use super::{document_manager::DocumentManager, icon};

/// The notification area icon, made the first time Paperback is minimized to the tray, and
/// hidden again whenever the window comes back.
pub type TrayIcon = Rc<RefCell<Option<Tray>>>;

/// Hides Paperback to the tray when it's minimized, while "Minimize to system tray" is on.
pub fn bind_tray_events(
	frame: Frame,
	visibility: &Rc<Visibility>,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	tray: &TrayIcon,
) {
	let config = Rc::clone(config);
	let (restoring, doc_manager, tray) = (Rc::clone(visibility), Rc::clone(doc_manager), Rc::clone(tray));
	visibility.hide_when_minimized(
		// Not waited for: a dialog up the stack holding the config would otherwise hang every idle event.
		move || config.try_lock().is_ok_and(|config| config.get_app_bool("minimize_to_tray", false)),
		move || {
			let mut slot = tray.borrow_mut();
			match slot.as_ref() {
				Some(icon) => icon.show(),
				None => *slot = Some(create_tray(frame, &restoring, &doc_manager, &tray)),
			}
		},
	);
}

fn create_tray(
	frame: Frame,
	visibility: &Rc<Visibility>,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	tray: &TrayIcon,
) -> Tray {
	let restore: Rc<dyn Fn()> = {
		let (visibility, doc_manager, tray) = (Rc::clone(visibility), Rc::clone(doc_manager), Rc::clone(tray));
		Rc::new(move || restore_from_tray(&visibility, &doc_manager, &tray))
	};
	let clicked = Rc::clone(&restore);
	let mut builder = Tray::builder("Paperback")
		.show_hide_label(
			// TRANSLATORS: System tray menu item to restore the main window from the tray
			&t("&Restore"),
			// TRANSLATORS: Status bar help text for the tray menu's "Restore" item
			&t("Restore Paperback"),
		)
		.on_click(move || clicked())
		.on_show_hide(move || restore());
	if let Some(bundle) = icon::notification_area_bundle() {
		builder = builder.icon_bundle(bundle);
	}
	let icon = builder.build(&frame);
	icon.on_exit(move || {
		frame.close(true);
	});
	icon
}

/// Brings Paperback back from the tray, onto the dialog the user was in if there was one, and
/// takes the icon away until it's minimized there again.
pub fn restore_from_tray(visibility: &Visibility, doc_manager: &Rc<Mutex<DocumentManager>>, tray: &TrayIcon) {
	visibility.come_forward();
	if !visibility.has_dialog() {
		doc_manager.lock().unwrap().restore_focus();
	}
	if let Some(icon) = tray.borrow().as_ref() {
		icon.hide();
	}
}
