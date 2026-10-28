use super::Ctx;
use crate::ui::selection;

pub fn set_start(ctx: &Ctx) {
	selection::handle_set_selection_start(ctx.dm, ctx.live_region_label, ctx.from_keyboard);
}

pub fn copy_from_start(ctx: &Ctx) {
	selection::handle_copy_from_selection_start(ctx.dm, ctx.live_region_label, ctx.from_keyboard);
}

pub fn jump_to_start(ctx: &Ctx) {
	selection::handle_jump_to_selection_start(ctx.dm, ctx.config, ctx.live_region_label, ctx.from_keyboard);
}
