#[cfg(target_os = "macos")]
#[path = "../src/ui/reader_input/macos.rs"]
pub mod macos;

#[cfg(target_os = "macos")]
#[allow(dead_code, unused_imports)]
#[path = "../src/text_window.rs"]
pub mod text_window;

#[cfg(target_os = "macos")]
pub mod ui {
	#[allow(dead_code)]
	#[path = "../../src/ui/reader_ctrl.rs"]
	pub mod reader_ctrl;
}

#[cfg(target_os = "macos")]
use std::{cell::Cell, rc::Rc};

#[cfg(target_os = "macos")]
use wxdragon::prelude::*;

#[cfg(not(target_os = "macos"))]
fn main() {}

// AppKit must initialize on the main thread, so this target uses its own test harness.
#[cfg(target_os = "macos")]
fn main() {
	let passed = Rc::new(Cell::new(false));
	let passed_for_app = Rc::clone(&passed);
	wxdragon::main(move |_| {
		let frame = Frame::builder().with_title("Paperback navigation checks").with_size(Size::new(180, 400)).build();
		let panel = Panel::builder(&frame).build();
		let text_ctrl = TextCtrl::builder(&panel)
			.with_size(Size::new(160, 350))
			.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2 | TextCtrlStyle::WordWrap)
			.build();
		let reader = ui::reader_ctrl::ReaderCtrl::new(text_ctrl);
		let set_line_start = |pos: i64| macos::set_line_start(reader, text_window::CtrlPos::from_raw(pos));
		let adjacent_line_start =
			|down: bool| macos::try_adjacent_line_start(reader, down).map(|line| line.map(|(pos, col)| (pos.get(), col)));
		text_ctrl.set_value("First line\nSecond line\n\nUnicode café 🦉 text\nLast line\n");
		let starts = [0_i64, 11, 23, 24, 45, 55];
		for (line, start) in starts.into_iter().enumerate() {
			let end = starts.get(line + 1).copied().map_or_else(|| text_ctrl.get_last_position(), |p| p - 1);
			for pos in [start, start.midpoint(end), end] {
				for down in [false, true] {
					set_line_start(pos);
					let target = if down { starts.get(line + 1).copied() } else { line.checked_sub(1).map(|i| starts[i]) };
					let actual = adjacent_line_start(down);
					assert_eq!(actual, Some(target.map(|p| (p, 0))), "line {line}, position {pos}, down {down}");
				}
			}
		}
		text_ctrl.set_value("The quick brown fox jumps over the lazy dog again and again until this paragraph wraps across several visual lines.\nNext paragraph.");
		let mut starts = vec![0_i64];
		while let Some(Some((next, _))) = adjacent_line_start(true) {
			assert!(next > *starts.last().unwrap());
			starts.push(next);
			set_line_start(next);
		}
		assert!(starts.len() > 3, "the narrow control must wrap");
		for (line, start) in starts.iter().copied().enumerate() {
			for pos in [start, start + 2] {
				for down in [false, true] {
					set_line_start(pos);
					let target = if down { starts.get(line + 1).copied() } else { line.checked_sub(1).map(|i| starts[i]) };
					assert_eq!(adjacent_line_start(down), Some(target.map(|p| (p, 0))), "wrapped line {line}, position {pos}, down {down}");
					if let Some(target) = target {
						set_line_start(target);
						assert_eq!(text_ctrl.get_insertion_point(), target);
					}
				}
			}
		}
		text_ctrl.set_insertion_point(starts[1]);
		assert_eq!(adjacent_line_start(true), Some(Some((starts[1], 0))), "upstream affinity represents the end of the preceding wrapped line");
		set_line_start(starts[1]);
		assert_eq!(adjacent_line_start(true), Some(Some((starts[2], 0))), "downstream affinity represents the start of the next wrapped line");
		text_ctrl.set_value("");
		assert_eq!(adjacent_line_start(true), Some(None));
		assert_eq!(adjacent_line_start(false), Some(None));
		passed_for_app.set(true);
		frame.close(true);
		wxdragon::call_after(Box::new(|| wxdragon::app::get_app_instance().unwrap().exit_main_loop()));
	}).unwrap();
	assert!(passed.get(), "native navigation checks did not complete");
	println!("Passed real wxWidgets/macOS navigation checks");
}
