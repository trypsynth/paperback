//! A screen reader reads exactly what `EM_GETPARAFORMAT` reports, and the formatting must not change anything a sighted reader sees.

// The included modules' own unit tests compile here too, unused.
#[cfg(target_os = "windows")]
#[allow(dead_code, unused_imports)]
#[path = "../src/text_format.rs"]
pub mod text_format;

#[cfg(target_os = "windows")]
#[allow(dead_code, unused_imports)]
#[path = "../src/rtf.rs"]
pub mod rtf;

#[cfg(target_os = "windows")]
#[path = "../src/ui/rtf_stream.rs"]
pub mod rtf_stream;

#[cfg(target_os = "windows")]
#[path = "../src/ui/readability.rs"]
pub mod readability;

#[cfg(target_os = "windows")]
#[path = "../src/ui/paragraph_format.rs"]
pub mod paragraph_format;

#[cfg(target_os = "windows")]
#[allow(dead_code, unused_imports)]
#[path = "../src/text_window.rs"]
pub mod text_window;

#[cfg(target_os = "windows")]
#[allow(dead_code)]
#[path = "../src/ui/reader_ctrl.rs"]
pub mod reader_ctrl;

// Where the included modules look for the reader control.
#[cfg(target_os = "windows")]
pub mod ui {
	pub use super::reader_ctrl;
}

#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
mod checks {
	use paperback_core::{document::DocPos, session::ParagraphFormat};
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{
				CHARFORMAT2W, CHARFORMATW, EM_GETCHARFORMAT, EM_GETPARAFORMAT, PARAFORMAT2, PFN_BULLET, PFNS_NONUMBER,
				SCF_SELECTION,
			},
			WindowsAndMessaging::SendMessageW,
		},
	};
	use wxdragon::prelude::*;

	use crate::{
		paragraph_format::apply_paragraph_formats,
		readability::apply_readability_format_to_ctrl,
		rtf::{RtfFontInfo, build_rtf, build_rtf_for_whole_control},
		rtf_stream::{append_rtf_into_ctrl, stream_rtf_into_ctrl},
		text_window::CtrlPos,
		ui::reader_ctrl::ReaderCtrl,
	};

	const CONTENT: &str = "Intro\nTitle\n• one\n• two\nBody";
	const INTRO: i64 = 0;
	const TITLE: i64 = 6;
	const LIST: [i64; 2] = [12, 18];
	const BODY: i64 = 24;

	/// Every char of `CONTENT` is one unit of the control, so positions and units agree.
	const FORMATS: [ParagraphFormat; 2] = [
		ParagraphFormat {
			start: DocPos::from_raw(6),
			end: DocPos::from_raw(12),
			heading_level: Some(2),
			list_item: false,
		},
		ParagraphFormat {
			start: DocPos::from_raw(12),
			end: DocPos::from_raw(24),
			heading_level: None,
			list_item: true,
		},
	];

	fn unit(position: DocPos) -> CtrlPos {
		CtrlPos::from_raw(i64::try_from(position.get()).unwrap())
	}

	fn hwnd(text_ctrl: TextCtrl) -> HWND {
		HWND(text_ctrl.get_handle())
	}

	#[allow(clippy::cast_possible_truncation, reason = "Win32 struct sizes fit a u32")]
	fn paragraph_at(text_ctrl: TextCtrl, position: i64) -> PARAFORMAT2 {
		let mut pf = PARAFORMAT2::default();
		pf.Base.cbSize = size_of::<PARAFORMAT2>() as u32;
		text_ctrl.set_selection(position, position);
		unsafe {
			SendMessageW(hwnd(text_ctrl), EM_GETPARAFORMAT, Some(WPARAM(0)), Some(LPARAM(&raw mut pf as isize)));
		}
		pf
	}

	#[allow(clippy::cast_possible_truncation, reason = "Win32 struct sizes fit a u32")]
	fn characters_at(text_ctrl: TextCtrl, position: i64) -> CHARFORMATW {
		let mut cf = CHARFORMAT2W::default();
		cf.Base.cbSize = size_of::<CHARFORMAT2W>() as u32;
		text_ctrl.set_selection(position, position + 1);
		unsafe {
			SendMessageW(
				hwnd(text_ctrl),
				EM_GETCHARFORMAT,
				Some(WPARAM(SCF_SELECTION as usize)),
				Some(LPARAM(&raw mut cf as isize)),
			);
		}
		cf.Base
	}

	type Layout = (i32, i32, i32, i32, i32, i32);

	fn layout(pf: &PARAFORMAT2) -> Layout {
		(
			pf.dySpaceBefore,
			pf.dySpaceAfter,
			pf.dyLineSpacing,
			pf.Base.dxStartIndent,
			pf.Base.dxOffset,
			pf.Base.dxRightIndent,
		)
	}

	fn is_list_item(pf: &PARAFORMAT2) -> bool {
		pf.Base.wNumbering == PFN_BULLET && (pf.wNumberingStyle.0 & PFNS_NONUMBER.0) != 0
	}

	fn appearance(text_ctrl: TextCtrl) -> Vec<(CHARFORMATW, Layout)> {
		(0..text_ctrl.get_last_position())
			.map(|position| (characters_at(text_ctrl, position), layout(&paragraph_at(text_ctrl, position))))
			.collect()
	}

	fn assert_formatted(text_ctrl: TextCtrl, path: &str) {
		assert_eq!(paragraph_at(text_ctrl, TITLE).sStyle, -3, "{path}: the title is a level 2 heading");
		for plain in [INTRO, BODY] {
			assert_eq!(paragraph_at(text_ctrl, plain).sStyle, -1, "{path}: {plain} keeps the normal style");
			assert!(!is_list_item(&paragraph_at(text_ctrl, plain)), "{path}: {plain} is not in a list");
		}
		assert!(!is_list_item(&paragraph_at(text_ctrl, TITLE)), "{path}: the title is not in a list");
		for item in LIST {
			assert!(is_list_item(&paragraph_at(text_ctrl, item)), "{path}: {item} is a list item without a marker");
		}
	}

	fn assert_looks_unchanged(text_ctrl: TextCtrl, unformatted: &[(CHARFORMATW, Layout)], path: &str) {
		for (position, (now, before)) in appearance(text_ctrl).iter().zip(unformatted).enumerate() {
			assert_eq!(now, before, "{path}: position {position} looks as it does without paragraph formats");
		}
	}

	fn font_info(text_ctrl: TextCtrl) -> RtfFontInfo {
		let font = text_ctrl.get_font().expect("the control has a font");
		RtfFontInfo { face_name: font.get_face_name(), point_size: font.get_point_size() }
	}

	pub fn run(text_ctrl: TextCtrl) {
		text_ctrl.set_value(CONTENT);
		let unformatted = appearance(text_ctrl);
		apply_paragraph_formats(ReaderCtrl::new(text_ctrl), unit, &FORMATS);
		assert_eq!(text_ctrl.get_value(), CONTENT, "plain: the text is untouched");
		assert_formatted(text_ctrl, "plain");
		assert_looks_unchanged(text_ctrl, &unformatted, "plain");
		assert!(!text_ctrl.is_editable(), "plain: the control is read-only again");
		apply_readability_format_to_ctrl(text_ctrl, 1, 1, 0, 1);
		assert_eq!(paragraph_at(text_ctrl, TITLE).sStyle, -3, "readability: the heading style survives");
		assert!(is_list_item(&paragraph_at(text_ctrl, LIST[0])), "readability: the list format survives");
		assert!(stream_rtf_into_ctrl(text_ctrl, &build_rtf(CONTENT, &[], &[], &font_info(text_ctrl))));
		let unformatted = appearance(text_ctrl);
		assert!(stream_rtf_into_ctrl(text_ctrl, &build_rtf(CONTENT, &[], &FORMATS, &font_info(text_ctrl))));
		assert_eq!(text_ctrl.get_value(), CONTENT, "rtf: the text is untouched");
		assert_formatted(text_ctrl, "rtf");
		assert_looks_unchanged(text_ctrl, &unformatted, "rtf");
		// An appended chunk's formats are local to the chunk, as a window extension's are.
		let heading = [ParagraphFormat {
			start: DocPos::ZERO,
			end: DocPos::from_raw(6),
			heading_level: Some(1),
			list_item: false,
		}];
		let font = font_info(text_ctrl);
		assert!(stream_rtf_into_ctrl(text_ctrl, &build_rtf_for_whole_control("Intro\n", &[], &[], &font)));
		assert!(append_rtf_into_ctrl(text_ctrl, &build_rtf("Title\nBody", &[], &heading, &font)));
		assert_appended_heading(text_ctrl, "rtf append");
		text_ctrl.set_value("Intro\n");
		let base = text_ctrl.get_last_position();
		text_ctrl.append_text("Title\nBody");
		apply_paragraph_formats(ReaderCtrl::new(text_ctrl), |offset| unit(offset) + base, &heading);
		assert_appended_heading(text_ctrl, "plain append");
	}

	fn assert_appended_heading(text_ctrl: TextCtrl, path: &str) {
		let text = text_ctrl.get_value();
		assert_eq!(text, "Intro\nTitle\nBody", "{path}: the chunk lands on its own paragraphs");
		for (paragraph, style) in [("Intro", -1), ("Title", -2), ("Body", -1)] {
			let position = i64::try_from(text.find(paragraph).unwrap()).unwrap();
			assert_eq!(paragraph_at(text_ctrl, position).sStyle, style, "{path}: {paragraph} has style {style}");
		}
	}
}

// wxWidgets runs its windows on the main thread; Rust's default test harness uses worker threads.
#[cfg(target_os = "windows")]
fn main() {
	use std::{cell::Cell, rc::Rc};

	use wxdragon::prelude::*;
	let passed = Rc::new(Cell::new(false));
	let passed_for_app = Rc::clone(&passed);
	wxdragon::main(move |_| {
		let frame =
			Frame::builder().with_title("Paperback paragraph style checks").with_size(Size::new(400, 400)).build();
		let panel = Panel::builder(&frame).build();
		let text_ctrl = TextCtrl::builder(&panel)
			.with_size(Size::new(380, 350))
			.with_style(
				TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2 | TextCtrlStyle::WordWrap,
			)
			.build();
		checks::run(text_ctrl);
		passed_for_app.set(true);
		frame.close(true);
		wxdragon::call_after(Box::new(|| wxdragon::app::get_app_instance().unwrap().exit_main_loop()));
	})
	.unwrap();
	assert!(passed.get(), "paragraph style checks did not complete");
	println!("Passed real RichEdit paragraph style checks");
}
