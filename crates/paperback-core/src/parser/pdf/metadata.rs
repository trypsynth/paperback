//! Mapping pdfium's document-open errors to user-facing messages, and reading sanitized
//! metadata fields (title/author) off an opened document.

use anyhow::anyhow;

use super::text::sanitize_pdf_text;
use crate::{
	parser::PASSWORD_REQUIRED_ERROR_PREFIX,
	pdfium::{PdfDocument, PdfError},
	t,
	util::text::trim_string,
};

pub(super) fn map_load_error(err: PdfError) -> anyhow::Error {
	match err {
		PdfError::PasswordRequired => {
			// TRANSLATORS: Error detail shown when a PDF's password is missing or wrong (the internal sentinel prefix before it is not translated)
			anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password required or incorrect"))
		}
		// TRANSLATORS: Error shown when a PDF fails to open for a reason other than a password; {} is the underlying error detail
		other => anyhow!(t("Failed to open PDF document: {}").replace("{}", &other.to_string())),
	}
}

pub(super) fn metadata_value(document: &PdfDocument, key: &str) -> Option<String> {
	document.metadata(key).map(|value| trim_string(&sanitize_pdf_text(&value))).filter(|value| !value.is_empty())
}

/// The extensions a `/Title` carries when it is really the name of the file the PDF was made
/// from. Layout and word processors write their own document's name into the metadata, so
/// these turn up constantly in print-production PDFs. `.pdf` is here too: a title of
/// `book.pdf` is the export's own name, never what the book is called.
const AUTHORING_EXTENSIONS: [&str; 12] =
	[".indd", ".qxd", ".qxp", ".doc", ".docx", ".odt", ".rtf", ".tex", ".fm", ".pmd", ".pub", ".pdf"];

/// Titles that carry no information, which several tools write when the author never set one.
const PLACEHOLDER_TITLES: [&str; 5] = ["untitled", "untitled document", "unnamed", "no title", "document1"];

/// The book's title from the PDF's metadata, ignoring the values that are not titles at all.
///
/// A PDF's `/Title` is whatever produced it chose to put there, and print-production tools
/// routinely put their own source filename in: the reported case was a book whose metadata
/// title was `i-xx_Phobias-fm.indd`. The name of the file on disk is a better answer than
/// that, so returning nothing here lets the caller fall back to it.
pub(super) fn metadata_title(document: &PdfDocument) -> Option<String> {
	metadata_value(document, "Title").filter(|title| !is_not_really_a_title(title))
}

fn is_not_really_a_title(title: &str) -> bool {
	let lowered = title.to_lowercase();
	// "Microsoft Word - Chapter 3 final.doc" and its PowerPoint equivalent: the tool's name,
	// a dash, and then the file it was exported from.
	if lowered.starts_with("microsoft word - ") || lowered.starts_with("microsoft powerpoint - ") {
		return true;
	}
	if PLACEHOLDER_TITLES.contains(&lowered.as_str()) {
		return true;
	}
	AUTHORING_EXTENSIONS.iter().any(|extension| lowered.ends_with(extension))
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::is_not_really_a_title;

	/// <https://github.com/trypsynth/paperback/issues/882>: a book whose metadata title was the
	/// `InDesign` file it was laid out in, so the reader saw `i-xx_Phobias-fm.indd` where the
	/// book's name should be.
	#[rstest]
	#[case("i-xx_Phobias-fm.indd")]
	#[case("chapter 3 final.doc")]
	#[case("Book Layout.QXD")]
	#[case("thesis.tex")]
	#[case("The Encyclopedia of Phobias.pdf")]
	#[case("Microsoft Word - Front matter.doc")]
	#[case("Microsoft PowerPoint - deck")]
	#[case("Untitled")]
	#[case("untitled document")]
	fn a_source_filename_is_not_a_title(#[case] title: &str) {
		assert!(is_not_really_a_title(title), "{title} should have been rejected");
	}

	/// Far more important than catching the junk: never throwing away a real title. A book
	/// named after a file format, or one whose name merely contains a dot, has to survive.
	#[rstest]
	#[case("The Encyclopedia of Phobias, Fears, and Anxieties")]
	#[case("Programming in D")]
	#[case("Dr. No")]
	#[case("The Art of Computer Programming, Vol. 1")]
	#[case("R.U.R.")]
	#[case("Document Engineering")]
	#[case("Microsoft Word Step by Step")]
	#[case("A History of the Doc Holliday Legend")]
	#[case("Untitled Symphony No. 2")]
	fn a_real_title_is_kept(#[case] title: &str) {
		assert!(!is_not_really_a_title(title), "{title} should have been kept");
	}
}
