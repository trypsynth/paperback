use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use rusqlite::{Connection, OpenFlags};

/// Kept as written in a URI path, so only what SQLite's URI parser would misread is escaped.
const URI_PATH: &AsciiSet =
	&NON_ALPHANUMERIC.remove(b'/').remove(b':').remove(b'-').remove(b'_').remove(b'.').remove(b'~');

/// Read-only and immutable: no journal, no lock, and nothing written beside the file, which may be on read-only media.
pub fn open_read_only(path: &str) -> rusqlite::Result<Connection> {
	Connection::open_with_flags(
		read_only_uri(path),
		OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_NO_MUTEX,
	)
}

/// A URI, so the query string can ask for read-only and immutable. Windows paths turn their backslashes and verbatim prefixes into what SQLite's URI parser expects, and UNC paths keep their leading slashes.
fn read_only_uri(path: &str) -> String {
	let path = path.strip_prefix(r"\\?\UNC\").map_or_else(
		|| path.strip_prefix(r"\\?\").unwrap_or(path).replace('\\', "/"),
		|rest| format!("//{}", rest.replace('\\', "/")),
	);
	let encoded = utf8_percent_encode(&path, URI_PATH).to_string();
	let encoded = if encoded.starts_with('/') {
		format!("//{encoded}")
	} else if encoded.as_bytes().get(1) == Some(&b':') {
		format!("///{encoded}")
	} else {
		encoded
	};
	format!("file:{encoded}?mode=ro&immutable=1")
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::read_only_uri;

	#[rstest]
	#[case(r"C:\Users\A B\help#1.qch", "file:///C:/Users/A%20B/help%231.qch?mode=ro&immutable=1")]
	#[case(r"\\?\C:\docs\qtcore.qch", "file:///C:/docs/qtcore.qch?mode=ro&immutable=1")]
	#[case(r"\\server\share\qtcore.qch", "file:////server/share/qtcore.qch?mode=ro&immutable=1")]
	#[case(r"\\?\UNC\server\share\a.qch", "file:////server/share/a.qch?mode=ro&immutable=1")]
	#[case("/home/me/what?.qch", "file:///home/me/what%3F.qch?mode=ro&immutable=1")]
	#[case("relative/50%.qch", "file:relative/50%25.qch?mode=ro&immutable=1")]
	fn builds_read_only_uris(#[case] path: &str, #[case] expected: &str) {
		assert_eq!(read_only_uri(path), expected);
	}
}
