//! Checks which caret moves count as external: another process's, through the Text Object Model or UI Automation, and none of the app's own, also while it handles another process's message.

#[cfg(target_os = "windows")]
#[allow(dead_code, unused_imports)]
#[path = "../src/text_window.rs"]
pub mod text_window;

#[cfg(target_os = "windows")]
#[allow(dead_code)]
#[path = "../src/ui/reader_ctrl.rs"]
pub mod reader_ctrl;

#[cfg(target_os = "windows")]
#[path = "../src/ui/external_caret.rs"]
pub mod external_caret;

// Where the included modules look for the reader control.
#[cfg(target_os = "windows")]
pub mod ui {
	pub use super::reader_ctrl;
}

#[cfg(not(target_os = "windows"))]
fn main() {}

/// The caret positions each move goes to, in order.
#[cfg(target_os = "windows")]
mod moves {
	/// The app's own moves, from its own thread.
	pub const OWN: [(&str, i32); 3] = [("set_insertion_point", 5), ("EM_SETSEL", 8), ("EM_EXSETSEL", 11)];
	/// The client's moves.
	pub const EXTERNAL: [(&str, i32); 2] = [("the Text Object Model", 20), ("UI Automation", 25)];
	/// The app's own move by `EM_EXSETSEL`, while it handles a message the client sent.
	pub const OWN_FOR_CLIENT: i32 = 40;
	/// The message the client sends the control's parent to ask for that move.
	pub const OWN_FOR_CLIENT_MESSAGE: &str = "PaperbackExternalCaretChecksOwnMove";
}

/// The client process: moves the caret in another process's control each way in turn.
#[cfg(target_os = "windows")]
mod client {
	use std::{ffi::c_void, ptr::null_mut};

	use windows::{
		Win32::{
			Foundation::{HWND, WPARAM},
			System::Com::{CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx},
			UI::{
				Accessibility::{
					AccessibleObjectFromWindow, CUIAutomation, IUIAutomation, IUIAutomationTextPattern,
					TextPatternRangeEndpoint_End, TextPatternRangeEndpoint_Start, TextUnit_Character,
					UIA_TextPatternId,
				},
				Controls::RichEdit::ITextDocument,
				WindowsAndMessaging::{GetParent, OBJID_NATIVEOM, RegisterWindowMessageW, SendMessageW},
			},
		},
		core::{HSTRING, Interface},
	};

	use crate::moves;

	/// Moves the caret in `hwnd` by each of [`moves::EXTERNAL`], then asks its process for [`moves::OWN_FOR_CLIENT`].
	pub fn move_caret(hwnd: usize) -> windows::core::Result<()> {
		let hwnd = HWND(hwnd as *mut c_void);
		unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok()? };
		let [(_, by_tom), (_, by_uia)] = moves::EXTERNAL;
		text_object_model(hwnd, by_tom)?;
		ui_automation(hwnd, by_uia)?;
		let message = unsafe { RegisterWindowMessageW(&HSTRING::from(moves::OWN_FOR_CLIENT_MESSAGE)) };
		let parent = unsafe { GetParent(hwnd)? };
		let position = usize::try_from(moves::OWN_FOR_CLIENT).expect("a position");
		unsafe { SendMessageW(parent, message, Some(WPARAM(position)), None) };
		Ok(())
	}

	fn text_object_model(hwnd: HWND, position: i32) -> windows::core::Result<()> {
		let mut object: *mut c_void = null_mut();
		let id = u32::from_ne_bytes(OBJID_NATIVEOM.0.to_ne_bytes());
		unsafe { AccessibleObjectFromWindow(hwnd, id, &ITextDocument::IID, &raw mut object)? };
		let document = unsafe { ITextDocument::from_raw(object) };
		let selection = unsafe { document.GetSelection()? };
		unsafe {
			selection.SetStart(position)?;
			selection.SetEnd(position)
		}
	}

	fn ui_automation(hwnd: HWND, position: i32) -> windows::core::Result<()> {
		let automation: IUIAutomation = unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)? };
		let element = unsafe { automation.ElementFromHandle(hwnd)? };
		let pattern: IUIAutomationTextPattern = unsafe { element.GetCurrentPatternAs(UIA_TextPatternId)? };
		let range = unsafe { pattern.DocumentRange()? };
		unsafe {
			range.MoveEndpointByRange(TextPatternRangeEndpoint_End, &range, TextPatternRangeEndpoint_Start)?;
			range.Move(TextUnit_Character, position)?;
			range.Select()
		}
	}
}

#[cfg(target_os = "windows")]
mod checks {
	use std::{
		cell::RefCell,
		ffi::c_void,
		process::{Child, Command},
		rc::Rc,
	};

	use windows::{
		Win32::{
			Foundation::{HWND, LPARAM, LRESULT, WPARAM},
			UI::{
				Controls::{
					EM_SETSEL,
					RichEdit::{CHARRANGE, EM_EXSETSEL},
				},
				Shell::{DefSubclassProc, SetWindowSubclass},
				WindowsAndMessaging::{GetParent, RegisterWindowMessageW, SendMessageW},
			},
		},
		core::HSTRING,
	};
	use wxdragon::prelude::*;

	use crate::{
		external_caret::{CaretMove, watch_caret},
		moves,
		reader_ctrl::ReaderCtrl,
		text_window::CtrlPos,
	};

	const TEXT: &str = "One two three four five six seven eight nine ten.";

	pub struct Watched {
		pub moves: Rc<RefCell<Vec<CaretMove>>>,
		pub client: Child,
	}

	/// Moves the caret itself each of [`moves::OWN`], then starts the client that moves it from another process.
	pub fn start(text_ctrl: TextCtrl) -> Watched {
		let text_ctrl = ReaderCtrl::new(text_ctrl);
		text_ctrl.set_value(TEXT);
		text_ctrl.set_insertion_point(CtrlPos::from_raw(2));
		let reported = Rc::new(RefCell::new(Vec::new()));
		let recorded = Rc::clone(&reported);
		watch_caret(text_ctrl, move |moved| recorded.borrow_mut().push(moved));
		let hwnd = HWND(text_ctrl.widget().get_handle());
		let [(_, by_wx), (_, by_setsel), (_, by_exsetsel)] = moves::OWN;
		text_ctrl.set_insertion_point(CtrlPos::from_raw(i64::from(by_wx)));
		setsel(hwnd, by_setsel);
		exsetsel(hwnd, by_exsetsel);
		let message = unsafe { RegisterWindowMessageW(&HSTRING::from(moves::OWN_FOR_CLIENT_MESSAGE)) };
		let parent = unsafe { GetParent(hwnd) }.expect("the control's parent");
		let installed = unsafe { SetWindowSubclass(parent, Some(move_for_client), message as usize, hwnd.0.addr()) };
		assert!(installed.as_bool(), "watching the parent for the client's message");
		let client = Command::new(std::env::current_exe().expect("the test's own path"))
			.args(["--move-caret", &hwnd.0.addr().to_string()])
			.spawn()
			.expect("starting the client");
		Watched { moves: reported, client }
	}

	/// Sends `EM_SETSEL` from this thread.
	fn setsel(hwnd: HWND, position: i32) {
		let position = usize::try_from(position).expect("a position");
		unsafe { SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(position)), Some(LPARAM(position.cast_signed()))) };
	}

	/// Sends `EM_EXSETSEL` from this thread.
	fn exsetsel(hwnd: HWND, position: i32) {
		let range = CHARRANGE { cpMin: position, cpMax: position };
		unsafe { SendMessageW(hwnd, EM_EXSETSEL, None, Some(LPARAM((&raw const range) as isize))) };
	}

	/// On the client's message, whose number is `id`, moves the caret of the control `data` to the position in `wparam`.
	unsafe extern "system" fn move_for_client(
		hwnd: HWND,
		message: u32,
		wparam: WPARAM,
		lparam: LPARAM,
		id: usize,
		data: usize,
	) -> LRESULT {
		if message as usize == id {
			exsetsel(HWND(data as *mut c_void), i32::try_from(wparam.0).expect("a position"));
			return LRESULT(0);
		}
		unsafe { DefSubclassProc(hwnd, message, wparam, lparam) }
	}

	pub fn check(reported: &[CaretMove]) {
		let at = |position: i32, how: &str| {
			*reported
				.iter()
				.find(|moved| moved.after.get() == i64::from(position))
				.unwrap_or_else(|| panic!("no selection change arrived for the move by {how}: {reported:?}"))
		};
		for (how, position) in moves::OWN {
			let moved = at(position, how);
			assert!(!moved.external, "the app's own move by {how} counts as external: {moved:?}");
		}
		let mut before = moves::OWN[2].1;
		for (how, position) in moves::EXTERNAL {
			let moved = at(position, how);
			assert!(moved.external, "the client's move by {how} wasn't recognised as external: {moved:?}");
			assert_eq!(moved.before.get(), i64::from(before), "the client's move by {how} starts where the caret was");
			before = position;
		}
		let moved = at(moves::OWN_FOR_CLIENT, "the app while it handles the client's message");
		assert!(
			!moved.external,
			"the app's own move while it handles the client's message counts as external: {moved:?}"
		);
	}
}

#[cfg(target_os = "windows")]
fn main() {
	use std::{
		cell::RefCell,
		panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
		rc::Rc,
		time::{Duration, Instant},
	};

	use wxdragon::prelude::*;

	let args: Vec<String> = std::env::args().collect();
	if let [_, flag, hwnd] = args.as_slice()
		&& flag == "--move-caret"
	{
		if let Err(error) = client::move_caret(hwnd.parse().unwrap()) {
			eprintln!("the client couldn't move the caret: {error}");
			std::process::exit(1);
		}
		return;
	}
	let outcome = Rc::new(RefCell::new(None));
	let outcome_for_app = Rc::clone(&outcome);
	wxdragon::main(move |_| {
		let frame =
			Frame::builder().with_title("Paperback external caret checks").with_size(Size::new(400, 400)).build();
		let panel = Panel::builder(&frame).build();
		let text_ctrl = TextCtrl::builder(&panel)
			.with_size(Size::new(380, 350))
			.with_style(
				TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2 | TextCtrlStyle::WordWrap,
			)
			.build();
		let mut watched = checks::start(text_ctrl);
		let deadline = Instant::now() + Duration::from_secs(20);
		let timer = Rc::new(Timer::new(&frame));
		let timer_for_tick = Rc::clone(&timer);
		// A failed check ends the app, and the failure is raised again once its loop has stopped.
		timer.on_tick(move |_| {
			let checked = catch_unwind(AssertUnwindSafe(|| {
				let status = watched.client.try_wait().expect("waiting for the client");
				let Some(status) = status else {
					assert!(Instant::now() < deadline, "the client never finished");
					return false;
				};
				assert!(status.success(), "the client failed: {status}");
				checks::check(&watched.moves.borrow());
				true
			}));
			if matches!(checked, Ok(false)) {
				return;
			}
			if checked.is_err() {
				watched.client.kill().ok();
			}
			timer_for_tick.stop();
			outcome_for_app.replace(Some(checked.map(drop)));
			frame.close(true);
			wxdragon::call_after(Box::new(|| wxdragon::app::get_app_instance().unwrap().exit_main_loop()));
		});
		timer.start(20, false);
	})
	.unwrap();
	match outcome.take() {
		Some(Ok(())) => println!("Passed real RichEdit external caret checks"),
		Some(Err(failure)) => resume_unwind(failure),
		None => panic!("external caret checks did not complete"),
	}
}
