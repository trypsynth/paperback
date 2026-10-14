#[cfg(target_os = "macos")]
#[path = "../src/ui/reader_input/activation.rs"]
pub mod activation;

#[cfg(target_os = "macos")]
#[path = "../src/ui/dialogs/activation.rs"]
pub mod dialog_activation;

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
fn link_session() -> paperback_core::session::DocumentSession {
	use std::{fs::File, io::Write};

	use paperback_core::{document::ParseSettings, session::DocumentSession};
	use zip::{ZipWriter, write::SimpleFileOptions};
	let path = std::env::temp_dir().join(format!("paperback-link-checks-{}.epub", std::process::id()));
	let mut zip = ZipWriter::new(File::create(&path).unwrap());
	for (name, content) in [
		("mimetype", "application/epub+zip"),
		(
			"META-INF/container.xml",
			r#"<container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0"><rootfiles><rootfile full-path="content.opf" media-type="application/oebps-package+xml"/></rootfiles></container>"#,
		),
		(
			"content.opf",
			r#"<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="id"><metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title>Link checks</dc:title><dc:identifier id="id">link-checks</dc:identifier></metadata><manifest><item id="chapter" href="chapter.xhtml" media-type="application/xhtml+xml"/></manifest><spine><itemref idref="chapter"/></spine></package>"#,
		),
		(
			"chapter.xhtml",
			r##"<html xmlns="http://www.w3.org/1999/xhtml"><head><title>Links</title></head><body><p>Start</p><p><a href="#destination">Internal link</a></p><p><a href="https://example.com">External link</a></p><p id="destination">Destination</p><table><caption>Test table</caption><tr><td>Table cell</td></tr></table><p><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>x</mi><mo>=</mo><mn>1</mn></math></p></body></html>"##,
		),
	] {
		zip.start_file(name, SimpleFileOptions::default()).unwrap();
		zip.write_all(content.as_bytes()).unwrap();
	}
	zip.finish().unwrap();
	let session = DocumentSession::new(
		path.to_str().unwrap(),
		"",
		"",
		ParseSettings { render_tables_inline: false, ..ParseSettings::default() },
	)
	.unwrap();
	std::fs::remove_file(path).unwrap();
	session
}

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

	pub fn send_key(frame: impl WxWidget + Copy, text: &str, modifiers: usize, hardware_key: u16) {
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
		let text = TextCtrl::builder(&panel)
			.with_style(
				TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2 | TextCtrlStyle::WordWrap,
			)
			.build();
		let mut config = ConfigManager::new();
		assert!(config.initialize(config_path_for_app.clone()));
		let config = Rc::new(Mutex::new(config));
		let from_keyboard = Rc::new(Cell::new(false));
		let received = Rc::new(RefCell::new(Vec::new()));
		let session = Rc::new(link_session());
		text.set_value(&session.get_text_range_display(0, session.document_len()));
		let session_for_menu = Rc::clone(&session);
		let activated = Rc::new(RefCell::new(Vec::new()));
		let opened_elements = Rc::new(RefCell::new(Vec::new()));
		let opened_for_key = Rc::clone(&opened_elements);
		let activated_for_key = Rc::clone(&activated);
		let session_for_key = Rc::clone(&session);
		activation::bind_activation(text, move |enter| {
			let position = text.get_insertion_point();
			if let Some(html) = session_for_key.get_formula_at_position(position) {
				opened_for_key.borrow_mut().push(("formula", html));
				return;
			}
			if let Some(html) = session_for_key.get_table_at_position(position) {
				opened_for_key.borrow_mut().push(("table", html));
				return;
			}
			let result = session_for_key.activate_link(position);
			if result.action == paperback_core::session::LinkAction::Internal {
				text.set_insertion_point(result.offset);
			}
			activated_for_key.borrow_mut().push((enter, result));
		});
		let received_for_menu = Rc::clone(&received);
		let source_for_menu = Rc::clone(&from_keyboard);
		frame.on_menu(move |event| {
			received_for_menu.borrow_mut().push((event.get_id(), source_for_menu.replace(false)));
			let position = text.get_insertion_point();
			let result = match event.get_id() {
				menu_ids::NEXT_LINK => Some(session_for_menu.navigate_link(position, false, true)),
				menu_ids::NEXT_TABLE => Some(session_for_menu.navigate_table(position, false, true)),
				menu_ids::NEXT_FORMULA => Some(session_for_menu.navigate_formula(position, false, true)),
				_ => None,
			};
			if let Some(result) = result {
				assert!(result.found);
				text.set_insertion_point(result.offset);
			}
		});
		let menu = Menu::builder()
			.append_item(menu_ids::FIND_NEXT, "Find Next\tCtrl+G", "")
			.append_item(menu_ids::FIND_PREVIOUS, "Find Previous\tCtrl+Shift+G", "")
			.append_item(menu_ids::NEXT_HEADING, "Next Heading\tH", "")
			.append_item(menu_ids::PREVIOUS_HEADING, "Previous Heading\tShift+H", "")
			.append_item(menu_ids::NEXT_LINK, "Next Link\tK", "")
			.append_item(menu_ids::NEXT_TABLE, "Next Table\tT", "")
			.append_item(menu_ids::NEXT_FORMULA, "Next Formula\tM", "")
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
		assert!(activated.borrow().is_empty(), "Control+Space must stay an audio shortcut");
		for (character, hardware_key, enter) in [("\r", 36, true), ("\u{3}", 76, true), (" ", 49, false)] {
			text.set_insertion_point(0);
			expect_command(&received, frame, "k", 0, 40, menu_ids::NEXT_LINK);
			let link_position = text.get_insertion_point();
			activated.borrow_mut().clear();
			send_key(frame, character, 0, hardware_key);
			let results = activated.borrow();
			assert_eq!(results.len(), 1, "{character:?} must activate exactly once");
			assert_eq!(results[0].0, enter);
			assert_eq!(results[0].1.action, paperback_core::session::LinkAction::Internal);
			assert!(results[0].1.offset > link_position);
			assert_eq!(text.get_insertion_point(), results[0].1.offset);
			assert!(session.get_line_text(results[0].1.offset).contains("Destination"));
			drop(results);
			text.set_insertion_point(0);
			send_key(frame, "k", 0, 40);
			send_key(frame, "k", 0, 40);
			activated.borrow_mut().clear();
			send_key(frame, character, 0, hardware_key);
			let results = activated.borrow();
			assert_eq!(results.len(), 1);
			assert_eq!(results[0].1.action, paperback_core::session::LinkAction::External);
			assert_eq!(results[0].1.url, "https://example.com");
		}
		for (shortcut, key, id, kind, expected_html) in [
			("t", 17, menu_ids::NEXT_TABLE, "table", "Table cell"),
			("m", 46, menu_ids::NEXT_FORMULA, "formula", "<mi>x</mi>"),
		] {
			for (character, hardware_key) in [("\r", 36), ("\u{3}", 76), (" ", 49)] {
				text.set_insertion_point(0);
				expect_command(&received, frame, shortcut, 0, key, id);
				opened_elements.borrow_mut().clear();
				activated.borrow_mut().clear();
				send_key(frame, character, 0, hardware_key);
				let opened = opened_elements.borrow();
				assert_eq!(opened.len(), 1, "{kind} activation with {character:?}");
				assert_eq!(opened[0].0, kind);
				assert!(opened[0].1.contains(expected_html), "{}", opened[0].1);
				assert!(activated.borrow().is_empty(), "opening {kind} must not activate a link too");
			}
		}
		activated.borrow_mut().clear();
		text.set_insertion_point(0);
		send_key(frame, "\r", 0, 36);
		assert_eq!(activated.borrow().len(), 1);
		assert!(!activated.borrow()[0].1.found, "ordinary text must not follow a link");
		activated.borrow_mut().clear();
		for (character, key) in [("\r", 36), (" ", 49)] {
			send_key(frame, character, CONTROL | OPTION, key);
			assert!(activated.borrow().is_empty(), "VoiceOver chords must not activate a link");
		}
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
		let element_dialog = Dialog::builder(&other, "Element activation checks").build();
		element_dialog.show(true);
		let tree = DataViewTreeCtrl::builder(&element_dialog).build();
		let heading = tree.append_item(&DataViewItem::default(), "Heading", -1);
		tree.select(&heading);
		let activated_rows = Rc::new(Cell::new(0));
		let activated_for_tree = Rc::clone(&activated_rows);
		tree.on_item_activated(move |event| {
			event.event.skip(false);
			activated_for_tree.set(activated_for_tree.get() + 1);
		});
		let ok = Button::builder(&element_dialog).with_id(ID_OK).with_label("OK").build();
		element_dialog.set_affirmative_id(ID_OK);
		ok.set_default();
		let activated_for_ok = Rc::clone(&activated_rows);
		ok.on_click(move |event| {
			event.event.skip(false);
			activated_for_ok.set(activated_for_ok.get() + 1);
		});
		dialog_activation::bind_enter_button(element_dialog, ok);
		tree.set_focus();
		send_key(element_dialog, "\r", 0, 36);
		assert_eq!(activated_rows.get(), 1, "Elements heading trees must activate with Return");
		send_key(element_dialog, "\u{3}", 0, 76);
		assert_eq!(activated_rows.get(), 2, "keypad Enter must confirm through the default OK button");
		let list = DataViewListCtrl::builder(&element_dialog).with_style(DataViewStyle::RowLines).build();
		list.append_text_column("", 0, DataViewAlign::Left, 400, DataViewColumnFlags::Resizable);
		list.append_item(&[Variant::from("Element")]);
		list.select_row(0);
		let activated_for_list = Rc::clone(&activated_rows);
		list.on_item_activated(move |event| {
			event.event.skip(false);
			activated_for_list.set(activated_for_list.get() + 1);
		});
		list.set_focus();
		send_key(element_dialog, "\r", 0, 36);
		assert_eq!(activated_rows.get(), 3, "Elements lists must activate with Return");
		send_key(element_dialog, "\u{3}", 0, 76);
		assert_eq!(activated_rows.get(), 4, "keypad Enter must confirm the list through the default OK button");
		element_dialog.destroy();
		received.borrow_mut().clear();
		other.raise();
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
	println!("Passed real wxWidgets/macOS menu shortcut and EPUB element activation checks");
}
