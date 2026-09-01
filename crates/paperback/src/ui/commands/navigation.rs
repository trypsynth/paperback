//! Moving through the document by something other than an element: the reading history, and
//! the ends of the list or table the reader is inside.

use super::Ctx;
use crate::ui::navigation;

pub fn go_back(ctx: &Ctx) {
	navigation::handle_history_navigation(ctx.dm, ctx.config, ctx.live_region_label, false);
}

pub fn go_forward(ctx: &Ctx) {
	navigation::handle_history_navigation(ctx.dm, ctx.config, ctx.live_region_label, true);
}

pub fn container_start(ctx: &Ctx) {
	navigation::handle_container_navigation(ctx.dm, ctx.config, ctx.live_region_label, false);
}

pub fn container_end(ctx: &Ctx) {
	navigation::handle_container_navigation(ctx.dm, ctx.config, ctx.live_region_label, true);
}
