//! Checking and naming a document that comes from a link.

use std::path::Path;

use paperback_formats::{ALL, FormatMeta};
use percent_encoding::percent_decode_str;

use crate::parser::{parser_supports_extension, url_extension, url_file_name};

#[cfg(feature = "fetch")]
mod net;
#[cfg(feature = "fetch")]
pub use net::{FetchError, Remote, RemoteInfo, open};

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
/// server's file name and its MIME type. A web page sent where the names promise another kind of
/// document gets [`Verdict::Warn`].
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
	let other_extension =
		|extension: &String| html_format().is_none_or(|format| !format.extensions.contains(&extension.as_str()));
	if content_type.is_some_and(is_web_page) && extensions.iter().any(other_extension) {
		return Verdict::Warn;
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
	let essence = essence(content_type);
	ALL.iter()
		.find(|format| format.mime_types.contains(&essence.as_str()))
		.and_then(|format| format.extensions.first().copied())
		.filter(|extension| parser_supports_extension(extension))
}

/// Whether `content_type` is one of the HTML format's MIME types, matched without parameters and
/// case.
#[must_use]
pub fn is_web_page(content_type: &str) -> bool {
	let essence = essence(content_type);
	html_format().is_some_and(|format| format.mime_types.contains(&essence.as_str()))
}

/// The format that `text/html` belongs to.
fn html_format() -> Option<&'static FormatMeta> {
	ALL.iter().copied().find(|format| format.mime_types.contains(&"text/html"))
}

/// `content_type` without parameters, lowercased.
fn essence(content_type: &str) -> String {
	content_type.split(';').next().unwrap_or_default().trim().to_ascii_lowercase()
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

/// The name a link's working copy is saved under: the server's file name, else the final link's
/// last segment, else `document`. A name without a supported extension gets the MIME type's
/// extension, else the supported extension of the final link, else that of the link given.
#[must_use]
pub fn file_name_for(
	given_url: &str,
	final_url: &str,
	disposition_name: Option<&str>,
	content_type: Option<&str>,
) -> String {
	let name = disposition_name
		.and_then(safe_file_name)
		.or_else(|| url_file_name(final_url).as_deref().and_then(safe_file_name))
		.unwrap_or_else(|| "document".to_string());
	if name_extension(&name).is_some_and(|extension| parser_supports_extension(&extension)) {
		return name;
	}
	let extension = content_type.and_then(extension_for_mime).map(str::to_string).or_else(|| {
		[final_url, given_url]
			.into_iter()
			.filter_map(url_extension)
			.find(|extension| parser_supports_extension(extension))
	});
	match extension {
		Some(extension) => format!("{name}.{extension}"),
		None => name,
	}
}

fn name_extension(name: &str) -> Option<String> {
	Path::new(name).extension().and_then(|extension| extension.to_str()).map(str::to_ascii_lowercase)
}

/// The longest file name saved, in bytes, leaving room for `.part` and the folder in the path.
const MAX_NAME_BYTES: usize = 200;

/// The part after the last `/` or `\`, with control characters and `<>:"|?*` replaced by `_`,
/// trailing dots and spaces removed, `_` put before a Windows device name, and the name cut to
/// [`MAX_NAME_BYTES`] with its extension kept; `None` if nothing is left.
fn safe_file_name(name: &str) -> Option<String> {
	let base = name.rsplit(['/', '\\']).next().unwrap_or_default();
	let replaced: String =
		base.chars().map(|c| if c.is_control() || r#"<>:"|?*"#.contains(c) { '_' } else { c }).collect();
	let trimmed = replaced.trim().trim_end_matches(['.', ' ']);
	if trimmed.is_empty() {
		return None;
	}
	let named = if is_device_name(trimmed) { format!("_{trimmed}") } else { trimmed.to_string() };
	Some(shortened(&named))
}

/// Whether Windows reads `name` as a device: `CON`, `PRN`, `AUX`, `NUL`, or `COM` or `LPT` followed
/// by a digit or one of `¹²³`, in any case and with any extension.
fn is_device_name(name: &str) -> bool {
	let stem = name.split('.').next().unwrap_or_default().trim_end().to_ascii_uppercase();
	let characters: Vec<char> = stem.chars().collect();
	matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
		|| (characters.len() == 4
			&& (stem.starts_with("COM") || stem.starts_with("LPT"))
			&& matches!(characters[3], '0'..='9' | '\u{b9}' | '\u{b2}' | '\u{b3}'))
}

/// `name` cut to [`MAX_NAME_BYTES`] on a character boundary, keeping an extension of up to 15 bytes.
fn shortened(name: &str) -> String {
	if name.len() <= MAX_NAME_BYTES {
		return name.to_string();
	}
	let extension = Path::new(name)
		.extension()
		.and_then(|extension| extension.to_str())
		.filter(|extension| extension.len() < 16)
		.map_or_else(String::new, |extension| format!(".{extension}"));
	let mut end = MAX_NAME_BYTES - extension.len();
	while !name.is_char_boundary(end) {
		end -= 1;
	}
	format!("{}{extension}", &name[..end])
}

#[cfg(test)]
mod tests;
