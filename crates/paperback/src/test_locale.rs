//! Serialization for tests that depend on the active patois locale.
//!
//! patois keeps the locale in a process-global registry, and cargo runs a crate's tests as
//! threads in one process, so a test that switches locale changes what every other test sees
//! while it runs. That is not hypothetical: `duration_format`'s tests assert English text and
//! began failing on CI with `"2 heures"`, and with `"1 heure, 1 minute, 5 seconds"`, which is
//! the locale flipping back to English between two `nt()` calls inside one call.
//!
//! Any test that reads or writes the locale must hold [`lock`] for as long as it cares about
//! the answer. Reading tests should also pin the locale themselves via [`pinned_to`], since
//! holding the lock only keeps other locale tests out; it does not say what the locale is.

use std::sync::{Mutex, MutexGuard};

static LOCALE_LOCK: Mutex<()> = Mutex::new(());

/// Serializes locale-dependent tests against each other.
///
/// A test that panics while holding this poisons the mutex; the poison is deliberately ignored,
/// because the data behind it is `()` and the alternative is one genuine failure cascading into
/// every other locale test.
pub fn lock() -> MutexGuard<'static, ()> {
	LOCALE_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Holds the locale lock and sets the locale to `locale`, restoring the previous one on drop.
///
/// Restoring from `Drop` rather than at the end of the test body means a failing assertion
/// leaves the locale as it found it instead of stranding the rest of the run in another
/// language.
pub struct LocaleGuard {
	previous: String,
	_guard: MutexGuard<'static, ()>,
}

impl Drop for LocaleGuard {
	fn drop(&mut self) {
		patois::set_locale(&self.previous);
	}
}

/// Pins the active locale for the lifetime of the returned guard.
#[must_use]
pub fn pinned_to(locale: &str) -> LocaleGuard {
	let guard = lock();
	let previous = patois::get_locale();
	patois::set_locale(locale);
	LocaleGuard { previous, _guard: guard }
}
