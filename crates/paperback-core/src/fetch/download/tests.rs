use std::{
	io::{BufRead, BufReader, Write},
	net::TcpListener,
	thread,
};

use super::*;
use crate::util::test_support::TempDir;

/// Answers every request with `body` as `content_type`.
fn serve(content_type: &'static str, body: &'static [u8]) -> String {
	let listener = TcpListener::bind("127.0.0.1:0").expect("bind a local port");
	let base = format!("http://{}", listener.local_addr().expect("local address"));
	thread::spawn(move || {
		for stream in listener.incoming() {
			let Ok(mut stream) = stream else { break };
			let mut reader = BufReader::new(stream.try_clone().expect("clone the stream"));
			let mut line = String::new();
			while reader.read_line(&mut line).is_ok_and(|read| read > 2) {
				line.clear();
			}
			let head = format!(
				"HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n",
				body.len()
			);
			let _ = stream.write_all(head.as_bytes());
			let _ = stream.write_all(body);
		}
	});
	base
}

fn failure(result: Result<LinkDownload, LinkDownloadError>) -> String {
	match result {
		Err(LinkDownloadError::Failed(message)) => message,
		Err(LinkDownloadError::Cancelled) => panic!("cancelled"),
		Ok(_) => panic!("connected"),
	}
}

#[test]
fn text_that_is_not_a_link_is_refused_before_anything_is_asked() {
	assert!(failure(LinkDownload::connect("see chapter 3")).starts_with("see chapter 3: "));
}

#[test]
fn a_link_to_a_format_paperback_cannot_read_is_refused_by_its_name() {
	assert!(
		failure(LinkDownload::connect("http://127.0.0.1:9/setup.exe")).starts_with("http://127.0.0.1:9/setup.exe: ")
	);
}

#[test]
fn a_document_downloads_into_the_folder_under_its_own_name() {
	let base = serve("text/plain", b"Hello from the server.");
	let download = LinkDownload::connect(&format!("{base}/notes.txt")).unwrap();
	assert_eq!(
		download.info(),
		LinkDownloadInfoFfi { file_name: "notes.txt".to_string(), size: 22, warning: String::new() }
	);
	let dir = TempDir::new("link-download");
	let saved = download.save_into(dir.path()).unwrap();
	assert_eq!(saved, dir.path().join("notes.txt"));
	assert_eq!(std::fs::read_to_string(&saved).unwrap(), "Hello from the server.");
	assert_eq!(download.progress(), LinkProgressFfi { done: 22, total: 22 });
}

#[test]
fn a_second_download_of_the_same_name_gets_a_number_rather_than_replacing_the_first() {
	let dir = TempDir::new("link-download-names");
	std::fs::write(dir.path().join("book.epub"), "first").unwrap();
	std::fs::write(dir.path().join("book (2).epub"), "second").unwrap();
	assert_eq!(free_path(dir.path(), "book.epub"), dir.path().join("book (3).epub"));
	assert_eq!(free_path(dir.path(), "README"), dir.path().join("README"));
}

#[test]
fn a_web_page_sent_for_a_document_link_asks_first() {
	let base = serve("text/html", b"<html><body>Sign in</body></html>");
	let link = format!("{base}/book.pdf");
	let warning = LinkDownload::connect(&link).unwrap().info().warning;
	assert!(warning.starts_with(&link), "{warning}");
	assert!(warning.contains("text/html"), "{warning}");
}

#[test]
fn a_cancelled_download_leaves_nothing_behind() {
	let base = serve("text/plain", b"Never kept.");
	let download = LinkDownload::connect(&format!("{base}/notes.txt")).unwrap();
	download.cancel();
	let dir = TempDir::new("link-download-cancel");
	assert!(matches!(download.save_into(dir.path()), Err(LinkDownloadError::Cancelled)));
	assert!(!dir.path().join("notes.txt").exists());
}
