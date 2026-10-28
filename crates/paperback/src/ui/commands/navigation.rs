use super::Ctx;
use crate::ui::navigation;

pub fn go_back(ctx: &Ctx) {
	navigation::handle_history_navigation(ctx.dm, ctx.config, ctx.live_region_label, false, ctx.from_keyboard);
}

pub fn go_forward(ctx: &Ctx) {
	navigation::handle_history_navigation(ctx.dm, ctx.config, ctx.live_region_label, true, ctx.from_keyboard);
}

pub fn container_start(ctx: &Ctx) {
	navigation::handle_container_navigation(ctx.dm, ctx.config, ctx.live_region_label, false, ctx.from_keyboard);
}

pub fn container_end(ctx: &Ctx) {
	navigation::handle_container_navigation(ctx.dm, ctx.config, ctx.live_region_label, true, ctx.from_keyboard);
}
