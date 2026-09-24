//! Bookmarks and notes: setting them, stepping between them, and the dialogs that list them.

use paperback_core::types::BookmarkFilterType;

use super::Ctx;
use crate::ui::bookmarks;

pub fn previous_bookmark(ctx: &Ctx) {
	bookmarks::handle_bookmark_navigation(ctx.dm, ctx.config, ctx.live_region_label, false, false, ctx.from_keyboard);
}

pub fn next_bookmark(ctx: &Ctx) {
	bookmarks::handle_bookmark_navigation(ctx.dm, ctx.config, ctx.live_region_label, true, false, ctx.from_keyboard);
}

pub fn previous_note(ctx: &Ctx) {
	bookmarks::handle_bookmark_navigation(ctx.dm, ctx.config, ctx.live_region_label, false, true, ctx.from_keyboard);
}

pub fn next_note(ctx: &Ctx) {
	bookmarks::handle_bookmark_navigation(ctx.dm, ctx.config, ctx.live_region_label, true, true, ctx.from_keyboard);
}

pub fn jump_to_all(ctx: &Ctx) {
	bookmarks::handle_bookmark_dialog(ctx.frame, ctx.dm, ctx.config, ctx.live_region_label, BookmarkFilterType::All);
}

pub fn jump_to_bookmarks_only(ctx: &Ctx) {
	bookmarks::handle_bookmark_dialog(
		ctx.frame,
		ctx.dm,
		ctx.config,
		ctx.live_region_label,
		BookmarkFilterType::BookmarksOnly,
	);
}

pub fn jump_to_notes_only(ctx: &Ctx) {
	bookmarks::handle_bookmark_dialog(
		ctx.frame,
		ctx.dm,
		ctx.config,
		ctx.live_region_label,
		BookmarkFilterType::NotesOnly,
	);
}

pub fn view_note_text(ctx: &Ctx) {
	bookmarks::handle_view_note_text(ctx.frame, ctx.dm, ctx.config);
}

pub fn toggle(ctx: &Ctx) {
	bookmarks::handle_toggle_bookmark(ctx.dm, ctx.config, ctx.live_region_label, ctx.from_keyboard);
}

pub fn with_note(ctx: &Ctx) {
	bookmarks::handle_bookmark_with_note(ctx.frame, ctx.dm, ctx.config, ctx.live_region_label, ctx.from_keyboard);
}
