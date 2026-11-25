use std::{
	io::{BufRead, BufReader},
	net::TcpListener,
	thread,
};

use serde_json::{Value, json};

use super::*;
use crate::{document::DocPos, util::test_support::TempDir};

struct NoBookConnection;

impl BookTransfer for NoBookConnection {
	fn available(&self) -> bool {
		false
	}

	fn local_content_hash(&self, _locator: String) -> Option<String> {
		None
	}

	fn transfer(&self, _: BookTransferRequest) -> BookTransferResult {
		panic!("whole-book transfer on a disallowed connection");
	}
}

fn local_book(path: PathBuf) -> LocalBook {
	LocalBook {
		key: path.to_string_lossy().into_owned(),
		locator: path.to_string_lossy().into_owned(),
		title: "Book".into(),
		path,
		reading: ReadingData::default(),
		baseline: None,
		upload_approval: None,
	}
}

fn serve_metadata(replies: Vec<(&'static str, Value)>) -> (String, thread::JoinHandle<()>) {
	let listener = TcpListener::bind("127.0.0.1:0").unwrap();
	let url = format!("http://{}", listener.local_addr().unwrap());
	let server = thread::spawn(move || {
		for (expected, response) in replies {
			let (mut stream, _) = listener.accept().unwrap();
			let mut reader = BufReader::new(&mut stream);
			let mut line = String::new();
			reader.read_line(&mut line).unwrap();
			assert!(line.starts_with(expected), "unexpected request: {line}");
			let mut length = 0;
			loop {
				line.clear();
				if reader.read_line(&mut line).unwrap() <= 2 {
					break;
				}
				if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
					length = value.trim().parse().unwrap();
				}
			}
			reader.read_exact(&mut vec![0; length]).unwrap();
			let body = serde_json::to_vec(&response).unwrap();
			write!(
				stream,
				"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n",
				body.len()
			)
			.unwrap();
			stream.write_all(&body).unwrap();
		}
	});
	(url, server)
}

#[test]
fn small_reading_data_syncs_while_book_file_waits_for_wifi() {
	let dir = TempDir::new("drive-wifi-policy");
	let mut book = local_book(dir.write("my-book.m4b", b"book bytes"));
	book.upload_approval = UploadApproval::new(&digest(b"account"), &book.path, 10);
	let id = file_hash(&book.path).unwrap();
	let device = digest(b"another device");
	let owner = ReplicaOwner { book: &id, device: &device, name: "Other device", title: "Book" };
	let remote_reading = ReadingData {
		resume: ResumePosition { text: DocPos::from_raw(15), audio_ms: Some(183_021) },
		..ReadingData::default()
	};
	let mut replicas = BTreeMap::new();
	record(&mut replicas, &owner, &ReadingData::default(), &remote_reading, false);
	let (url, server) = serve_metadata(vec![
		("GET /about?", json!({ "user": { "permissionId": "account", "displayName": "Reader" } })),
		("GET /files?spaces=appDataFolder", json!({ "files": [{ "id": "remote", "version": "1" }] })),
		("GET /files/remote?alt=media", serde_json::to_value(&replicas[&device]).unwrap()),
		("POST /upload/files?uploadType=multipart", json!({ "id": "local" })),
		("GET /files?spaces=drive", json!({ "files": [] })),
	]);
	let upload = format!("{url}/upload/files");
	let drive = Drive::with_test_endpoints("test", &url, &upload);
	let sync = DriveSync::new(dir.join("state"), "Android".into());
	let report = sync.sync_with_drive(&drive, vec![book.clone()], &Command::Refresh, &NoBookConnection).unwrap();
	assert_eq!(report.readings[0].baseline.applied, remote_reading);
	assert!(report.files_waiting);
	assert!(!report.file_error);
	assert!(report.finished_uploads.is_empty());
	server.join().unwrap();
	let cached = sync.offline_report(vec![book], &NoBookConnection).unwrap();
	assert_eq!(cached.books[0].resume, remote_reading.resume);
	assert!(cached.files_waiting);
}

#[test]
fn a_renamed_local_copy_is_matched_but_matching_names_are_not_enough() {
	let dir = TempDir::new("drive-content-identity");
	let book = local_book(dir.write("old/book.epub", b"edition one"));
	let renamed = local_book(dir.write("new/renamed.epub", b"edition one"));
	let other = local_book(dir.write("other/book.epub", b"edition two"));
	let id = file_hash(&book.path).unwrap();
	let mut cache = HashMap::new();
	let local = identify_books(vec![renamed, other], "account", &mut cache, &NoBookConnection);
	assert_eq!(local.len(), 2);
	assert!(local[&id].source.path.ends_with("renamed.epub"));
	assert!(local[&id].readable);
}

#[test]
fn uploading_requires_the_approved_account_and_file_revision() {
	let dir = TempDir::new("drive-upload-approval");
	let path = dir.write("book.epub", b"book");
	let account = digest(b"first account");
	let approval = UploadApproval::new(&account, &path, 4).unwrap();
	assert!(approval.allows(&account, &path));
	assert!(!approval.allows(&digest(b"another account"), &path));
	fs::write(&path, b"different book").unwrap();
	assert!(!approval.allows(&account, &path));
	assert!(UploadApproval::new(&account, &path, 4).is_none());
}

#[test]
fn a_cloud_action_cannot_run_after_switching_accounts() {
	let dir = TempDir::new("drive-account-change");
	let (url, server) = serve_metadata(vec![("GET /about?", json!({ "user": { "permissionId": "another account" } }))]);
	let drive = Drive::with_test_endpoints("test", &url, &url);
	let state = dir.join("state");
	let sync = DriveSync::new(state.clone(), "Windows".into());
	let command = Command::RemoveCloudFile(CloudBookKey { id: digest(b"book"), account: digest(b"first account") });
	let error = sync.sync_with_drive(&drive, vec![], &command, &NoBookConnection).unwrap_err();
	assert!(error.to_string().contains("account changed"));
	assert!(!state.exists());
	server.join().unwrap();
}

struct BookFixture {
	body: Vec<u8>,
}

impl BookTransfer for BookFixture {
	fn available(&self) -> bool {
		true
	}

	fn local_content_hash(&self, _locator: String) -> Option<String> {
		None
	}

	fn transfer(&self, request: BookTransferRequest) -> BookTransferResult {
		let start = usize::try_from(request.offset).unwrap();
		let length = usize::try_from(request.length).unwrap();
		let mut file = fs::OpenOptions::new().append(true).create(true).open(&request.path).unwrap();
		assert_eq!(file.metadata().unwrap().len(), request.offset);
		file.write_all(&self.body[start..start + length]).unwrap();
		BookTransferResult { status: 206, range: String::new(), body: Vec::new() }
	}
}

#[test]
fn an_interrupted_book_resumes_from_its_partial_file_and_is_verified() {
	let dir = TempDir::new("drive-partial-book");
	let target = dir.join("book.epub");
	let body = b"contents of a book".to_vec();
	let partial = 9;
	fs::write(target.with_extension("part"), &body[..partial]).unwrap();
	let file = serde_json::from_value(json!({ "id": "book", "size": body.len().to_string() })).unwrap();
	let digest = digest(&body);
	let drive = Drive::new("test");
	let transfer = BookFixture { body };
	drive.download_book(&file, &transfer, &target, &digest).unwrap();
	assert_eq!(fs::read(&target).unwrap(), transfer.body);
	assert!(!target.with_extension("part").exists());
}
