//! Run app shortcuts before Cocoa selects their menu items and `VoiceOver` reads their labels.

use std::{cell::Cell, ptr, ptr::NonNull, rc::Rc, sync::Mutex};

use block2::RcBlock;
use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSApplication, NSEvent, NSEventMask, NSEventModifierFlags, NSView, NSWindow};
use paperback_core::config::{ActionId, ConfigManager, KeyChord};
use wxdragon::prelude::*;

use crate::ui::menu_ids;

// NSEventModifierFlags, from AppKit.
const SHIFT: usize = 1 << 17;
const CONTROL: usize = 1 << 18;
const OPTION: usize = 1 << 19;
const COMMAND: usize = 1 << 20;

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

/// The event is a key event, as the monitor only asks for key downs.
fn event_key(event: &NSEvent) -> Option<(usize, i32)> {
	let modifiers = event.modifierFlags().0;
	let characters = event.charactersIgnoringModifiers()?;
	if characters.length() == 1 {
		let character = characters.characterAtIndex(0);
		// Preserve AppKit's special codes; applying modifiers turns some into control characters.
		if character == 0x7f || (0xf700..=0xf8ff).contains(&character) {
			return Some((modifiers, wx_key_code(character)?));
		}
	}
	// Remove Shift as well, so Shift+/ matches the base '/' key and a separate Shift flag.
	let characters = event.charactersByApplyingModifiers(NSEventModifierFlags(0))?;
	if characters.length() != 1 {
		return None;
	}
	Some((modifiers, wx_key_code(characters.characterAtIndex(0))?))
}

// Plain Ctrl in the config means Command on macOS and RawCtrl means physical Control; test both so VoiceOver's Control+Option commands pass through unless that exact chord was assigned.
fn chord_matches(chord: &KeyChord, key: i32, modifiers: usize) -> bool {
	chord.ctrl == (modifiers & COMMAND != 0)
		&& chord.raw_ctrl == (modifiers & CONTROL != 0)
		&& chord.matches(key, chord.ctrl || chord.raw_ctrl, modifiers & OPTION != 0, modifiers & SHIFT != 0)
}

pub fn install(
	frame: Frame,
	config: Rc<Mutex<ConfigManager>>,
	from_keyboard: Rc<Cell<bool>>,
	reader_has_focus: impl Fn() -> bool + 'static,
) {
	let Some(main_thread) = MainThreadMarker::new() else { return };
	let monitor = RcBlock::new(move |event_ptr: NonNull<NSEvent>| -> *mut NSEvent {
		let unchanged = event_ptr.as_ptr();
		if !frame.is_valid() || !frame.is_enabled() {
			return unchanged;
		}
		// AppKit lends the event for the duration of this UI-thread callback, and wxWidgets owns the frame's view while it's valid.
		let (event, view) = unsafe { (event_ptr.as_ref(), &*frame.get_handle().cast::<NSView>()) };
		let address = |window: Option<Retained<NSWindow>>| window.map(|window| Retained::as_ptr(&window));
		let window = address(view.window());
		// Child controls belong to the frame's NSWindow too. Dialogs and other windows do not.
		if window.is_none()
			|| window != address(event.window(main_thread))
			|| NSApplication::sharedApplication(main_thread).modalWindow().is_some()
		{
			return unchanged;
		}
		let Some((modifiers, key)) = event_key(event) else { return unchanged };
		let action =
			config.try_lock().ok().and_then(|cfg| cfg.find_shortcut(|chord| chord_matches(chord, key, modifiers)));
		// Percentage announcements have no menu item and already run in the reader's key handler.
		if action == Some(ActionId::AnnouncePercent) {
			return unchanged;
		}
		let copy_key = key == 'C' as i32 && modifiers & (COMMAND | CONTROL | OPTION | SHIFT) == COMMAND;
		let id = if let Some(action) = action {
			if action == ActionId::Options { menu_ids::PREFERENCES } else { menu_ids::action_to_menu_id(action) }
		} else if copy_key && reader_has_focus() {
			// Copy is a stock wx command outside ActionId; catch it before Cocoa reads "Copy" so the app announces the character count instead.
			menu_ids::COPY
		} else {
			return unchanged;
		};
		let item = frame.get_menu_bar().and_then(|bar| bar.find_item(id));
		if !item.is_some_and(|item| item.is_enabled()) {
			return unchanged;
		}
		// Synchronous dispatch keeps the keyboard mark from leaking to a later menu click.
		from_keyboard.set(true);
		// wx callbacks default to Skip(true), so this can return false even after our dispatcher ran; the matched command must still consume the native key.
		frame.process_menu_command(id);
		from_keyboard.set(false);
		ptr::null_mut()
	});
	// The handler only runs on the main thread, where AppKit delivers local events.
	let Some(token) =
		(unsafe { NSEvent::addLocalMonitorForEventsMatchingMask_handler(NSEventMask::KeyDown, &monitor) })
	else {
		return;
	};
	let frame_handle = frame.handle_ptr();
	frame.bind_internal(EventType::DESTROY, move |event| {
		// Destroy events bubble from children too: closing a tab must not remove this monitor. The raw pointer, since `get_event_object` gives a window being deleted a null one.
		if event.event_object_ptr() == frame_handle {
			// The token is the one adding the monitor returned.
			unsafe { NSEvent::removeMonitor(&token) };
		}
		event.skip(true);
	});
}

#[cfg(test)]
mod tests {
	use paperback_core::config::ShortcutsConfig;

	use super::*;

	fn shortcut_action(shortcuts: &ShortcutsConfig, key: i32, modifiers: usize) -> Option<ActionId> {
		shortcuts.chords().into_iter().find(|(_, chord)| chord_matches(chord, key, modifiers)).map(|(action, _)| action)
	}

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
