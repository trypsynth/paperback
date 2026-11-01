//! The patois locale is process-global and tests run as threads, so a locale switch once made other tests fail on CI with `"2 heures"`.
//! Hold [`lock`] while the locale matters, and pin it with [`pinned_to`], since the lock alone doesn't say what the locale is.

use std::sync::{Mutex, MutexGuard};

static LOCALE_LOCK: Mutex<()> = Mutex::new(());

/// Poison is ignored: the data is `()`, and one real failure shouldn't cascade into every locale test.
pub fn lock() -> MutexGuard<'static, ()> {
	LOCALE_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Restores in `Drop` so a failing assertion doesn't strand the rest of the run in another language.
pub struct LocaleGuard {
	previous: String,
	_guard: MutexGuard<'static, ()>,
}

impl Drop for LocaleGuard {
	fn drop(&mut self) {
		patois::set_locale(&self.previous);
	}
}

#[must_use]
pub fn pinned_to(locale: &str) -> LocaleGuard {
	let guard = lock();
	let previous = patois::get_locale();
	patois::set_locale(locale);
	LocaleGuard { previous, _guard: guard }
}
