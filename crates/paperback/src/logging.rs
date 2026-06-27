use std::{fs, path::Path};

pub use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{EnvFilter, fmt};

/// Initialise file logging.  Returns a guard that must be kept alive for the
/// duration of the process — dropping it flushes and closes the log file.
pub fn init(log_dir: &Path) -> Option<WorkerGuard> {
	if let Err(e) = fs::create_dir_all(log_dir) {
		eprintln!("paperback: could not create log directory: {e}");
		return None;
	}
	let appender = rolling::never(log_dir, "paperback.log");
	let (writer, guard) = non_blocking(appender);
	let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
	fmt().with_writer(writer).with_env_filter(filter).with_ansi(false).init();
	Some(guard)
}
