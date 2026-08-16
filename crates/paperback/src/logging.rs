use std::{
	fs::{self, File},
	path::Path,
};

use tracing_appender::non_blocking;
pub use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt};

/// Initialise file logging. Returns a guard that must be kept alive for the
/// duration of the process; dropping it flushes and closes the log file.
///
/// Logs go to `paperback.log`, truncated at the start of each run. The
/// previous run's log is kept alongside it as `paperback.log.1`, so only the
/// last two sessions are ever on disk.
pub fn init(log_dir: &Path) -> Option<WorkerGuard> {
	if let Err(e) = fs::create_dir_all(log_dir) {
		eprintln!("paperback: could not create log directory: {e}");
		return None;
	}
	let current = log_dir.join("paperback.log");
	let previous = log_dir.join("paperback.log.1");
	let _ = fs::rename(&current, &previous);
	let file = match File::create(&current) {
		Ok(file) => file,
		Err(e) => {
			eprintln!("paperback: could not initialise log file: {e}");
			return None;
		}
	};
	let (writer, guard) = non_blocking(file);
	let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
	fmt().with_writer(writer).with_env_filter(filter).with_ansi(false).init();
	Some(guard)
}
