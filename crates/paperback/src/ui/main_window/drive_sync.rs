mod oauth;

use std::{
	cell::RefCell,
	collections::VecDeque,
	path::{Path, PathBuf},
	rc::Rc,
	sync::{Mutex, mpsc},
	thread,
	time::{Duration, Instant},
};

use anyhow::Context;
use oauth::{AccessToken, Authorization, Client};
use paperback_core::{
	book_sync::{
		CloudBook, CloudBookKey, Command, DesktopBookTransfer, DriveSync, ENABLED_SETTING, LocalBook, ResumeChoice,
		SyncReport,
	},
	config::ConfigManager,
};
use patois::t;
use wxdragon::{prelude::*, timer::Timer};

use super::{DocumentManager, ensure_parser_ready_for_path, menu_ids, rebuild_menu_bar, update_title_from_manager};
use crate::{config_ext::config_dir, ui::navigation::set_caret_to_doc_offset};

enum ResultReady {
	Authorization(anyhow::Result<AccessToken>),
	Synchronized(anyhow::Result<(AccessToken, SyncReport)>),
}

#[derive(Default)]
struct UiState {
	enabled: bool,
	authorization_expired: bool,
	needs_refresh: bool,
	running: Option<mpsc::Receiver<ResultReady>>,
	commands: VecDeque<(Command, Option<String>)>,
	open_after: Option<String>,
	token: Option<AccessToken>,
	report: Option<SyncReport>,
	previous: Vec<LocalBook>,
	last_poll: Option<Instant>,
	last_scan: Option<Instant>,
	last_success: Option<Instant>,
	last_attempt: Option<Instant>,
}

pub(in crate::ui) struct DriveWindow {
	frame: Frame,
	config: Rc<Mutex<ConfigManager>>,
	dm: Rc<Mutex<DocumentManager>>,
	root: PathBuf,
	credentials: PathBuf,
	state: RefCell<UiState>,
}

impl DriveWindow {
	pub(super) fn bind(
		frame: &Frame,
		dm: &Rc<Mutex<DocumentManager>>,
		config: &Rc<Mutex<ConfigManager>>,
		timers: &mut Vec<Rc<Timer<Frame>>>,
	) -> Rc<Self> {
		let root = config_dir().join("google-drive");
		let drive = Rc::new(Self {
			frame: *frame,
			dm: Rc::clone(dm),
			config: Rc::clone(config),
			credentials: root.join("oauth.dat"),
			root,
			state: RefCell::new(UiState::default()),
		});
		let timer = Rc::new(Timer::new(frame));
		let receiving = Rc::clone(&drive);
		timer.on_tick(move |_| receiving.tick());
		timer.start(250, false);
		timers.push(timer);
		let activated = Rc::clone(&drive);
		frame.on_activate(move |event| {
			event.skip(true);
			if let WindowEventData::Activate(activate) = &event
				&& activate.is_active()
			{
				activated.state.borrow_mut().needs_refresh = true;
			}
		});
		drive
	}

	pub(super) fn handle(&self, id: i32) -> bool {
		match id {
			menu_ids::DRIVE_UPLOAD_BOOK => {
				self.upload_active();
				true
			}
			menu_ids::DRIVE_LIBRARY => {
				self.library();
				true
			}
			_ => false,
		}
	}

	fn tick(&self) {
		{
			let mut state = self.state.borrow_mut();
			if state.last_poll.is_some_and(|last| last.elapsed() < Duration::from_millis(200)) {
				return;
			}
			state.last_poll = Some(Instant::now());
		}
		self.receive();
		let Ok(config) = self.config.try_lock() else { return };
		let enabled = config.get_app_bool(ENABLED_SETTING, false);
		drop(config);
		let changed = {
			let mut state = self.state.borrow_mut();
			let previous = state.enabled;
			state.enabled = enabled;
			if !enabled {
				state.commands.clear();
			}
			enabled && !previous
		};
		if changed {
			self.state.borrow_mut().needs_refresh = true;
		}
		if !enabled || !self.is_connected() {
			return;
		}
		let work = {
			let mut state = self.state.borrow_mut();
			if state.running.is_some() {
				return;
			}
			if let Some(explicit) = state.commands.pop_front() {
				Some(explicit)
			} else {
				let scan = state.last_scan.is_none_or(|last| last.elapsed() >= Duration::from_secs(3));
				let changed = if scan {
					state.last_scan = Some(Instant::now());
					let Ok(config) = self.config.try_lock() else { return };
					config.drive_book_snapshot() != state.previous
				} else {
					false
				};
				let retry =
					state.last_attempt.zip(state.last_success).is_none_or(|(attempt, success)| attempt > success);
				let failed_files = state.report.as_ref().is_some_and(|report| report.file_error);
				let waiting = state.report.as_ref().is_some_and(|report| report.files_waiting);
				let retry_due = state.last_attempt.is_none_or(|last| last.elapsed() >= Duration::from_secs(20));
				let refresh = state.needs_refresh
					&& state.last_attempt.is_none_or(|last| last.elapsed() >= Duration::from_secs(3));
				if changed || refresh || ((failed_files || retry) && retry_due) {
					Some((Command::Refresh, None))
				} else if waiting && !failed_files && !retry {
					Some((Command::ContinueFiles, None))
				} else {
					None
				}
			}
		};
		if let Some((command, open)) = work {
			self.start(command, open);
		}
	}

	pub(in crate::ui) fn is_connected(&self) -> bool {
		self.credentials.is_file() && !self.state.borrow().authorization_expired
	}

	fn require_connection(&self, parent: &dyn WxWidget) -> bool {
		if self.is_connected() {
			return true;
		}
		let dialog =
			MessageDialog::builder(parent, &t("No Google account is connected."), &t("Google Drive synchronization"))
				.with_style(MessageDialogStyle::YesNo | MessageDialogStyle::NoDefault)
				.build();
		dialog.set_yes_no_labels(&t("Connect Google account"), &t("Close"));
		if dialog.show_modal() == ID_YES {
			self.connect(parent);
		}
		false
	}

	pub(in crate::ui) fn connect(&self, parent: &dyn WxWidget) {
		if self.state.borrow().running.is_some() {
			return;
		}
		let Some(client) = Client::bundled() else {
			drive_message(parent, &t("Google Drive is unavailable in this build of Paperback."));
			return;
		};
		let Ok(authorization) = Authorization::start(client) else {
			drive_message(parent, &t("Google Drive could not synchronize. You can retry."));
			return;
		};
		if !wxdragon::utils::launch_default_browser(&authorization.url, wxdragon::utils::BrowserLaunchFlags::Default) {
			drive_message(parent, &t("Could not open the browser for Google Drive."));
			return;
		}
		let credentials = self.credentials.clone();
		if std::fs::create_dir_all(&self.root).is_err() {
			drive_message(parent, &t("Google Drive could not synchronize. You can retry."));
			return;
		}
		let (send, receive) = mpsc::channel();
		self.state.borrow_mut().running = Some(receive);
		thread::spawn(move || {
			let result = authorization.complete(credentials);
			let _ = send.send(ResultReady::Authorization(result));
		});
	}

	fn enqueue(&self, command: Command, open: Option<String>) {
		self.state.borrow_mut().commands.push_back((command, open));
		self.tick();
	}

	fn start(&self, command: Command, open: Option<String>) {
		let Ok(config) = self.config.try_lock() else {
			self.state.borrow_mut().commands.push_front((command, open));
			return;
		};
		let books = config.drive_book_snapshot();
		drop(config);
		let mut state = self.state.borrow_mut();
		let token = state.token.clone().filter(AccessToken::fresh);
		let root = self.root.clone();
		let credentials = self.credentials.clone();
		let (send, receive) = mpsc::channel();
		if !matches!(command, Command::ContinueFiles) {
			state.previous.clone_from(&books);
		}
		state.running = Some(receive);
		if open.is_some() {
			state.open_after = open;
		}
		if !matches!(command, Command::ContinueFiles) {
			state.needs_refresh = false;
		}
		state.last_attempt = Some(Instant::now());
		thread::spawn(move || {
			let result = (|| -> anyhow::Result<_> {
				let token = match token {
					Some(token) => token,
					None => Client::bundled().context("Google Drive OAuth client missing")?.refresh(&credentials)?,
				};
				let report = DriveSync::new(root, "Windows".into()).sync(
					&token.value,
					books,
					&command,
					&DesktopBookTransfer::default(),
				)?;
				Ok((token, report))
			})();
			let _ = send.send(ResultReady::Synchronized(result));
		});
	}

	fn receive(&self) {
		if self.dm.try_lock().is_err() || self.config.try_lock().is_err() {
			return;
		}
		let result = {
			let mut state = self.state.borrow_mut();
			let Some(running) = state.running.as_ref() else { return };
			match running.try_recv() {
				Ok(result) => {
					state.running = None;
					Some(result)
				}
				Err(mpsc::TryRecvError::Empty) => None,
				Err(mpsc::TryRecvError::Disconnected) => {
					state.running = None;
					None
				}
			}
		};
		match result {
			Some(ResultReady::Authorization(Ok(token))) => {
				let mut state = self.state.borrow_mut();
				state.token = Some(token);
				state.authorization_expired = false;
				state.needs_refresh = true;
				state.report = None;
				state.commands.clear();
				state.open_after = None;
			}
			Some(ResultReady::Synchronized(Ok((token, report)))) => {
				let protected = self.dm.try_lock().map_or_else(|_| Vec::new(), |dm| dm.open_paths());
				if let Ok(config) = self.config.try_lock()
					&& config.get_app_bool(ENABLED_SETTING, false)
				{
					config.apply_drive_book_report(&report, &protected);
				}
				let open = {
					let mut state = self.state.borrow_mut();
					state.token = Some(token);
					state.last_success = Some(Instant::now());
					let waiting_for_open = state.open_after.as_ref().is_some_and(|id| {
						report.books.iter().any(|book| {
							&book.id == id
								&& book.cloud_bytes.is_some()
								&& book.offline_path.is_none()
								&& book.local_locator.is_none()
						})
					});
					let open = if waiting_for_open && !report.file_error { None } else { state.open_after.take() };
					state.report = Some(report);
					open
				};
				if let Some(id) = open {
					self.open(&id);
				}
			}
			Some(ResultReady::Synchronized(Err(error))) => {
				tracing::warn!(%error, "Google Drive synchronization failed");
				let open = {
					let mut state = self.state.borrow_mut();
					state.token = None;
					if matches!(error.downcast_ref::<ureq::Error>(), Some(ureq::Error::StatusCode(400 | 401))) {
						state.authorization_expired = true;
					}
					state.open_after.take()
				};
				if open.is_some() {
					self.message(&t("Google Drive could not synchronize. You can retry."));
				}
			}
			Some(ResultReady::Authorization(Err(error))) => {
				tracing::warn!(%error, "Google Drive authorization failed");
				self.message(&t("Connect to Google Drive to sync."));
			}
			None => {}
		}
	}

	fn upload_active(&self) {
		let Some(path) = self.dm.try_lock().ok().and_then(|dm| dm.active_tab().map(|tab| tab.file_path.clone())) else {
			if self.require_connection(&self.frame) {
				self.message(&t("Open a book first."));
			}
			return;
		};
		self.queue_book_upload(&self.frame, &path.to_string_lossy());
	}

	pub(in crate::ui) fn queue_book_upload(&self, parent: &dyn WxWidget, path: &str) {
		if !self.require_connection(parent) {
			return;
		}
		let account = self.state.borrow().report.as_ref().map(|report| report.account.clone());
		let Some(account) = account.filter(|key| !key.is_empty()) else {
			self.enqueue(Command::Refresh, None);
			drive_message(parent, &t("The Google Drive account is still being checked. Try again shortly."));
			return;
		};
		let title = t("Google Drive synchronization");
		let size = self.config.lock().unwrap().drive_book_file_size(path);
		if let Some(size) = size {
			let question = t("Upload the book file to Google Drive (%s)?").replace("%s", &format_bytes(size));
			if !wx_utils::confirm(parent, &question, &title) {
				return;
			}
			let queued = self.config.lock().unwrap().request_drive_book_file(path, &account, size);
			if queued {
				self.enqueue(Command::Refresh, None);
				return;
			}
		}
		drive_message(parent, &t("The book file is not available on this device yet."));
	}

	fn library(&self) {
		if !self.require_connection(&self.frame) {
			return;
		}
		if self.state.borrow().report.is_none() {
			let books = self.config.lock().unwrap().drive_book_snapshot();
			let report = DriveSync::new(self.root.clone(), "Windows".into())
				.offline_report(books, &DesktopBookTransfer::default());
			self.state.borrow_mut().report = report.ok();
		}
		let contents = self
			.state
			.borrow()
			.report
			.as_ref()
			.map(|report| (report.account.clone(), report.account_name.clone(), report.books.clone()));
		let Some((account_key, account, books)) = contents.filter(|(_, _, books)| !books.is_empty()) else {
			self.enqueue(Command::Refresh, None);
			self.message(&t("Cloud books will appear after synchronization."));
			return;
		};
		let size = books.iter().filter_map(|book| book.cloud_bytes).fold(0, u64::saturating_add);
		let message = t("Book files on Google Drive: %s").replace("%s", &format_bytes(size));
		let message = if account.is_empty() { message } else { format!("{account}\n{message}") };
		let labels: Vec<_> = books.iter().map(book_label).collect();
		if let Some(index) = self.choose(&t("Cloud books"), &message, &labels) {
			self.book_actions(account_key, &books[index]);
		}
	}

	fn book_actions(&self, account: String, book: &CloudBook) {
		let mut labels = vec![t("Resume")];
		let can_remove_copy =
			book.offline_path.as_deref().is_some_and(|path| self.dm.lock().unwrap().find_tab_by_path(path).is_none());
		if can_remove_copy {
			labels.push(t("Remove offline copy"));
		}
		if book.cloud_bytes.is_some() {
			labels.push(t("Remove cloud file"));
		}
		let Some(selected) = self.choose(&t("Cloud books"), &book.title, &labels) else { return };
		let key = CloudBookKey { account, id: book.id.clone() };
		if selected == 0 {
			if book.choices.is_empty() && (book.local_locator.is_some() || book.offline_path.is_some()) {
				if self.state.borrow().report.as_ref().is_some_and(|report| report.account == key.account) {
					self.open(&book.id);
				}
				self.enqueue(Command::Refresh, None);
				return;
			}
			let command = if book.choices.is_empty() {
				Command::Prioritize(key)
			} else {
				let labels: Vec<_> = book.choices.iter().map(position_label).collect();
				let Some(index) = self.choose(&t("Choose a place to resume"), &book.title, &labels) else { return };
				Command::ChoosePosition(key, book.choices[index].position)
			};
			self.enqueue(command, Some(book.id.clone()));
		} else if selected == 1 && can_remove_copy {
			self.enqueue(Command::RemoveOfflineFile(key), None);
		} else if self.confirm(&t("Remove book file from Google Drive?")) {
			self.enqueue(Command::RemoveCloudFile(key), None);
		}
	}

	fn open(&self, id: &str) {
		let path = {
			let state = self.state.borrow();
			let Some(report) = state.report.as_ref() else { return };
			let Some(book) = report.books.iter().find(|book| book.id == id) else { return };
			let Some(path) =
				book.local_locator.as_deref().or_else(|| book.offline_path.as_deref().and_then(Path::to_str))
			else {
				drop(state);
				self.message(&t("The book file is not available on this device yet."));
				return;
			};
			let applied = self.config.lock().unwrap().apply_drive_book_for_open(report, id, path);
			(path.to_owned(), applied.then_some(book.resume))
		};
		let (path, resume) = path;
		let path = Path::new(&path);
		if !ensure_parser_ready_for_path(&self.frame, path, &self.config) {
			return;
		}
		let mut dm = self.dm.lock().unwrap();
		if dm.open_file(&self.dm, path) {
			if let Some(resume) = resume
				&& let Some(tab) = dm.active_tab_mut()
			{
				set_caret_to_doc_offset(tab, resume.text);
				if let (Some(player), Some(ms)) = (tab.audio_player.as_mut(), resume.audio_ms) {
					player.seek_to_ms(ms);
				}
			}
			update_title_from_manager(&self.frame, &dm);
			dm.focus_document_text();
			drop(dm);
			rebuild_menu_bar(&self.frame, &self.dm, &self.config);
		}
	}

	fn choose(&self, title: &str, message: &str, options: &[String]) -> Option<usize> {
		let options: Vec<_> = options.iter().map(String::as_str).collect();
		let dialog = SingleChoiceDialog::builder(&self.frame, message, title, &options).build();
		(dialog.show_modal() == ID_OK)
			.then(|| usize::try_from(dialog.get_selection()).ok())
			.flatten()
			.filter(|&index| index < options.len())
	}

	fn message(&self, message: &str) {
		drive_message(&self.frame, message);
	}

	fn confirm(&self, message: &str) -> bool {
		MessageDialog::builder(&self.frame, message, &t("Google Drive synchronization"))
			.with_style(MessageDialogStyle::YesNo | MessageDialogStyle::NoDefault)
			.build()
			.show_modal()
			== ID_YES
	}
}

fn drive_message(parent: &dyn WxWidget, message: &str) {
	MessageDialog::builder(parent, message, &t("Google Drive synchronization"))
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation)
		.build()
		.show_modal();
}

fn book_label(book: &CloudBook) -> String {
	let status = if let Some(bytes) = book.cloud_bytes {
		let state = if book.local_locator.is_some() || book.offline_path.is_some() {
			t("Available offline")
		} else {
			t("On Google Drive")
		};
		format!("{state}, {}", format_bytes(bytes))
	} else {
		t("Book data only")
	};
	format!("{}, {status}", book.title)
}

fn position_label(choice: &ResumeChoice) -> String {
	let place = if let Some(ms) = choice.position.audio_ms {
		let seconds = ms / 1000;
		format!("{}:{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60)
	} else {
		t("Character %s").replace("%s", &choice.position.text.get().saturating_add(1).to_string())
	};
	format!("{}, {place}", choice.device_name)
}

fn format_bytes(bytes: u64) -> String {
	if bytes >= 1_048_576 {
		format!("{:.1} MiB", bytes as f64 / 1_048_576.0)
	} else {
		format!("{:.1} KiB", bytes as f64 / 1024.0)
	}
}
