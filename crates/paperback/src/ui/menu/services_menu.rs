//! The Services menu: online services Paperback works with, each in a submenu of its own, starting with Scribe for Documents.

use patois::t;

use super::builder::{MenuEntry, item, submenu};
use crate::ui::menu_ids;

pub fn entries() -> Vec<MenuEntry> {
	// TRANSLATORS: Item in the Scribe submenu of the Services menu that uploads a document to Scribe to convert
	let convert = item(menu_ids::CONVERT_WITH_SCRIBE, t("&Convert Document..."));
	// TRANSLATORS: Submenu of the Services menu for Scribe for Documents, Pneuma Solutions' document conversion service
	vec![submenu(t("&Scribe"), String::new(), vec![convert])]
}
