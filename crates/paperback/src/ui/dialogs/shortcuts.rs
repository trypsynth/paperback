use paperback_core::config::{ActionId, KeyChord, ShortcutCategory, ShortcutsConfig};
use wx_utils::shortcuts::{ShortcutModel, TabScope};
use wxdragon::prelude::*;

/// A newtype because the trait and `ShortcutsConfig` are both foreign; lookups ignore `tab`, since categories only decide where an action is listed.
#[derive(Clone)]
struct CategorizedShortcuts(ShortcutsConfig);

impl ShortcutModel for CategorizedShortcuts {
	type Action = ActionId;

	fn tabs(&self) -> Vec<String> {
		// TRANSLATORS: Tab label in the Keyboard Shortcuts dialog naming a category of shortcuts (e.g. Navigation, File).
		ShortcutCategory::all().iter().map(|c| c.display_name()).collect()
	}

	fn tab_scope(&self, _tab: usize) -> TabScope {
		// Every category views one set of bindings, so a chord must be unique across all tabs.
		TabScope::Mode(0)
	}

	fn actions(&self, tab: usize) -> Vec<ActionId> {
		ShortcutCategory::all().get(tab).map(|c| c.actions()).unwrap_or_default()
	}

	fn action_name(&self, action: ActionId) -> String {
		action.display_name()
	}

	fn chord(&self, _tab: usize, action: ActionId) -> Option<KeyChord> {
		self.0.get_chord(action)
	}

	fn set_chord(&mut self, _tab: usize, action: ActionId, chord: Option<KeyChord>) {
		self.0.set_chord(action, chord);
	}

	fn reset_action(&mut self, _tab: usize, action: ActionId) {
		self.0.reset_action(action);
	}

	fn reset_all(&mut self, _tab: usize) {
		self.0.reset_all();
	}
}

pub fn prompt_for_shortcuts(parent: &dyn WxWidget, initial: &ShortcutsConfig) -> Option<ShortcutsConfig> {
	let model = CategorizedShortcuts(initial.clone());
	wx_utils::shortcuts::prompt_for_shortcuts(parent, &model).map(|updated| updated.0)
}
