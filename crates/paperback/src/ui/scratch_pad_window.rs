//! A little window to paste bits of a document into, and nothing more.
//!
//! This is the one thing about the feature that must stay true: the window is a scratch space and
//! not a reader. It has an edit box worth of features and no more - no Find, no navigation, no
//! formatting, no saving. Everything that would make it useful in the reader (the status bar, the
//! accessible search, go-to-line) is deliberately left out, and so is everything that would make it
//! a real editor (a save button, file support). The reader window is what reads; this is what
//! you type into.
//!
//! It is a separate top-level window rather than a tab or a dialog so it can sit next to the reader
//! and be switched to with Alt+Tab like any other window. A dialog with the main frame as parent
//! would be modal at worst and at best a second window with no place of its own in the taskbar.

use std::{rc::Rc, sync::Mutex};

use patois::t;
use wxdragon::prelude::*;

/// The scratch pad window, or `None` once it has been shown and closed for good.
///
/// Only one scratch pad exists, deliberately: this is a notepad for what is in front of you, not a
/// launcher for a new instance per use. The window is hidden rather than destroyed when closed, so
/// its text survives.
pub struct ScratchPadWindowState {
	window: Frame,
	text_ctrl: TextCtrl,
}

impl ScratchPadWindowState {
	/// Creates the window, hidden, with an empty edit box and a Close button.
	///
	/// `owner` is the main reader window: the scratch pad stays above it and is closed with it,
	/// which is what "part of the app" means while still having a window of its own.
	#[must_use]
	pub fn new(owner: &Frame) -> Self {
		// TRANSLATORS: Title of the scratch pad window, a separate window holding text you paste while reading; never saved.
		let window =
			Frame::builder().with_parent(owner).with_title(&t("Scratch Pad")).with_size(Size::new(500, 400)).build();
		let text_ctrl = TextCtrl::builder(&window)
			// ProcessTab makes the control claim Tab for itself rather than let it move focus to
			// the Close button - which is what the reader's own text control, and any text editor,
			// do. This is also the only thing that keeps Tab working after the window has been
			// hidden and shown again, at which point wx's own tab-traversal appears to reclaim it.
			.with_style(
				TextCtrlStyle::MultiLine | TextCtrlStyle::Rich2 | TextCtrlStyle::DontWrap | TextCtrlStyle::ProcessTab,
			)
			.with_size(window.from_dip(Size::new(-1, -1)))
			.build();
		// TRANSLATORS: Button in the scratch pad window that hides it. The text stays in the window; nothing is saved.
		let close_btn = Button::builder(&window).with_id(ID_CLOSE).with_label(&t("&Close")).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 0);
		sizer.add(&close_btn, 0, SizerFlag::Expand | SizerFlag::All, 0);
		window.set_sizer(sizer, true);
		// A frame's default close handling destroys the window, which would throw the text away.
		// Vetoing turns the X into the same thing the Close button does: hide, keep the text.
		let for_close = window;
		close_btn.on_click(move |_| {
			for_close.show(false);
		});
		let for_close_frame = window;
		window.on_close(move |event| {
			if let WindowEventData::General(ref close_event) = event
				&& close_event.can_veto()
			{
				close_event.veto();
			}
			for_close_frame.show(false);
		});
		// On Windows, an edit control does not emit its own focus (WM_SETFOCUS, which becomes an
		// MSAA focus change) when the window is re-activated: Alt+Tab back and screen readers keep
		// announcing the frame, not the text. The reader restored focus the same way. SetFocus on a
		// control that already has focus emits nothing at all, so the focus event is fired
		// explicitly as well - the same NotifyWinEvent the reader sends for its text control.
		let text_ctrl_for_activate = text_ctrl;
		window.on_activate(move |event| {
			if let WindowEventData::Activate(activate) = &event
				&& activate.is_active()
			{
				text_ctrl_for_activate.set_focus();
				// EVENT_OBJECT_FOCUS = 0x8005, OBJID_CLIENT = -4, CHILDID_SELF = 0
				#[cfg(target_os = "windows")]
				unsafe {
					windows::Win32::UI::Accessibility::NotifyWinEvent(
						0x8005,
						windows::Win32::Foundation::HWND(text_ctrl_for_activate.get_handle()),
						-4,
						0,
					);
				}
			}
		});
		// Escape hides the window, as the Close button and the frame's X do: hide, keep the text.
		// A top-level window's char hook sees the key whatever child has focus, which is why the
		// frame gets it rather than the edit box - the same choice `bind_tab_cycling` makes.
		let for_escape = window;
		window.bind_internal(EventType::CHAR_HOOK, move |event| {
			if event.get_key_code() != Some(WXK_ESCAPE) {
				return;
			}
			event.skip(false);
			for_escape.show(false);
		});
		Self { window, text_ctrl }
	}

	/// Shows the window and gives the edit box focus, or brings it to the front if it is already
	/// visible. The text it held before is still there: hiding never cleared it.
	pub fn show(&self) {
		self.window.show(true);
		self.window.raise();
		self.text_ctrl.set_focus();
	}
}

/// The single scratch pad window, hidden while not in use.
#[derive(Clone)]
pub struct ScratchPadWindow {
	state: Rc<Mutex<Option<ScratchPadWindowState>>>,
}

impl ScratchPadWindow {
	#[must_use]
	pub fn new() -> Self {
		Self { state: Rc::new(Mutex::new(None)) }
	}

	/// Shows the scratch pad window, building it on first use.
	pub fn show(&self, owner: &Frame) {
		let mut state = self.state.lock().unwrap();
		state.get_or_insert_with(|| ScratchPadWindowState::new(owner)).show();
	}
}
