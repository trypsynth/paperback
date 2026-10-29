use anyhow::anyhow;

use super::text::sanitize_pdf_text;
use crate::{
	parser::PASSWORD_REQUIRED_ERROR_PREFIX,
	pdf_engine::{PdfDocument, PdfError},
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

/// Layout and word processors write their own source file's name into `/Title`, and a title of `book.pdf` is the export's own name.
const AUTHORING_EXTENSIONS: [&str; 12] =
	[".indd", ".qxd", ".qxp", ".doc", ".docx", ".odt", ".rtf", ".tex", ".fm", ".pmd", ".pub", ".pdf"];

const PLACEHOLDER_TITLES: [&str; 5] = ["untitled", "untitled document", "unnamed", "no title", "document1"];

/// Words no book is called on their own, which turn up as a whole title when a tool picks a stray word off a page (#986 had `FROM`). Articles, prepositions and conjunctions only: pronouns name real books (*It*, *Us*, *We*, *Them*), and so does *If—*.
const FUNCTION_WORDS: [&str; 18] = [
	"a", "an", "the", "and", "or", "but", "nor", "of", "to", "from", "in", "on", "at", "by", "for", "with", "into",
	"than",
];

/// Returning nothing lets the caller fall back to the file name on disk, which beats a source filename like `i-xx_Phobias-fm.indd`.
pub(super) fn metadata_title(document: &PdfDocument) -> Option<String> {
	metadata_value(document, "Title").filter(|title| !is_not_really_a_title(title))
}

fn is_not_really_a_title(title: &str) -> bool {
	let lowered = title.to_lowercase();
	if lowered.starts_with("microsoft word - ") || lowered.starts_with("microsoft powerpoint - ") {
		return true;
	}
	if PLACEHOLDER_TITLES.contains(&lowered.as_str()) || FUNCTION_WORDS.contains(&lowered.as_str()) {
		return true;
	}
	AUTHORING_EXTENSIONS.iter().any(|extension| lowered.ends_with(extension)) || looks_like_a_job_name(title)
}

/// Whether a title is one unbroken token with a dot and a digit in it, which is the name a typesetting system gave the job rather than anything the book is called: #1003 had `RudzkaV33.pod.O241`. A title of one word has no dot (*Emma*, *1984*), and one with dots but no digit is initials (*R.U.R.*).
fn looks_like_a_job_name(title: &str) -> bool {
	!title.contains(char::is_whitespace) && title.contains('.') && title.contains(|c: char| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::is_not_really_a_title;

	/// <https://github.com/trypsynth/paperback/issues/882>: the metadata title was the `InDesign` file the book was laid out in.
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
	#[case("FROM")]
	#[case("the")]
	#[case("RudzkaV33.pod.O241")]
	#[case("ms_v12.final")]
	fn a_source_filename_is_not_a_title(#[case] title: &str) {
		assert!(is_not_really_a_title(title), "{title} should have been rejected");
	}

	/// Never throwing away a real title matters far more than catching the junk.
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
	#[case("It")]
	#[case("Us")]
	#[case("If—")]
	#[case("From Here to Eternity")]
	#[case("1984")]
	#[case("Catch-22")]
	#[case("S.T.A.L.K.E.R.")]
	#[case("Web 2.0 Architectures")]
	fn a_real_title_is_kept(#[case] title: &str) {
		assert!(!is_not_really_a_title(title), "{title} should have been kept");
	}
}
