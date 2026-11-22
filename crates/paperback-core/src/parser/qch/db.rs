use std::collections::HashSet;

use anyhow::{Context, Result, bail};
use diesel::{prelude::*, sqlite::SqliteConnection};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};

use super::{
	compress::q_uncompress,
	schema::{contents, file_data, file_names, folders, metadata, namespaces, sqlite_master},
};

/// Kept as written in a URI path, so only what SQLite's URI parser would misread is escaped.
const URI_PATH: &AsciiSet =
	&NON_ALPHANUMERIC.remove(b'/').remove(b':').remove(b'-').remove(b'_').remove(b'.').remove(b'~');

/// Every table the reader queries; a file without them is some other SQLite database, or a collection file (`.qhc`), which holds no pages.
const REQUIRED_TABLES: [&str; 5] = ["NamespaceTable", "FolderTable", "FileNameTable", "FileDataTable", "ContentsTable"];

#[derive(Queryable, Selectable)]
#[diesel(table_name = file_names, check_for_backend(diesel::sqlite::Sqlite))]
struct FileNameRow {
	name: Option<String>,
	file_id: Option<i64>,
}

/// A file of the book's folder, in the order `qhelpgenerator` stored it, which is the order of the project's file list.
#[derive(Debug, Clone)]
pub(super) struct FileEntry {
	pub name: String,
	pub file_id: i64,
}

pub(super) struct QchDatabase {
	connection: SqliteConnection,
	has_metadata: bool,
}

impl QchDatabase {
	/// Read-only and immutable, as Qt's reader opens it: no journal, no lock, and nothing written beside the file, which may be on read-only media.
	pub fn open(path: &str) -> Result<Self> {
		let uri = sqlite_uri(path);
		let mut connection =
			SqliteConnection::establish(&uri).with_context(|| format!("Failed to open Qt help file: {path}"))?;
		let present: HashSet<String> = sqlite_master::table
			.filter(sqlite_master::kind.eq("table"))
			.select(sqlite_master::name)
			.load::<Option<String>>(&mut connection)
			.with_context(|| format!("Not a Qt help file: {path}"))?
			.into_iter()
			.flatten()
			.collect();
		if let Some(missing) = REQUIRED_TABLES.iter().find(|table| !present.contains(**table)) {
			bail!("Not a Qt help file, as it has no {missing}: {path}");
		}
		Ok(Self { connection, has_metadata: present.contains("MetaDataTable") })
	}

	/// What Qt's `QHelpDBReader::namespaceName` returns: the first namespace.
	pub fn namespace(&mut self) -> Result<String> {
		namespaces::table
			.select(namespaces::name)
			.order(namespaces::id)
			.first::<Option<String>>(&mut self.connection)
			.optional()?
			.flatten()
			.filter(|name| !name.is_empty())
			.context("Qt help file names no namespace")
	}

	/// Qt's reader takes the folder with id 1, the only one `qhelpgenerator` writes.
	pub fn virtual_folder(&mut self) -> Result<(i64, String)> {
		folders::table
			.select((folders::id, folders::name))
			.order(folders::id)
			.first::<(i64, Option<String>)>(&mut self.connection)
			.optional()?
			.and_then(|(id, name)| Some((id, name.filter(|name| !name.is_empty())?)))
			.context("Qt help file has no virtual folder")
	}

	/// Files made before Qt 4.4 have no metadata table, and so no value.
	pub fn metadata(&mut self, name: &str) -> Result<Option<String>> {
		if !self.has_metadata {
			return Ok(None);
		}
		let values = metadata::table
			.filter(metadata::name.eq(name))
			.select(metadata::value)
			.load::<Option<Vec<u8>>>(&mut self.connection)?;
		// Qt answers only when the name is there once.
		Ok(match values.as_slice() {
			[Some(value)] => Some(String::from_utf8_lossy(value).into_owned()),
			_ => None,
		})
	}

	/// Only names in the folder count, as in Qt's reader, which leaves out the nameless page `qhelpgenerator` points missing links at.
	pub fn files(&mut self, folder_id: i64) -> Result<Vec<FileEntry>> {
		let rows = file_names::table
			.inner_join(folders::table.on(file_names::folder_id.eq(folders::id.nullable())))
			.filter(folders::id.eq(folder_id))
			.order(file_names::file_id)
			.select(FileNameRow::as_select())
			.load(&mut self.connection)?;
		let mut seen = HashSet::new();
		Ok(rows
			.into_iter()
			.filter_map(|row| {
				let name = row.name.filter(|name| !name.is_empty())?;
				let name = name.strip_prefix("./").map(str::to_string).unwrap_or(name);
				seen.insert(name.clone()).then_some(FileEntry { name, file_id: row.file_id? })
			})
			.collect())
	}

	/// In order, without the copies a project gets when two filter sections list the same contents.
	pub fn contents(&mut self) -> Result<Vec<Vec<u8>>> {
		let rows =
			contents::table.select(contents::data).order(contents::id).load::<Option<Vec<u8>>>(&mut self.connection)?;
		let mut seen = HashSet::new();
		Ok(rows.into_iter().flatten().filter(|data| !data.is_empty() && seen.insert(data.clone())).collect())
	}

	pub fn file_data(&mut self, file_id: i64) -> Result<Vec<u8>> {
		let data = file_data::table
			.find(file_id)
			.select(file_data::data)
			.first::<Option<Vec<u8>>>(&mut self.connection)
			.optional()?
			.with_context(|| format!("Qt help file has no data for file {file_id}"))?
			.unwrap_or_default();
		q_uncompress(&data).with_context(|| format!("file {file_id} of the Qt help file is unreadable"))
	}

	/// For a picture, named as the parser named it; `qhelpgenerator` may have kept a leading `./`, which Qt's reader also tries.
	pub fn file_data_by_name(&mut self, name: &str) -> Result<Vec<u8>> {
		let (folder_id, _) = self.virtual_folder()?;
		let dotted = format!("./{name}");
		let file_id = file_names::table
			.inner_join(folders::table.on(file_names::folder_id.eq(folders::id.nullable())))
			.filter(folders::id.eq(folder_id))
			.filter(file_names::name.eq(name).or(file_names::name.eq(&dotted)))
			.select(file_names::file_id)
			.first::<Option<i64>>(&mut self.connection)
			.optional()?
			.flatten()
			.with_context(|| format!("Qt help file has no file named {name}"))?;
		self.file_data(file_id)
	}
}

/// A URI, so the query string can ask for read-only and immutable. Windows paths turn their backslashes and verbatim prefixes into what SQLite's URI parser expects, and UNC paths keep their leading slashes.
fn sqlite_uri(path: &str) -> String {
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

	use super::sqlite_uri;

	#[rstest]
	#[case(r"C:\Users\A B\help#1.qch", "file:///C:/Users/A%20B/help%231.qch?mode=ro&immutable=1")]
	#[case(r"\\?\C:\docs\qtcore.qch", "file:///C:/docs/qtcore.qch?mode=ro&immutable=1")]
	#[case(r"\\server\share\qtcore.qch", "file:////server/share/qtcore.qch?mode=ro&immutable=1")]
	#[case(r"\\?\UNC\server\share\a.qch", "file:////server/share/a.qch?mode=ro&immutable=1")]
	#[case("/home/me/what?.qch", "file:///home/me/what%3F.qch?mode=ro&immutable=1")]
	#[case("relative/50%.qch", "file:relative/50%25.qch?mode=ro&immutable=1")]
	fn builds_read_only_uris(#[case] path: &str, #[case] expected: &str) {
		assert_eq!(sqlite_uri(path), expected);
	}
}
