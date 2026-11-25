mod drive;
mod state;

use std::{
	collections::{BTreeMap, HashMap},
	fmt::Write as _,
	fs::{self, File},
	io::{ErrorKind, Read, Write},
	path::{Path, PathBuf},
	time::SystemTime,
};

use anyhow::{Result, bail};
pub use drive::{BookTransfer, BookTransferRequest, BookTransferResult, DRIVE_SCOPES, DesktopBookTransfer};
use drive::{Drive, DriveFile};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use state::{PROTOCOL_VERSION, Replica, ReplicaOwner, merge, record};
pub use state::{ReadingData, ResumeChoice, ResumePosition};

pub const ENABLED_SETTING: &str = "google_drive_book_sync";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadApproval {
	account: String,
	size: u64,
	modified: SystemTime,
}

impl UploadApproval {
	pub fn new(account: &str, path: &Path, confirmed_size: u64) -> Option<Self> {
		if !valid_id(account) || !standalone_file(path) || confirmed_size == 0 {
			return None;
		}
		let metadata =
			path.metadata().ok().filter(|metadata| metadata.is_file() && metadata.len() == confirmed_size)?;
		Some(Self { account: account.into(), size: confirmed_size, modified: metadata.modified().ok()? })
	}

	fn allows(&self, account: &str, path: &Path) -> bool {
		self.account == account && Self::new(account, path, self.size).as_ref() == Some(self)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncBaseline {
	pub account: String,
	pub sha256: String,
	pub applied: ReadingData,
	pub revision: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalBook {
	pub key: String,
	pub locator: String,
	pub path: PathBuf,
	pub title: String,
	pub reading: ReadingData,
	pub baseline: Option<SyncBaseline>,
	pub upload_approval: Option<UploadApproval>,
}

struct IdentifiedBook {
	source: LocalBook,
	readable: bool,
	approved_upload: bool,
}

#[derive(Debug)]
pub struct SyncedReading {
	pub local_key: String,
	pub expected: ReadingData,
	pub baseline: SyncBaseline,
}

#[derive(Clone, Debug)]
pub struct CloudBook {
	pub id: String,
	pub title: String,
	pub local_locator: Option<String>,
	pub offline_path: Option<PathBuf>,
	pub cloud_bytes: Option<u64>,
	pub resume: ResumePosition,
	pub choices: Vec<ResumeChoice>,
	pub reading: ReadingData,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
#[derive(Debug, Default)]
pub struct SyncReport {
	pub account: String,
	pub account_name: String,
	pub books: Vec<CloudBook>,
	pub readings: Vec<SyncedReading>,
	pub files_waiting: bool,
	pub file_error: bool,
	pub(crate) finished_uploads: Vec<(String, UploadApproval)>,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Default)]
pub enum Command {
	#[default]
	Refresh,
	ContinueFiles,
	Prioritize(CloudBookKey),
	ChoosePosition(CloudBookKey, ResumePosition),
	RemoveCloudFile(CloudBookKey),
	RemoveOfflineFile(CloudBookKey),
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CloudBookKey {
	pub account: String,
	pub id: String,
}

impl Command {
	fn book(&self) -> Option<&CloudBookKey> {
		match self {
			Self::Prioritize(key)
			| Self::ChoosePosition(key, _)
			| Self::RemoveCloudFile(key)
			| Self::RemoveOfflineFile(key) => Some(key),
			Self::Refresh | Self::ContinueFiles => None,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
struct Fingerprint {
	path: PathBuf,
	size: u64,
	modified: SystemTime,
	hash: String,
}

#[derive(Default, Serialize, Deserialize)]
struct BookEntry {
	replicas: BTreeMap<String, Replica>,
	observed: Option<ReadingData>,
	applied_revision: Option<u64>,
	#[serde(default)]
	pending_metadata: bool,
	#[serde(default)]
	remote_id: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct AccountStore {
	books: BTreeMap<String, BookEntry>,
	fingerprints: HashMap<String, Fingerprint>,
	remote_versions: HashMap<String, String>,
	remote_files: BTreeMap<String, DriveFile>,
	pending_uploads: BTreeMap<String, PendingUpload>,
	#[serde(default)]
	do_not_prefetch: Vec<String>,
	#[serde(default)]
	priority: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct PendingUpload {
	path: PathBuf,
	session: Option<String>,
	#[serde(default)]
	verified: Option<(u64, SystemTime)>,
}

pub struct DriveSync {
	root: PathBuf,
	device_name: String,
}

impl DriveSync {
	pub const fn new(root: PathBuf, device_name: String) -> Self {
		Self { root, device_name }
	}

	pub fn sync(
		&self,
		token: &str,
		books: Vec<LocalBook>,
		command: &Command,
		transfer: &dyn BookTransfer,
	) -> Result<SyncReport> {
		self.sync_with_drive(&Drive::new(token), books, command, transfer)
	}

	fn sync_with_drive(
		&self,
		drive: &Drive<'_>,
		books: Vec<LocalBook>,
		command: &Command,
		transfer: &dyn BookTransfer,
	) -> Result<SyncReport> {
		let account = drive.account()?;
		let account_key = digest(account.id.as_bytes());
		if command.book().is_some_and(|book| book.account != account_key) {
			bail!("The Google Drive account changed before the book action could run");
		}
		let directory = self.root.join(&account_key);
		fs::create_dir_all(&directory)?;
		persist(&self.root.join("account.json"), &Some(&account_key))?;
		let device = device_id(&self.root)?;
		let state_path = directory.join("state.json");
		let mut store: AccountStore = read_state(&state_path)?;
		let continuing = matches!(command, Command::ContinueFiles);
		let local = identify_books(books, &account_key, &mut store.fingerprints, transfer);
		store.pending_uploads.retain(|id, upload| {
			local.get(id).is_some_and(|book| book.approved_upload && book.source.path == upload.path)
		});
		for (id, identified) in local.iter().filter(|_| !continuing) {
			let book = &identified.source;
			let entry = store.books.entry(id.clone()).or_default();
			let baseline =
				book.baseline.as_ref().filter(|baseline| baseline.account == account_key && baseline.sha256 == *id);
			let revision = baseline.map(|baseline| baseline.revision);
			let previous = if revision == entry.applied_revision {
				entry.observed.as_ref().or_else(|| baseline.map(|baseline| &baseline.applied))
			} else {
				baseline.map(|baseline| &baseline.applied)
			};
			let previous = previous.cloned().unwrap_or_default();
			let owner = ReplicaOwner { book: id, device: &device, name: &self.device_name, title: &book.title };
			entry.pending_metadata |= record(&mut entry.replicas, &owner, &previous, &book.reading, false);
			entry.observed = Some(book.reading.clone());
			entry.applied_revision = revision;
			if identified.approved_upload {
				store.pending_uploads.entry(id.clone()).or_insert_with(|| PendingUpload {
					path: book.path.clone(),
					session: None,
					verified: None,
				});
			}
		}
		persist(&state_path, &store)?;
		if !continuing {
			pull_replicas(drive, &mut store, &device)?;
		}
		if let Command::ChoosePosition(book, position) = command
			&& let Some(entry) = store.books.get_mut(&book.id)
		{
			let id = &book.id;
			let original = local.get(id).map_or_else(ReadingData::default, |book| book.source.reading.clone());
			let replacement = ReadingData { resume: *position, ..original.clone() };
			let title = entry.replicas.values().next().map_or_else(|| id.clone(), |replica| replica.title.clone());
			let owner = ReplicaOwner { book: id, device: &device, name: &self.device_name, title: &title };
			entry.pending_metadata |= record(&mut entry.replicas, &owner, &original, &replacement, true);
		}
		persist(&state_path, &store)?;
		for entry in store.books.values_mut().filter(|entry| !continuing && entry.pending_metadata) {
			if let Some(replica) = entry.replicas.get(&device) {
				let id = drive.put_replica(replica, entry.remote_id.as_deref())?;
				entry.remote_id = Some(id);
				entry.pending_metadata = false;
			}
		}
		persist(&state_path, &store)?;
		let files = update_files(drive, &mut store, &local, &directory, command, transfer);
		if let Err(error) = &files {
			tracing::warn!(%error, "Google Drive book transfer paused");
		}
		let saved = persist(&state_path, &store);
		if let Err(error) = &saved {
			tracing::warn!(%error, "could not save the sync state");
		}
		let mut report = make_report(&store, &account_key, account.name, &local, &directory);
		if let Command::RemoveCloudFile(book) = command
			&& files.is_ok()
			&& let Some(identified) = local.get(&book.id).filter(|book| book.approved_upload)
			&& let Some(approval) = &identified.source.upload_approval
		{
			report.finished_uploads.push((identified.source.key.clone(), approval.clone()));
		}
		if continuing {
			report.readings.clear();
		}
		report.file_error |= files.is_err() || saved.is_err();
		Ok(report)
	}

	pub fn offline_report(&self, books: Vec<LocalBook>, transfer: &dyn BookTransfer) -> Result<SyncReport> {
		let account: Option<String> = read_state(&self.root.join("account.json"))?;
		let Some(account) = account.filter(|key| valid_id(key)) else { return Ok(SyncReport::default()) };
		let directory = self.root.join(&account);
		let mut store: AccountStore = read_state(&directory.join("state.json"))?;
		let known = identify_books(books, &account, &mut store.fingerprints, transfer);
		Ok(make_report(&store, &account, String::new(), &known, &directory))
	}
}

fn pull_replicas(drive: &Drive, store: &mut AccountStore, device: &str) -> Result<()> {
	for item in drive.list_replicas()? {
		if store.remote_versions.get(&item.id) == Some(&item.version) {
			continue;
		}
		let replica = drive.get_replica(&item.id)?;
		if replica.version == PROTOCOL_VERSION && valid_id(&replica.book) && valid_id(&replica.device) {
			let entry = store.books.entry(replica.book.clone()).or_default();
			if replica.device == device && entry.remote_id.is_none() {
				entry.remote_id = Some(item.id.clone());
			}
			let current = entry.replicas.entry(replica.device.clone()).or_insert_with(|| replica.clone());
			if replica.generation > current.generation {
				*current = replica;
			}
		}
		store.remote_versions.insert(item.id, item.version);
	}
	Ok(())
}

fn update_files(
	drive: &Drive,
	store: &mut AccountStore,
	local: &BTreeMap<String, IdentifiedBook>,
	directory: &Path,
	command: &Command,
	transfer: &dyn BookTransfer,
) -> Result<()> {
	if !matches!(command, Command::ContinueFiles) {
		store.remote_files = drive.list_books()?;
	}
	if let Command::RemoveOfflineFile(book) = command
		&& let Some(file) = store.remote_files.get(&book.id)
	{
		let id = &book.id;
		let path = offline_path(directory, id, &file.name);
		remove_file_if_present(&path)?;
		if !store.do_not_prefetch.contains(id) {
			store.do_not_prefetch.push(id.clone());
		}
		if store.priority.as_ref() == Some(id) {
			store.priority = None;
		}
	}
	if let Command::RemoveCloudFile(book) = command {
		let id = &book.id;
		if let Some(file) = store.remote_files.get(id) {
			drive.trash(&file.id)?;
			store.remote_files.remove(id);
		}
		store.pending_uploads.remove(id);
	}
	if let Command::Prioritize(book) | Command::ChoosePosition(book, _) = command {
		store.do_not_prefetch.retain(|old| old != &book.id);
		store.priority = Some(book.id.clone());
	}
	store.pending_uploads.retain(|id, _| !store.remote_files.contains_key(id));
	if !transfer.available() {
		return Ok(());
	}
	fs::create_dir_all(directory.join("books"))?;
	if let Some(id) = store.priority.as_ref() {
		if let Some((file, path)) = missing_download(store, local, directory, id) {
			return drive.download_book(file, transfer, &path, id);
		}
		store.priority = None;
	}
	if let Some((id, upload)) = store.pending_uploads.iter_mut().find(|(_, upload)| upload.path.is_file()) {
		if let Some(file) = upload_if_unchanged(drive, transfer, id, upload)? {
			let id = id.clone();
			store.remote_files.insert(id.clone(), file);
			store.pending_uploads.remove(&id);
		}
		return Ok(());
	}
	for id in store.remote_files.keys() {
		if let Some((file, path)) = missing_download(store, local, directory, id) {
			return drive.download_book(file, transfer, &path, id);
		}
	}
	Ok(())
}

fn missing_download<'a>(
	store: &'a AccountStore,
	local: &BTreeMap<String, IdentifiedBook>,
	directory: &Path,
	id: &str,
) -> Option<(&'a DriveFile, PathBuf)> {
	let file = store.remote_files.get(id)?;
	if local.get(id).is_some_and(|book| book.readable) || store.do_not_prefetch.iter().any(|old| old == id) {
		return None;
	}
	let path = offline_path(directory, id, &file.name);
	(!path.is_file()).then_some((file, path))
}

fn upload_if_unchanged(
	drive: &Drive,
	transfer: &dyn BookTransfer,
	id: &str,
	upload: &mut PendingUpload,
) -> Result<Option<DriveFile>> {
	let metadata = upload.path.metadata()?;
	let version = (metadata.len(), metadata.modified()?);
	if upload.verified != Some(version) {
		if file_hash(&upload.path)? != id {
			bail!("The book changed after it was selected");
		}
		upload.verified = Some(version);
	}
	drive.upload_book(&upload.path, id, transfer, &mut upload.session)
}

fn make_report(
	store: &AccountStore,
	account: &str,
	account_name: String,
	local: &BTreeMap<String, IdentifiedBook>,
	directory: &Path,
) -> SyncReport {
	let mut report = SyncReport { account: account.into(), account_name, ..SyncReport::default() };
	report.files_waiting = store.pending_uploads.values().any(|upload| upload.path.is_file());
	for (id, entry) in &store.books {
		let identified = local.get(id);
		let source = identified.map(|book| &book.source);
		let saved = source.map_or_else(ReadingData::default, |book| book.reading.clone());
		let merged = merge(&entry.replicas, saved.resume);
		let file = store.remote_files.get(id);
		let offline_path = file.map(|file| offline_path(directory, id, &file.name)).filter(|path| path.is_file());
		let title = file
			.map(|file| file.name.clone())
			.or_else(|| source.map(|book| book.title.clone()))
			.or_else(|| entry.replicas.values().next().map(|replica| replica.title.clone()))
			.unwrap_or_default();
		if let Some(book) = source {
			let old_revision = book.baseline.as_ref().map_or(0, |baseline| baseline.revision);
			let baseline = SyncBaseline {
				account: account.into(),
				sha256: id.clone(),
				applied: merged.data.clone(),
				revision: old_revision.saturating_add(1),
			};
			report.readings.push(SyncedReading { local_key: book.key.clone(), expected: saved, baseline });
			if let Some(approval) = &book.upload_approval
				&& approval.account == account
			{
				let changed_file = book.path.is_file() && !approval.allows(account, &book.path);
				if file.is_some() || changed_file {
					report.finished_uploads.push((book.key.clone(), approval.clone()));
					report.file_error |= changed_file;
				}
			}
		}
		report.files_waiting |= missing_download(store, local, directory, id).is_some();
		report.books.push(CloudBook {
			id: id.clone(),
			title,
			local_locator: identified
				.filter(|book| book.readable && !crate::parser::is_remote_url(&book.source.locator))
				.map(|book| book.source.locator.clone()),
			offline_path,
			cloud_bytes: file.map(|file| file.size),
			resume: merged.data.resume,
			choices: merged.choices,
			reading: merged.data,
		});
	}
	report.books.sort_by(|first, second| first.title.cmp(&second.title).then_with(|| first.id.cmp(&second.id)));
	report
}

fn identify_books(
	books: Vec<LocalBook>,
	account: &str,
	cache: &mut HashMap<String, Fingerprint>,
	transfer: &dyn BookTransfer,
) -> BTreeMap<String, IdentifiedBook> {
	let mut local = BTreeMap::new();
	for book in books {
		let Some((id, readable)) = identify(&book, account, cache, transfer) else { continue };
		let approved_upload =
			book.upload_approval.as_ref().is_some_and(|approval| approval.allows(account, &book.path));
		let candidate = IdentifiedBook { source: book, readable, approved_upload };
		match local.entry(id) {
			std::collections::btree_map::Entry::Vacant(entry) => {
				entry.insert(candidate);
			}
			std::collections::btree_map::Entry::Occupied(mut entry) => {
				if (candidate.readable, candidate.approved_upload) > (entry.get().readable, entry.get().approved_upload)
				{
					entry.insert(candidate);
				}
			}
		}
	}
	local
}

fn identify(
	book: &LocalBook,
	account: &str,
	cache: &mut HashMap<String, Fingerprint>,
	transfer: &dyn BookTransfer,
) -> Option<(String, bool)> {
	let fallback = || {
		book.baseline
			.as_ref()
			.filter(|baseline| baseline.account == account && valid_id(&baseline.sha256))
			.map(|baseline| baseline.sha256.clone())
	};
	let Some(metadata) = fs::metadata(&book.path).ok().filter(fs::Metadata::is_file) else {
		if book.locator.starts_with("content://")
			&& let Some(hash) = transfer.local_content_hash(book.locator.clone()).filter(|hash| valid_id(hash))
		{
			return Some((hash, true));
		}
		return cache.get(&book.key).map(|item| item.hash.clone()).or_else(fallback).map(|hash| (hash, false));
	};
	let size = metadata.len();
	let modified = metadata.modified().ok()?;
	if let Some(fingerprint) =
		cache.get(&book.key).filter(|item| item.path == book.path && item.size == size && item.modified == modified)
	{
		return Some((fingerprint.hash.clone(), true));
	}
	let hash = file_hash(&book.path).ok()?;
	cache.insert(book.key.clone(), Fingerprint { path: book.path.clone(), size, modified, hash: hash.clone() });
	Some((hash, true))
}

pub fn standalone_file(path: &Path) -> bool {
	!path
		.extension()
		.and_then(|extension| extension.to_str())
		.is_some_and(|extension| extension.eq_ignore_ascii_case("opf"))
}

pub(super) fn file_hash(path: &Path) -> Result<String> {
	let mut file = File::open(path)?;
	let mut hasher = Sha256::new();
	let mut buffer = vec![0; 128 * 1024];
	loop {
		let count = file.read(&mut buffer)?;
		if count == 0 {
			break;
		}
		hasher.update(&buffer[..count]);
	}
	Ok(hex(&hasher.finalize()))
}

fn digest(value: &[u8]) -> String {
	hex(&Sha256::digest(value))
}

fn hex(bytes: &[u8]) -> String {
	let mut encoded = String::with_capacity(bytes.len() * 2);
	for byte in bytes {
		let _ = write!(encoded, "{byte:02x}");
	}
	encoded
}

fn valid_id(value: &str) -> bool {
	value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn offline_path(directory: &Path, id: &str, filename: &str) -> PathBuf {
	let extension = filename.rsplit_once('.').map(|(_, suffix)| suffix).filter(|suffix| {
		!suffix.is_empty() && suffix.len() <= 12 && suffix.bytes().all(|byte| byte.is_ascii_alphanumeric())
	});
	let name = extension.map_or_else(|| id.to_owned(), |suffix| format!("{id}.{}", suffix.to_ascii_lowercase()));
	directory.join("books").join(name)
}

fn device_id(root: &Path) -> Result<String> {
	let path = root.join("device.json");
	let saved: Option<String> = read_state(&path)?;
	if let Some(value) = saved.filter(|key| valid_id(key)) {
		return Ok(value);
	}
	let mut random = [0u8; 32];
	getrandom::fill(&mut random).map_err(|error| anyhow::anyhow!("{error}"))?;
	let value = digest(&random);
	persist(&path, &Some(&value))?;
	Ok(value)
}

fn read_state<T: DeserializeOwned + Default>(path: &Path) -> Result<T> {
	match File::open(path) {
		Ok(file) => Ok(serde_json::from_reader(file)?),
		Err(error) if error.kind() == ErrorKind::NotFound => Ok(T::default()),
		Err(error) => Err(error.into()),
	}
}

fn persist(path: &Path, value: &impl Serialize) -> Result<()> {
	let temporary = path.with_extension("tmp");
	let mut output = File::create(&temporary)?;
	serde_json::to_writer(&mut output, value)?;
	output.flush()?;
	output.sync_all()?;
	drop(output);
	fs::rename(temporary, path)?;
	Ok(())
}

pub(super) fn remove_file_if_present(path: &Path) -> Result<()> {
	match fs::remove_file(path) {
		Ok(()) => Ok(()),
		Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
		Err(error) => Err(error.into()),
	}
}

#[cfg(test)]
mod tests;
