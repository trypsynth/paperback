//! The account's defaults for new conversions, which the website and the Scribe apps share, saved on OK like the rest of Settings.

use std::{cell::RefCell, rc::Rc};

use paperback_core::scribe::{Catalog, Settings, SettingsUpdate, VoiceChoice};
use patois::t;
use wxdragon::prelude::*;

use super::super::{OPTION_PADDING, labeled_choice, labeled_row};

/// Scribe's speaking rates, in tenths of normal speed, as the iOS app offers them.
const RATES: std::ops::RangeInclusive<u32> = 5..=20;
const BRAILLE_SIZES: (i32, i32) = (1, 200);

struct Loaded {
	catalog: Catalog,
	choice: VoiceChoice,
	/// What the form held when it was filled, so OK sends nothing when nothing changed.
	unchanged: serde_json::Value,
}

impl Loaded {
	fn languages(&self) -> Vec<(&str, &str)> {
		self.catalog.languages.iter().map(|language| (language.0.as_str(), language.1.as_str())).collect()
	}

	fn dialects(&self) -> Vec<(&str, &str)> {
		self.catalog
			.dialects_of(self.choice.language.as_deref())
			.iter()
			.map(|dialect| (dialect.0.as_str(), dialect.1.as_str()))
			.collect()
	}

	fn voices(&self) -> Vec<(&str, &str)> {
		self.catalog.voices_of(self.choice.dialect()).iter().map(|voice| (voice.0.as_str(), voice.1.as_str())).collect()
	}

	fn tables(&self) -> Vec<(&str, &str)> {
		self.catalog.braille_tables.iter().map(|table| (table.0.as_str(), table.1.as_str())).collect()
	}
}

#[derive(Clone)]
pub(super) struct Defaults {
	/// The tab, laid out again as the form shows and hides.
	tab: Panel,
	pub(super) form: Panel,
	language: Choice,
	dialect_label: StaticText,
	dialect: Choice,
	voice_label: StaticText,
	voice: Choice,
	gender: Choice,
	rate: Choice,
	table: Choice,
	cells: SpinCtrl,
	split: CheckBox,
	lines_label: StaticText,
	lines: SpinCtrl,
	large_print: CheckBox,
	image_descriptions: CheckBox,
	math: CheckBox,
	notify: CheckBox,
	/// The table ids in `table`'s order, with the saved one added when Scribe no longer lists it.
	table_ids: Rc<RefCell<Vec<String>>>,
	loaded: Rc<RefCell<Option<Loaded>>>,
}

impl Defaults {
	/// Hidden until Scribe sends the account's settings.
	pub(super) fn new(tab: Panel, padding: i32) -> Self {
		let form = Panel::builder(&tab).build();
		// TRANSLATORS: Label of the language conversions speak and read in, on the Scribe tab of Settings
		let (language_label, language) = labeled_choice(form, &t("Conversion &language:"), &[]);
		// TRANSLATORS: Label of the dialect of the conversion language, such as British English, on the Scribe tab of Settings
		let (dialect_label, dialect) = labeled_choice(form, &t("&Dialect:"), &[]);
		// TRANSLATORS: Label of the text-to-speech voice for converted audio, on the Scribe tab of Settings
		let (voice_label, voice) = labeled_choice(form, &t("&Voice:"), &[]);
		let genders = [
			automatic(),
			// TRANSLATORS: Option of the voice gender on the Scribe tab of Settings
			t("Female"),
			// TRANSLATORS: Option of the voice gender on the Scribe tab of Settings
			t("Male"),
			// TRANSLATORS: Option of the voice gender on the Scribe tab of Settings, a gender-neutral voice
			t("Neutral"),
		];
		// TRANSLATORS: Label of the voice gender, used when no voice is chosen, on the Scribe tab of Settings
		let (gender_label, gender) = labeled_choice(form, &t("Voice &gender:"), &genders);
		// TRANSLATORS: An option of the speaking rate on the Scribe tab of Settings. %s is how many times normal speed, such as 1.5.
		let rates: Vec<String> =
			RATES.map(|tenths| t("%s times").replacen("%s", &format!("{:.1}", f64::from(tenths) / 10.0), 1)).collect();
		// TRANSLATORS: Label of the text-to-speech speaking rate on the Scribe tab of Settings
		let (rate_label, rate) = labeled_choice(form, &t("Speaking &rate:"), &rates);
		// TRANSLATORS: Label of the braille translation table for converted braille, on the Scribe tab of Settings
		let (table_label, table) = labeled_choice(form, &t("&Braille table:"), &[]);
		// TRANSLATORS: Label on the Scribe tab of Settings
		let cells_label = StaticText::builder(&form).with_label(&t("Braille cells per li&ne:")).build();
		let cells = SpinCtrl::builder(&form).with_range(BRAILLE_SIZES.0, BRAILLE_SIZES.1).build();
		// TRANSLATORS: Option on the Scribe tab of Settings
		let split = CheckBox::builder(&form).with_label(&t("Spli&t braille into pages")).build();
		// TRANSLATORS: Label on the Scribe tab of Settings
		let lines_label = StaticText::builder(&form).with_label(&t("Braille lines &per page:")).build();
		let lines = SpinCtrl::builder(&form).with_range(BRAILLE_SIZES.0, BRAILLE_SIZES.1).build();
		// TRANSLATORS: Option on the Scribe tab of Settings to make converted documents in large print
		let large_print = CheckBox::builder(&form).with_label(&t("Large pr&int")).build();
		// TRANSLATORS: Option on the Scribe tab of Settings to describe the images in converted documents
		let image_descriptions = CheckBox::builder(&form).with_label(&t("&Add image descriptions")).build();
		// TRANSLATORS: Option on the Scribe tab of Settings. Beta means the feature is still being tested.
		let math = CheckBox::builder(&form).with_label(&t("Process mat&hematical notation (beta)")).build();
		// TRANSLATORS: Option on the Scribe tab of Settings for Scribe to send an email when a conversion finishes
		let notify = CheckBox::builder(&form).with_label(&t("&Email me when a conversion is complete")).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		for (label, choice) in [
			(language_label, language),
			(dialect_label, dialect),
			(voice_label, voice),
			(gender_label, gender),
			(rate_label, rate),
			(table_label, table),
		] {
			sizer.add_sizer(&labeled_row(label, &choice, padding), 0, SizerFlag::All, OPTION_PADDING);
		}
		sizer.add_sizer(&labeled_row(cells_label, &cells, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&split, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(lines_label, &lines, padding), 0, SizerFlag::All, OPTION_PADDING);
		for checkbox in [large_print, image_descriptions, math, notify] {
			sizer.add(&checkbox, 0, SizerFlag::All, OPTION_PADDING);
		}
		form.set_sizer(sizer, true);
		let defaults = Self {
			tab,
			form,
			language,
			dialect_label,
			dialect,
			voice_label,
			voice,
			gender,
			rate,
			table,
			cells,
			split,
			lines_label,
			lines,
			large_print,
			image_descriptions,
			math,
			notify,
			table_ids: Rc::default(),
			loaded: Rc::default(),
		};
		defaults.set_shown(false);
		let changed = defaults.clone();
		language.on_selection_changed(move |_| {
			if let Some(loaded) = changed.loaded.borrow_mut().as_mut() {
				loaded.choice.language = chosen(changed.language, &loaded.languages());
			}
			changed.show_dialects();
		});
		let changed = defaults.clone();
		dialect.on_selection_changed(move |_| {
			if let Some(loaded) = changed.loaded.borrow_mut().as_mut() {
				let dialect = chosen(changed.dialect, &loaded.dialects());
				loaded.choice.set_dialect(dialect);
			}
			changed.show_voices();
		});
		let changed = defaults.clone();
		voice.on_selection_changed(move |_| {
			if let Some(loaded) = changed.loaded.borrow_mut().as_mut() {
				let voice = chosen(changed.voice, &loaded.voices());
				loaded.choice.set_voice(voice);
			}
		});
		split.on_toggled(move |_| {
			lines_label.enable(split.is_checked());
			lines.enable(split.is_checked());
		});
		defaults
	}

	fn set_shown(&self, shown: bool) {
		self.form.show(shown);
		self.tab.layout();
	}

	/// Signing out hides the form and forgets what it held, so OK sends nothing.
	pub(super) fn clear(&self) {
		self.loaded.borrow_mut().take();
		self.set_shown(false);
	}

	pub(super) fn fill(&self, settings: &Settings, catalog: Catalog) {
		let choice = VoiceChoice::from_settings(settings);
		*self.loaded.borrow_mut() = Some(Loaded { catalog, choice, unchanged: serde_json::Value::Null });
		if let Some(loaded) = self.loaded.borrow().as_ref() {
			fill_with_automatic(self.language, &loaded.languages(), loaded.choice.language.as_deref());
			let mut tables: Vec<(String, String)> =
				loaded.tables().iter().map(|(name, id)| ((*name).to_string(), (*id).to_string())).collect();
			if !tables.iter().any(|(_, id)| *id == settings.braille_translation_table) {
				tables.push((settings.braille_translation_table.clone(), settings.braille_translation_table.clone()));
			}
			self.table.clear();
			for (name, _) in &tables {
				self.table.append(name);
			}
			let selected = tables.iter().position(|(_, id)| *id == settings.braille_translation_table).unwrap_or(0);
			self.table.set_selection(u32::try_from(selected).unwrap_or(0));
			*self.table_ids.borrow_mut() = tables.into_iter().map(|(_, id)| id).collect();
		}
		let gender = ["", "female", "male", "neutral"]
			.iter()
			.position(|gender| Some(*gender) == settings.tts_gender.as_deref())
			.unwrap_or(0);
		self.gender.set_selection(u32::try_from(gender).unwrap_or(0));
		self.rate.set_selection(rate_index(settings.tts_rate));
		self.cells.set_value(i32::try_from(settings.braille_cells_per_line).unwrap_or(BRAILLE_SIZES.1));
		self.split.set_value(settings.braille_split_into_pages);
		self.lines.set_value(i32::try_from(settings.braille_lines_per_page).unwrap_or(BRAILLE_SIZES.1));
		self.lines_label.enable(settings.braille_split_into_pages);
		self.lines.enable(settings.braille_split_into_pages);
		self.large_print.set_value(settings.large_print);
		self.image_descriptions.set_value(settings.add_image_descriptions);
		self.math.set_value(settings.math);
		self.notify.set_value(settings.notify_when_complete);
		self.set_shown(true);
		self.show_dialects();
		let unchanged =
			self.update().map(|update| serde_json::to_value(update).unwrap_or_default()).unwrap_or_default();
		if let Some(loaded) = self.loaded.borrow_mut().as_mut() {
			loaded.unchanged = unchanged;
		}
	}

	fn show_dialects(&self) {
		if let Some(loaded) = self.loaded.borrow().as_ref() {
			let dialects = loaded.dialects();
			fill_with_automatic(self.dialect, &dialects, loaded.choice.dialect());
			self.dialect_label.show(!dialects.is_empty());
			self.dialect.show(!dialects.is_empty());
		}
		self.show_voices();
	}

	fn show_voices(&self) {
		if let Some(loaded) = self.loaded.borrow().as_ref() {
			let voices = loaded.voices();
			fill_with_automatic(self.voice, &voices, loaded.choice.voice());
			self.voice_label.show(!voices.is_empty());
			self.voice.show(!voices.is_empty());
		}
		self.form.layout();
		self.tab.layout();
	}

	fn update(&self) -> Option<SettingsUpdate> {
		let loaded = self.loaded.borrow();
		let loaded = loaded.as_ref()?;
		let gender = ["", "female", "male", "neutral"]
			.get(self.gender.get_selection().unwrap_or(0) as usize)
			.copied()
			.unwrap_or_default();
		let rate = RATES.start() + self.rate.get_selection().unwrap_or(0);
		let table = self.table_ids.borrow().get(self.table.get_selection().unwrap_or(0) as usize).cloned();
		let mut update = SettingsUpdate {
			tts_gender: Some(gender.to_string()),
			tts_rate: Some(f64::from(rate) / 10.0),
			braille_translation_table: table,
			braille_cells_per_line: Some(i64::from(self.cells.value())),
			braille_split_into_pages: Some(self.split.is_checked()),
			braille_lines_per_page: Some(i64::from(self.lines.value())),
			large_print: Some(self.large_print.is_checked()),
			add_image_descriptions: Some(self.image_descriptions.is_checked()),
			math: Some(self.math.is_checked()),
			notify_when_complete: Some(self.notify.is_checked()),
			..SettingsUpdate::default()
		};
		loaded.choice.apply_to(&mut update);
		Some(update)
	}

	/// What to send Scribe on OK, or `None` when the form wasn't loaded or nothing changed.
	pub(super) fn changes(&self) -> Option<SettingsUpdate> {
		let update = self.update()?;
		let unchanged = self.loaded.borrow().as_ref().map(|loaded| loaded.unchanged.clone())?;
		(serde_json::to_value(&update).ok()? != unchanged).then_some(update)
	}
}

/// The first option of the Language, Dialect, Voice and Gender lists, for Scribe to choose.
fn automatic() -> String {
	// TRANSLATORS: First option of the conversion language, dialect, voice and gender lists on the Scribe tab of Settings: let Scribe choose
	t("Automatic")
}

/// Fills `choice` with Automatic and then each entry's name, selecting the entry whose id is `selected`.
fn fill_with_automatic(choice: Choice, entries: &[(&str, &str)], selected: Option<&str>) {
	choice.clear();
	choice.append(&automatic());
	for (name, _) in entries {
		choice.append(name);
	}
	let index =
		selected.and_then(|selected| entries.iter().position(|(_, id)| *id == selected)).map_or(0, |index| index + 1);
	choice.set_selection(u32::try_from(index).unwrap_or(0));
}

/// The id of the entry chosen in a list filled by `fill_with_automatic`, or `None` for Automatic.
fn chosen(choice: Choice, entries: &[(&str, &str)]) -> Option<String> {
	let index = (choice.get_selection()? as usize).checked_sub(1)?;
	entries.get(index).map(|(_, id)| (*id).to_string())
}

/// The nearest of `RATES` to `rate`, since the website takes any rate.
fn rate_index(rate: f64) -> u32 {
	let tenths = (rate * 10.0).round().clamp(f64::from(*RATES.start()), f64::from(*RATES.end()));
	#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to RATES first")]
	let tenths = tenths as u32;
	tenths - RATES.start()
}
