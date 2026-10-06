//! The heavier "View" menu handlers: opening the document in a web view and viewing its source.
//! Reload and the word wrap and full screen toggles stay inlined in `bind_menu_events`'s
//! dispatch match.

use std::{env, path::Path, rc::Rc, sync::Mutex};

use paperback_core::{parser::is_external_url, session::SourceView};
use patois::t;
use wxdragon::prelude::*;

use super::{DocumentManager, dialogs, navigation};

pub(super) fn handle_open_in_web_view(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	let Some(tab) = dm_ref.active_tab() else {
		return;
	};
	let current_pos = navigation::doc_caret(tab);
	let temp_dir = env::temp_dir().to_string_lossy().to_string();
	if let Some(target) = tab.session.webview_target_path(current_pos, &temp_dir) {
		let mut url = format!("file:///{}", target.path.replace('\\', "/"));
		let fragment = target.fragment.or_else(|| tab.session.webview_fragment_for_position(current_pos));
		if let Some(fragment) = fragment {
			url.push('#');
			url.push_str(&fragment);
		}
		drop(dm_ref);
		dialogs::show_web_view_dialog(
			frame,
			// TRANSLATORS: Title of the window that renders a document's content as HTML (e.g. for embedded web pages)
			&t("Web View"),
			&url,
			true,
			Some(Box::new(|url| {
				if is_external_url(url) {
					launch_default_browser(url, BrowserLaunchFlags::Default);
					false
				} else {
					true
				}
			})),
		);
	} else {
		tracing::warn!(path = %tab.file_path.display(), "could not determine web view content");
		let dialog = MessageDialog::builder(
			frame,
			// TRANSLATORS: Error shown when the document has no content that can be rendered in the Web View
			&t("Could not determine content to display in Web View."),
			// TRANSLATORS: Generic error dialog title
			&t("Error"),
		)
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
		.build();
		dialog.show_modal();
	}
}

pub(super) fn handle_view_source(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>) {
	// `None` => format has no text source; `Some(None)` => source could not
	// be loaded; `Some(Some(..))` => source ready. Locks are dropped before
	// any dialog is shown.
	let outcome: Option<Option<(SourceView, String)>> = {
		let Ok(dm_ref) = dm.try_lock() else {
			return;
		};
		let Some(tab) = dm_ref.active_tab() else {
			return;
		};
		if tab.session.source_view_available() {
			let current_pos = navigation::doc_caret(tab);
			let orig_name = tab
				.file_path
				.file_name()
				// TRANSLATORS: Fallback file name stem used when the document's path has no file stem
				.map_or_else(|| t("document"), |name| name.to_string_lossy().to_string());
			let temp_dir = env::temp_dir().to_string_lossy().to_string();
			Some(tab.session.view_source(current_pos, &temp_dir).map(|view| (view, orig_name)))
		} else {
			None
		}
	};
	match outcome {
		Some(Some((view, orig_name))) => {
			// TRANSLATORS: Prefix before the file name in the tab title for a "View Source" tab, e.g. "Source: book.epub"
			let title = format!("{} {orig_name}", t("Source:"));
			let opened = dm.lock().unwrap().open_source_file(dm, Path::new(&view.path), &title);
			if opened {
				let dm_ref = dm.lock().unwrap();
				if let Some(tab) = dm_ref.active_tab() {
					tab.text_ctrl.set_insertion_point(view.caret);
					tab.text_ctrl.show_position(view.caret);
				}
			}
		}
		unavailable => {
			let message = if unavailable.is_none() {
				tracing::debug!("source view not available for this format");
				// TRANSLATORS: Error shown when "View Source" is used on a document format that has no raw source to view
				t("Source view is not available for this document format.")
			} else {
				tracing::warn!("failed to load document source for view source");
				// TRANSLATORS: Error shown when "View Source" fails to load the document's underlying source
				t("Could not load the document source.")
			};
			// TRANSLATORS: Generic error dialog title
			let dialog = MessageDialog::builder(frame, &message, &t("Error"))
				.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
				.build();
			dialog.show_modal();
		}
	}
}
