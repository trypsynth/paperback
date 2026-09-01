//! How a document's text looks in the reading area: the font, the colours, and the spacing and
//! alignment that go with them.
//!
//! Separate from what the control holds and from which document is open. Three other modules
//! reach for this while rebuilding a control's appearance, which is why it sits on its own
//! rather than inside the document manager.

#[cfg(target_os = "windows")]
use std::ptr::addr_of_mut;

use paperback_core::config::{ConfigManager, ReadabilityFont};
use wxdragon::prelude::*;

pub struct ReadabilityStyle {
	pub rf: ReadabilityFont,
	pub line_spacing: i32,
	pub bg_color: i32,
	pub text_alignment: i32,
	pub letter_spacing: i32,
	pub paragraph_spacing: i32,
}

pub fn readability_style(cfg: &ConfigManager) -> ReadabilityStyle {
	ReadabilityStyle {
		rf: cfg.get_readability_font(),
		line_spacing: cfg.get_line_spacing(),
		bg_color: cfg.get_bg_color(),
		text_alignment: cfg.get_text_alignment(),
		letter_spacing: cfg.get_letter_spacing(),
		paragraph_spacing: cfg.get_paragraph_spacing(),
	}
}

pub fn apply_line_spacing_to_ctrl(text_ctrl: TextCtrl, line_spacing: i32) {
	let mut attr = wxdragon::widgets::textctrl::TextAttr::new();
	attr.set_line_spacing(match line_spacing {
		1 => 15,
		2 => 20,
		_ => 10,
	});
	text_ctrl.set_style(0, text_ctrl.get_last_position(), &attr);
}

pub fn build_font_from_readability(rf: &ReadabilityFont) -> Option<Font> {
	if rf.is_default() {
		return None;
	}
	let point_size = if rf.point_size > 0 { rf.point_size } else { 10 };
	let mut font = Font::new_with_details(
		point_size,
		FontFamily::Default.as_i32(),
		rf.style,
		rf.weight,
		rf.underlined,
		&rf.face_name,
	)?;
	if rf.strikethrough {
		font.set_strikethrough(true);
	}
	if rf.encoding != 0 {
		font.set_encoding(rf.encoding);
	}
	Some(font)
}

pub fn apply_foreground_color_to_ctrl(text_ctrl: TextCtrl, color: i32) {
	if color >= 0 {
		let r = ((color >> 16) & 0xFF) as u8;
		let g = ((color >> 8) & 0xFF) as u8;
		let b = (color & 0xFF) as u8;
		text_ctrl.set_foreground_color(Colour::rgb(r, g, b));
	}
}

pub fn apply_bg_color_to_ctrl(text_ctrl: TextCtrl, color: i32) {
	if color >= 0 {
		let r = ((color >> 16) & 0xFF) as u8;
		let g = ((color >> 8) & 0xFF) as u8;
		let b = (color & 0xFF) as u8;
		text_ctrl.set_background_color(Colour::rgb(r, g, b));
	}
}

pub fn apply_text_alignment_to_ctrl(text_ctrl: TextCtrl, alignment: i32) {
	let mut attr = wxdragon::widgets::textctrl::TextAttr::new();
	attr.set_alignment(match alignment {
		1 => 2,
		2 => 3,
		3 => 4,
		_ => 1,
	});
	text_ctrl.set_style(0, text_ctrl.get_last_position(), &attr);
}

#[cfg(target_os = "windows")]
pub fn apply_letter_spacing_to_ctrl(text_ctrl: TextCtrl, spacing: i32) {
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{CFM_SPACING, CHARFORMAT2W},
			WindowsAndMessaging::SendMessageW,
		},
	};
	const EM_GETSEL: u32 = 176;
	const EM_SETSEL: u32 = 177;
	const EM_SETCHARFORMAT: u32 = 1092;
	const SCF_ALL: u32 = 4;
	let hwnd_ptr = text_ctrl.get_handle();
	if hwnd_ptr.is_null() {
		return;
	}
	let hwnd = HWND(hwnd_ptr);
	// spacing_twips: 0=normal, 1=20 twips (~1pt extra), 2=40 twips (~2pt extra)
	let spacing_twips: i16 = match spacing {
		1 => 20,
		2 => 40,
		_ => 0,
	};
	unsafe {
		let mut caret: u32 = 0;
		SendMessageW(hwnd, EM_GETSEL, Some(WPARAM(addr_of_mut!(caret) as usize)), None);
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(-1_isize)));
		let mut cf = CHARFORMAT2W::default();
		cf.Base.cbSize = size_of::<CHARFORMAT2W>() as u32;
		cf.Base.dwMask = CFM_SPACING;
		cf.sSpacing = spacing_twips;
		SendMessageW(hwnd, EM_SETCHARFORMAT, Some(WPARAM(SCF_ALL as usize)), Some(LPARAM(&raw const cf as isize)));
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(caret as usize)), Some(LPARAM(caret as isize)));
	}
}

#[cfg(not(target_os = "windows"))]
pub fn apply_letter_spacing_to_ctrl(_text_ctrl: TextCtrl, _spacing: i32) {}

pub fn apply_paragraph_spacing_to_ctrl(text_ctrl: TextCtrl, spacing: i32) {
	let mut attr = wxdragon::widgets::textctrl::TextAttr::new();
	attr.set_paragraph_spacing_after(match spacing {
		1 => 120,
		2 => 240,
		_ => 0,
	});
	text_ctrl.set_style(0, text_ctrl.get_last_position(), &attr);
}

pub fn apply_readability_format_to_ctrl(
	text_ctrl: TextCtrl,
	line_spacing: i32,
	para_spacing: i32,
	letter_spacing: i32,
	alignment: i32,
) {
	if line_spacing == 0 && para_spacing == 0 && letter_spacing == 0 && alignment == 0 {
		return;
	}
	#[cfg(not(target_os = "windows"))]
	let _ = letter_spacing;
	#[cfg(target_os = "windows")]
	let windows_data = {
		use windows::Win32::{
			Foundation::{HWND, LPARAM, WPARAM},
			UI::WindowsAndMessaging::SendMessageW,
		};
		const EM_GETSEL: u32 = 176;
		const EM_SETSEL: u32 = 177;
		const WM_SETREDRAW: u32 = 11;
		let hwnd_ptr = text_ctrl.get_handle();
		if hwnd_ptr.is_null() {
			None
		} else {
			let hwnd = HWND(hwnd_ptr);
			let mut caret: u32 = 0;
			unsafe {
				SendMessageW(hwnd, EM_GETSEL, Some(WPARAM(addr_of_mut!(caret) as usize)), None);
				SendMessageW(hwnd, WM_SETREDRAW, Some(WPARAM(0)), None);
				SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(-1_isize)));
			}
			Some((hwnd, caret))
		}
	};
	let mut attr = wxdragon::widgets::textctrl::TextAttr::new();
	if line_spacing > 0 {
		attr.set_line_spacing(match line_spacing {
			1 => 15,
			2 => 20,
			_ => 10,
		});
	}
	if para_spacing > 0 {
		attr.set_paragraph_spacing_after(match para_spacing {
			1 => 120,
			2 => 240,
			_ => 0,
		});
	}
	if alignment > 0 {
		attr.set_alignment(match alignment {
			1 => 2,
			2 => 3,
			3 => 4,
			_ => 1,
		});
	}
	text_ctrl.set_style(0, text_ctrl.get_last_position(), &attr);
	#[cfg(target_os = "windows")]
	if let Some((hwnd, caret)) = windows_data {
		unsafe {
			use windows::Win32::{
				Foundation::{LPARAM, RECT, WPARAM},
				Graphics::Gdi::InvalidateRect,
				UI::{
					Controls::RichEdit::{CFM_SPACING, CHARFORMAT2W},
					WindowsAndMessaging::SendMessageW,
				},
			};
			const EM_SETSEL: u32 = 177;
			const EM_SETCHARFORMAT: u32 = 1092;
			const SCF_ALL: u32 = 4;
			const WM_SETREDRAW: u32 = 11;
			if letter_spacing != 0 {
				let spacing_twips: i16 = match letter_spacing {
					1 => 20,
					2 => 40,
					_ => 0,
				};
				let mut cf = CHARFORMAT2W::default();
				cf.Base.cbSize = size_of::<CHARFORMAT2W>() as u32;
				cf.Base.dwMask = CFM_SPACING;
				cf.sSpacing = spacing_twips;
				SendMessageW(
					hwnd,
					EM_SETCHARFORMAT,
					Some(WPARAM(SCF_ALL as usize)),
					Some(LPARAM(&raw const cf as isize)),
				);
			}
			SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(caret as usize)), Some(LPARAM(caret as isize)));
			SendMessageW(hwnd, WM_SETREDRAW, Some(WPARAM(1)), None);
			let _ = InvalidateRect(Some(hwnd), None::<*const RECT>, true);
		}
	}
}
