//! The parts of opening documents from links that need no window: reading the links out of what
//! the reader typed, and wording the download window, the security warning and what went wrong.

use std::path::Path;

use paperback_core::{
	fetch::{self, FetchError},
	parser::{is_remote_url, parser_supports_extension},
};
use patois::t;

use crate::working_copy::WorkingCopy;

/// The links in `text`, one per line, each once and in order, and the lines that are not links.
pub fn from_text(text: &str) -> (Vec<String>, Vec<String>) {
	let mut links: Vec<String> = Vec::new();
	let mut not_links = Vec::new();
	for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
		if !is_remote_url(line) {
			not_links.push(line.to_string());
		} else if !links.iter().any(|link| link == line) {
			links.push(line.to_string());
		}
	}
	(links, not_links)
}

/// What the download window is doing with a link.
#[derive(Debug, Clone, Copy)]
pub enum Stage {
	/// Asking the server what it sends.
	Checking,
	/// Downloading the document.
	Downloading,
}

/// The download window's message for link `n` of `m`, naming `subject`: the link while it is
/// checked, the document's file name while it downloads.
pub fn step(stage: Stage, n: usize, m: usize, subject: &str) -> String {
	let template = match (stage, m > 1) {
		// TRANSLATORS: Download window message while Paperback asks a web server about a link; %s is the link
		(Stage::Checking, false) => t("Checking %s"),
		// TRANSLATORS: Download window message while Paperback asks a web server about one of several links; the first %d is the link's number, the second how many links there are, and %s is the link
		(Stage::Checking, true) => t("Checking %d of %d: %s"),
		// TRANSLATORS: Download window message while a document opened from a link downloads; %s is its file name
		(Stage::Downloading, false) => t("Downloading %s"),
		// TRANSLATORS: Download window message while one of several documents opened from links downloads; the first %d is its number, the second how many there are, and %s is its file name
		(Stage::Downloading, true) => t("Downloading %d of %d: %s"),
	};
	// The subject goes in last, so a % sign in a link is never taken for a placeholder.
	template.replacen("%d", &n.to_string(), 1).replacen("%d", &m.to_string(), 1).replacen("%s", subject, 1)
}

/// The question asked before downloading a link whose server does not say it sends a document
/// Paperback reads.
pub fn warning_text(link: &str, content_type: Option<&str>) -> String {
	let reason = match content_type {
		Some(content_type) if fetch::is_web_page(content_type) => {
			// TRANSLATORS: Security warning before downloading a document from a link; %s is the type the server gave, such as text/html
			t("The server sends a web page (%s) instead of the document the link names.").replacen(
				"%s",
				content_type,
				1,
			)
		}
		// TRANSLATORS: Security warning before downloading a document from a link; %s is the type the server gave, such as application/octet-stream
		Some(content_type) => {
			t("The server sends %s, which is not a format Paperback reads.").replacen("%s", content_type, 1)
		}
		// TRANSLATORS: Security warning before downloading a document from a link whose server gave no type
		None => t("The server does not say what it sends."),
	};
	// TRANSLATORS: Question at the end of the security warning before downloading a document from a link
	format!("{link}\n{reason}\n{}", t("Download it anyway?"))
}

/// Whether the reader picks a format before a downloaded document opens: after a security
/// warning, and when its file name has no extension Paperback reads, unless a format was already
/// picked for the link.
pub fn needs_open_as(warned: bool, copy_name: &str, saved_format: &str) -> bool {
	if !saved_format.is_empty() && parser_supports_extension(saved_format) {
		return false;
	}
	warned || !Path::new(copy_name).extension().and_then(|ext| ext.to_str()).is_some_and(parser_supports_extension)
}

/// What became of one link.
pub enum Outcome {
	Downloaded {
		link: String,
		copy: WorkingCopy,
		/// The reader said yes to a security warning for it.
		warned: bool,
	},
	/// The link names a kind of file Paperback does not read.
	Refused {
		link: String,
		extension: String,
	},
	Failed {
		link: String,
		error: FetchError,
	},
	/// The reader said no to a security warning, or cancelled.
	Skipped,
}

/// The lines of the error shown after a batch: one per line that was not a link, then one per
/// link that was refused or failed.
pub fn report(outcomes: &[Outcome], not_links: &[String]) -> Vec<String> {
	// TRANSLATORS: Follows a line typed into Open from URL that is not an http or https link, in the list of what could not be opened
	let not_links = not_links.iter().map(|line| format!("{line}: {}", t("this is not a web link")));
	let problems = outcomes.iter().filter_map(|outcome| match outcome {
		Outcome::Refused { link, extension } => {
			// TRANSLATORS: Follows a link in the list of what could not be opened; %s is the file extension the link ends in, such as exe
			let message = t("Paperback cannot read .%s files, so it was not downloaded").replacen("%s", extension, 1);
			Some(format!("{link}: {message}"))
		}
		Outcome::Failed { link, error } => Some(format!("{link}: {error}")),
		Outcome::Downloaded { .. } | Outcome::Skipped => None,
	});
	not_links.chain(problems).collect()
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::{test_locale, working_copy::WorkingCopy};

	#[test]
	fn from_text_keeps_each_link_once_in_order() {
		let text = "https://a.org/x.epub\r\n\n  https://b.org/y.pdf  \nsee chapter 3\nhttps://a.org/x.epub\n";
		let (links, not_links) = from_text(text);
		assert_eq!(links, vec!["https://a.org/x.epub", "https://b.org/y.pdf"]);
		assert_eq!(not_links, vec!["see chapter 3"]);
	}

	#[test]
	fn from_text_of_blank_text_is_empty() {
		assert_eq!(from_text(" \n\t\n"), (Vec::new(), Vec::new()));
	}

	#[rstest]
	#[case::one_link(Stage::Checking, 1, 1, "https://a.org/x.epub", "Checking https://a.org/x.epub")]
	#[case::one_of_several(Stage::Downloading, 2, 5, "book.epub", "Downloading 2 of 5: book.epub")]
	#[case::one_download(Stage::Downloading, 1, 1, "book.epub", "Downloading book.epub")]
	#[case::percent_signs_in_the_link(
		Stage::Checking,
		1,
		2,
		"https://a.org/100%d%s.pdf",
		"Checking 1 of 2: https://a.org/100%d%s.pdf"
	)]
	fn step_names_the_link_and_its_place(
		#[case] stage: Stage,
		#[case] n: usize,
		#[case] m: usize,
		#[case] subject: &str,
		#[case] expected: &str,
	) {
		let _locale = test_locale::pinned_to("en");
		assert_eq!(step(stage, n, m, subject), expected);
	}

	#[rstest]
	#[case::a_web_page(
		Some("text/html; charset=utf-8"),
		"https://a.org/book.pdf\nThe server sends a web page (text/html; charset=utf-8) instead of the document the link names.\nDownload it anyway?"
	)]
	#[case::a_type_paperback_does_not_read(
		Some("application/octet-stream"),
		"https://a.org/book.pdf\nThe server sends application/octet-stream, which is not a format Paperback reads.\nDownload it anyway?"
	)]
	#[case::no_type(None, "https://a.org/book.pdf\nThe server does not say what it sends.\nDownload it anyway?")]
	fn warning_text_says_what_the_server_sends(#[case] content_type: Option<&str>, #[case] expected: &str) {
		let _locale = test_locale::pinned_to("en");
		assert_eq!(warning_text("https://a.org/book.pdf", content_type), expected);
	}

	#[rstest]
	#[case::a_name_paperback_reads(false, "book.epub", "", false)]
	#[case::the_reader_accepted_the_warning(true, "book.epub", "", true)]
	#[case::a_name_without_an_extension(false, "download", "", true)]
	#[case::a_name_paperback_does_not_read(false, "download.php", "", true)]
	#[case::a_format_already_chosen(true, "download", "html", false)]
	#[case::a_chosen_format_paperback_no_longer_reads(false, "download", "xyz", true)]
	fn needs_open_as_unless_the_format_is_known(
		#[case] warned: bool,
		#[case] copy_name: &str,
		#[case] saved_format: &str,
		#[case] expected: bool,
	) {
		assert_eq!(needs_open_as(warned, copy_name, saved_format), expected);
	}

	#[test]
	fn report_lists_what_was_not_opened_and_why() {
		let _locale = test_locale::pinned_to("en");
		let outcomes = vec![
			Outcome::Downloaded {
				link: "https://a.org/x.epub".to_string(),
				copy: WorkingCopy::new("x.epub").unwrap(),
				warned: false,
			},
			Outcome::Refused { link: "https://a.org/setup.exe".to_string(), extension: "exe".to_string() },
			Outcome::Failed { link: "https://a.org/gone.pdf".to_string(), error: FetchError::Http(404) },
			Outcome::Skipped,
		];
		assert_eq!(
			report(&outcomes, &["see chapter 3".to_string()]),
			vec![
				"see chapter 3: this is not a web link",
				"https://a.org/setup.exe: Paperback cannot read .exe files, so it was not downloaded",
				"https://a.org/gone.pdf: the server answered with status 404",
			]
		);
	}
}
