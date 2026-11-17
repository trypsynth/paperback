//! A screen reader reads headings and list items from `EM_GETPARAFORMAT`, and links from `CFE_LINK` or the Text Object Model's `tomLink` unit.
//! Paragraph formats must not change anything a sighted reader sees.

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
#[path = "../src/ui/link_format.rs"]
pub mod link_format;

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
mod support {
	use paperback_core::document::DocPos;
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{CHARFORMAT2W, CHARFORMATW, EM_GETCHARFORMAT, SCF_SELECTION},
			WindowsAndMessaging::SendMessageW,
		},
	};
	use wxdragon::prelude::*;

	use crate::{rtf::RtfFontInfo, text_window::CtrlPos};

	/// Every char of the test texts is one unit of the control, so positions and units agree.
	pub fn unit(position: DocPos) -> CtrlPos {
		CtrlPos::from_raw(i64::try_from(position.get()).unwrap())
	}

	pub fn hwnd(text_ctrl: TextCtrl) -> HWND {
		HWND(text_ctrl.get_handle())
	}

	#[allow(clippy::cast_possible_truncation, reason = "Win32 struct sizes fit a u32")]
	pub fn characters_at(text_ctrl: TextCtrl, position: i64) -> CHARFORMATW {
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

	pub fn font_info(text_ctrl: TextCtrl) -> RtfFontInfo {
		let font = text_ctrl.get_font().expect("the control has a font");
		RtfFontInfo { face_name: font.get_face_name(), point_size: font.get_point_size() }
	}
}

#[cfg(target_os = "windows")]
mod paragraph_styles {
	use paperback_core::{document::DocPos, session::ParagraphFormat};
	use windows::Win32::{
		Foundation::{LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{CHARFORMATW, EM_GETPARAFORMAT, PARAFORMAT2, PFN_BULLET, PFNS_NONUMBER},
			WindowsAndMessaging::SendMessageW,
		},
	};
	use wxdragon::prelude::*;

	use crate::{
		paragraph_format::apply_paragraph_formats,
		readability::apply_readability_format_to_ctrl,
		rtf::{build_rtf, build_rtf_for_whole_control},
		rtf_stream::{append_rtf_into_ctrl, stream_rtf_into_ctrl},
		support::{characters_at, font_info, hwnd, unit},
		ui::reader_ctrl::ReaderCtrl,
	};

	const CONTENT: &str = "Intro\nTitle\n• one\n• two\nBody";
	const INTRO: i64 = 0;
	const TITLE: i64 = 6;
	const LIST: [i64; 2] = [12, 18];
	const BODY: i64 = 24;

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

#[cfg(target_os = "windows")]
mod link_marks {
	use paperback_core::{
		document::{DocLen, DocPos, MarkerType},
		session::LineMarker,
	};
	use windows::Win32::UI::Controls::RichEdit::{CFE_LINK, tomLink, tomStart};
	use wxdragon::prelude::*;

	use crate::{
		link_format::apply_link_marks,
		paragraph_format::text_document,
		readability::apply_readability_format_to_ctrl,
		rtf::{build_rtf, build_rtf_for_whole_control},
		rtf_stream::{append_rtf_into_ctrl, stream_rtf_into_ctrl},
		support::{characters_at, font_info, unit},
		text_window::CtrlPos,
		ui::reader_ctrl::ReaderCtrl,
	};

	const CONTENT: &str = "Intro link Body\nMore text";
	const LINK: i64 = 6;
	const LINK_LEN: i64 = 4;
	const SELECTION: (i64, i64) = (2, 3);

	fn marker(mtype: MarkerType, position: usize, text: &str) -> LineMarker {
		LineMarker {
			mtype,
			position: DocPos::from_raw(position),
			text: text.to_string(),
			reference: "#target".to_string(),
			level: 0,
			length: DocLen::of(text),
		}
	}

	/// A bold run on "Intro" and a link on "link".
	fn markers(link_position: usize) -> [LineMarker; 2] {
		[marker(MarkerType::Bold, 0, "Intro"), marker(MarkerType::Link, link_position, "link")]
	}

	fn char_format_says_link(text_ctrl: TextCtrl, position: i64) -> bool {
		characters_at(text_ctrl, position).dwEffects.0 & CFE_LINK.0 != 0
	}

	/// NVDA's check: collapse to the start, then expand to the link around it.
	fn tom_says_link(text_ctrl: TextCtrl, position: i64) -> bool {
		let document = text_document(text_ctrl).expect("the control has a text document");
		let position = i32::try_from(position).unwrap();
		unsafe {
			let range = document.Range(position, position + 1).expect("a range in the text");
			range.Collapse(tomStart.0).expect("collapse to the start");
			range.Expand(tomLink.0).expect("expand to a link") > 0
		}
	}

	fn assert_marked(text_ctrl: TextCtrl, link: i64, path: &str) {
		for position in [link, link + LINK_LEN - 1] {
			assert!(
				char_format_says_link(text_ctrl, position),
				"{path}: EM_GETCHARFORMAT reports a link at {position}"
			);
		}
		assert!(tom_says_link(text_ctrl, link), "{path}: tomLink expands from the link's start at {link}");
		for position in [link - 1, link + LINK_LEN] {
			assert!(
				!char_format_says_link(text_ctrl, position),
				"{path}: EM_GETCHARFORMAT reports no link at {position}"
			);
			assert!(!tom_says_link(text_ctrl, position), "{path}: tomLink finds no link from {position}");
		}
	}

	fn mark(text_ctrl: TextCtrl, place: impl Fn(DocPos) -> CtrlPos, markers: &[LineMarker], path: &str) {
		let text = text_ctrl.get_value();
		text_ctrl.set_selection(SELECTION.0, SELECTION.1);
		apply_link_marks(ReaderCtrl::new(text_ctrl), place, markers);
		assert_eq!(text_ctrl.get_value(), text, "{path}: the text is untouched");
		assert_eq!(text_ctrl.get_selection(), SELECTION, "{path}: the selection is untouched");
		assert!(!text_ctrl.is_editable(), "{path}: the control is read-only again");
	}

	pub fn run(text_ctrl: TextCtrl) {
		text_ctrl.set_value(CONTENT);
		mark(text_ctrl, unit, &markers(6), "plain");
		assert_marked(text_ctrl, LINK, "plain");
		assert!(!char_format_says_link(text_ctrl, 0), "plain: the bold run is not a link");
		apply_readability_format_to_ctrl(text_ctrl, 1, 1, 1, 1);
		assert_marked(text_ctrl, LINK, "readability");
		let font = font_info(text_ctrl);
		assert!(stream_rtf_into_ctrl(text_ctrl, &build_rtf_for_whole_control(CONTENT, &[], &[], &font)));
		mark(text_ctrl, unit, &markers(6), "rtf");
		assert_marked(text_ctrl, LINK, "rtf");
		// An appended chunk's markers are local to the chunk, as a window extension's are.
		assert!(stream_rtf_into_ctrl(text_ctrl, &build_rtf_for_whole_control("Intro\n", &[], &[], &font)));
		let base = text_ctrl.get_last_position();
		assert!(append_rtf_into_ctrl(text_ctrl, &build_rtf("Text link Body", &[], &[], &font)));
		assert_eq!(text_ctrl.get_value(), "Intro\nText link Body", "rtf append: the chunk lands after the text");
		mark(text_ctrl, |offset| unit(offset) + base, &markers(5), "rtf append");
		assert_marked(text_ctrl, base + 5, "rtf append");
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
			Frame::builder().with_title("Paperback reader format checks").with_size(Size::new(400, 400)).build();
		let panel = Panel::builder(&frame).build();
		let text_ctrl = TextCtrl::builder(&panel)
			.with_size(Size::new(380, 350))
			.with_style(
				TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2 | TextCtrlStyle::WordWrap,
			)
			.build();
		paragraph_styles::run(text_ctrl);
		link_marks::run(text_ctrl);
		passed_for_app.set(true);
		frame.close(true);
		wxdragon::call_after(Box::new(|| wxdragon::app::get_app_instance().unwrap().exit_main_loop()));
	})
	.unwrap();
	assert!(passed.get(), "reader format checks did not complete");
	println!("Passed real RichEdit reader format checks");
}
