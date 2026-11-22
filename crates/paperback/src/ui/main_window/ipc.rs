use patois::t;
use wxdragon::prelude::*;

use super::{MainWindow, dialogs};
use crate::ipc::IpcCommand;

impl MainWindow {
	pub fn handle_ipc_command(&self, command: IpcCommand) {
		tracing::info!(command = ?command, "received IPC command");
		let mut web_view_dialog = None;
		dialogs::ACTIVE_WEB_VIEW.with(|v| {
			web_view_dialog = v.get();
		});
		if let Some(parent_dialog) = web_view_dialog {
			wx_utils::show_warning(
				&parent_dialog,
				// TRANSLATORS: Message shown when the user tries to perform an action while a help/documentation Web View window is open
				t("Paperback cannot perform any actions while Web View is open."),
				// TRANSLATORS: Title of a warning dialog
				&t("Warning"),
			);
			return;
		}
		match command {
			IpcCommand::Activate => {
				self.activate_from_ipc();
			}
			#[cfg(target_os = "windows")]
			IpcCommand::ToggleVisibility => {
				self.toggle_visibility();
			}
			IpcCommand::Open(target) => {
				self.activate_from_ipc();
				self.open_target(target, false);
				self.frame.raise();
				self.doc_manager.lock().unwrap().focus_document_text();
			}
		}
	}

	#[cfg(target_os = "windows")]
	fn toggle_visibility(&self) {
		if self.frame.is_shown() && self.visibility.is_active() {
			// With a dialog open the two are hidden together; otherwise the window is minimized,
			// which "Minimize to system tray" can take on to the tray.
			if self.visibility.has_dialog() {
				self.visibility.hide();
			} else {
				self.frame.iconize(true);
			}
		} else {
			self.activate_from_ipc();
		}
	}

	fn activate_from_ipc(&self) {
		self.visibility.come_forward();
		self.frame.request_user_attention(UserAttentionFlag::Info);
		if !self.visibility.has_dialog() {
			self.doc_manager.lock().unwrap().restore_focus();
		}
		// Back from the tray, the icon goes until Paperback is minimized there again.
		#[cfg(target_os = "windows")]
		if let Some(icon) = self.tray.borrow().as_ref() {
			icon.hide();
		}
	}
}
