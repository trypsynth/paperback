use std::{
	collections::{BTreeMap, HashMap},
	fs::{self, File, OpenOptions},
	io::{self, Read, Seek, SeekFrom},
	path::Path,
	time::Duration,
};

use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use ureq::{Agent, Body, http::Response};

use super::{file_hash, remove_file_if_present, state::Replica, valid_id};

const API: &str = "https://www.googleapis.com/drive/v3";
const UPLOAD: &str = "https://www.googleapis.com/upload/drive/v3/files";
const REPLICA_QUERY: &str = "appProperties has { key='paperbackKind' and value='reading-v1' }";
const BOOK_QUERY: &str = "trashed=false and appProperties has { key='paperbackKind' and value='book-v1' }";
const FOLDER_QUERY: &str = "trashed=false and appProperties has { key='paperbackKind' and value='folder-v1' }";
const MAX_JSON: u64 = 16 * 1024 * 1024;
const CHUNK: u64 = 8 * 1024 * 1024;

pub const DRIVE_SCOPES: [&str; 2] =
	["https://www.googleapis.com/auth/drive.appdata", "https://www.googleapis.com/auth/drive.file"];

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct BookTransferRequest {
	pub url: String,
	pub token: String,
	pub upload: bool,
	pub probe: bool,
	pub path: String,
	pub offset: u64,
	pub length: u64,
	pub total: u64,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct BookTransferResult {
	pub status: u16,
	pub range: String,
	pub body: Vec<u8>,
}

#[cfg_attr(feature = "uniffi", uniffi::export(callback_interface))]
pub trait BookTransfer: Send + Sync {
	fn available(&self) -> bool;
	fn local_content_hash(&self, locator: String) -> Option<String>;
	fn transfer(&self, request: BookTransferRequest) -> BookTransferResult;
}

pub struct DesktopBookTransfer(Agent);

impl Default for DesktopBookTransfer {
	fn default() -> Self {
		Self(agent())
	}
}

impl BookTransfer for DesktopBookTransfer {
	fn available(&self) -> bool {
		true
	}

	fn local_content_hash(&self, _locator: String) -> Option<String> {
		None
	}

	fn transfer(&self, request: BookTransferRequest) -> BookTransferResult {
		self.send(&request).unwrap_or(BookTransferResult { status: 0, range: String::new(), body: Vec::new() })
	}
}

impl DesktopBookTransfer {
	fn send(&self, request: &BookTransferRequest) -> Result<BookTransferResult> {
		let authorization = format!("Bearer {}", request.token);
		if request.upload {
			let mut bytes = Vec::new();
			if !request.probe {
				let mut source = File::open(&request.path)?;
				source.seek(SeekFrom::Start(request.offset))?;
				source.take(request.length).read_to_end(&mut bytes)?;
				ensure!(u64::try_from(bytes.len())? == request.length, "Short read from the book");
			}
			let range = upload_range(request);
			let response = self
				.0
				.put(&request.url)
				.header("Authorization", authorization)
				.header("Content-Range", range)
				.send(bytes.as_slice())?;
			return response_result(response);
		}
		let range = format!("bytes={}-{}", request.offset, request.offset + request.length - 1);
		let mut response =
			self.0.get(&request.url).header("Authorization", authorization).header("Range", range).call()?;
		let status = response.status().as_u16();
		if status == 206 || (status == 200 && request.offset == 0) {
			let maximum = if status == 206 { request.length } else { request.total };
			let mut output = OpenOptions::new().create(true).write(true).truncate(false).open(&request.path)?;
			output.set_len(request.offset)?;
			output.seek(SeekFrom::Start(request.offset))?;
			let bytes = io::copy(&mut response.body_mut().as_reader().take(maximum + 1), &mut output)?;
			ensure!(bytes == maximum, "Incomplete book fragment");
		}
		Ok(BookTransferResult { status, range: String::new(), body: Vec::new() })
	}
}

fn upload_range(request: &BookTransferRequest) -> String {
	if request.probe {
		format!("bytes */{}", request.total)
	} else {
		format!("bytes {}-{}/{}", request.offset, request.offset + request.length - 1, request.total)
	}
}

fn response_result(mut response: Response<Body>) -> Result<BookTransferResult> {
	let status = response.status().as_u16();
	let range = response.headers().get("Range").and_then(|value| value.to_str().ok()).unwrap_or_default().to_owned();
	let body = if (200..300).contains(&status) {
		response.body_mut().with_config().limit(MAX_JSON).read_to_vec()?
	} else {
		Vec::new()
	};
	Ok(BookTransferResult { status, range, body })
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DriveFile {
	pub id: String,
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub version: String,
	#[serde(default, deserialize_with = "int_from_string")]
	pub size: u64,
	#[serde(default)]
	app_properties: HashMap<String, String>,
}

fn int_from_string<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
	#[derive(Deserialize)]
	#[serde(untagged)]
	enum Number {
		Decimal(String),
		Integer(u64),
	}
	match Number::deserialize(deserializer)? {
		Number::Decimal(value) => value.parse().map_err(serde::de::Error::custom),
		Number::Integer(value) => Ok(value),
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileList {
	#[serde(default)]
	files: Vec<DriveFile>,
	next_page_token: Option<String>,
}

pub(super) struct Account {
	pub id: String,
	pub name: String,
}

pub(super) struct Drive<'a> {
	agent: Agent,
	token: &'a str,
	api: &'a str,
	upload: &'a str,
}

fn agent() -> Agent {
	Agent::config_builder()
		.timeout_connect(Some(Duration::from_secs(10)))
		.timeout_global(Some(Duration::from_secs(35)))
		.http_status_as_error(false)
		.max_redirects(0)
		.max_redirects_will_error(false)
		.build()
		.into()
}

fn read_json<T: DeserializeOwned>(mut response: Response<Body>) -> Result<T> {
	ensure!(response.status().is_success(), "Google Drive responded with {}", response.status());
	let bytes = response.body_mut().with_config().limit(MAX_JSON).read_to_vec()?;
	Ok(serde_json::from_slice(&bytes)?)
}

impl<'a> Drive<'a> {
	pub(super) fn new(token: &'a str) -> Self {
		Self { agent: agent(), token, api: API, upload: UPLOAD }
	}

	#[cfg(test)]
	pub(super) fn with_test_endpoints(token: &'a str, api: &'a str, upload: &'a str) -> Self {
		Self { token, api, upload, agent: agent() }
	}

	fn files(&self) -> String {
		format!("{}/files", self.api)
	}

	fn authorization(&self) -> String {
		format!("Bearer {}", self.token)
	}

	pub(super) fn account(&self) -> Result<Account> {
		let response = self
			.agent
			.get(format!("{}/about", self.api))
			.query("fields", "user(permissionId,emailAddress,displayName)")
			.header("Authorization", self.authorization())
			.call()?;
		let data: Value = read_json(response)?;
		let user = &data["user"];
		Ok(Account {
			id: user["permissionId"].as_str().context("Google Drive did not identify the account")?.to_owned(),
			name: user["emailAddress"].as_str().or_else(|| user["displayName"].as_str()).unwrap_or_default().to_owned(),
		})
	}

	fn list(&self, space: &str, filter: &str) -> Result<Vec<DriveFile>> {
		let mut files = Vec::new();
		let mut page_token = None;
		loop {
			let mut request = self
				.agent
				.get(self.files())
				.query("spaces", space)
				.query("q", filter)
				.query("fields", "nextPageToken,files(id,name,size,version,appProperties)")
				.query("pageSize", "1000")
				.header("Authorization", self.authorization());
			if let Some(ref token) = page_token {
				request = request.query("pageToken", token);
			}
			let page: FileList = read_json(request.call()?)?;
			files.extend(page.files);
			page_token = page.next_page_token;
			if page_token.is_none() {
				return Ok(files);
			}
		}
	}

	pub(super) fn list_replicas(&self) -> Result<Vec<DriveFile>> {
		self.list("appDataFolder", REPLICA_QUERY)
	}

	pub(super) fn get_replica(&self, id: &str) -> Result<Replica> {
		let url = file_url(&self.files(), id)?;
		read_json(self.agent.get(url).query("alt", "media").header("Authorization", self.authorization()).call()?)
	}

	pub(super) fn put_replica(&self, replica: &Replica, remote_id: Option<&str>) -> Result<String> {
		let mut payload = serde_json::to_vec(replica)?;
		if let Some(id) = remote_id {
			let url = file_url(self.upload, id)?;
			let response = self
				.agent
				.patch(url)
				.query("uploadType", "media")
				.header("Authorization", self.authorization())
				.content_type("application/json")
				.send(payload.as_slice())?;
			if response.status().as_u16() != 404 {
				return Ok(read_json::<DriveFile>(response)?.id);
			}
		}
		let metadata = json!({
			"name": format!("{}-{}.json", replica.book, replica.device),
			"parents": ["appDataFolder"],
			"appProperties": { "paperbackKind": "reading-v1" }
		});
		let boundary = "paperback-reading-state";
		let mut content = format!("--{boundary}\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{metadata}\r\n--{boundary}\r\nContent-Type: application/json\r\n\r\n").into_bytes();
		content.append(&mut payload);
		content.extend(format!("\r\n--{boundary}--\r\n").as_bytes());
		let response = self
			.agent
			.post(self.upload)
			.query("uploadType", "multipart")
			.header("Authorization", self.authorization())
			.content_type(format!("multipart/related; boundary={boundary}"))
			.send(content.as_slice())?;
		Ok(read_json::<DriveFile>(response)?.id)
	}

	pub(super) fn list_books(&self) -> Result<BTreeMap<String, DriveFile>> {
		let mut books: BTreeMap<String, DriveFile> = BTreeMap::new();
		for file in self.list("drive", BOOK_QUERY)? {
			if let Some(hash) = file.app_properties.get("paperbackBook").filter(|hash| valid_id(hash)) {
				let current = books.entry(hash.clone()).or_insert_with(|| file.clone());
				if file.id < current.id {
					*current = file;
				}
			}
		}
		Ok(books)
	}

	pub(super) fn trash(&self, id: &str) -> Result<()> {
		let payload = serde_json::to_vec(&json!({ "trashed": true }))?;
		let response = self
			.agent
			.patch(file_url(&self.files(), id)?)
			.header("Authorization", self.authorization())
			.content_type("application/json")
			.send(payload.as_slice())?;
		ensure!(response.status().is_success(), "Google Drive responded with {}", response.status());
		Ok(())
	}

	fn folder(&self) -> Result<String> {
		if let Some(folder) =
			self.list("drive", FOLDER_QUERY)?.into_iter().min_by(|first, second| first.id.cmp(&second.id))
		{
			return Ok(folder.id);
		}
		let payload = serde_json::to_vec(&json!({
			"name": "Paperback",
			"mimeType": "application/vnd.google-apps.folder",
			"appProperties": { "paperbackKind": "folder-v1" }
		}))?;
		let response = self
			.agent
			.post(self.files())
			.header("Authorization", self.authorization())
			.content_type("application/json")
			.send(payload.as_slice())?;
		Ok(read_json::<DriveFile>(response)?.id)
	}

	pub(super) fn upload_book(
		&self,
		path: &Path,
		hash: &str,
		transfer: &dyn BookTransfer,
		session: &mut Option<String>,
	) -> Result<Option<DriveFile>> {
		let name = path.file_name().and_then(|name| name.to_str()).context("Missing book name")?;
		let size = path.metadata()?.len();
		ensure!(size > 0, "Empty book");
		let source = path.to_str().context("Invalid source path")?.to_owned();
		let (url, offset) = if let Some(url) = session.as_ref() {
			ensure!(valid_session(url), "Invalid Google Drive upload session");
			let result = transfer.transfer(BookTransferRequest {
				url: url.clone(),
				token: self.token.into(),
				upload: true,
				probe: true,
				path: source.clone(),
				offset: 0,
				length: 0,
				total: size,
			});
			match result.status {
				200 | 201 => return Ok(Some(serde_json::from_slice(&result.body)?)),
				308 => (url.clone(), next_offset(&result.range)?),
				404 | 410 => {
					*session = None;
					bail!("Google Drive upload session expired");
				}
				status => bail!("Could not resume uploading the book ({status})"),
			}
		} else {
			let payload = serde_json::to_vec(&json!({
				"name": name,
				"parents": [self.folder()?],
				"appProperties": { "paperbackKind": "book-v1", "paperbackBook": hash }
			}))?;
			let response = self
				.agent
				.post(self.upload)
				.query("uploadType", "resumable")
				.query("fields", "id,name,size,version,appProperties")
				.header("Authorization", self.authorization())
				.header("X-Upload-Content-Length", size.to_string())
				.header("X-Upload-Content-Type", "application/octet-stream")
				.content_type("application/json")
				.send(payload.as_slice())?;
			ensure!(response.status().is_success(), "Google Drive responded with {}", response.status());
			let url = response
				.headers()
				.get("Location")
				.and_then(|location| location.to_str().ok())
				.filter(|location| valid_session(location))
				.context("Missing Google Drive upload session")?
				.to_owned();
			*session = Some(url.clone());
			(url, 0)
		};
		ensure!(offset < size, "Google Drive did not confirm the uploaded book");
		ensure!(transfer.available(), "Book transfer is waiting for its selected connection");
		let length = (size - offset).min(CHUNK);
		let result = transfer.transfer(BookTransferRequest {
			url,
			token: self.token.into(),
			upload: true,
			probe: false,
			path: source,
			offset,
			length,
			total: size,
		});
		match result.status {
			200 | 201 => Ok(Some(serde_json::from_slice(&result.body)?)),
			308 => {
				let next = next_offset(&result.range)?;
				ensure!(next > offset && next <= offset + length, "Invalid offset from Google Drive");
				Ok(None)
			}
			404 | 410 => {
				*session = None;
				bail!("Google Drive upload session expired");
			}
			status => bail!("Could not upload the book ({status})"),
		}
	}

	pub(super) fn download_book(
		&self,
		file: &DriveFile,
		transfer: &dyn BookTransfer,
		path: &Path,
		hash: &str,
	) -> Result<()> {
		let url = format!("{}?alt=media", file_url(&self.files(), &file.id)?);
		let temporary = path.with_extension("part");
		let mut offset = temporary.metadata().map_or(0, |metadata| metadata.len());
		if offset > file.size {
			remove_file_if_present(&temporary)?;
			offset = 0;
		}
		let destination = temporary.to_str().context("Invalid destination")?.to_owned();
		if offset < file.size {
			ensure!(transfer.available(), "Book transfer is waiting for its selected connection");
			let length = (file.size - offset).min(CHUNK);
			let result = transfer.transfer(BookTransferRequest {
				url,
				token: self.token.into(),
				upload: false,
				probe: false,
				path: destination,
				offset,
				length,
				total: file.size,
			});
			let end = match (result.status, offset) {
				(206, _) => offset + length,
				(200, 0) => file.size,
				(status, _) => bail!("Could not download the book ({status})"),
			};
			ensure!(temporary.metadata()?.len() == end, "Incomplete book fragment");
			offset = end;
		}
		if offset < file.size {
			return Ok(());
		}
		if file_hash(&temporary)? != hash {
			remove_file_if_present(&temporary)?;
			bail!("Book integrity check failed");
		}
		fs::rename(temporary, path)?;
		Ok(())
	}
}

fn file_url(base: &str, id: &str) -> Result<String> {
	ensure!(
		!id.is_empty() && id.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_')),
		"Invalid Google Drive file identifier"
	);
	Ok(format!("{base}/{id}"))
}

fn valid_session(url: &str) -> bool {
	url.starts_with("https://www.googleapis.com/upload/drive/v3/files?")
}

fn next_offset(range: &str) -> Result<u64> {
	if range.is_empty() {
		return Ok(0);
	}
	let end: u64 = range.strip_prefix("bytes=0-").context("Invalid Google Drive range")?.parse()?;
	end.checked_add(1).context("Invalid Google Drive offset")
}
