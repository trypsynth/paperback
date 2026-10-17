//! Checks against a real `RichEdit` control that a window loaded as RTF keeps its last paragraph
//! mark, so a window extension appended after it starts a paragraph of its own, whether the
//! extension arrives as RTF or as plain text.

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

#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
mod checks {
	use wxdragon::prelude::*;

	use crate::{
		rtf::{RtfFontInfo, build_rtf, build_rtf_for_whole_control},
		rtf_stream::{append_rtf_into_ctrl, stream_rtf_into_ctrl},
	};

	/// A window ends on a paragraph boundary unless it reaches the end of the document.
	const WINDOW: &str = "Intro\n";
	const EXTENSION: &str = "Title\nBody\n";

	fn font_info(text_ctrl: TextCtrl) -> RtfFontInfo {
		let font = text_ctrl.get_font().expect("the control has a font");
		RtfFontInfo { face_name: font.get_face_name(), point_size: font.get_point_size() }
	}

	fn load_window(text_ctrl: TextCtrl) {
		assert!(stream_rtf_into_ctrl(text_ctrl, &build_rtf_for_whole_control(WINDOW, &[], &font_info(text_ctrl))));
		assert_eq!(text_ctrl.get_value(), WINDOW, "the loaded window keeps its last paragraph mark");
		assert_eq!(text_ctrl.get_last_position(), 6, "the loaded window stores every display unit");
	}

	fn assert_extended(text_ctrl: TextCtrl, path: &str) {
		assert_eq!(
			text_ctrl.get_value(),
			"Intro\nTitle\nBody\n",
			"{path}: the extension starts a paragraph of its own"
		);
		assert_eq!(text_ctrl.get_last_position(), 17, "{path}: every display unit is stored");
	}

	pub fn run(text_ctrl: TextCtrl) {
		load_window(text_ctrl);
		assert!(append_rtf_into_ctrl(text_ctrl, &build_rtf(EXTENSION, &[], &font_info(text_ctrl))));
		assert_extended(text_ctrl, "RTF extension");
		load_window(text_ctrl);
		text_ctrl.append_text(EXTENSION);
		assert_extended(text_ctrl, "plain extension");
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
		let frame = Frame::builder().with_title("Paperback RTF window checks").with_size(Size::new(400, 400)).build();
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
	assert!(passed.get(), "RTF window checks did not complete");
	println!("Passed real RichEdit RTF window checks");
}
