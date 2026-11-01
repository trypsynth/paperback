use std::{
	env,
	fs::{self, File},
	io::{self, Write},
	path::{Path, PathBuf},
	sync::OnceLock,
};

use tracing_appender::non_blocking;
pub use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt, reload, util::SubscriberInitExt};

use crate::config_ext::LogLevel;

static FILTER: OnceLock<reload::Handle<EnvFilter, Registry>> = OnceLock::new();

/// Created on the first line written, so with logging off no file is made at all.
struct LazyLogFile {
	dir: PathBuf,
	file: Option<File>,
	failed: bool,
}

impl LazyLogFile {
	fn open(&mut self) -> io::Result<&mut File> {
		if self.file.is_none() {
			fs::create_dir_all(&self.dir)?;
			let current = self.dir.join("paperback.log");
			let _ = fs::rename(&current, self.dir.join("paperback.log.1"));
			self.file = Some(File::create(current)?);
		}
		Ok(self.file.as_mut().expect("the log file was just created"))
	}
}

impl Write for LazyLogFile {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		// Given up after the first failure rather than retried on every line.
		if self.failed {
			return Ok(buf.len());
		}
		match self.open() {
			Ok(file) => file.write(buf),
			Err(e) => {
				self.failed = true;
				eprintln!("paperback: could not create the log file: {e}");
				Ok(buf.len())
			}
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		self.file.as_mut().map_or(Ok(()), File::flush)
	}
}

/// `RUST_LOG` wins, so a filter set by hand for debugging is never overridden by the setting.
fn filter_for(level: LogLevel) -> EnvFilter {
	EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.filter()))
}

/// Keep the guard alive for the whole process; dropping it flushes and closes the log.
pub fn init(log_dir: &Path, level: LogLevel) -> WorkerGuard {
	// Otherwise the last two runs' logs would stay on disk indefinitely.
	if level == LogLevel::Off && env::var_os("RUST_LOG").is_none() {
		let _ = fs::remove_file(log_dir.join("paperback.log"));
		let _ = fs::remove_file(log_dir.join("paperback.log.1"));
	}
	let (writer, guard) = non_blocking(LazyLogFile { dir: log_dir.to_path_buf(), file: None, failed: false });
	let (filter, handle) = reload::Layer::new(filter_for(level));
	let _ = FILTER.set(handle);
	let layer = fmt::layer().with_writer(writer).with_ansi(false);
	let _ = tracing_subscriber::registry().with(filter).with(layer).try_init();
	guard
}

pub fn set_level(level: LogLevel) {
	if let Some(handle) = FILTER.get() {
		let _ = handle.reload(filter_for(level));
	}
}
