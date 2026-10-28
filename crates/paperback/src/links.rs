use std::path::Path;

use paperback_core::{
	fetch::{self, FetchError},
	parser::{is_remote_url, parser_supports_path},
};
use patois::t;

use crate::working_copy::WorkingCopy;

pub fn prefill(clipboard: &str) -> String {
	clipboard.lines().map(str::trim).find(|line| is_remote_url(line)).unwrap_or_default().to_string()
}

pub fn not_a_link(text: &str) -> String {
	// TRANSLATORS: Follows text typed into Open from URL that is not an http or https link
	format!("{text}: {}", t("this is not a web link"))
}

#[derive(Debug, Clone, Copy)]
pub enum Stage {
	Checking,
	Downloading,
}

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

pub fn needs_open_as(warned: bool, copy: &Path) -> bool {
	warned || !parser_supports_path(copy)
}

pub enum Outcome {
	Downloaded { link: String, copy: WorkingCopy, warned: bool },
	Failed { link: String, error: FetchError },
	Skipped,
}

pub fn report(outcomes: &[Outcome]) -> Vec<String> {
	outcomes
		.iter()
		.filter_map(|outcome| match outcome {
			Outcome::Failed { link, error } => Some(format!("{link}: {error}")),
			Outcome::Downloaded { .. } | Outcome::Skipped => None,
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::{test_locale, working_copy::WorkingCopy};

	#[test]
	fn prefill_takes_the_first_link_on_the_clipboard() {
		assert_eq!(prefill("notes\r\n  https://a.org/x.epub \nhttps://b.org/y.pdf"), "https://a.org/x.epub");
	}

	#[test]
	fn prefill_ignores_a_link_inside_a_sentence() {
		assert_eq!(prefill("Read this: https://a.org/x.epub"), "");
	}

	#[test]
	fn not_a_link_names_what_was_typed() {
		let _locale = test_locale::pinned_to("en");
		assert_eq!(not_a_link("see chapter 3"), "see chapter 3: this is not a web link");
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
	#[case::a_name_paperback_reads(false, "book.epub", false)]
	#[case::the_reader_accepted_the_warning(true, "book.epub", true)]
	#[case::a_name_without_an_extension(false, "download", true)]
	#[case::a_name_paperback_does_not_read(false, "download.php", true)]
	#[case::a_gzipped_man_page(false, "ls.1.gz", false)]
	fn needs_open_as_after_a_warning_or_for_a_file_paperback_does_not_read(
		#[case] warned: bool,
		#[case] copy_name: &str,
		#[case] expected: bool,
	) {
		assert_eq!(needs_open_as(warned, Path::new(copy_name)), expected);
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
			Outcome::Failed {
				link: "https://a.org/setup.exe".to_string(),
				error: FetchError::Refused("exe".to_string()),
			},
			Outcome::Failed { link: "https://a.org/gone.pdf".to_string(), error: FetchError::Http(404) },
			Outcome::Skipped,
		];
		assert_eq!(
			report(&outcomes),
			vec![
				"https://a.org/setup.exe: .exe files cannot be read, so the link was not downloaded",
				"https://a.org/gone.pdf: the server answered with status 404",
			]
		);
	}
}
