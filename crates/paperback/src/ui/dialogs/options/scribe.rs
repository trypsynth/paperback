//! The Scribe tab: signing in to Pneuma Solutions' Scribe for Documents, and the account's conversion defaults. Signing in and out takes effect at once, as it happens in the browser; the defaults are saved on OK.

mod defaults;

use std::{
	cell::RefCell,
	rc::Rc,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};

use defaults::Defaults;
use paperback_core::scribe::{AccountInfo, Error, Loopback};
use patois::{nt, t};
use wx_utils::tasks::{background, background_blocking, spawn_for};
use wxdragon::{
	prelude::*,
	utils::{BrowserLaunchFlags, launch_default_browser},
};

use super::OPTION_PADDING;
use crate::{scribe, translation_manager::TranslationManager};

/// Long enough to make an account in the browser first.
const SIGN_IN_TIMEOUT: Duration = Duration::from_mins(10);

#[derive(Clone)]
pub(super) struct ScribeTab {
	panel: Panel,
	status: StaticText,
	account: Button,
	manage: Button,
	defaults: Defaults,
	/// Set while the browser is out signing in, to cancel it.
	waiting: Rc<RefCell<Option<Arc<AtomicBool>>>>,
}

/// Stops the sign-in listener when the wait for it is dropped, as it is when Settings closes.
struct CancelOnDrop(Arc<AtomicBool>);

impl Drop for CancelOnDrop {
	fn drop(&mut self) {
		self.0.store(true, Ordering::Relaxed);
	}
}

impl ScribeTab {
	pub(super) fn new(panel: Panel, padding: i32) -> Self {
		let status = StaticText::builder(&panel).build();
		let _ = live_region::set_live_region(&status);
		let account = Button::builder(&panel).build();
		// TRANSLATORS: Button on the Scribe tab of Settings that opens the reader's Scribe account page in the browser
		let manage = Button::builder(&panel).with_label(&t("&Manage Account...")).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&status, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&account, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&manage, 0, SizerFlag::All, OPTION_PADDING);
		let defaults = Defaults::new(panel, padding);
		sizer.add(&defaults.form, 0, SizerFlag::Expand, 0);
		panel.set_sizer(sizer, true);
		let tab = Self { panel, status, account, manage, defaults, waiting: Rc::default() };
		if scribe::shared().is_signed_in() {
			// TRANSLATORS: Shown on the Scribe tab of Settings while loading the account
			tab.show(true, &t("Checking your Scribe account..."), false);
			tab.load_account(false);
		} else {
			tab.show(false, &signed_out(), false);
		}
		let clicked = tab.clone();
		account.on_click(move |_| clicked.account_clicked());
		let opener = tab.clone();
		manage.on_click(move |_| opener.open_account_page());
		tab
	}

	/// `announce` is for what the reader just did, not what loads while they are on another tab.
	fn show(&self, signed_in: bool, message: &str, announce: bool) {
		self.status.set_label(message);
		// A screen reader skips text that can't take focus, but reads a property page's description as focus moves into it.
		self.panel.set_accessibility_description(message);
		self.status.wrap(self.panel.get_size().width.max(400));
		self.account.set_label(&if signed_in {
			// TRANSLATORS: Button on the Scribe tab of Settings
			t("Sign &Out")
		} else {
			// TRANSLATORS: Button on the Scribe tab of Settings that opens the browser to sign in to Scribe
			t("&Sign In...")
		});
		self.manage.enable(signed_in);
		if !signed_in {
			self.defaults.clear();
		}
		self.panel.layout();
		if announce {
			live_region::announce(self.status, message);
		}
	}

	fn account_clicked(&self) {
		if let Some(cancel) = self.waiting.borrow().as_ref() {
			cancel.store(true, Ordering::Relaxed);
		} else if scribe::shared().is_signed_in() {
			self.sign_out();
		} else {
			self.sign_in();
		}
	}

	fn sign_in(&self) {
		let loopback = match Loopback::bind() {
			Ok(loopback) => loopback,
			Err(error) => {
				tracing::warn!(%error, "couldn't open a port for the Scribe sign-in");
				// TRANSLATORS: Shown on the Scribe tab of Settings when the sign-in can't start
				self.show(false, &t("Paperback couldn't start the sign-in. Please try again."), true);
				return;
			}
		};
		let scribe = scribe::shared();
		let sign_in = scribe.begin_sign_in(&loopback.redirect_uri());
		if !launch_default_browser(sign_in.url.as_str(), BrowserLaunchFlags::Default) {
			// TRANSLATORS: Shown on the Scribe tab of Settings when the browser doesn't open to sign in
			self.show(false, &t("Paperback couldn't open your browser to sign in."), true);
			return;
		}
		let cancel = Arc::new(AtomicBool::new(false));
		*self.waiting.borrow_mut() = Some(Arc::clone(&cancel));
		// TRANSLATORS: Shown on the Scribe tab of Settings while the reader signs in in the browser
		self.show(false, &t("Sign in to Scribe in your browser, then come back to Paperback."), false);
		// TRANSLATORS: Button on the Scribe tab of Settings that stops waiting for the browser sign-in
		self.account.set_label(&t("&Cancel Sign-In"));
		let language = TranslationManager::instance().lock().unwrap().current_language().replace('_', "-");
		// TRANSLATORS: The page the browser shows once it hands the Scribe sign-in back to Paperback
		let reply = t("You can close this page and go back to Paperback.");
		let tab = self.clone();
		spawn_for(&self.panel, async move {
			let guard = CancelOnDrop(Arc::clone(&cancel));
			let redirect =
				background_blocking(move || loopback.wait(&cancel, SIGN_IN_TIMEOUT, &language, &reply)).await;
			tab.waiting.borrow_mut().take();
			let Some(redirect) = redirect else {
				let message = if guard.0.load(Ordering::Relaxed) {
					// TRANSLATORS: Shown on the Scribe tab of Settings after the reader cancels signing in
					t("Sign-in cancelled.")
				} else {
					// TRANSLATORS: Shown on the Scribe tab of Settings when the browser didn't come back in time
					t("The sign-in took too long. Please try again.")
				};
				tab.show(false, &message, true);
				return;
			};
			// TRANSLATORS: Shown on the Scribe tab of Settings while finishing the sign-in
			tab.show(false, &t("Signing in..."), false);
			let result = background(async move { scribe.finish_sign_in(sign_in, &redirect).await }).await;
			match result {
				Ok(_) => tab.load_account(true),
				Err(error) => tab.show(false, &describe(&error), true),
			}
		});
	}

	/// Loads the account and the conversion defaults at once.
	fn load_account(&self, announce: bool) {
		let scribe = scribe::shared();
		let tab = self.clone();
		spawn_for(&self.panel, async move {
			let account = background({
				let scribe = Arc::clone(&scribe);
				async move { Ok::<_, Error>((scribe.account().await?, scribe.pages_remaining().await?)) }
			});
			let defaults =
				background(async move { Ok::<_, Error>((scribe.settings().await?, scribe.catalog().await?)) });
			let status = match account.await {
				Ok((account, pages)) => signed_in(&account, pages),
				Err(Error::NotSignedIn) => return tab.show(false, &signed_out(), announce),
				Err(error) => describe(&error),
			};
			tab.show(true, &status, announce);
			match defaults.await {
				Ok((settings, catalog)) => tab.defaults.fill(&settings, catalog),
				Err(Error::NotSignedIn) => tab.show(false, &signed_out(), announce),
				// TRANSLATORS: Shown on the Scribe tab of Settings when the conversion settings don't load. %s is the reason.
				Err(error) => tab.show(
					true,
					&t("Paperback couldn't load your conversion settings: %s").replacen("%s", &describe(&error), 1),
					announce,
				),
			}
		});
	}

	/// Sends the conversion defaults changed on this tab, from `frame` since Settings is closing.
	pub(super) fn save(&self, frame: Frame) {
		let Some(update) = self.defaults.changes() else { return };
		let scribe = scribe::shared();
		spawn_for(&frame, async move {
			if let Err(error) = background(async move { scribe.update_settings(update).await }).await {
				// TRANSLATORS: Error shown after Settings closes when Scribe doesn't take the changed conversion settings. %s is the reason.
				let message = t("Paperback couldn't save your Scribe conversion settings: %s").replacen(
					"%s",
					&describe(&error),
					1,
				);
				// TRANSLATORS: Title of an error message
				MessageDialog::builder(&frame, &message, &t("Error"))
					.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
					.build()
					.show_modal();
			}
		});
	}

	fn sign_out(&self) {
		let scribe = scribe::shared();
		let tab = self.clone();
		spawn_for(&self.panel, async move {
			background(async move { scribe.sign_out().await }).await;
			// TRANSLATORS: Shown on the Scribe tab of Settings after signing out
			tab.show(false, &t("You've signed out of Scribe."), true);
		});
	}

	fn open_account_page(&self) {
		let scribe = scribe::shared();
		let tab = self.clone();
		spawn_for(&self.panel, async move {
			match background(async move { scribe.account_page_url().await }).await {
				Ok(url) => {
					if !launch_default_browser(&url, BrowserLaunchFlags::Default) {
						tab.show(true, &t("Paperback couldn't open your browser."), true);
					}
				}
				Err(Error::NotSignedIn) => tab.show(false, &signed_out(), true),
				Err(error) => tab.show(true, &describe(&error), true),
			}
		});
	}
}

fn signed_out() -> String {
	// TRANSLATORS: Shown on the Scribe tab of Settings when the reader isn't signed in
	t("You're not signed in to Scribe.")
}

fn signed_in(account: &AccountInfo, pages: i64) -> String {
	let name = [account.first_name.as_deref(), account.last_name.as_deref()]
		.into_iter()
		.flatten()
		.collect::<Vec<_>>()
		.join(" ");
	let name = if name.trim().is_empty() { account.email.clone().unwrap_or_default() } else { name };
	// TRANSLATORS: Shown on the Scribe tab of Settings. %s is the reader's name or email address; %d is how many pages they can still convert. Plural form is chosen by that number.
	nt("Signed in as %s. %d page left.", "Signed in as %s. %d pages left.", pages.unsigned_abs())
		.replacen("%s", &name, 1)
		.replacen("%d", &pages.to_string(), 1)
}

/// Scribe's own errors come in the reader's language from the server, and the rest are Paperback's.
fn describe(error: &Error) -> String {
	match error {
		Error::NotSignedIn => signed_out(),
		// TRANSLATORS: Shown on the Scribe tab of Settings when the reader turns down the sign-in in the browser
		Error::Cancelled => t("Sign-in cancelled."),
		// TRANSLATORS: Shown on the Scribe tab of Settings when the browser comes back with an answer that doesn't belong to this sign-in
		Error::WrongState | Error::NoCode => t("The sign-in didn't finish. Please try again."),
		Error::Scribe(_) | Error::Failed(_) => error.to_string(),
	}
}
