//! Checking and naming a document that comes from a link.

use std::path::Path;

use paperback_formats::ALL;
use percent_encoding::percent_decode_str;

use crate::parser::{parser_supports_extension, url_extension, url_file_name};

/// What may happen to a link once its names and type are known.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
	/// A supported extension, or no extension and a supported type: download it.
	Pass,
	/// No extension anywhere, and a type no format claims: ask before downloading.
	Warn,
	/// The given link, the final link or the server's file name has this unsupported extension.
	Refuse(String),
}

/// The unsupported extension the link itself names, checked before contacting the server.
#[must_use]
pub fn refused_extension(url: &str) -> Option<String> {
	url_extension(url).filter(|extension| !parser_supports_extension(extension))
}

/// The [`Verdict`] for a link, from the link given, the link it ended at after redirects, the
/// server's file name and its MIME type.
#[must_use]
pub fn verdict(
	given_url: &str,
	final_url: &str,
	disposition_name: Option<&str>,
	content_type: Option<&str>,
) -> Verdict {
	let extensions: Vec<String> =
		[url_extension(given_url), url_extension(final_url), disposition_name.and_then(name_extension)]
			.into_iter()
			.flatten()
			.collect();
	if let Some(refused) = extensions.iter().find(|extension| !parser_supports_extension(extension)) {
		return Verdict::Refuse(refused.clone());
	}
	if !extensions.is_empty() || content_type.and_then(extension_for_mime).is_some() {
		Verdict::Pass
	} else {
		Verdict::Warn
	}
}

/// The first extension of the format whose MIME types include `content_type`, matched without
/// parameters and case.
#[must_use]
pub fn extension_for_mime(content_type: &str) -> Option<&'static str> {
	let essence = content_type.split(';').next().unwrap_or_default().trim().to_ascii_lowercase();
	ALL.iter()
		.find(|format| format.mime_types.contains(&essence.as_str()))
		.and_then(|format| format.extensions.first().copied())
		.filter(|extension| parser_supports_extension(extension))
}

/// The file name in a `Content-Disposition` header; `filename*` wins over `filename`.
#[must_use]
pub fn disposition_file_name(header: &str) -> Option<String> {
	let mut plain = None;
	for part in header.split(';') {
		let Some((key, value)) = part.split_once('=') else { continue };
		let value = value.trim();
		match key.trim().to_ascii_lowercase().as_str() {
			"filename*" => {
				if let Some(encoded) = value.splitn(3, '\'').nth(2) {
					return safe_file_name(&percent_decode_str(encoded).decode_utf8_lossy());
				}
			}
			"filename" => plain = safe_file_name(value.trim_matches('"')),
			_ => {}
		}
	}
	plain
}

/// The name a link's working copy is saved under: the server's file name, else the link's last
/// segment, else `document`; the MIME type's extension is appended when the name has no
/// supported one.
#[must_use]
pub fn file_name_for(final_url: &str, disposition_name: Option<&str>, content_type: Option<&str>) -> String {
	let name = disposition_name
		.and_then(safe_file_name)
		.or_else(|| url_file_name(final_url).as_deref().and_then(safe_file_name))
		.unwrap_or_else(|| "document".to_string());
	let supported = name_extension(&name).is_some_and(|extension| parser_supports_extension(&extension));
	match content_type.and_then(extension_for_mime) {
		Some(extension) if !supported => format!("{name}.{extension}"),
		_ => name,
	}
}

fn name_extension(name: &str) -> Option<String> {
	Path::new(name).extension().and_then(|extension| extension.to_str()).map(str::to_ascii_lowercase)
}

/// The part after the last `/` or `\`, with control characters and `<>:"|?*` replaced by `_` and
/// trailing dots and spaces removed; `None` if nothing is left.
fn safe_file_name(name: &str) -> Option<String> {
	let base = name.rsplit(['/', '\\']).next().unwrap_or_default();
	let replaced: String =
		base.chars().map(|c| if c.is_control() || r#"<>:"|?*"#.contains(c) { '_' } else { c }).collect();
	let trimmed = replaced.trim().trim_end_matches(['.', ' ']);
	(!trimmed.is_empty()).then(|| trimmed.to_string())
}

#[cfg(test)]
mod tests;
