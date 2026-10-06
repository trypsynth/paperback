//! Playing a document's recorded narration, and the size of a seek through it.

use super::Ctx;
use crate::ui::audio;

pub fn toggle_play_pause(ctx: &Ctx) {
	audio::handle_toggle_play_pause_audio(ctx.dm, ctx.live_region_label, ctx.from_keyboard);
}

pub fn seek_forward(ctx: &Ctx) {
	audio::handle_seek_audio(ctx.dm, ctx.config, ctx.live_region_label, true, ctx.from_keyboard);
}

pub fn seek_backward(ctx: &Ctx) {
	audio::handle_seek_audio(ctx.dm, ctx.config, ctx.live_region_label, false, ctx.from_keyboard);
}

pub fn increase_seek_amount(ctx: &Ctx) {
	audio::handle_change_seek_amount(ctx.config, ctx.live_region_label, true, ctx.from_keyboard);
}

pub fn decrease_seek_amount(ctx: &Ctx) {
	audio::handle_change_seek_amount(ctx.config, ctx.live_region_label, false, ctx.from_keyboard);
}

pub fn increase_speed(ctx: &Ctx) {
	audio::handle_change_audio_speed(ctx.dm, ctx.live_region_label, true, ctx.from_keyboard);
}

pub fn decrease_speed(ctx: &Ctx) {
	audio::handle_change_audio_speed(ctx.dm, ctx.live_region_label, false, ctx.from_keyboard);
}
