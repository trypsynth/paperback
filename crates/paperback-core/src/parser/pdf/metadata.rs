//! Mapping pdfium's document-open errors to user-facing messages, and reading sanitized
//! metadata fields (title/author) off an opened document.

use anyhow::anyhow;
use pdfium::{PdfiumDocument, PdfiumError};

use super::text::sanitize_pdf_text;
use crate::{parser::PASSWORD_REQUIRED_ERROR_PREFIX, t, util::text::trim_string};

pub(super) fn map_load_error(err: PdfiumError) -> anyhow::Error {
	match err {
		PdfiumError::PasswordError => {
			// TRANSLATORS: Error detail shown when a PDF's password is missing or wrong (the internal sentinel prefix before it is not translated)
			anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password required or incorrect"))
		}
		// TRANSLATORS: Error shown when a PDF fails to open for a reason other than a password; {} is the underlying error detail
		other => anyhow!(t("Failed to open PDF document: {}").replace("{}", &other.to_string())),
	}
}

pub(super) fn metadata_value(document: &PdfiumDocument, key: &str) -> Option<String> {
	document
		.metadata_value(key)
		.ok()
		.map(|value| trim_string(&sanitize_pdf_text(&value)))
		.filter(|value| !value.is_empty())
}
