//! Qt Assistant shows a page at `qthelp://<namespace>/<folder>/<name>`, so a link is resolved in that space: relative to the page's own URL, and inside the book only while it stays in the book's folder. A relative link can climb out into another book's folder, as Qt's own modules link to each other, and those, like `qthelp://` links to other namespaces, are left as `qthelp://` URLs that no position in this book answers.

use crate::{
	parser::{is_data_uri, is_external_url, util::path::resolve_relative_path},
	util::text::url_decode,
};

pub(super) struct BookLocation {
	pub namespace: String,
	pub folder: String,
}

/// Where a link or picture points.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Target {
	/// A file of this book, by its name in `FileNameTable`, and the fragment after `#`, if any.
	Internal { name: String, fragment: Option<String> },
	/// Anything else, kept as an address for the reader to open or report.
	External(String),
}

impl Target {
	/// Paths in the document are the file's name with a leading slash, the shape the reader's link lookup shares with CHM.
	pub fn into_reference(self) -> String {
		match self {
			Self::Internal { name, fragment: Some(fragment) } => format!("/{name}#{fragment}"),
			Self::Internal { name, fragment: None } => format!("/{name}"),
			Self::External(url) => url,
		}
	}
}

pub(super) fn document_path(name: &str) -> String {
	format!("/{name}")
}

impl BookLocation {
	/// `current_name` is the page the link is on, or empty for the table of contents, whose links are relative to the folder.
	pub fn resolve(&self, current_name: &str, href: &str) -> Target {
		let href = href.trim();
		if is_external_url(href) || is_data_uri(href) {
			return Target::External(href.to_string());
		}
		let (address, fragment) =
			href.split_once('#').map_or((href, None), |(address, fragment)| (address, Some(fragment)));
		let fragment = fragment.filter(|fragment| !fragment.is_empty()).map(str::to_string);
		let address = address.split_once('?').map_or(address, |(address, _)| address);
		if let Some(rest) = strip_prefix_ignore_case(address, "qthelp://") {
			let (authority, path) = rest.split_once('/').unwrap_or((rest, ""));
			if !authority.eq_ignore_ascii_case(&self.namespace) {
				return Target::External(href.to_string());
			}
			return self.internal_or_foreign(&resolve_relative_path("", &url_decode(path)), fragment, href);
		}
		if has_scheme(address) {
			return Target::External(href.to_string());
		}
		if address.is_empty() {
			return if current_name.is_empty() {
				Target::External(href.to_string())
			} else {
				Target::Internal { name: current_name.to_string(), fragment }
			};
		}
		let base = match current_name.rsplit_once('/') {
			Some((directory, _)) => format!("{}/{directory}", self.folder),
			None => self.folder.clone(),
		};
		let resolved = resolve_relative_path(&base, &url_decode(address));
		self.internal_or_foreign(&resolved, fragment, href)
	}

	/// `path` is under the namespace, so it starts with a folder name.
	fn internal_or_foreign(&self, path: &str, fragment: Option<String>, href: &str) -> Target {
		match path.split_once('/') {
			Some((folder, name)) if folder == self.folder && !name.is_empty() => {
				Target::Internal { name: name.to_string(), fragment }
			}
			_ if href.to_ascii_lowercase().starts_with("qthelp:") => Target::External(href.to_string()),
			_ => {
				let fragment = fragment.map(|fragment| format!("#{fragment}")).unwrap_or_default();
				Target::External(format!("qthelp://{}/{path}{fragment}", self.namespace))
			}
		}
	}
}

fn strip_prefix_ignore_case<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
	text.get(..prefix.len()).filter(|head| head.eq_ignore_ascii_case(prefix)).map(|_| &text[prefix.len()..])
}

/// A drive letter isn't a scheme, though no page in a Qt help file would have one.
fn has_scheme(address: &str) -> bool {
	address.split_once(':').is_some_and(|(scheme, _)| {
		scheme.len() > 1
			&& scheme.starts_with(|c: char| c.is_ascii_alphabetic())
			&& scheme.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
	})
}

#[cfg(test)]
mod tests;
