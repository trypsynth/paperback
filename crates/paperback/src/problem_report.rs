//! Problem reports, sent through mailbag to a server that emails them to the developer.

use std::sync::{Mutex, MutexGuard, PoisonError};

use logkeeper::Logkeeper;
use mailbag_wx::{Reporter, Session};
use paperback_core::version;
use wxdragon::prelude::WxWidget;

use crate::config_ext;

const ENDPOINT: &str = "https://reports.quinbox.xyz/v1/report";

static SESSION: Mutex<Option<Session>> = Mutex::new(None);

fn session() -> MutexGuard<'static, Option<Session>> {
	SESSION.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Marks this run as running. Called early in `main`, so a crash during startup counts.
pub fn begin_session() {
	*session() = Some(Session::begin(config_ext::config_dir(), "paperback"));
}

/// Records a normal exit. Every path that ends the process normally calls it, since one that doesn't is offered as a crash at the next start.
pub fn end_session() {
	let taken = session().take();
	if let Some(session) = taken {
		session.end();
	}
}

/// Asks whether to report the problem if the last run ended unexpectedly.
pub fn offer_after_crash(parent: &dyn WxWidget) {
	// Taken out so the lock isn't held while the question and the dialog are modal.
	let Some(taken) = session().take() else { return };
	reporter().offer_after_crash(parent, &taken);
	*session() = Some(taken);
}

/// The Report a Problem dialog, from the Help menu.
pub fn show(parent: &dyn WxWidget) {
	reporter().show(parent, false);
}

fn reporter() -> Reporter {
	// The same paths logkeeper writes, so the report attaches this run's log and, after a crash, the crashed run's.
	let logs = Logkeeper::new(config_ext::config_dir(), "paperback");
	Reporter::new("Paperback", env!("CARGO_PKG_VERSION"), ENDPOINT)
		.with_commit(version::COMMIT_HASH)
		.with_log(logs.log_path())
		.with_log(logs.previous_log_path())
}
