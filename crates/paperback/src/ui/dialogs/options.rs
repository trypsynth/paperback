//! The Settings dialog. Each tab lives in its own module, with the settings it edits and how they load and save.

use paperback_core::config::ConfigManager;
use patois::t;
#[cfg(target_os = "windows")]
use wxdragon::accessible::AccRole;
use wxdragon::prelude::*;

use super::{add_ok_cancel_footer, build_ok_cancel_buttons, dialog_padding};

mod advanced;
mod appearance;
mod audio;
mod documents;
mod font;
mod general;
mod reading;

pub use advanced::AdvancedSettings;
pub use appearance::AppearanceSettings;
pub(crate) use audio::AUDIO_SEEK_AMOUNTS_SECONDS;
pub use audio::AudioSettings;
pub use documents::DocumentSettings;
pub use general::GeneralSettings;
pub use reading::ReadingSettings;

/// Space around each option, inside its tab.
const OPTION_PADDING: i32 = 5;

/// Everything the Settings dialog edits, by tab.
#[derive(Clone, Debug)]
pub struct OptionsDialogResult {
	pub general: GeneralSettings,
	pub reading: ReadingSettings,
	pub audio: AudioSettings,
	pub appearance: AppearanceSettings,
	pub documents: DocumentSettings,
	pub advanced: AdvancedSettings,
}

impl OptionsDialogResult {
	pub fn load(config: &ConfigManager) -> Self {
		Self {
			general: GeneralSettings::load(config),
			reading: ReadingSettings::load(config),
			audio: AudioSettings::load(config),
			appearance: AppearanceSettings::load(config),
			documents: DocumentSettings::load(config),
			advanced: AdvancedSettings::load(config),
		}
	}

	pub fn save(&self, config: &ConfigManager) {
		self.general.save(config);
		self.reading.save(config);
		self.audio.save(config);
		self.appearance.save(config);
		self.documents.save(config);
		self.advanced.save(config);
	}
}

pub fn show_options_dialog(parent: &Frame, config: &ConfigManager) -> Option<OptionsDialogResult> {
	let initial = OptionsDialogResult::load(config);
	// TRANSLATORS: Title of the Settings dialog
	let dialog = Dialog::builder(parent, &t("Settings")).build();
	let padding = dialog_padding(&dialog);
	let notebook = Notebook::builder(&dialog).with_style(NotebookStyle::Top).build();
	let page = || Panel::builder(&notebook).build();
	let (general_panel, reading_panel, audio_panel) = (page(), page(), page());
	let (appearance_panel, documents_panel, advanced_panel) = (page(), page(), page());
	let general = general::GeneralTab::new(general_panel, dialog, &initial.general, padding);
	let reading = reading::ReadingTab::new(reading_panel, &initial.reading, padding);
	let audio = audio::AudioTab::new(audio_panel, &initial.audio, padding);
	let appearance = appearance::AppearanceTab::new(appearance_panel, dialog, &initial.appearance, padding);
	let documents = documents::DocumentsTab::new(documents_panel, &initial.documents);
	let advanced = advanced::AdvancedTab::new(advanced_panel, initial.advanced, padding);
	let tabs = [
		// TRANSLATORS: Tab label for the General options panel
		(general_panel, t("General")),
		// TRANSLATORS: Tab label for the Reading options panel, about moving through and searching a document
		(reading_panel, t("Reading")),
		// TRANSLATORS: Tab label for the Settings panel about playing audio books and narration
		(audio_panel, t("Audio")),
		// TRANSLATORS: Tab label for the Settings panel about how text looks: font, colour, spacing and alignment
		(appearance_panel, t("Appearance")),
		// TRANSLATORS: Tab label for the Settings panel about how a file is read, such as whether tables are shown inline
		(documents_panel, t("Documents")),
		// TRANSLATORS: Tab label for the Settings panel with options most people never need, such as the update channel and log level
		(advanced_panel, t("Advanced")),
	];
	for (index, (panel, label)) in tabs.iter().enumerate() {
		panel.set_accessibility_label(label);
		#[cfg(target_os = "windows")]
		panel.set_accessibility_role(AccRole::PropertyPage);
		notebook.add_page(panel, label, index == 0, None);
	}
	// TRANSLATORS: Label for the confirmation button
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("OK"));
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&notebook, 1, SizerFlag::Expand | SizerFlag::All, padding);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	let result = (dialog.show_modal() == ID_OK).then(|| OptionsDialogResult {
		general: general.values(),
		reading: reading.values(),
		audio: audio.values(),
		appearance: appearance.values(),
		documents: documents.values(),
		advanced: advanced.values(),
	});
	dialog.destroy();
	result
}

/// A label and the control it names, side by side.
fn labeled_row(label: StaticText, control: &impl WxWidget, padding: i32) -> BoxSizer {
	let row = BoxSizer::builder(Orientation::Horizontal).build();
	row.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, padding);
	row.add(control, 0, SizerFlag::AlignCenterVertical, 0);
	row
}
