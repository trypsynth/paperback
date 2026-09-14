//! Copying a lot of text without shift-arrowing through it: marking where a selection begins,
//! then taking everything from there to wherever the reader has got to.

use super::Ctx;
use crate::ui::selection;

pub fn set_start(ctx: &Ctx) {
	selection::handle_set_selection_start(ctx.dm, ctx.live_region_label);
}

pub fn copy_from_start(ctx: &Ctx) {
	selection::handle_copy_from_selection_start(ctx.dm, ctx.live_region_label);
}
