//! Commands from a second instance of Paperback, which hands its arguments to this one over IPC
//! rather than opening a second window.

#[cfg(target_os = "windows")]
use std::sync::atomic::{AtomicIsize, Ordering};

use patois::t;
use wxdragon::prelude::*;

#[cfg(target_os = "windows")]
use super::tray;
use super::{MainWindow, dialogs};
use crate::ipc::IpcCommand;

#[cfg(target_os = "windows")]
static HIDDEN_POPUP: AtomicIsize = AtomicIsize::new(0);

impl MainWindow {
	pub fn handle_ipc_command(&self, command: IpcCommand) {
		tracing::info!(command = ?command, "received IPC command");
		let mut web_view_dialog = None;
		dialogs::ACTIVE_WEB_VIEW.with(|v| {
			web_view_dialog = v.get();
		});
		if let Some(parent_dialog) = web_view_dialog {
			let dialog = MessageDialog::builder(
				&parent_dialog,
				// TRANSLATORS: Message shown when the user tries to perform an action while a help/documentation Web View window is open
				&t("Paperback cannot perform any actions while Web View is open."),
				// TRANSLATORS: Title of a warning dialog
				&t("Warning"),
			)
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconWarning | MessageDialogStyle::Centre)
			.build();
			dialog.show_modal();
			return;
		}
		match command {
			IpcCommand::Activate => {
				self.activate_from_ipc();
			}
			IpcCommand::ToggleVisibility => {
				self.toggle_visibility();
			}
			IpcCommand::OpenFile(path) => {
				self.activate_from_ipc();
				self.open_file(&path, false);
				self.frame.raise();
				self.doc_manager.lock().unwrap().focus_document_text();
			}
			IpcCommand::OpenLink(link) => {
				self.activate_from_ipc();
				self.frame.raise();
				self.open_links(vec![link]);
			}
		}
	}

	fn toggle_visibility(&self) {
		let is_shown = self.frame.is_shown();
		if is_shown && self.is_window_active() {
			let mut has_popup = false;
			#[cfg(target_os = "windows")]
			{
				use windows::Win32::{
					Foundation::HWND,
					UI::WindowsAndMessaging::{GetLastActivePopup, SW_HIDE, ShowWindow},
				};
				let handle = self.frame.get_handle();
				if !handle.is_null() {
					let frame_hwnd = HWND(handle);
					let active_popup = unsafe { GetLastActivePopup(frame_hwnd) };
					if active_popup != frame_hwnd {
						has_popup = true;
						HIDDEN_POPUP.store(active_popup.0 as isize, Ordering::SeqCst);
						let _ = unsafe { ShowWindow(active_popup, SW_HIDE) };
					}
				}
			}
			if has_popup {
				self.frame.show(false);
			} else {
				self.frame.iconize(true);
			}
		} else {
			self.activate_from_ipc();
		}
	}

	fn activate_from_ipc(&self) {
		self.frame.show(true);
		self.frame.iconize(false);
		self.frame.request_user_attention(UserAttentionFlag::Info);
		self.frame.raise();
		#[allow(unused_mut)]
		let mut has_popup = false;
		#[cfg(target_os = "windows")]
		{
			use windows::Win32::{
				Foundation::HWND,
				UI::WindowsAndMessaging::{GetLastActivePopup, SW_SHOW, SetForegroundWindow, ShowWindow},
			};
			let handle = self.frame.get_handle();
			if !handle.is_null() {
				let frame_hwnd = HWND(handle);
				let hidden = HIDDEN_POPUP.swap(0, Ordering::SeqCst);
				if hidden != 0 {
					let active_popup = HWND(hidden as _);
					let _ = unsafe { ShowWindow(active_popup, SW_SHOW) };
					let _ = unsafe { SetForegroundWindow(active_popup) };
					has_popup = true;
				} else {
					let active_popup = unsafe { GetLastActivePopup(frame_hwnd) };
					has_popup = active_popup != frame_hwnd;
					let _ = unsafe { SetForegroundWindow(active_popup) };
				}
			}
		}
		if !has_popup {
			self.doc_manager.lock().unwrap().restore_focus();
		}
		#[cfg(not(target_os = "linux"))]
		if let Some(state) = self.tray_state.lock().unwrap().as_mut() {
			tray::set_tray_icon(&state.icon);
		}
	}

	fn is_window_active(&self) -> bool {
		#[cfg(target_os = "windows")]
		{
			use windows::Win32::{
				Foundation::HWND,
				UI::WindowsAndMessaging::{GetForegroundWindow, GetLastActivePopup},
			};
			let handle = self.frame.get_handle();
			if handle.is_null() {
				return self.frame.has_focus();
			}
			let frame_hwnd = HWND(handle);
			let foreground = unsafe { GetForegroundWindow() };
			let active_popup = unsafe { GetLastActivePopup(frame_hwnd) };
			foreground == frame_hwnd || foreground == active_popup
		}
		#[cfg(not(target_os = "windows"))]
		{
			self.frame.has_focus()
		}
	}
}
