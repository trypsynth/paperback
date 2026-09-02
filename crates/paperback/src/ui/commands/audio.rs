//! Playing a document's recorded narration, and the size of a seek through it.

use super::Ctx;
use crate::ui::audio;

pub fn toggle_play_pause(ctx: &Ctx) {
	audio::handle_toggle_play_pause_audio(ctx.dm, ctx.live_region_label);
}

pub fn seek_forward(ctx: &Ctx) {
	audio::handle_seek_audio(ctx.dm, ctx.config, ctx.live_region_label, true);
}

pub fn seek_backward(ctx: &Ctx) {
	audio::handle_seek_audio(ctx.dm, ctx.config, ctx.live_region_label, false);
}

pub fn increase_seek_amount(ctx: &Ctx) {
	audio::handle_change_seek_amount(ctx.config, ctx.live_region_label, true);
}

pub fn decrease_seek_amount(ctx: &Ctx) {
	audio::handle_change_seek_amount(ctx.config, ctx.live_region_label, false);
}
