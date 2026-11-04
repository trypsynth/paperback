//! The reading control: Scintilla, through wxStyledTextCtrl.
//!
//! Scintilla counts positions in UTF-8 bytes and lays out only what is visible, so a whole book can be loaded without the relayout cost
//! the native rich-edit control paid. What it does not provide, and Paperback therefore loses, is heading and list paragraph semantics,
//! paragraph spacing, letter spacing and text alignment.

use paperback_core::document::DocPos;
use wxdragon::{
	event::{Event, EventToken, EventType, WindowEventData, WxEvtHandler},
	prelude::*,
};

use super::readability::{self, ReadabilityStyle};
use crate::{text_format::FormatSegment, text_window::CtrlPos};

/// Style numbers are the bold, italic and underline bits: 1, 2 and 4.
const STYLE_COUNT: i32 = 8;

#[derive(Clone, Copy)]
pub struct ReaderCtrl(StyledTextCtrl);

impl ReaderCtrl {
	pub const fn new(ctrl: StyledTextCtrl) -> Self {
		Self(ctrl)
	}

	pub const fn control(self) -> StyledTextCtrl {
		self.0
	}

	pub fn get_handle(self) -> *mut core::ffi::c_void {
		self.0.get_handle()
	}

	/// Sets up encoding, accessibility, wrapping and margins. Scintilla is left-aligned and has no paragraph model, so the other
	/// readability settings are applied only where they exist.
	pub fn configure(self, word_wrap: bool) {
		let ctrl = self.0;
		ctrl.set_read_only(true);
		ctrl.set_wrap_mode(if word_wrap { WrapMode::Word } else { WrapMode::None });
		for margin in 0..5 {
			ctrl.set_margin_width(margin, 0);
		}
		// A readable default, which the configured readability font replaces when there is one.
		if let Some(font) = Font::new_with_details(
			10,
			FontFamily::Default.as_i32(),
			FontStyle::Normal.as_i32(),
			FontWeight::Normal.as_i32(),
			false,
			"",
		) {
			for style in 0..STYLE_COUNT {
				ctrl.style_set_font(style, &font);
			}
		}
		#[cfg(target_os = "windows")]
		{
			const SCI_SETCODEPAGE: u32 = 2037;
			const SC_CP_UTF8: i32 = 65001;
			const SCI_SETACCESSIBILITY: u32 = 2702;
			sci_message(ctrl, SCI_SETCODEPAGE, SC_CP_UTF8, 0);
			sci_message(ctrl, SCI_SETACCESSIBILITY, 1, 0);
			install_dialog_navigation(self);
		}
	}

	pub fn get_insertion_point(self) -> CtrlPos {
		CtrlPos::from_raw(i64::from(self.0.get_current_pos()))
	}

	pub fn set_insertion_point(self, pos: CtrlPos) {
		let pos = to_i32(pos);
		// The caret move raises the event a screen reader follows; setting the anchor to the same place afterwards collapses any
		// selection left from the previous position, without a selection change to cancel the announcement the command makes.
		self.0.set_current_pos(pos);
		self.clear_anchor(pos);
	}

	pub fn show_position(self, pos: CtrlPos) {
		let pos = to_i32(pos);
		self.0.set_current_pos(pos);
		self.clear_anchor(pos);
		self.0.ensure_caret_visible();
	}

	#[cfg(target_os = "windows")]
	fn clear_anchor(self, pos: i32) {
		// SCI_SETANCHOR only moves the selection's other end, so the selection stays empty and no selection event is raised.
		sci_message(self.0, 2026, pos, 0);
	}

	#[cfg(not(target_os = "windows"))]
	fn clear_anchor(self, pos: i32) {
		self.0.set_selection(pos, pos);
	}

	pub fn get_last_position(self) -> CtrlPos {
		CtrlPos::from_raw(i64::from(self.0.get_length()))
	}

	pub fn get_selection(self) -> (CtrlPos, CtrlPos) {
		let (from, to) = self.0.get_selection();
		(CtrlPos::from_raw(i64::from(from)), CtrlPos::from_raw(i64::from(to)))
	}

	pub fn set_selection(self, from: CtrlPos, to: CtrlPos) {
		self.0.set_selection(to_i32(from), to_i32(to));
	}

	pub fn get_string_selection(self) -> String {
		self.0.get_selected_text()
	}

	pub fn select_all(self) {
		self.0.select_all();
	}

	pub fn set_focus(self) {
		self.prepare_realized();
		self.0.set_focus();
	}

	/// The Scintilla window only takes the code page and accessibility opt-in once its native window exists, so send them again
	/// wherever the control is about to be used.
	pub fn prepare_realized(self) {
		#[cfg(target_os = "windows")]
		{
			const SCI_SETCODEPAGE: u32 = 2037;
			const SC_CP_UTF8: i32 = 65001;
			const SCI_SETACCESSIBILITY: u32 = 2702;
			sci_message(self.0, SCI_SETCODEPAGE, SC_CP_UTF8, 0);
			sci_message(self.0, SCI_SETACCESSIBILITY, 1, 0);
			install_dialog_navigation(self);
		}
	}

	pub fn has_focus(self) -> bool {
		self.0.has_focus()
	}

	pub fn is_valid(self) -> bool {
		self.0.is_valid()
	}

	pub fn destroy(self) {
		self.0.destroy();
	}

	pub fn popup_menu(self, menu: &mut Menu, position: Option<Point>) {
		self.0.popup_menu(menu, position);
	}

	pub fn on_char<F>(self, callback: F) -> EventToken
	where
		F: FnMut(WindowEventData) + 'static,
	{
		self.0.on_char(callback)
	}

	pub fn on_key_down<F>(self, callback: F) -> EventToken
	where
		F: FnMut(WindowEventData) + 'static,
	{
		self.0.on_key_down(callback)
	}

	pub fn bind_internal<F>(self, event_type: EventType, callback: F) -> EventToken
	where
		F: FnMut(Event) + 'static,
	{
		self.0.bind_internal(event_type, callback)
	}

	pub fn add_to_sizer(self, sizer: BoxSizer) {
		sizer.add(&self.0, 1, SizerFlag::Expand | SizerFlag::All, 0);
	}

	pub fn set_value(self, text: &str) {
		self.prepare_realized();
		self.writable(|| {
			#[cfg(target_os = "windows")]
			{
				const SCI_SETTEXT: u32 = 2181;
				// `wxStyledTextCtrl::SetText` converts the UTF-8 text to a UTF-16 `wxString` and back again; call Scintilla
				// with the UTF-8 bytes directly instead, adding only the terminator `SCI_SETTEXT` requires.
				let mut terminated = Vec::with_capacity(text.len() + 1);
				terminated.extend_from_slice(text.as_bytes());
				terminated.push(0);
				sci_message(self.0, SCI_SETTEXT, 0, terminated.as_ptr() as isize);
			}
			#[cfg(not(target_os = "windows"))]
			self.0.set_text(text);
		});
	}

	/// Runs a text or style change while the control is editable, then makes it read-only again. Scintilla's read-only mode makes
	/// text changes no-ops, including the text its own `SetText` wants to write, and the read-only state set before the native window
	/// exists can be lost, so it is always restored here.
	fn writable<T>(self, action: impl FnOnce() -> T) -> T {
		if self.0.is_read_only() {
			self.0.set_read_only(false);
		}
		let result = action();
		self.0.set_read_only(true);
		result
	}

	pub fn set_font(self, font: &Font) {
		for style in 0..STYLE_COUNT {
			self.0.style_set_font(style, font);
		}
	}

	pub fn refresh(self) {
		self.0.refresh(true, None);
	}

	pub fn apply_foreground_color(self, color: i32) {
		if let Some(colour) = readability::stored_colour(color) {
			for style in 0..STYLE_COUNT {
				self.0.style_set_foreground(style, colour);
			}
		}
	}

	pub fn apply_background_color(self, color: i32) {
		if let Some(colour) = readability::stored_colour(color) {
			for style in 0..STYLE_COUNT {
				self.0.style_set_background(style, colour);
			}
		}
	}

	/// Extra leading above and below each line, roughly matching the native control's 1.5x and double spacing.
	pub fn apply_line_spacing(self, line_spacing: i32) {
		#[cfg(target_os = "windows")]
		{
			let extra = match line_spacing {
				1 => 3,
				2 => 6,
				_ => 0,
			};
			sci_message(self.0, 2525, extra, 0);
			sci_message(self.0, 2527, extra, 0);
		}
		#[cfg(not(target_os = "windows"))]
		let _ = (self, line_spacing);
	}

	/// Applies the readability settings the control can express. Scintilla has no paragraph spacing, letter spacing or alignment.
	pub fn apply_readability_format(self, style: &ReadabilityStyle) {
		self.apply_line_spacing(style.line_spacing);
	}

	/// Gives each styled span its style number. `place` turns the slice's character offsets into control units.
	pub fn apply_span_styles(self, segments: &[FormatSegment], place: impl Fn(DocPos) -> CtrlPos) {
		self.writable(|| {
			for style in 0..STYLE_COUNT {
				self.0.style_set_bold(style, style & 1 != 0);
				self.0.style_set_italic(style, style & 2 != 0);
				self.0.style_set_underline(style, style & 4 != 0);
			}
			let len = self.0.get_length();
			self.0.start_styling(0);
			self.0.set_styling(len, 0);
			for seg in segments {
				let style = i32::from(seg.bold) | (i32::from(seg.italic) << 1) | (i32::from(seg.underline) << 2);
				if style == 0 {
					continue;
				}
				let start = to_i32(place(seg.start));
				let end = to_i32(place(seg.end));
				if end > start {
					self.0.start_styling(start);
					self.0.set_styling(end - start, style);
				}
			}
		});
	}

	#[cfg(target_os = "windows")]
	pub fn position_to_xy(self, pos: CtrlPos) -> Option<(i64, i64)> {
		let pos = to_i32(pos);
		let line = self.0.line_from_position(pos);
		let start = self.0.position_from_line(line);
		(start >= 0).then(|| (i64::from(pos - start), i64::from(line)))
	}

	#[cfg(target_os = "windows")]
	pub fn xy_to_position(self, _x: i64, y: i64) -> CtrlPos {
		CtrlPos::from_raw(i64::from(self.0.position_from_line(i32::try_from(y).unwrap_or(i32::MAX))))
	}

	#[cfg(target_os = "windows")]
	pub fn get_line_length(self, line: i64) -> i32 {
		self.0.get_line_length(i32::try_from(line).unwrap_or(i32::MAX))
	}
}

fn to_i32(pos: CtrlPos) -> i32 {
	i32::try_from(pos.get()).unwrap_or(i32::MAX)
}

#[cfg(target_os = "windows")]
fn sci_message(ctrl: StyledTextCtrl, message: u32, wparam: i32, lparam: isize) {
	#[link(name = "user32")]
	unsafe extern "system" {
		fn SendMessageW(hwnd: *mut core::ffi::c_void, msg: u32, wparam: usize, lparam: isize) -> isize;
	}
	unsafe {
		SendMessageW(ctrl.get_handle(), message, usize::try_from(wparam).unwrap_or(0), lparam);
	}
}

/// The system default beep a native text control plays when the caret cannot move, so `MB_OK` keeps the user's sound scheme.
#[cfg(target_os = "windows")]
fn beep_at_document_edge() {
	#[link(name = "user32")]
	unsafe extern "system" {
		fn MessageBeep(utype: u32) -> i32;
	}
	unsafe {
		MessageBeep(0);
	}
}

#[cfg(target_os = "windows")]
std::thread_local! {
	/// Whether a movement key that cannot move the caret plays the system beep. Off unless the reader enables it in Settings.
	static EDGE_BEEP_ENABLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Enables or disables the edge-of-document beep for every reader control on this thread.
pub fn set_edge_beep(enabled: bool) {
	#[cfg(target_os = "windows")]
	EDGE_BEEP_ENABLED.set(enabled);
	#[cfg(not(target_os = "windows"))]
	{
		let _ = enabled;
	}
}

/// Scintilla answers `WM_GETDLGCODE` with `DLGC_WANTALLKEYS`, which tells wx's dialog-key handling to hand it every key: Tab stops
/// traversing the window. Arrows and characters are kept, since Scintilla needs both.
///
/// Scintilla also swallows a bare Alt press, which is the system chord that activates the menu bar. The flag is tracked across the
/// key to raise the menu bar when Alt is released on its own, exactly as Windows does, while a chord like Alt+F4 or Alt+F still acts
/// normally and no auto-repeat can fire it more than once.
#[cfg(target_os = "windows")]
fn install_dialog_navigation(ctrl: ReaderCtrl) {
	use std::cell::Cell;

	use windows::Win32::{
		Foundation::{HWND, LPARAM, LRESULT, WPARAM},
		UI::{
			Input::KeyboardAndMouse::GetKeyState,
			Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
			WindowsAndMessaging::{DefWindowProcW, GA_ROOT, GetAncestor, PostMessageW, SendMessageW},
		},
	};

	const SUBCLASS_ID: usize = 0x7062_5f73;
	const WM_GETDLGCODE: u32 = 0x0087;
	const WM_NCDESTROY: u32 = 0x0082;
	const WM_KILLFOCUS: u32 = 0x0008;
	const WM_SETTEXT: u32 = 0x000c;
	const WM_SYSKEYDOWN: u32 = 0x0104;
	const WM_SYSKEYUP: u32 = 0x0105;
	const WM_KEYDOWN: u32 = 0x0100;
	const WM_SYSCOMMAND: u32 = 0x0112;
	const SC_KEYMENU: usize = 0xf100;
	const SC_CLOSE: usize = 0xf060;
	const VK_MENU: usize = 0x12;
	const VK_PRIOR: usize = 0x21;
	const VK_NEXT: usize = 0x22;
	const VK_LEFT: usize = 0x25;
	const VK_UP: usize = 0x26;
	const VK_RIGHT: usize = 0x27;
	const VK_DOWN: usize = 0x28;
	const VK_F4: usize = 0x73;
	const SCI_GETCURRENTPOS: u32 = 2008;
	const SCI_GETANCHOR: u32 = 2009;
	const DLGC_WANTARROWS: isize = 0x0001;
	const DLGC_WANTTAB: isize = 0x0002;
	const DLGC_WANTALLKEYS: isize = 0x0004;
	const DLGC_WANTCHARS: isize = 0x0080;

	const fn is_caret_move(key: usize) -> bool {
		matches!(key, VK_LEFT | VK_UP | VK_RIGHT | VK_DOWN | VK_PRIOR | VK_NEXT)
	}

	fn alt_is_up() -> bool {
		unsafe { GetKeyState(i32::try_from(VK_MENU).unwrap_or(0)) >= 0 }
	}

	unsafe fn sci_position(hwnd: HWND, message: u32) -> isize {
		unsafe { SendMessageW(hwnd, message, None, None).0 }
	}

	std::thread_local! {
		/// Still a bare Alt press: set on Alt down, cleared by any other key before the release.
		static BARE_ALT: Cell<bool> = const { Cell::new(false) };
	}

	unsafe extern "system" fn subclass_proc(
		hwnd: HWND,
		msg: u32,
		wparam: WPARAM,
		lparam: LPARAM,
		_subclass_id: usize,
		_ref_data: usize,
	) -> LRESULT {
		unsafe {
			if msg == WM_GETDLGCODE {
				let answer = DefSubclassProc(hwnd, msg, wparam, lparam).0;
				return LRESULT((answer & !(DLGC_WANTALLKEYS | DLGC_WANTTAB)) | DLGC_WANTARROWS | DLGC_WANTCHARS);
			}
			if msg == WM_KEYDOWN && EDGE_BEEP_ENABLED.get() && is_caret_move(wparam.0) && alt_is_up() {
				// A native text control beeps when a movement key cannot move the caret; Scintilla does not. Let the key run and
				// beep only if neither end of the selection moved, so collapsing a selection at an edge stays silent. Shift and
				// Ctrl are included: Shift+Left at the document start and Ctrl+Left there cannot move either. Alt is not, since
				// those chords belong to the menu and to navigation shortcuts.
				BARE_ALT.set(false);
				let before = (sci_position(hwnd, SCI_GETCURRENTPOS), sci_position(hwnd, SCI_GETANCHOR));
				let result = DefSubclassProc(hwnd, msg, wparam, lparam);
				let after = (sci_position(hwnd, SCI_GETCURRENTPOS), sci_position(hwnd, SCI_GETANCHOR));
				if before == after {
					beep_at_document_edge();
				}
				return result;
			}
			if msg == WM_SYSKEYDOWN || msg == WM_KEYDOWN {
				// Alt+F4 closes the window, but Scintilla's own default-procedure path loses it. Posted so the window is not
				// destroyed while this message is still being dispatched.
				if msg == WM_SYSKEYDOWN && wparam.0 == VK_F4 && lparam.0 & (1 << 29) != 0 {
					BARE_ALT.set(false);
					let root = GetAncestor(hwnd, GA_ROOT);
					let _ = PostMessageW(Some(root), WM_SYSCOMMAND, WPARAM(SC_CLOSE), LPARAM(0));
					return LRESULT(0);
				}
				// Auto-repeat keeps this set; only a different key turns the press into a chord.
				BARE_ALT.set(wparam.0 == VK_MENU);
			} else if msg == WM_SYSKEYUP && wparam.0 == VK_MENU {
				if BARE_ALT.replace(false) {
					let root = GetAncestor(hwnd, GA_ROOT);
					SendMessageW(root, WM_SYSCOMMAND, Some(WPARAM(SC_KEYMENU)), None);
				}
			} else if msg == WM_KILLFOCUS || msg == WM_NCDESTROY {
				BARE_ALT.set(false);
			}
			if msg == WM_NCDESTROY {
				let _ = RemoveWindowSubclass(hwnd, Some(subclass_proc), SUBCLASS_ID);
			}
			DefSubclassProc(hwnd, msg, wparam, lparam)
		}
	}

	let hwnd = HWND(ctrl.get_handle());
	if !hwnd.0.is_null() {
		// Reinstalling the same procedure only updates the reference data, so calling this from both setup paths is safe.
		unsafe {
			let _ = SetWindowSubclass(hwnd, Some(subclass_proc), SUBCLASS_ID, 0);
		}
		// wx creates the control with its internal name as the window title, so NVDA reads "stcwindow" as the control's
		// accessible name and announces it before the text. `DefWindowProc` replaces the stored title without going through
		// Scintilla, which treats `WM_SETTEXT` as "replace the document".
		unsafe {
			let empty = [0_u16];
			DefWindowProcW(hwnd, WM_SETTEXT, WPARAM(0), LPARAM(empty.as_ptr() as isize));
		}
	}
}
