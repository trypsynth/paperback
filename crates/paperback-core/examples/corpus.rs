//! Snapshots how Paperback reads every document in a folder, so a change to a parser or a library under it shows up as a diff rather than as a reader's bug report, and a file that panics, hangs or eats memory shows up at all.
//!
//! `cargo run -p paperback-core --release --example corpus -- <folder> [<subfolder>...] [--bless] [--jobs N] [--timeout SECS] [--memory MB]`; an encrypted file reads its password from a `<file>.password` beside it. Each file is read in a child process so one bad file can't take the run down, and `<folder>/report.tsv` lists every file's outcome, time and peak memory.
//!
//! The documents themselves are never committed: most of them are other people's.

use std::{
	env,
	fmt::Write as _,
	fs,
	io::Read as _,
	path::{Path, PathBuf},
	process::{Command, ExitCode, Stdio},
	sync::{
		Mutex,
		atomic::{AtomicUsize, Ordering},
	},
	thread,
	time::{Duration, Instant},
};

use paperback_core::{
	document::{Document, MarkerType, ParserContext, TocItem},
	parser::parse_document,
};

fn main() -> ExitCode {
	let args: Vec<String> = env::args().skip(1).collect();
	if let [flag, path] = args.as_slice()
		&& flag == "--one"
	{
		let dump = match read(Path::new(path)) {
			Ok(document) => dump(&document),
			Err(error) => format!("error: {error}\n"),
		};
		print!("{dump}");
		return ExitCode::SUCCESS;
	}
	let mut options = Options {
		bless: false,
		jobs: thread::available_parallelism().map_or(4, |n| n.get() / 2).max(1),
		timeout: Duration::from_secs(120),
		memory_mb: 4096,
	};
	let mut paths = Vec::new();
	let mut rest = args.iter();
	while let Some(arg) = rest.next() {
		let mut value = || rest.next().and_then(|value| value.parse::<u64>().ok());
		match arg.as_str() {
			"--bless" => options.bless = true,
			"--jobs" => {
				options.jobs =
					value().and_then(|jobs| usize::try_from(jobs).ok()).map_or(options.jobs, |jobs| jobs.max(1));
			}
			"--timeout" => options.timeout = value().map_or(options.timeout, Duration::from_secs),
			"--memory" => options.memory_mb = value().unwrap_or(options.memory_mb),
			_ => paths.push(PathBuf::from(arg)),
		}
	}
	let Some((folder, subfolders)) = paths.split_first() else {
		eprintln!("usage: corpus <folder> [<subfolder>...] [--bless] [--jobs N] [--timeout SECS] [--memory MB]");
		return ExitCode::FAILURE;
	};
	let snapshots = folder.join("snapshots");
	let mut files = Vec::new();
	if subfolders.is_empty() {
		collect(folder, &snapshots, &mut files);
	}
	for subfolder in subfolders {
		collect(subfolder, &snapshots, &mut files);
	}
	files.sort();
	let next = AtomicUsize::new(0);
	let results = Mutex::new(Vec::with_capacity(files.len()));
	thread::scope(|scope| {
		for _ in 0..options.jobs {
			scope.spawn(|| {
				while let Some(file) = files.get(next.fetch_add(1, Ordering::Relaxed)) {
					let relative = file.strip_prefix(folder).unwrap_or(file).to_path_buf();
					let outcome = run_one(file, &options);
					let status = compare(&snapshot_path(&snapshots, &relative), &outcome, options.bless);
					println!(
						"{:>6} ms {:>6} MB  {:<8} {status:<24} {}",
						outcome.millis,
						outcome.peak_mb.map_or_else(|| "?".to_string(), |mb| mb.to_string()),
						outcome.kind,
						relative.display()
					);
					results.lock().unwrap().push((relative, outcome, status));
				}
			});
		}
	});
	let mut results = results.into_inner().unwrap();
	results.sort_by(|a, b| a.0.cmp(&b.0));
	let mut report = String::from("path\tstatus\tsnapshot\tms\tpeak_mb\tchars\tdetail\n");
	for (relative, outcome, status) in &results {
		let _ = writeln!(
			report,
			"{}\t{}\t{status}\t{}\t{}\t{}\t{}",
			relative.display(),
			outcome.kind,
			outcome.millis,
			outcome.peak_mb.map_or_else(String::new, |mb| mb.to_string()),
			outcome.chars,
			outcome.detail.replace(['\t', '\n', '\r'], " ")
		);
	}
	let _ = fs::write(folder.join("report.tsv"), report);
	let count = |kind: &str| results.iter().filter(|(_, outcome, _)| outcome.kind == kind).count();
	let changed = results.iter().filter(|(_, _, status)| status.starts_with("CHANGED") || status == "blessed").count();
	let new = results.iter().filter(|(_, _, status)| status == "new").count();
	println!(
		"{} files: {} same, {changed} changed, {new} new; {} read, {} failed to open, {} panicked, {} timed out, {} out of memory, {} crashed",
		results.len(),
		results.len() - changed - new,
		count("ok"),
		count("error"),
		count("panic"),
		count("timeout"),
		count("memory"),
		count("crash")
	);
	if changed > 0 && !options.bless { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

struct Options {
	bless: bool,
	jobs: usize,
	timeout: Duration,
	memory_mb: u64,
}

struct Outcome {
	kind: &'static str,
	dump: String,
	detail: String,
	millis: u128,
	peak_mb: Option<u64>,
	chars: usize,
}

fn collect(folder: &Path, snapshots: &Path, out: &mut Vec<PathBuf>) {
	let Ok(entries) = fs::read_dir(folder) else { return };
	for entry in entries.flatten() {
		let path = entry.path();
		if path == snapshots {
			continue;
		}
		if path.is_dir() {
			collect(&path, snapshots, out);
		} else if path.extension().and_then(|extension| extension.to_str()).is_some_and(|extension| {
			paperback_formats::ALL
				.iter()
				.any(|format| format.extensions.iter().any(|known| known.eq_ignore_ascii_case(extension)))
		}) && path.file_name().is_none_or(|name| name != "SOURCES.md")
		{
			out.push(path);
		}
	}
}

/// PDF snapshots keep the names they had when this only read PDFs; every other format appends, so `a.doc` and `a.txt` don't share one.
fn snapshot_path(snapshots: &Path, relative: &Path) -> PathBuf {
	if relative.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("pdf")) {
		snapshots.join(relative).with_extension("txt")
	} else {
		let mut path = snapshots.join(relative).into_os_string();
		path.push(".txt");
		PathBuf::from(path)
	}
}

fn run_one(file: &Path, options: &Options) -> Outcome {
	let started = Instant::now();
	let failed = |kind, detail: String| Outcome {
		kind,
		dump: format!("{kind}: {detail}\n"),
		detail,
		millis: started.elapsed().as_millis(),
		peak_mb: None,
		chars: 0,
	};
	let mut child = match Command::new(env::current_exe().unwrap())
		.arg("--one")
		.arg(file)
		.stdin(Stdio::null())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
	{
		Ok(child) => child,
		Err(error) => return failed("crash", format!("could not start: {error}")),
	};
	let job = limit::Job::new(options.memory_mb * 1024 * 1024, &child);
	let mut stdout = child.stdout.take().unwrap();
	let mut stderr = child.stderr.take().unwrap();
	// Drained on their own threads, or a document's text fills the pipe and the child blocks forever.
	let out_reader = thread::spawn(move || {
		let mut text = String::new();
		let _ = stdout.read_to_string(&mut text);
		text
	});
	let err_reader = thread::spawn(move || {
		let mut text = String::new();
		let _ = stderr.read_to_string(&mut text);
		text
	});
	let status = loop {
		match child.try_wait() {
			Ok(Some(status)) => break Some(status),
			Ok(None) if started.elapsed() > options.timeout => {
				let _ = child.kill();
				let _ = child.wait();
				break None;
			}
			Ok(None) => thread::sleep(Duration::from_millis(10)),
			Err(_) => break None,
		}
	};
	let millis = started.elapsed().as_millis();
	let dump = out_reader.join().unwrap_or_default();
	let stderr = err_reader.join().unwrap_or_default();
	let peak_mb = job.as_ref().map(|job| job.peak_bytes() / (1024 * 1024));
	let (kind, detail) = match status {
		None => ("timeout", format!("killed after {} s", options.timeout.as_secs())),
		Some(status) if status.success() && dump.starts_with("error: ") => {
			("error", dump.lines().next().unwrap_or_default().trim_start_matches("error: ").to_string())
		}
		Some(status) if status.success() => ("ok", String::new()),
		Some(_) if stderr.contains("memory allocation of") => ("memory", format!("over {} MB", options.memory_mb)),
		Some(status) => {
			let panic =
				stderr.lines().skip_while(|line| !line.contains("panicked at")).take(2).collect::<Vec<_>>().join(" ");
			if panic.is_empty() {
				("crash", format!("{status}: {}", stderr.lines().last().unwrap_or_default()))
			} else {
				("panic", panic)
			}
		}
	};
	let dump = if kind == "ok" || kind == "error" { dump } else { format!("{kind}: {detail}\n") };
	let chars = if kind == "ok" { dump.split_once("\ntext:\n").map_or(0, |(_, text)| text.chars().count()) } else { 0 };
	Outcome { kind, dump, detail, millis, peak_mb, chars }
}

fn compare(snapshot: &Path, outcome: &Outcome, bless: bool) -> String {
	if let Some(parent) = snapshot.parent() {
		let _ = fs::create_dir_all(parent);
	}
	let mut pending = snapshot.as_os_str().to_owned();
	pending.push(".new");
	match fs::read_to_string(snapshot) {
		Ok(previous) if previous == outcome.dump => {
			let _ = fs::remove_file(&pending);
			"same".to_string()
		}
		Ok(previous) if !bless => {
			let _ = fs::write(&pending, &outcome.dump);
			format!("CHANGED at line {}", first_difference(&previous, &outcome.dump))
		}
		Ok(_) => {
			let _ = fs::write(snapshot, &outcome.dump);
			let _ = fs::remove_file(&pending);
			"blessed".to_string()
		}
		Err(_) => {
			let _ = fs::write(snapshot, &outcome.dump);
			"new".to_string()
		}
	}
}

#[cfg(windows)]
mod limit {
	use std::{os::windows::io::AsRawHandle, process::Child};

	use windows::Win32::{
		Foundation::{CloseHandle, HANDLE},
		System::JobObjects::{
			AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
			JOB_OBJECT_LIMIT_PROCESS_MEMORY, JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
			QueryInformationJobObject, SetInformationJobObject,
		},
	};

	/// A job per child, so its peak memory is that one file's.
	pub struct Job(HANDLE);

	impl Job {
		pub fn new(memory_bytes: u64, child: &Child) -> Option<Self> {
			let job = Self(unsafe { CreateJobObjectW(None, None) }.ok()?);
			let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
			info.BasicLimitInformation.LimitFlags =
				JOB_OBJECT_LIMIT_PROCESS_MEMORY | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
			info.ProcessMemoryLimit = usize::try_from(memory_bytes).ok()?;
			let size = u32::try_from(size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>()).ok()?;
			unsafe {
				SetInformationJobObject(job.0, JobObjectExtendedLimitInformation, (&raw const info).cast(), size)
					.ok()?;
				AssignProcessToJobObject(job.0, HANDLE(child.as_raw_handle())).ok()?;
			}
			Some(job)
		}

		pub fn peak_bytes(&self) -> u64 {
			let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
			let size = u32::try_from(size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>()).unwrap_or_default();
			let read = unsafe {
				QueryInformationJobObject(
					Some(self.0),
					JobObjectExtendedLimitInformation,
					(&raw mut info).cast(),
					size,
					None,
				)
			};
			if read.is_ok() { info.PeakProcessMemoryUsed as u64 } else { 0 }
		}
	}

	impl Drop for Job {
		fn drop(&mut self) {
			let _ = unsafe { CloseHandle(self.0) };
		}
	}
}

#[cfg(not(windows))]
mod limit {
	use std::process::Child;

	/// Memory is neither capped nor measured off Windows.
	pub struct Job;

	#[allow(clippy::unnecessary_wraps, clippy::unused_self, reason = "keeps the Windows job's interface")]
	impl Job {
		pub fn new(_memory_bytes: u64, _child: &Child) -> Option<Self> {
			None
		}

		pub fn peak_bytes(&self) -> u64 {
			0
		}
	}
}

fn read(path: &Path) -> anyhow::Result<Document> {
	let mut context = ParserContext::new(path.to_string_lossy().into_owned());
	let mut password_file = path.as_os_str().to_owned();
	password_file.push(".password");
	if let Ok(password) = fs::read_to_string(password_file) {
		context = context.with_password(password.trim_end_matches(['\r', '\n']).to_string());
	}
	parse_document(&context)
}

fn dump(document: &Document) -> String {
	let mut out = format!("title: {}\nauthor: {}\ntoc:\n", document.title, document.author);
	dump_toc(&document.toc_items, 1, &mut out);
	out.push_str("text:\n");
	out.push_str(&annotated_text(document));
	if !out.ends_with('\n') {
		out.push('\n');
	}
	out
}

fn dump_toc(items: &[TocItem], depth: usize, out: &mut String) {
	for item in items {
		let _ = writeln!(out, "{}{} @{}", "  ".repeat(depth), item.name, item.offset);
		dump_toc(&item.children, depth + 1, out);
	}
}

fn annotated_text(document: &Document) -> String {
	let buffer = &document.buffer;
	let mut tags = Vec::new();
	for (order, marker) in buffer.markers.iter().enumerate() {
		let kind = kind(marker.mtype);
		let mut open = format!("⟦{kind}");
		if marker.level != 0 {
			let _ = write!(open, " level={}", marker.level);
		}
		if !marker.reference.is_empty() {
			let _ = write!(open, " ref={}", marker.reference);
		}
		open.push('⟧');
		tags.push(Tag { at: marker.position.get(), opens: true, order, text: open });
		if !marker.length.is_zero() {
			let at = (marker.position + marker.length).get();
			tags.push(Tag { at, opens: false, order, text: format!("⟦/{kind}⟧") });
		}
	}
	// A marker that ends where the next one starts closes first, so the two never read as nested.
	tags.sort_by_key(|tag| (tag.at, tag.opens, tag.order));
	let mut out = String::with_capacity(buffer.content.len() + tags.len() * 12);
	let mut pending = tags.iter().peekable();
	for (index, ch) in buffer.content.chars().enumerate() {
		while let Some(tag) = pending.next_if(|tag| tag.at <= index) {
			out.push_str(&tag.text);
		}
		out.push(ch);
	}
	for tag in pending {
		out.push_str(&tag.text);
	}
	out
}

struct Tag {
	at: usize,
	opens: bool,
	order: usize,
	text: String,
}

fn kind(mtype: MarkerType) -> String {
	format!("{mtype:?}").to_lowercase()
}

fn first_difference(previous: &str, current: &str) -> usize {
	previous
		.lines()
		.zip(current.lines())
		.position(|(a, b)| a != b)
		.unwrap_or_else(|| previous.lines().count().min(current.lines().count()))
		+ 1
}
