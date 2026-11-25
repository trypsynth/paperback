//! The newest Qt documentation set and CMake's help file, found and downloaded when the test runs, so it keeps up with what people will open; about 115 MB, so it runs only when asked for:
//!
//! `cargo test --release -p paperback-core --test qch -- --ignored`
//!
//! Downloads are kept in Cargo's test temporary directory under `qch-latest`, and are fetched again only when a newer release, or a changed archive, comes out.

use std::{
	collections::HashMap,
	fmt::Write as _,
	fs,
	io::{Cursor, Read},
	path::{Path, PathBuf},
	time::Duration,
};

use rayon::prelude::*;
use sha1::{Digest, Sha1};
use ureq::Agent;

use super::check::{Book, read_book, split_qthelp};

/// The online installer's repository, where every Qt release's documentation is published.
const QT_REPOSITORY: &str = "https://download.qt.io/online/qtsdkrepository/all_os/qt/";
/// The repository's folders for a release's sources hold its documentation, under these two names: the first since Qt 6.9, the second up to 6.8.
const QT_DOCUMENTATION_FOLDERS: [&str; 2] = ["unix_line_endings_src", "src_doc_examples"];
const CMAKE_LATEST: &str = "https://cmake.org/files/LatestRelease/cmake-latest-files-v1.json";
/// The most of a book's links into the other books of its set that may lead nowhere, for Qt's own broken links.
const MAX_BROKEN_SHARE: f64 = 0.01;
const ATTEMPTS: u64 = 4;

fn agent() -> Agent {
	Agent::config_builder()
		.user_agent("paperback-tests")
		.timeout_connect(Some(Duration::from_secs(30)))
		.timeout_global(Some(Duration::from_mins(10)))
		.build()
		.into()
}

/// Qt's mirrors now and then drop a connection partway, so a request that fails for the network is tried again; one the server answers with an error status is not.
fn get(agent: &Agent, url: &str) -> Vec<u8> {
	let mut attempt = 1;
	loop {
		let result = agent
			.get(url)
			.call()
			.and_then(|mut response| response.body_mut().with_config().limit(1 << 30).read_to_vec());
		match result {
			Ok(bytes) => return bytes,
			Err(err @ ureq::Error::StatusCode(_)) => panic!("{url}: {err}"),
			Err(err) if attempt == ATTEMPTS => panic!("{url}: {err}, after {ATTEMPTS} attempts"),
			Err(err) => {
				println!("{url}: {err}; trying again");
				std::thread::sleep(Duration::from_secs(2 * attempt));
				attempt += 1;
			}
		}
	}
}

fn get_text(agent: &Agent, url: &str) -> String {
	String::from_utf8(get(agent, url)).unwrap_or_else(|err| panic!("{url}: {err}"))
}

fn cache_dir(name: &str) -> PathBuf {
	let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("qch-latest").join(name);
	fs::create_dir_all(&dir).unwrap();
	dir
}

/// Written beside the file and renamed into place, so an interrupted download isn't taken for a finished one next time.
fn write_atomically(path: &Path, bytes: &[u8]) {
	let partial = path.with_extension("part");
	fs::write(&partial, bytes).unwrap();
	fs::rename(&partial, path).unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Version(u32, u32, u32);

/// A folder such as `qt6_6121_unix_line_endings_src` names Qt 6.12.1: the major version, then the major, minor and patch numbers run together, the patch always one digit.
fn parse_release_folder(folder: &str) -> Option<(Version, String)> {
	let rest = folder.strip_prefix("qt")?;
	let (major, rest) = rest.split_once('_')?;
	let (digits, kind) = rest.split_once('_')?;
	if !QT_DOCUMENTATION_FOLDERS.contains(&kind) || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
		return None;
	}
	let minor_and_patch = digits.strip_prefix(major)?;
	if minor_and_patch.len() < 2 {
		return None;
	}
	let (minor, patch) = minor_and_patch.split_at(minor_and_patch.len() - 1);
	Some((Version(major.parse().ok()?, minor.parse().ok()?, patch.parse().ok()?), digits.to_string()))
}

struct QtDocumentation {
	version: String,
	/// Where the package's archives are, each named by the package's version then the archive's own name.
	package_url: String,
	package_version: String,
	archives: Vec<String>,
}

/// The documentation package of a release, `qt.qt6.6121.doc`, which is the set Qt Assistant installs; modules under the GPL come in packages of their own.
fn find_documentation(agent: &Agent, folder: &str, major: u32, digits: &str) -> Option<QtDocumentation> {
	let updates = get_text(agent, &format!("{QT_REPOSITORY}{folder}/Updates.xml"));
	let xml = roxmltree::Document::parse(&updates).unwrap_or_else(|err| panic!("{folder}/Updates.xml: {err}"));
	let package_name = format!("qt.qt{major}.{digits}.doc");
	let package = xml.root_element().children().find(|node| {
		node.has_tag_name("PackageUpdate")
			&& node.children().any(|child| child.has_tag_name("Name") && child.text() == Some(package_name.as_str()))
	})?;
	let field = |name: &str| {
		package
			.children()
			.find(|child| child.has_tag_name(name))
			.and_then(|child| child.text())
			.unwrap_or_default()
			.trim()
			.to_string()
	};
	let package_version = field("Version");
	let archives: Vec<String> = field("DownloadableArchives")
		.split(',')
		.map(str::trim)
		.filter(|archive| !archive.is_empty())
		.map(str::to_string)
		.collect();
	if archives.is_empty() {
		return None;
	}
	Some(QtDocumentation {
		version: package_version.split_once('-').map_or(package_version.as_str(), |(version, _)| version).to_string(),
		package_url: format!("{QT_REPOSITORY}{folder}/{package_name}/"),
		package_version,
		archives,
	})
}

/// The newest release with a documentation package; a release can be listed before its packages are up, so an older one is tried next.
fn latest_qt_documentation(agent: &Agent) -> QtDocumentation {
	let listing = get_text(agent, QT_REPOSITORY);
	let mut releases: Vec<(Version, String, String)> = listing
		.split("href=\"")
		.skip(1)
		.filter_map(|rest| rest.split_once('"').map(|(href, _)| href.trim_end_matches('/')))
		.filter_map(|folder| {
			parse_release_folder(folder).map(|(version, digits)| (version, digits, folder.to_string()))
		})
		.collect();
	releases.sort_by(|a, b| b.cmp(a));
	assert!(!releases.is_empty(), "no Qt releases in {QT_REPOSITORY}");
	releases
		.iter()
		.find_map(|(version, digits, folder)| find_documentation(agent, folder, version.0, digits))
		.unwrap_or_else(|| panic!("no Qt release in {QT_REPOSITORY} has a documentation package"))
}

fn sha1_hex(bytes: &[u8]) -> String {
	Sha1::digest(bytes).iter().fold(String::with_capacity(40), |mut hex, byte| {
		let _ = write!(hex, "{byte:02x}");
		hex
	})
}

/// Each archive is checked against the SHA-1 Qt publishes beside it, and its `.qch` files are unpacked into `dir`.
fn download_qt_archive(agent: &Agent, docs: &QtDocumentation, archive: &str, dir: &Path) {
	let url = format!("{}{}{archive}", docs.package_url, docs.package_version);
	let published = get_text(agent, &format!("{url}.sha1"));
	let expected =
		published.split_whitespace().next().unwrap_or_else(|| panic!("{url}.sha1 is empty")).to_ascii_lowercase();
	let marker = dir.join(format!("{archive}.sha1"));
	if fs::read_to_string(&marker).is_ok_and(|done| done == expected) {
		return;
	}
	let bytes = get(agent, &url);
	assert_eq!(sha1_hex(&bytes), expected, "{url} doesn't match its published SHA-1");
	let mut tar = tar::Archive::new(lzma_rust2::XzReader::new(Cursor::new(bytes), true));
	for entry in tar.entries().unwrap_or_else(|err| panic!("{url}: {err}")) {
		let mut entry = entry.unwrap_or_else(|err| panic!("{url}: {err}"));
		let path = entry.path().unwrap().into_owned();
		if path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("qch")) {
			let mut data = Vec::new();
			entry.read_to_end(&mut data).unwrap();
			write_atomically(&dir.join(path.file_name().unwrap()), &data);
		}
	}
	fs::write(marker, expected).unwrap();
}

/// CMake doesn't publish a checksum for its help file, so it comes over HTTPS from the release's own help pages, and the book has to name the release.
fn download_cmake(agent: &Agent) -> (String, PathBuf) {
	let latest: serde_json::Value = serde_json::from_slice(&get(agent, CMAKE_LATEST)).unwrap();
	let version = &latest["version"];
	let release = version["string"].as_str().expect("CMake's latest release names its version").to_string();
	let url = format!("https://cmake.org/cmake/help/v{}.{}/CMake.qch", version["major"], version["minor"]);
	let path = cache_dir(&format!("cmake-{release}")).join("CMake.qch");
	if !path.is_file() {
		write_atomically(&path, &get(agent, &url));
	}
	(release, path)
}

#[test]
#[ignore = "downloads about 115 MB of Qt and CMake documentation"]
fn latest_qt_and_cmake_documentation() {
	let agent = agent();
	let docs = latest_qt_documentation(&agent);
	let qt_dir = cache_dir(&format!("qt-{}", docs.version));
	println!("Qt {}: {} archives from {}", docs.version, docs.archives.len(), docs.package_url);
	// A few at a time, to be polite to Qt's mirrors.
	rayon::ThreadPoolBuilder::new()
		.num_threads(4)
		.build()
		.unwrap()
		.install(|| docs.archives.par_iter().for_each(|archive| download_qt_archive(&agent, &docs, archive, &qt_dir)));
	let (cmake_release, cmake_path) = download_cmake(&agent);
	println!("CMake {cmake_release}: {}", cmake_path.display());
	let mut qt_files: Vec<PathBuf> = fs::read_dir(&qt_dir)
		.unwrap()
		.map(|entry| entry.unwrap().path())
		.filter(|path| path.extension().is_some_and(|extension| extension == "qch"))
		.collect();
	qt_files.sort();
	assert!(!qt_files.is_empty(), "Qt {}'s documentation has no .qch files", docs.version);
	let qt_books: Vec<Book> = qt_files.par_iter().map(|path| read_book(&path.to_string_lossy())).collect();
	for (path, book) in qt_files.iter().zip(&qt_books) {
		println!("{}: {:?} {:?}", path.file_name().unwrap().to_string_lossy(), book.title, book.report);
	}
	check_links_between(&qt_books);
	let cmake = read_book(&cmake_path.to_string_lossy());
	println!("CMake.qch: {:?} {:?}", cmake.title, cmake.report);
	assert!(cmake.title.contains(&cmake_release), "CMake {cmake_release}'s help file is titled {:?}", cmake.title);
	assert_eq!(cmake.report.contents_missing_from_qt, 0, "every CMake contents entry has its page");
}

/// Qt Assistant follows a link into another book of the set by that book's folder, so each such link has to name a page that book has.
fn check_links_between(books: &[Book]) {
	let by_folder: HashMap<&str, &Book> = books.iter().map(|book| (book.folder.as_str(), book)).collect();
	for book in books {
		let (mut found, mut broken) = (0usize, Vec::new());
		for link in &book.foreign_links {
			let (_, folder, target) = split_qthelp(link).unwrap();
			let Some(other) = by_folder.get(folder) else { continue };
			let target = target.split_once('?').map_or(target.as_str(), |(page, _)| page);
			let page = target.split_once('#').map_or(target, |(page, _)| page);
			if other.targets.contains(page) {
				found += 1;
			} else {
				broken.push(link.as_str());
			}
		}
		let total = found + broken.len();
		#[expect(clippy::cast_precision_loss, reason = "a share of link counts")]
		let share = if total == 0 { 0.0 } else { broken.len() as f64 / total as f64 };
		assert!(
			share <= MAX_BROKEN_SHARE,
			"{}: {} of {total} links into other books lead nowhere, such as {:?}",
			book.folder,
			broken.len(),
			&broken[..broken.len().min(5)]
		);
	}
}
