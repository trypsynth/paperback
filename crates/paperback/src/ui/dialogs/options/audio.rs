//! The Audio tab: playing audio books and narration, and the voice that reads aloud.

use std::{cell::RefCell, rc::Rc};

use paperback_core::{
	config::{ConfigManager, DocumentSpeech},
	speech::Voice,
};
use patois::t;
use wxdragon::prelude::*;

use super::{OPTION_PADDING, labeled_row};
use crate::read_aloud;

/// Indexed by the seek amount choice's selection and also stepped through by `audio::handle_change_seek_amount`, so it must match the choices in the tab.
pub(crate) const AUDIO_SEEK_AMOUNTS_SECONDS: [i32; 9] = [5, 10, 30, 60, 120, 300, 600, 1800, 3600];

const DEFAULT_SEEK_AMOUNT_SECONDS: i32 = 10;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AudioSettings {
	pub bookmark_sounds: bool,
	pub sync_caret_to_audio: bool,
	pub audio_seek_amount_seconds: i32,
	pub audio_seek_continues_into_next_file: bool,
	/// `None` where read aloud has no engine yet.
	pub speech: Option<SpeechSettings>,
}

impl AudioSettings {
	/// `book` is the open document, which can keep a voice of its own.
	pub fn load(config: &ConfigManager, book: Option<&str>) -> Self {
		Self {
			bookmark_sounds: config.get_app_bool("bookmark_sounds", true),
			sync_caret_to_audio: config.get_app_bool("sync_caret_to_audio", true),
			audio_seek_amount_seconds: config.get_app_int("audio_seek_amount_seconds", DEFAULT_SEEK_AMOUNT_SECONDS),
			audio_seek_continues_into_next_file: config.get_app_bool("audio_seek_continues_into_next_file", false),
			speech: SpeechSettings::load(config, book),
		}
	}

	pub fn save(&self, config: &ConfigManager, book: Option<&str>) {
		config.set_app_bool("bookmark_sounds", self.bookmark_sounds);
		config.set_app_bool("sync_caret_to_audio", self.sync_caret_to_audio);
		config.set_app_int("audio_seek_amount_seconds", self.audio_seek_amount_seconds);
		config.set_app_bool("audio_seek_continues_into_next_file", self.audio_seek_continues_into_next_file);
		if let Some(speech) = &self.speech {
			speech.save(config, book);
		}
	}
}

/// The voice that reads aloud, kept for each engine since each has voices and a range of its own.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpeechSettings {
	/// The speech engine, such as `onecore`.
	pub engine: String,
	/// A [`Voice::id`], or `None` for the system's default voice.
	pub voice: Option<String>,
	/// From 0 to 100, where 50 is the voice's normal.
	pub rate: i32,
	pub pitch: i32,
	pub paragraph_pause_ms: i32,
	/// Voice, rate and pitch belong to the open book alone, rather than to every book.
	pub this_book_only: bool,
}

impl SpeechSettings {
	fn load(config: &ConfigManager, book: Option<&str>) -> Option<Self> {
		let engines = read_aloud::engines();
		let stored = config.get_app_string("tts_engine", "");
		let engine = engines.iter().find(|(id, _)| *id == stored).or_else(|| engines.first())?.0.to_owned();
		let (voice_key, rate_key, pitch_key) = keys(&engine);
		let own = book.map(|path| config.get_document_speech(path)).unwrap_or_default();
		let app_voice = config.get_app_string(&voice_key, "");
		Some(Self {
			this_book_only: own.voice.is_some() || own.rate_percent.is_some() || own.pitch_percent.is_some(),
			voice: own.voice.clone().or_else(|| Some(app_voice).filter(|voice| !voice.is_empty())),
			rate: own.rate_percent.unwrap_or_else(|| config.get_app_int(&rate_key, 50)),
			pitch: own.pitch_percent.unwrap_or_else(|| config.get_app_int(&pitch_key, 50)),
			paragraph_pause_ms: config.get_app_int("tts_paragraph_pause_ms", 0),
			engine,
		})
	}

	fn save(&self, config: &ConfigManager, book: Option<&str>) {
		let (voice_key, rate_key, pitch_key) = keys(&self.engine);
		config.set_app_string("tts_engine", &self.engine);
		config.set_app_int("tts_paragraph_pause_ms", self.paragraph_pause_ms);
		if let Some(path) = book
			&& self.this_book_only
		{
			let own = DocumentSpeech {
				voice: self.voice.clone(),
				rate_percent: Some(self.rate),
				pitch_percent: Some(self.pitch),
			};
			config.set_document_speech(path, &own);
			return;
		}
		config.set_app_string(&voice_key, self.voice.as_deref().unwrap_or_default());
		config.set_app_int(&rate_key, self.rate);
		config.set_app_int(&pitch_key, self.pitch);
		// Unchecking the box hands the book back to the app's settings.
		if let Some(path) = book {
			config.set_document_speech(path, &DocumentSpeech::default());
		}
	}
}

fn keys(engine: &str) -> (String, String, String) {
	(format!("tts_voice_{engine}"), format!("tts_rate_{engine}"), format!("tts_pitch_{engine}"))
}

/// Speaks a sample with the engine, voice, rate and pitch given.
pub type PlaySample = Rc<dyn Fn(&str, Option<String>, i32, i32)>;

pub(super) struct AudioTab {
	bookmark_sounds: CheckBox,
	sync_caret_to_audio: CheckBox,
	seek_amount: Choice,
	seek_continues_into_next_file: CheckBox,
	speech: Option<SpeechSection>,
}

impl AudioTab {
	/// `has_book` enables keeping the voice to the open book.
	pub(super) fn new(
		panel: Panel,
		settings: &AudioSettings,
		padding: i32,
		has_book: bool,
		play_sample: PlaySample,
	) -> Self {
		let bookmark_sounds =
			// TRANSLATORS: Option to play sound effects when bookmarks or notes are encountered
			CheckBox::builder(&panel).with_label(&t("Play &sounds on bookmarks and notes")).build();
		let sync_caret_to_audio =
			// TRANSLATORS: Option to move the reading caret to follow along with audio narration as it plays
			CheckBox::builder(&panel).with_label(&t("&Sync caret to audio playback")).build();
		// TRANSLATORS: Label for the audio seek amount dropdown, used when skipping audio narration backward/forward
		let seek_amount_label_text = t("&Audio seek amount:");
		let seek_amount_label = StaticText::builder(&panel).with_label(&seek_amount_label_text).build();
		let seek_amount = Choice::builder(&panel).build();
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("5 seconds"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("10 seconds"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("30 seconds"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("1 minute"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("2 minutes"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("5 minutes"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("10 minutes"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("30 minutes"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("1 hour"));
		#[cfg(target_os = "macos")]
		seek_amount.set_accessibility_label(seek_amount_label_text.replace('&', "").trim_end_matches(':').trim());
		let seek_continues_into_next_file =
			// TRANSLATORS: Option controlling whether seeking audio narration past the end of a chapter's file continues into the next chapter, or stops at the end of the current one
			CheckBox::builder(&panel).with_label(&t("Seeking &past the end of a chapter continues into the next")).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&bookmark_sounds, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&sync_caret_to_audio, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(seek_amount_label, &seek_amount, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&seek_continues_into_next_file, 0, SizerFlag::All, OPTION_PADDING);
		let speech = settings.speech.as_ref().map(|speech| {
			let (section, section_sizer) = SpeechSection::new(panel, speech, padding, has_book, play_sample);
			sizer.add_sizer(&section_sizer, 0, SizerFlag::Expand | SizerFlag::All, OPTION_PADDING);
			section
		});
		panel.set_sizer(sizer, true);
		bookmark_sounds.set_value(settings.bookmark_sounds);
		sync_caret_to_audio.set_value(settings.sync_caret_to_audio);
		let seek_index = AUDIO_SEEK_AMOUNTS_SECONDS
			.iter()
			.position(|&seconds| seconds == settings.audio_seek_amount_seconds)
			.or_else(|| AUDIO_SEEK_AMOUNTS_SECONDS.iter().position(|&seconds| seconds == DEFAULT_SEEK_AMOUNT_SECONDS))
			.unwrap_or(1);
		seek_amount.set_selection(u32::try_from(seek_index).unwrap_or(1));
		seek_continues_into_next_file.set_value(settings.audio_seek_continues_into_next_file);
		Self { bookmark_sounds, sync_caret_to_audio, seek_amount, seek_continues_into_next_file, speech }
	}

	pub(super) fn values(&self) -> AudioSettings {
		AudioSettings {
			bookmark_sounds: self.bookmark_sounds.is_checked(),
			sync_caret_to_audio: self.sync_caret_to_audio.is_checked(),
			audio_seek_amount_seconds: self
				.seek_amount
				.get_selection()
				.and_then(|index| AUDIO_SEEK_AMOUNTS_SECONDS.get(index as usize).copied())
				.unwrap_or(DEFAULT_SEEK_AMOUNT_SECONDS),
			audio_seek_continues_into_next_file: self.seek_continues_into_next_file.is_checked(),
			speech: self.speech.as_ref().map(SpeechSection::values),
		}
	}
}

/// The Read Aloud box: engine, voice, rate, pitch and pause, and a sample of them.
struct SpeechSection {
	engine: Choice,
	voice: Choice,
	/// The ids of the voices listed after "System default", in that order.
	voice_ids: Rc<RefCell<Vec<String>>>,
	rate: Slider,
	pitch: Slider,
	pause: SpinCtrl,
	this_book_only: CheckBox,
	has_book: bool,
}

impl SpeechSection {
	fn new(
		panel: Panel,
		settings: &SpeechSettings,
		padding: i32,
		has_book: bool,
		play_sample: PlaySample,
	) -> (Self, StaticBoxSizer) {
		// TRANSLATORS: Label of the group of read aloud settings on the Audio tab of Settings: engine, voice, rate, pitch and pauses
		let group = StaticBox::builder(&panel).with_label(&t("Read Aloud")).build();
		let sizer = StaticBoxSizerBuilder::new_with_box(&group, Orientation::Vertical).build();
		// TRANSLATORS: Label of the list of speech engines, such as OneCore or SAPI 5, on the Audio tab of Settings
		let engine_label = StaticText::builder(&panel).with_label(&t("&Engine:")).build();
		let engine = Choice::builder(&panel).build();
		let engines = read_aloud::engines();
		for (_, name) in engines {
			engine.append(name);
		}
		let engine_index = engines.iter().position(|(id, _)| *id == settings.engine).unwrap_or(0);
		engine.set_selection(u32::try_from(engine_index).unwrap_or(0));
		// TRANSLATORS: Label of the list of voices on the Audio tab of Settings
		let voice_label = StaticText::builder(&panel).with_label(&t("&Voice:")).build();
		let voice = Choice::builder(&panel).build();
		let voice_ids = Rc::new(RefCell::new(fill_voices(voice, voices(&settings.engine), settings.voice.as_deref())));
		// TRANSLATORS: Label of the speech rate slider on the Audio tab of Settings
		let rate_label = StaticText::builder(&panel).with_label(&t("&Rate:")).build();
		let rate = slider(panel, settings.rate);
		// TRANSLATORS: Label of the speech pitch slider on the Audio tab of Settings
		let pitch_label = StaticText::builder(&panel).with_label(&t("Pi&tch:")).build();
		let pitch = slider(panel, settings.pitch);
		// TRANSLATORS: Label of the silence added after each paragraph read aloud, in milliseconds
		let pause_label =
			StaticText::builder(&panel).with_label(&t("Pause after each para&graph (milliseconds):")).build();
		let pause = SpinCtrl::builder(&panel).with_range(0, 1000).build();
		pause.set_value(settings.paragraph_pause_ms.clamp(0, 1000));
		// TRANSLATORS: Check box on the Audio tab of Settings that keeps the voice, rate and pitch to the open book
		let this_book_only =
			CheckBox::builder(&panel).with_label(&t("Use this voice, rate and pitch for this &book only")).build();
		this_book_only.set_value(has_book && settings.this_book_only);
		this_book_only.enable(has_book);
		// TRANSLATORS: Button on the Audio tab of Settings that speaks a sample with the chosen voice, rate and pitch
		let sample = Button::builder(&panel).with_label(&t("Play Sa&mple")).build();
		sizer.add_sizer(&labeled_row(engine_label, &engine, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(voice_label, &voice, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(rate_label, &rate, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(pitch_label, &pitch, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(pause_label, &pause, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&this_book_only, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&sample, 0, SizerFlag::All, OPTION_PADDING);
		let section = Self { engine, voice, voice_ids, rate, pitch, pause, this_book_only, has_book };
		let reload_ids = Rc::clone(&section.voice_ids);
		// Each engine has voices of its own, so changing it lists them, starting from the system's default.
		engine.on_selection_changed(move |_| {
			*reload_ids.borrow_mut() = fill_voices(voice, voices(&chosen_engine(engine)), None);
		});
		let sample_ids = Rc::clone(&section.voice_ids);
		sample.on_click(move |_| {
			let chosen = chosen_voice(voice, &sample_ids.borrow());
			play_sample(&chosen_engine(engine), chosen, rate.get_value(), pitch.get_value());
		});
		(section, sizer)
	}

	fn values(&self) -> SpeechSettings {
		SpeechSettings {
			engine: chosen_engine(self.engine),
			voice: chosen_voice(self.voice, &self.voice_ids.borrow()),
			rate: self.rate.get_value(),
			pitch: self.pitch.get_value(),
			paragraph_pause_ms: self.pause.value(),
			this_book_only: self.has_book && self.this_book_only.is_checked(),
		}
	}
}

/// Up and Page Up raise the value, which a native Windows slider would lower (#1273).
fn slider(panel: Panel, value: i32) -> Slider {
	Slider::builder(&panel)
		.with_value(value.clamp(0, 100))
		.with_min_value(0)
		.with_max_value(100)
		.with_natural_keys(true)
		.build()
}

fn chosen_engine(engine: Choice) -> String {
	let index = usize::try_from(engine.get_selection().unwrap_or(0)).unwrap_or(0);
	read_aloud::engines().get(index).map_or_else(String::new, |(id, _)| (*id).to_owned())
}

/// The voice picked, or `None` for "System default", which comes first.
fn chosen_voice(voice: Choice, voice_ids: &[String]) -> Option<String> {
	let index = usize::try_from(voice.get_selection().unwrap_or(0)).unwrap_or(0);
	index.checked_sub(1).and_then(|index| voice_ids.get(index).cloned())
}

/// The voices of `engine`, or none if it can't list them.
fn voices(engine: &str) -> Vec<Voice> {
	read_aloud::voices(engine).unwrap_or_else(|error| {
		tracing::warn!(%error, engine, "read aloud: couldn't list voices");
		Vec::new()
	})
}

/// Lists "System default" then `voices` by language, selects `selected` if it's among them, and returns the voice ids in the order listed after "System default".
fn fill_voices(choice: Choice, mut voices: Vec<Voice>, selected: Option<&str>) -> Vec<String> {
	voices.sort_by_cached_key(|voice| (language_label(&voice.language), voice.name.clone()));
	choice.clear();
	// TRANSLATORS: First choice in the voice list on the Audio tab of Settings, which leaves the choice of voice to the system
	choice.append(&t("System default"));
	for voice in &voices {
		choice.append(&format!("{}, {}", voice.name, language_label(&voice.language)));
	}
	let index = selected.and_then(|id| voices.iter().position(|voice| voice.id == id)).map_or(0, |index| index + 1);
	choice.set_selection(u32::try_from(index).unwrap_or(0));
	voices.into_iter().map(|voice| voice.id).collect()
}

/// A voice's language as people know it, with its tag after it, since one language can have voices for several countries.
fn language_label(tag: &str) -> String {
	let base = tag.split(['-', '_']).next().unwrap_or(tag);
	format!("{} ({tag})", patois::language_name(base))
}
