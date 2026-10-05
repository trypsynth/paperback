use std::{
	fmt::Write as _,
	fs,
	io::{BufRead, BufReader, Write},
	net::{TcpListener, TcpStream},
	sync::{
		Arc,
		atomic::{AtomicBool, AtomicUsize, Ordering},
	},
	thread,
	time::Duration,
};

use super::*;
use crate::util::test_support::TempDir;

/// Serves one request per connection; `handler(method, path, base, stream)` writes the response.
fn serve(handler: impl Fn(&str, &str, &str, &mut TcpStream) + Send + 'static) -> String {
	let listener = TcpListener::bind("127.0.0.1:0").expect("bind a local port");
	let base = format!("http://{}", listener.local_addr().expect("local address"));
	let base_for_thread = base.clone();
	thread::spawn(move || {
		for stream in listener.incoming() {
			let Ok(mut stream) = stream else { break };
			let mut reader = BufReader::new(stream.try_clone().expect("clone the stream"));
			let mut request_line = String::new();
			if reader.read_line(&mut request_line).is_err() {
				continue;
			}
			let mut header = String::new();
			while reader.read_line(&mut header).is_ok_and(|read| read > 2) {
				header.clear();
			}
			let mut words = request_line.split_whitespace();
			let method = words.next().unwrap_or_default().to_string();
			let path = words.next().unwrap_or_default().to_string();
			handler(&method, &path, &base_for_thread, &mut stream);
		}
	});
	base
}

/// Writes a complete response; the body is left out for HEAD.
fn respond(stream: &mut TcpStream, method: &str, status: &str, headers: &[(&str, &str)], body: &[u8]) {
	let mut head = format!("HTTP/1.1 {status}\r\nConnection: close\r\nContent-Length: {}\r\n", body.len());
	for (name, value) in headers {
		let _ = write!(head, "{name}: {value}\r\n");
	}
	head.push_str("\r\n");
	let _ = stream.write_all(head.as_bytes());
	if method != "HEAD" {
		let _ = stream.write_all(body);
	}
}

fn save(remote: Remote, dest: &Path) -> Result<u64, FetchError> {
	remote.save(dest, &AtomicBool::new(false), |_, _| {})
}

#[test]
fn open_reads_the_type_size_and_name() {
	let base = serve(|method, _, _, stream| {
		respond(stream, method, "200 OK", &[("Content-Type", "application/epub+zip")], b"0123456789");
	});
	let remote = open(&format!("{base}/books/a.epub")).expect("open");
	let info = remote.info();
	assert_eq!(info.content_type.as_deref(), Some("application/epub+zip"));
	assert_eq!(info.size, Some(10));
	assert_eq!(info.file_name, "a.epub");
	assert_eq!(info.verdict(&format!("{base}/books/a.epub")), Verdict::Pass);
}

#[test]
fn a_link_is_requested_once_for_its_headers_and_its_body() {
	let requests = Arc::new(AtomicUsize::new(0));
	let counted = Arc::clone(&requests);
	let base = serve(move |method, _, _, stream| {
		counted.fetch_add(1, Ordering::SeqCst);
		respond(stream, method, "200 OK", &[("Content-Type", "application/epub+zip")], b"0123456789");
	});
	let dir = TempDir::new("fetch-once");
	let remote = open(&format!("{base}/a.epub")).expect("open");
	save(remote, &dir.path().join("a.epub")).expect("save");
	assert_eq!(requests.load(Ordering::SeqCst), 1);
}

#[test]
fn open_follows_a_301_and_a_302() {
	for status in ["301 Moved Permanently", "302 Found"] {
		let base = serve(move |method, path, base, stream| match path {
			"/old" => {
				respond(stream, method, status, &[("Location", format!("{base}/book.epub").as_str())], b"");
			}
			_ => respond(stream, method, "200 OK", &[("Content-Type", "application/epub+zip")], b"x"),
		});
		let remote = open(&format!("{base}/old")).expect(status);
		assert!(remote.info().final_url.ends_with("/book.epub"), "{status}: {}", remote.info().final_url);
		assert_eq!(remote.info().file_name, "book.epub", "{status}");
	}
}

#[test]
fn open_names_a_redirect_without_an_extension_after_the_link_given() {
	let base = serve(|method, path, base, stream| match path {
		"/files/report.pdf" => {
			respond(stream, method, "302 Found", &[("Location", format!("{base}/7f3a2b?sig=1").as_str())], b"");
		}
		_ => respond(stream, method, "200 OK", &[("Content-Type", "binary/octet-stream")], b"%PDF"),
	});
	let remote = open(&format!("{base}/files/report.pdf")).expect("open");
	assert_eq!(remote.info().file_name, "7f3a2b.pdf");
}

#[test]
fn open_takes_the_name_the_server_gives() {
	let base = serve(|method, _, _, stream| {
		respond(stream, method, "200 OK", &[("Content-Disposition", "attachment; filename=\"Report 2024.pdf\"")], b"x");
	});
	let remote = open(&format!("{base}/download?id=3")).expect("open");
	assert_eq!(remote.info().file_name, "Report 2024.pdf");
}

#[test]
fn an_https_link_is_only_followed_to_https() {
	assert!(config_for("https://example.org/a.epub").https_only());
	assert!(config_for("HTTPS://example.org/a.epub").https_only());
	assert!(!config_for("http://example.org/a.epub").https_only());
}

#[test]
fn a_redirect_from_https_to_plain_http_is_reported_as_insecure() {
	let error = FetchError::from(ureq::Error::RequireHttpsOnly("http://example.org/a.epub".to_string()));
	assert!(matches!(error, FetchError::Insecure), "{error:?}");
}

#[test]
fn the_limit_is_16_mib() {
	assert_eq!(MAX_SIZE, 16 * 1024 * 1024);
}

#[test]
fn a_document_announced_as_larger_than_the_limit_is_refused_before_its_body() {
	let base = serve(|method, _, _, stream| respond(stream, method, "200 OK", &[], &[b'x'; 100]));
	let result = open_within(&format!("{base}/a.epub"), 50);
	assert!(matches!(result, Err(FetchError::TooLarge(50))), "{:?}", result.err());
}

#[test]
fn a_document_growing_past_the_limit_is_stopped_and_removed() {
	let base = serve(|_, _, _, stream| {
		let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n");
		let _ = stream.write_all(&[b'x'; 100]);
	});
	let dir = TempDir::new("fetch-too-large");
	let dest = dir.path().join("a.epub");
	let remote = open_within(&format!("{base}/a.epub"), 50).expect("open");
	let result = save(remote, &dest);
	assert!(matches!(result, Err(FetchError::TooLarge(50))), "{result:?}");
	assert!(!dest.exists());
	assert!(!dir.path().join("a.epub.part").exists());
}

#[test]
fn open_reports_a_missing_document_by_status() {
	let base = serve(|method, _, _, stream| respond(stream, method, "404 Not Found", &[], b""));
	assert!(matches!(open(&format!("{base}/gone.epub")), Err(FetchError::Http(404))));
}

#[test]
fn save_writes_the_file_and_reports_progress() {
	let base = serve(|method, _, _, stream| respond(stream, method, "200 OK", &[], b"0123456789"));
	let dir = TempDir::new("fetch-download");
	let dest = dir.path().join("a.epub");
	let mut last = None;
	let remote = open(&format!("{base}/a.epub")).expect("open");
	let bytes = remote.save(&dest, &AtomicBool::new(false), |done, total| last = Some((done, total))).expect("save");
	assert_eq!(bytes, 10);
	assert_eq!(last, Some((10, Some(10))));
	assert_eq!(fs::read(&dest).expect("read"), b"0123456789");
	assert!(!dir.path().join("a.epub.part").exists());
}

#[test]
fn saving_a_link_that_ends_at_a_file_no_parser_reads_is_refused() {
	let base = serve(|method, path, base, stream| match path {
		"/book" => respond(stream, method, "302 Found", &[("Location", format!("{base}/setup.exe").as_str())], b""),
		_ => respond(stream, method, "200 OK", &[], b"MZ"),
	});
	let dir = TempDir::new("fetch-refuse");
	let dest = dir.path().join("book");
	let remote = open(&format!("{base}/book")).expect("open");
	assert_eq!(remote.info().verdict(&format!("{base}/book")), Verdict::Refuse("exe".to_string()));
	let result = save(remote, &dest);
	assert!(matches!(result, Err(FetchError::Refused(ref extension)) if extension == "exe"), "{result:?}");
	assert!(!dest.exists());
	assert!(!dir.path().join("book.part").exists());
}

#[test]
fn a_cancelled_download_leaves_nothing_behind() {
	let base = serve(|_, _, _, stream| {
		let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 16384\r\n\r\n");
		let _ = stream.write_all(&[b'x'; 8192]);
		let _ = stream.flush();
		thread::sleep(Duration::from_secs(5));
		let _ = stream.write_all(&[b'x'; 8192]);
	});
	let dir = TempDir::new("fetch-cancel");
	let dest = dir.path().join("a.epub");
	let cancel = AtomicBool::new(false);
	let remote = open(&format!("{base}/a.epub")).expect("open");
	let result = remote.save(&dest, &cancel, |_, _| cancel.store(true, Ordering::Relaxed));
	assert!(matches!(result, Err(FetchError::Cancelled)), "{result:?}");
	assert!(!dest.exists());
	assert!(!dir.path().join("a.epub.part").exists());
}

#[test]
fn a_download_cut_short_keeps_the_copy_already_there() {
	let base = serve(|_, _, _, stream| {
		let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 100\r\n\r\n0123456789");
	});
	let dir = TempDir::new("fetch-keep");
	let dest = dir.write("a.epub", "the copy from before");
	let remote = open(&format!("{base}/a.epub")).expect("open");
	let result = save(remote, &dest);
	assert!(matches!(result, Err(FetchError::Network(_))), "{result:?}");
	assert_eq!(fs::read_to_string(&dest).expect("read"), "the copy from before");
	assert!(!dir.path().join("a.epub.part").exists());
}

#[test]
fn a_download_never_writes_through_a_part_file_already_there() {
	let base = serve(|method, _, _, stream| respond(stream, method, "200 OK", &[], b"0123456789"));
	let dir = TempDir::new("fetch-planted");
	let planted = dir.write("a.epub.part", "planted");
	let dest = dir.path().join("a.epub");
	let remote = open(&format!("{base}/a.epub")).expect("open");
	let result = save(remote, &dest);
	assert!(matches!(result, Err(FetchError::Io(_))), "{result:?}");
	assert_eq!(fs::read_to_string(&planted).expect("read"), "planted");
	assert!(!dest.exists());
}
