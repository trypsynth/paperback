#[cfg(target_os = "macos")]
#[allow(unused_imports)]
#[path = "../src/ui/mac_shortcuts.rs"]
mod mac_shortcuts;
#[cfg(target_os = "macos")]
#[allow(dead_code, unused_imports)]
#[path = "../src/ui/menu_ids.rs"]
pub mod menu_ids;
#[cfg(target_os = "macos")]
mod ui {
	pub use crate::menu_ids;
}

#[cfg(not(target_os = "macos"))]
fn main() {}

#[cfg(target_os = "macos")]
mod native {
	use std::{cell::RefCell, ffi::CString, rc::Rc};

	use objc::{Encode, Encoding, class, msg_send, runtime::Object, sel, sel_impl};
	use wxdragon::prelude::*;

	#[repr(C)]
	#[derive(Clone, Copy)]
	struct Point {
		x: f64,
		y: f64,
	}

	unsafe impl Encode for Point {
		fn encode() -> Encoding {
			unsafe { Encoding::from_str("{CGPoint=dd}") }
		}
	}

	pub const COMMAND: usize = 1 << 20;
	pub const SHIFT: usize = 1 << 17;
	pub const CONTROL: usize = 1 << 18;
	pub const OPTION: usize = 1 << 19;

	pub fn send_key(frame: Frame, text: &str, modifiers: usize, hardware_key: u16) {
		let text = CString::new(text).unwrap();
		unsafe {
			let string: *mut Object = msg_send![class!(NSString), stringWithUTF8String: text.as_ptr()];
			let view = frame.get_handle().cast::<Object>();
			let window: *mut Object = msg_send![view, window];
			let number: isize = msg_send![window, windowNumber];
			let event: *mut Object = msg_send![class!(NSEvent),
				keyEventWithType: 10_usize
				location: Point { x: 0.0, y: 0.0 }
				modifierFlags: modifiers
				timestamp: 0.0_f64
				windowNumber: number
				context: std::ptr::null_mut::<Object>()
				characters: string
				charactersIgnoringModifiers: string
				isARepeat: false
				keyCode: hardware_key
			];
			assert!(!event.is_null());
			let app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
			let _: () = msg_send![app, sendEvent: event];
		}
	}

	pub fn expect_command(
		received: &Rc<RefCell<Vec<(i32, bool)>>>,
		frame: Frame,
		text: &str,
		flags: usize,
		key: u16,
		id: i32,
	) {
		received.borrow_mut().clear();
		send_key(frame, text, flags, key);
		assert_eq!(*received.borrow(), [(id, true)], "key {text:?}, modifiers {flags:#x}");
	}
}

// AppKit must initialize on the main thread, so this target uses its own test harness.
#[cfg(target_os = "macos")]
fn main() {
	use std::{
		cell::{Cell, RefCell},
		rc::Rc,
		sync::Mutex,
	};

	use native::{COMMAND, CONTROL, OPTION, SHIFT, expect_command, send_key};
	use paperback_core::config::{ActionId, ConfigManager, KeyChord};
	use wxdragon::prelude::*;

	let passed = Rc::new(Cell::new(false));
	let passed_for_app = Rc::clone(&passed);
	let config_path = std::env::temp_dir().join(format!("paperback-shortcut-checks-{}.toml", std::process::id()));
	let config_path_for_app = config_path.clone();
	wxdragon::main(move |_| {
		let frame = Frame::builder().with_title("Paperback shortcut checks").build();
		let panel = Panel::builder(&frame).build();
		let text = TextCtrl::builder(&panel).with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly).build();
		let mut config = ConfigManager::new();
		assert!(config.initialize(config_path_for_app.clone()));
		let config = Rc::new(Mutex::new(config));
		let from_keyboard = Rc::new(Cell::new(false));
		let received = Rc::new(RefCell::new(Vec::new()));
		let received_for_menu = Rc::clone(&received);
		let source_for_menu = Rc::clone(&from_keyboard);
		frame.on_menu(move |event| {
			received_for_menu.borrow_mut().push((event.get_id(), source_for_menu.replace(false)));
		});
		let menu = Menu::builder()
			.append_item(menu_ids::FIND_NEXT, "Find Next\tCtrl+G", "")
			.append_item(menu_ids::FIND_PREVIOUS, "Find Previous\tCtrl+Shift+G", "")
			.append_item(menu_ids::NEXT_HEADING, "Next Heading\tH", "")
			.append_item(menu_ids::PREVIOUS_HEADING, "Previous Heading\tShift+H", "")
			.append_item(menu_ids::PLAY_PAUSE_AUDIO, "Play/Pause\tRawCtrl+Space", "")
			.build();
		frame.set_menu_bar(MenuBar::builder().append(menu, "Commands").build());
		mac_shortcuts::install(frame, Rc::clone(&config), Rc::clone(&from_keyboard));
		frame.show(true);
		text.set_focus();
		expect_command(&received, frame, "g", COMMAND, 5, menu_ids::FIND_NEXT);
		expect_command(&received, frame, "G", COMMAND | SHIFT, 5, menu_ids::FIND_PREVIOUS);
		expect_command(&received, frame, "h", 0, 4, menu_ids::NEXT_HEADING);
		expect_command(&received, frame, "H", SHIFT, 4, menu_ids::PREVIOUS_HEADING);
		expect_command(&received, frame, " ", CONTROL, 49, menu_ids::PLAY_PAUSE_AUDIO);
		assert!(!from_keyboard.get());
		received.borrow_mut().clear();
		frame.process_menu_command(menu_ids::FIND_NEXT);
		assert_eq!(*received.borrow(), [(menu_ids::FIND_NEXT, false)], "menu clicks must retain their own source");
		let bar = frame.get_menu_bar().unwrap();
		bar.enable_item(menu_ids::NEXT_HEADING, false);
		received.borrow_mut().clear();
		send_key(frame, "h", 0, 4);
		assert!(received.borrow().is_empty(), "disabled commands must not run");
		bar.enable_item(menu_ids::NEXT_HEADING, true);
		send_key(frame, "h", CONTROL | OPTION, 4);
		assert!(received.borrow().is_empty(), "VoiceOver chords must pass through");
		let other = Frame::builder().with_title("Other window").build();
		let other_text = TextCtrl::builder(&other).build();
		other.show(true);
		other_text.set_focus();
		send_key(other, "h", 0, 4);
		assert!(received.borrow().is_empty(), "typing in another window must not navigate the book");
		other.close(true);
		text.set_focus();
		frame.enable(false);
		send_key(frame, "h", 0, 4);
		assert!(received.borrow().is_empty(), "a disabled main frame must not handle dialog keys");
		frame.enable(true);
		// Child destruction must not unregister the frame's monitor.
		let child = TextCtrl::builder(&panel).build();
		child.destroy();
		expect_command(&received, frame, "h", 0, 4, menu_ids::NEXT_HEADING);

		let mut shortcuts = config.lock().unwrap().get_shortcuts();
		shortcuts.set_chord(ActionId::NextHeading, Some(KeyChord::new(true, false, true, "/")));
		config.lock().unwrap().set_shortcuts(&shortcuts);
		expect_command(&received, frame, "?", COMMAND | SHIFT, 44, menu_ids::NEXT_HEADING);
		assert_eq!(bar.find_item(menu_ids::FIND_NEXT).unwrap().get_label(), "Find Next\tCtrl+G");
		passed_for_app.set(true);
		frame.close(true);
		wxdragon::call_after(Box::new(|| wxdragon::app::get_app_instance().unwrap().exit_main_loop()));
	})
	.unwrap();
	std::fs::remove_file(config_path).unwrap();
	assert!(passed.get(), "native shortcut checks did not complete");
	println!("Passed real wxWidgets/macOS menu shortcut checks");
}
