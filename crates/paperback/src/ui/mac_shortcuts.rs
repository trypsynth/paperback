//! Run app shortcuts before Cocoa selects their menu items and `VoiceOver` reads their labels.

use std::{cell::Cell, ptr, rc::Rc, sync::Mutex};

use block::ConcreteBlock;
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use paperback_core::config::{ActionId, ConfigManager, ShortcutsConfig};
use wxdragon::prelude::*;

use crate::ui::menu_ids;

// NSEventModifierFlags and NSEventMaskKeyDown, from AppKit.
const SHIFT: usize = 1 << 17;
const CONTROL: usize = 1 << 18;
const OPTION: usize = 1 << 19;
const COMMAND: usize = 1 << 20;
const KEY_DOWN: usize = 1 << 10;

fn wx_key_code(character: u16) -> Option<i32> {
	match character {
		0x03 | 0x0d => Some(WXK_RETURN),
		0x7f => Some(WXK_BACK),
		0xf700 => Some(WXK_UP),
		0xf701 => Some(WXK_DOWN),
		0xf702 => Some(WXK_LEFT),
		0xf703 => Some(WXK_RIGHT),
		0xf704..=0xf71b => Some(i32::from(character - 0xf704) + WXK_F1),
		0xf728 => Some(WXK_DELETE),
		0xf729 => Some(WXK_HOME),
		0xf72b => Some(WXK_END),
		0xf72c => Some(WXK_PAGEUP),
		0xf72d => Some(WXK_PAGEDOWN),
		_ => {
			let character = char::from_u32(u32::from(character))?;
			character.is_ascii().then(|| character.to_ascii_uppercase() as i32)
		}
	}
}

fn event_key(event: *mut Object) -> Option<(usize, i32)> {
	// NSEvents and their NSStrings are borrowed for the duration of this UI-thread callback.
	unsafe {
		let modifiers: usize = msg_send![event, modifierFlags];
		let characters: *mut Object = msg_send![event, charactersIgnoringModifiers];
		let length: usize = msg_send![characters, length];
		if length == 1 {
			let character: u16 = msg_send![characters, characterAtIndex: 0_usize];
			// Preserve AppKit's special codes; applying modifiers turns some into control characters.
			if character == 0x7f || (0xf700..=0xf8ff).contains(&character) {
				return Some((modifiers, wx_key_code(character)?));
			}
		}
		// Remove Shift as well, so Shift+/ matches the base '/' key and a separate Shift flag.
		let characters: *mut Object = msg_send![event, charactersByApplyingModifiers: 0_usize];
		let length: usize = msg_send![characters, length];
		if length != 1 {
			return None;
		}
		let character: u16 = msg_send![characters, characterAtIndex: 0_usize];
		Some((modifiers, wx_key_code(character)?))
	}
}

fn shortcut_action(shortcuts: &ShortcutsConfig, key: i32, modifiers: usize) -> Option<ActionId> {
	ActionId::all().iter().copied().find(|&action| {
		shortcuts.get_chord(action).is_some_and(|chord| {
			// Plain Ctrl in the config means Command on macOS and RawCtrl means physical Control; test both so VoiceOver's Control+Option commands pass through unless that exact chord was assigned.
			chord.ctrl == (modifiers & COMMAND != 0)
				&& chord.raw_ctrl == (modifiers & CONTROL != 0)
				&& chord.matches(key, chord.ctrl || chord.raw_ctrl, modifiers & OPTION != 0, modifiers & SHIFT != 0)
		})
	})
}

pub fn install(frame: Frame, config: Rc<Mutex<ConfigManager>>, from_keyboard: Rc<Cell<bool>>) {
	let monitor = ConcreteBlock::new(move |event: *mut Object| -> *mut Object {
		if !frame.is_valid() || !frame.is_enabled() {
			return event;
		}
		let view = frame.get_handle().cast::<Object>();
		let (window, event_window, modal_window): (*mut Object, *mut Object, *mut Object) = unsafe {
			let app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
			(msg_send![view, window], msg_send![event, window], msg_send![app, modalWindow])
		};
		// Child controls belong to the frame's NSWindow too. Dialogs and other windows do not.
		if window.is_null() || window != event_window || !modal_window.is_null() {
			return event;
		}
		let Some((modifiers, key)) = event_key(event) else { return event };
		let action = config.try_lock().ok().and_then(|cfg| shortcut_action(&cfg.get_shortcuts(), key, modifiers));
		let Some(action) = action else { return event };
		// Percentage announcements have no menu item and already run in the reader's key handler.
		if action == ActionId::AnnouncePercent {
			return event;
		}
		let id = if action == ActionId::Options { menu_ids::PREFERENCES } else { menu_ids::action_to_menu_id(action) };
		let item = frame.get_menu_bar().and_then(|bar| bar.find_item(id));
		if !item.is_some_and(|item| item.is_enabled()) {
			return event;
		}
		// Synchronous dispatch keeps the keyboard mark from leaking to a later menu click.
		from_keyboard.set(true);
		// wx callbacks default to Skip(true), so this can return false even after our dispatcher ran; the matched command must still consume the native key.
		frame.process_menu_command(id);
		from_keyboard.set(false);
		ptr::null_mut()
	})
	.copy();
	let token: *mut Object = unsafe {
		let token: *mut Object =
			msg_send![class!(NSEvent), addLocalMonitorForEventsMatchingMask: KEY_DOWN handler: &*monitor];
		let _: *mut Object = msg_send![token, retain];
		token
	};
	let frame_handle = frame.handle_ptr();
	frame.bind_internal(EventType::DESTROY, move |event| {
		// Destroy events bubble from children too: closing a tab must not remove this monitor.
		if event.get_event_object().is_some_and(|object| object.as_ptr() == frame_handle) {
			unsafe {
				let _: () = msg_send![class!(NSEvent), removeMonitor: token];
				let _: () = msg_send![token, release];
			}
		}
		event.skip(true);
	});
}

#[cfg(test)]
mod tests {
	use paperback_core::config::KeyChord;

	use super::*;

	#[test]
	fn native_shortcuts_match_command_control_and_shift_separately() {
		let shortcuts = ShortcutsConfig::default();
		assert_eq!(shortcut_action(&shortcuts, 'G' as i32, COMMAND), Some(ActionId::FindNext));
		assert_eq!(shortcut_action(&shortcuts, 'G' as i32, COMMAND | SHIFT), Some(ActionId::FindPrevious));
		assert_eq!(shortcut_action(&shortcuts, 'G' as i32, CONTROL), None);
		assert_eq!(shortcut_action(&shortcuts, 'H' as i32, 0), Some(ActionId::NextHeading));
		assert_eq!(shortcut_action(&shortcuts, 'H' as i32, SHIFT), Some(ActionId::PreviousHeading));
		assert_eq!(shortcut_action(&shortcuts, 'H' as i32, CONTROL | OPTION), None);
		assert_eq!(shortcut_action(&shortcuts, WXK_SPACE, CONTROL), Some(ActionId::PlayPauseAudio));
		assert_eq!(shortcut_action(&shortcuts, WXK_SPACE, COMMAND), None);
		assert_eq!(shortcut_action(&shortcuts, 'F' as i32, CONTROL | COMMAND), Some(ActionId::ToggleFullScreen));
		assert_eq!(shortcut_action(&shortcuts, 'F' as i32, COMMAND), Some(ActionId::Find));
	}

	#[test]
	fn native_shortcuts_follow_custom_bindings_and_unbound_actions() {
		let mut shortcuts = ShortcutsConfig::default();
		shortcuts.set_chord(ActionId::NextHeading, None);
		assert_eq!(shortcut_action(&shortcuts, 'H' as i32, 0), None);
		shortcuts.set_chord(ActionId::NextHeading, Some(KeyChord::new(true, false, true, "/")));
		assert_eq!(shortcut_action(&shortcuts, '/' as i32, COMMAND | SHIFT), Some(ActionId::NextHeading));
		assert_eq!(shortcut_action(&shortcuts, '/' as i32, COMMAND), None);
	}
}
