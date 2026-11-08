use paperback_core::speech::Voice;
use patois::t;
use wxdragon::prelude::*;

use super::{add_ok_cancel_footer, build_ok_cancel_buttons, dialog_padding};

/// What the Speech Settings dialog edits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeechValues {
	/// A [`Voice::id`], or `None` for the system's default voice.
	pub voice: Option<String>,
	/// From 0 to 100, where 50 is the voice's normal.
	pub rate: i32,
	pub pitch: i32,
	pub paragraph_pause_ms: i32,
	/// Voice, rate and pitch belong to the open book alone, rather than to every book.
	pub this_book_only: bool,
}

/// `has_book` enables the checkbox for keeping the voice to the open book, and `play_sample` speaks a sample with the voice, rate and pitch the dialog shows. Returns the values chosen, or `None` if cancelled.
pub fn show_speech_settings_dialog(
	parent: &Frame,
	voices: &[Voice],
	initial: &SpeechValues,
	has_book: bool,
	play_sample: impl Fn(Option<String>, i32, i32) + 'static,
) -> Option<SpeechValues> {
	// TRANSLATORS: Title of the dialog for choosing the voice, rate and pauses used to read aloud
	let dialog = Dialog::builder(parent, &t("Speech Settings")).build();
	let padding = dialog_padding(&dialog);
	let mut voices: Vec<&Voice> = voices.iter().collect();
	voices.sort_by_cached_key(|voice| (language_label(&voice.language), voice.name.clone()));
	// TRANSLATORS: Label of the list of voices in the Speech Settings dialog
	let voice_label = StaticText::builder(&dialog).with_label(&t("&Voice:")).build();
	let voice_choice = Choice::builder(&dialog).build();
	// TRANSLATORS: First choice in the Speech Settings dialog's voice list, which leaves the choice of voice to the system
	voice_choice.append(&t("System default"));
	for voice in &voices {
		voice_choice.append(&format!("{}, {}", voice.name, language_label(&voice.language)));
	}
	let selected = initial
		.voice
		.as_ref()
		.and_then(|id| voices.iter().position(|voice| voice.id == *id))
		.map_or(0, |index| index + 1);
	voice_choice.set_selection(u32::try_from(selected).unwrap_or(0));
	let ids: Vec<String> = voices.iter().map(|voice| voice.id.clone()).collect();
	let chosen_voice = move || {
		let index = usize::try_from(voice_choice.get_selection().unwrap_or(0)).unwrap_or(0);
		index.checked_sub(1).and_then(|index| ids.get(index)).cloned()
	};
	let slider = |value: i32| {
		Slider::builder(&dialog).with_value(value.clamp(0, 100)).with_min_value(0).with_max_value(100).build()
	};
	// TRANSLATORS: Label of the speech rate slider in the Speech Settings dialog
	let rate_label = StaticText::builder(&dialog).with_label(&t("&Rate:")).build();
	let rate = slider(initial.rate);
	// TRANSLATORS: Label of the speech pitch slider in the Speech Settings dialog
	let pitch_label = StaticText::builder(&dialog).with_label(&t("&Pitch:")).build();
	let pitch = slider(initial.pitch);
	// TRANSLATORS: Label of the silence added after each paragraph read aloud, in milliseconds
	let pause_label =
		StaticText::builder(&dialog).with_label(&t("Pause after each p&aragraph (milliseconds):")).build();
	let pause = SpinCtrl::builder(&dialog).with_range(0, 1000).build();
	pause.set_value(initial.paragraph_pause_ms.clamp(0, 1000));
	// TRANSLATORS: Check box in the Speech Settings dialog that keeps the voice, rate and pitch to the open book
	let this_book_only =
		CheckBox::builder(&dialog).with_label(&t("Use this voice, rate and pitch for this &book only")).build();
	this_book_only.set_value(has_book && initial.this_book_only);
	this_book_only.enable(has_book);
	// TRANSLATORS: Button in the Speech Settings dialog that speaks a sample with the chosen voice, rate and pitch
	let sample_button = Button::builder(&dialog).with_label(&t("Play &Sample")).build();
	let sample_voice = chosen_voice.clone();
	sample_button.on_click(move |_| play_sample(sample_voice(), rate.get_value(), pitch.get_value()));
	let content = BoxSizer::builder(Orientation::Vertical).build();
	add_row(content, voice_label, &voice_choice, padding);
	add_row(content, rate_label, &rate, padding);
	add_row(content, pitch_label, &pitch, padding);
	add_row(content, pause_label, &pause, padding);
	content.add(&this_book_only, 0, SizerFlag::Left | SizerFlag::Right, padding);
	content.add(&sample_button, 0, SizerFlag::All, padding);
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("OK"));
	add_ok_cancel_footer(content, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content, true);
	dialog.centre();
	voice_choice.set_focus();
	let chosen = (dialog.show_modal() == ID_OK).then(|| SpeechValues {
		voice: chosen_voice(),
		rate: rate.get_value(),
		pitch: pitch.get_value(),
		paragraph_pause_ms: pause.value(),
		this_book_only: has_book && this_book_only.is_checked(),
	});
	dialog.destroy();
	chosen
}

fn add_row(content: BoxSizer, label: StaticText, control: &impl WxWidget, padding: i32) {
	let row = BoxSizer::builder(Orientation::Horizontal).build();
	row.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, padding);
	row.add(control, 1, SizerFlag::AlignCenterVertical, 0);
	content.add_sizer(&row, 0, SizerFlag::Expand | SizerFlag::All, padding / 2);
}

/// A voice's language as people know it, with its tag after it, since one language can have voices for several countries.
fn language_label(tag: &str) -> String {
	let base = tag.split(['-', '_']).next().unwrap_or(tag);
	format!("{} ({tag})", patois::language_name(base))
}
