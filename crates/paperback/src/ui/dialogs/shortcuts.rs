#[cfg(target_os = "windows")]
use paperback_core::config::default_window_hotkey;
use paperback_core::config::{ActionId, KeyChord, ShortcutCategory, ShortcutsConfig};
#[cfg(target_os = "windows")]
use patois::t;
use wx_utils::shortcuts::{ShortcutModel, TabScope};
use wxdragon::prelude::*;

/// Everything the Keyboard Shortcuts dialog edits: the menu commands, and the window hotkey, which only Windows registers.
#[derive(Clone)]
pub struct Keymap {
	pub shortcuts: ShortcutsConfig,
	pub window_hotkey: Option<KeyChord>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shortcut {
	Command(ActionId),
	#[cfg(target_os = "windows")]
	WindowHotkey,
}

/// Lookups ignore `tab` for commands, since categories only decide where an action is listed.
#[derive(Clone)]
struct CategorizedShortcuts(Keymap);

impl CategorizedShortcuts {
	/// The system-wide tab comes after every category.
	#[cfg(target_os = "windows")]
	fn is_global(tab: usize) -> bool {
		tab == ShortcutCategory::all().len()
	}

	#[cfg(not(target_os = "windows"))]
	const fn is_global(_tab: usize) -> bool {
		false
	}
}

impl ShortcutModel for CategorizedShortcuts {
	type Action = Shortcut;

	fn tabs(&self) -> Vec<String> {
		// TRANSLATORS: Tab label in the Keyboard Shortcuts dialog naming a category of shortcuts (e.g. Navigation, File).
		let tabs = ShortcutCategory::all().iter().map(|c| c.display_name());
		// TRANSLATORS: Tab in the Keyboard Shortcuts dialog for shortcuts that work in every program, not only while Paperback has focus
		#[cfg(target_os = "windows")]
		let tabs = tabs.chain([t("System-Wide")]);
		tabs.collect()
	}

	fn tab_scope(&self, tab: usize) -> TabScope {
		// Every category views one set of bindings, so a chord must be unique across all of them.
		if Self::is_global(tab) { TabScope::Global } else { TabScope::Mode(0) }
	}

	fn actions(&self, tab: usize) -> Vec<Shortcut> {
		#[cfg(target_os = "windows")]
		if Self::is_global(tab) {
			return vec![Shortcut::WindowHotkey];
		}
		ShortcutCategory::all()
			.get(tab)
			.map(|c| c.actions().into_iter().map(Shortcut::Command).collect())
			.unwrap_or_default()
	}

	fn action_name(&self, action: Shortcut) -> String {
		match action {
			Shortcut::Command(action) => action.display_name(),
			// TRANSLATORS: The window hotkey's row in the Keyboard Shortcuts dialog; it brings Paperback to the front, or hides it again
			#[cfg(target_os = "windows")]
			Shortcut::WindowHotkey => t("Show or Hide Paperback"),
		}
	}

	fn chord(&self, _tab: usize, action: Shortcut) -> Option<KeyChord> {
		match action {
			Shortcut::Command(action) => self.0.shortcuts.get_chord(action),
			#[cfg(target_os = "windows")]
			Shortcut::WindowHotkey => self.0.window_hotkey.clone(),
		}
	}

	fn set_chord(&mut self, _tab: usize, action: Shortcut, chord: Option<KeyChord>) {
		match action {
			Shortcut::Command(action) => self.0.shortcuts.set_chord(action, chord),
			#[cfg(target_os = "windows")]
			Shortcut::WindowHotkey => self.0.window_hotkey = chord,
		}
	}

	fn reset_action(&mut self, _tab: usize, action: Shortcut) {
		match action {
			Shortcut::Command(action) => self.0.shortcuts.reset_action(action),
			#[cfg(target_os = "windows")]
			Shortcut::WindowHotkey => self.0.window_hotkey = Some(default_window_hotkey()),
		}
	}

	fn reset_all(&mut self, tab: usize) {
		#[cfg(target_os = "windows")]
		if Self::is_global(tab) {
			self.0.window_hotkey = Some(default_window_hotkey());
			return;
		}
		let _ = tab;
		self.0.shortcuts.reset_all();
	}
}

pub fn prompt_for_shortcuts(parent: &dyn WxWidget, initial: &Keymap) -> Option<Keymap> {
	wx_utils::shortcuts::prompt_for_shortcuts(parent, &CategorizedShortcuts(initial.clone())).map(|updated| updated.0)
}
