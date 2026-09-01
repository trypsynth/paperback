//! Bookmarks and notes: setting them, stepping between them, and the dialogs that list them.

use paperback_core::types::BookmarkFilterType;

use super::Ctx;
use crate::ui::navigation;

pub fn previous_bookmark(ctx: &Ctx) {
	navigation::handle_bookmark_navigation(ctx.dm, ctx.config, ctx.live_region_label, false, false);
}

pub fn next_bookmark(ctx: &Ctx) {
	navigation::handle_bookmark_navigation(ctx.dm, ctx.config, ctx.live_region_label, true, false);
}

pub fn previous_note(ctx: &Ctx) {
	navigation::handle_bookmark_navigation(ctx.dm, ctx.config, ctx.live_region_label, false, true);
}

pub fn next_note(ctx: &Ctx) {
	navigation::handle_bookmark_navigation(ctx.dm, ctx.config, ctx.live_region_label, true, true);
}

pub fn jump_to_all(ctx: &Ctx) {
	navigation::handle_bookmark_dialog(ctx.frame, ctx.dm, ctx.config, ctx.live_region_label, BookmarkFilterType::All);
}

pub fn jump_to_bookmarks_only(ctx: &Ctx) {
	navigation::handle_bookmark_dialog(
		ctx.frame,
		ctx.dm,
		ctx.config,
		ctx.live_region_label,
		BookmarkFilterType::BookmarksOnly,
	);
}

pub fn jump_to_notes_only(ctx: &Ctx) {
	navigation::handle_bookmark_dialog(
		ctx.frame,
		ctx.dm,
		ctx.config,
		ctx.live_region_label,
		BookmarkFilterType::NotesOnly,
	);
}

pub fn view_note_text(ctx: &Ctx) {
	navigation::handle_view_note_text(ctx.frame, ctx.dm, ctx.config);
}

pub fn toggle(ctx: &Ctx) {
	navigation::handle_toggle_bookmark(ctx.dm, ctx.config, ctx.live_region_label);
}

pub fn with_note(ctx: &Ctx) {
	navigation::handle_bookmark_with_note(ctx.frame, ctx.dm, ctx.config, ctx.live_region_label);
}
