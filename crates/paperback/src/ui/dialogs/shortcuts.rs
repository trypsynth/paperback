//! Paperback's side of the shared Customize Keyboard Shortcuts dialog.
//!
//! The dialog itself lives in `wx_utils::shortcuts`; all that is needed here is to describe
//! Paperback's keymap to it. Every category is a view onto one set of bindings, so a chord has
//! to be unique across all of them, which is what `TabKind::SharedKeymap` says.

use paperback_core::config::{ActionId, KeyChord, ShortcutCategory, ShortcutsConfig};
use wx_utils::shortcuts::{ShortcutModel, TabKind};
use wxdragon::prelude::*;

/// A [`ShortcutsConfig`] presented as tabs, one per [`ShortcutCategory`].
///
/// A newtype because both the trait and `ShortcutsConfig` are foreign to this crate. `tab` is an
/// index into [`ShortcutCategory::all`], and the lookups ignore it: the categories only decide
/// which tab an action is listed under, not where its binding is stored.
#[derive(Clone)]
struct CategorizedShortcuts(ShortcutsConfig);

impl ShortcutModel for CategorizedShortcuts {
	type Action = ActionId;

	fn tabs(&self) -> Vec<String> {
		// TRANSLATORS: Tab label in the Keyboard Shortcuts dialog naming a category of shortcuts (e.g. Navigation, File).
		ShortcutCategory::all().iter().map(|c| c.display_name()).collect()
	}

	fn tab_kind(&self) -> TabKind {
		TabKind::SharedKeymap
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
