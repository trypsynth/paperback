//! Services > Scribe > Convert Document: uploads a document to Scribe for Documents, and reads what comes back in a tab that fills in as the pages arrive. Paperback keeps no file of it: the tab reads a temporary copy, and the conversion stays in the reader's Scribe account.
#![allow(
	clippy::future_not_send,
	reason = "these run on the UI thread, which owns the windows and the document manager"
)]

use std::{
	fs,
	path::{Path, PathBuf},
	rc::Rc,
	sync::{Arc, Mutex},
	time::{Duration, Instant},
};

use live_region::{Priority, announce_with_priority};
use paperback_core::scribe::{Error, Stage, Update, status_page};
use patois::{nt, t};
use wx_utils::{
	show_error,
	tasks::{background, spawn_for},
};
use wxdragon::prelude::*;

use super::{
	document_manager::{DocumentManager, prompt_for_password},
	main_window::update_title_from_manager,
};
use crate::{scribe, working_copy::WorkingCopy};

/// Reloading as each page lands would keep interrupting the reader.
const RELOAD_INTERVAL: Duration = Duration::from_secs(5);

fn title() -> String {
	// TRANSLATORS: Title of messages about converting a document with Scribe for Documents, Pneuma Solutions' conversion service
	t("Convert with Scribe")
}

/// Converts the open document once the reader confirms, or one they choose when none is open or they'd rather another.
pub fn convert_with_scribe(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>, live_region: StaticText) {
	if !scribe::shared().is_signed_in() {
		// TRANSLATORS: Shown by Tools > Convert with Scribe before the reader has signed in
		show_error(frame, t("Sign in to Scribe first, on the Scribe tab of Settings."), &title());
		return;
	}
	let open = dm.lock().unwrap().active_tab().map(|tab| tab.file_path.clone()).filter(|path| path.is_file());
	let (frame, dm) = (*frame, Rc::clone(dm));
	spawn_for(&frame, async move {
		let source = match &open {
			Some(open) => match confirm(frame, open).await {
				ID_YES => Some(open.clone()),
				ID_NO => choose(frame, open.parent()),
				_ => None,
			},
			None => choose(frame, None),
		};
		if let Some(source) = source {
			start(frame, dm, live_region, source).await;
		}
	});
}

/// Yes converts `path`, No chooses another file, and Cancel does neither.
async fn confirm(frame: Frame, path: &Path) -> i32 {
	let scribe = scribe::shared();
	let pages = background(async move { scribe.pages_remaining().await }).await;
	let name = path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
	// TRANSLATORS: Asked by Tools > Convert with Scribe about the open document. %s is its file name.
	let question = t("Convert %s with Scribe?").replacen("%s", &name, 1);
	let cost = match pages {
		// TRANSLATORS: Under the question whether to convert a document with Scribe. %d is how many pages the reader can still convert. Plural form is chosen by that number.
		Ok(pages) => nt(
			"You have %d page left, and converting uses one for each page of the document.",
			"You have %d pages left, and converting uses one for each page of the document.",
			pages.unsigned_abs(),
		)
		.replacen("%d", &pages.to_string(), 1),
		// TRANSLATORS: Under the question whether to convert a document with Scribe, when the pages left couldn't be checked
		Err(_) => t("Converting uses one of your Scribe pages for each page of the document."),
	};
	let dialog = MessageDialog::builder(
		&frame,
		&format!(
			"{question}

{cost}"
		),
		&title(),
	)
	.with_style(MessageDialogStyle::YesNo | MessageDialogStyle::Cancel | MessageDialogStyle::IconQuestion)
	.build();
	// TRANSLATORS: Buttons of the question whether to convert the open document with Scribe
	dialog.set_yes_no_labels(&t("&Convert"), &t("C&hoose Another File..."));
	dialog.show_modal()
}

fn choose(frame: Frame, folder: Option<&Path>) -> Option<PathBuf> {
	// TRANSLATORS: Title of the file picker for the document to convert with Scribe
	let mut picker = FileDialog::builder(&frame)
		.with_message(&t("Choose a Document to Convert with Scribe"))
		.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist);
	if let Some(folder) = folder {
		picker = picker.with_default_dir(&folder.to_string_lossy());
	}
	let picker = picker.build();
	(picker.show_modal() == ID_OK).then(|| picker.get_path()).flatten().map(PathBuf::from)
}

async fn start(frame: Frame, dm: Rc<Mutex<DocumentManager>>, live_region: StaticText, source: PathBuf) {
	let name = source.file_stem().map(|stem| stem.to_string_lossy().into_owned()).unwrap_or_default();
	// TRANSLATORS: Tab title of a document converting with Scribe. %s is the original's name without its extension.
	let tab_title = t("%s (Scribe)").replacen("%s", &name, 1);
	// TRANSLATORS: Shown in a document's tab while it uploads to Scribe for converting
	let uploading = status_page(&name, &t("Uploading to Scribe..."));
	let copy =
		WorkingCopy::new(&format!("{name}.html")).and_then(|copy| fs::write(copy.path(), uploading).map(|()| copy));
	let copy = match copy {
		Ok(copy) => copy,
		Err(error) => {
			tracing::warn!(%error, "couldn't make a copy for the Scribe conversion");
			// TRANSLATORS: Shown when Paperback can't make the temporary file a Scribe conversion is read from. %s is the reason.
			let message = t("Paperback couldn't make a temporary copy to read the conversion from: %s").replacen(
				"%s",
				&error.to_string(),
				1,
			);
			show_error(&frame, message, &title());
			return;
		}
	};
	let output = copy.path().to_path_buf();
	if !dm.lock().unwrap().open_untracked_copy(&dm, copy, &tab_title) {
		return;
	}
	update_title_from_manager(&frame, &dm.lock().unwrap());
	convert(frame, dm, live_region, source, output, name).await;
}

/// Stops watching once the tab is closed, since the copy it writes goes with the tab; the conversion carries on in the reader's account. Its announcements wait behind what the screen reader is saying, as they matter less than the reading they would cut off.
async fn convert(
	frame: Frame,
	dm: Rc<Mutex<DocumentManager>>,
	live_region: StaticText,
	source: PathBuf,
	output: PathBuf,
	name: String,
) {
	let scribe = scribe::shared();
	let uploading = {
		let scribe = Arc::clone(&scribe);
		background(async move { scribe.upload(&source).await })
	};
	let mut conversion = match uploading.await {
		Ok(conversion) => conversion,
		Err(error) => return failed(&frame, &dm, &output, &name, &error, false),
	};
	let mut shown = String::new();
	let mut reloaded: Option<Instant> = None;
	loop {
		let shared = Arc::clone(&scribe);
		let (returned, update) = background(async move {
			let update = conversion.next(&shared).await;
			(conversion, update)
		})
		.await;
		conversion = returned;
		if dm.try_lock().is_ok_and(|dm| dm.find_tab_by_path(&output).is_none()) {
			return;
		}
		match update {
			Ok(Update::Stage { stage, progress }) if !conversion.has_pages() => {
				let message = stage_message(stage, progress);
				if message != shown {
					write_and_reload(&dm, &output, &status_page(&name, &message));
					shown = message;
				}
			}
			Ok(Update::Stage { .. }) => {}
			Ok(Update::Pages) => {
				if reloaded.is_none_or(|reloaded| reloaded.elapsed() >= RELOAD_INTERVAL) {
					write_and_reload(&dm, &output, &conversion.html());
					if reloaded.is_none() {
						// TRANSLATORS: Announced when the first pages of a document converting with Scribe can be read
						announce_with_priority(
							live_region,
							&t("Ready for reading. More pages are on the way."),
							Priority::Low,
						);
					}
					reloaded = Some(Instant::now());
				}
			}
			Ok(Update::PasswordRequired) => {
				let Some(password) = prompt_for_password(&frame) else {
					// TRANSLATORS: Shown in a document's tab when it needs a password to convert with Scribe and none was given
					write_and_reload(
						&dm,
						&output,
						&status_page(&name, &t("This document needs a password to convert.")),
					);
					return;
				};
				let shared = Arc::clone(&scribe);
				let (returned, unlocked) = background(async move {
					let unlocked = conversion.unlock(&shared, &password).await;
					(conversion, unlocked)
				})
				.await;
				conversion = returned;
				if let Err(error) = unlocked {
					return failed(&frame, &dm, &output, &name, &error, conversion.has_pages());
				}
			}
			Ok(Update::Complete) => {
				write_and_reload(&dm, &output, &conversion.html());
				// TRANSLATORS: Announced when every page of a document converting with Scribe has arrived. %s is the document's name.
				announce_with_priority(live_region, &t("%s is ready.").replacen("%s", &name, 1), Priority::Low);
				return;
			}
			Err(error) => return failed(&frame, &dm, &output, &name, &error, conversion.has_pages()),
		}
	}
}

fn stage_message(stage: Stage, progress: f64) -> String {
	#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to 0 to 100 first")]
	let percent = ((progress * 100.0).round().clamp(0.0, 100.0) as u32).to_string();
	match stage {
		// TRANSLATORS: Shown in a document's tab while Scribe has it queued to convert
		Stage::Queue => t("Waiting in line at Scribe..."),
		// TRANSLATORS: Shown in a document's tab as Scribe starts converting it
		Stage::Start | Stage::Complete => t("Scribe is starting the conversion..."),
		// TRANSLATORS: Shown in a document's tab while Scribe converts it. %s is how far through it is, as a percentage.
		Stage::Convert => t("Converting: %s%").replacen("%s", &percent, 1),
		// TRANSLATORS: Shown in a document's tab while Scribe describes its images. %s is how far through it is, as a percentage.
		Stage::AddImageDescriptions => t("Describing images: %s%").replacen("%s", &percent, 1),
	}
}

/// A reload skipped while a modal dialog holds the lock is caught up by the next.
fn write_and_reload(dm: &Rc<Mutex<DocumentManager>>, output: &Path, html: &str) {
	if let Err(error) = fs::write(output, html) {
		tracing::warn!(%error, path = %output.display(), "couldn't write the Scribe conversion");
		return;
	}
	// `try_lock` because a modal dialog's opener may hold the lock.
	if let Ok(mut dm) = dm.try_lock()
		&& let Some(index) = dm.find_tab_by_path(output)
	{
		dm.refresh_tab(index);
	}
}

/// The pages that arrived stay in the file, and the tab says what went wrong only when there are none.
fn failed(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>, output: &Path, name: &str, error: &Error, has_pages: bool) {
	let reason = match error {
		// TRANSLATORS: Shown when Tools > Convert with Scribe finds the reader signed out
		Error::NotSignedIn => t("Sign in to Scribe again, on the Scribe tab of Settings."),
		// TRANSLATORS: Shown when a Scribe conversion fails and Scribe gives no reason
		Error::Failed(None) => t("Scribe couldn't convert the document."),
		other => other.to_string(),
	};
	tracing::warn!(%error, "the Scribe conversion failed");
	if !has_pages {
		write_and_reload(dm, output, &status_page(name, &reason));
	}
	show_error(frame, reason, &title());
}
