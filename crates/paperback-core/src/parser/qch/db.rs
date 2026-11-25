use std::collections::HashSet;

use anyhow::{Context, Result, bail};
use rusqlite::{Connection, OptionalExtension, types::Value};

use super::compress::q_uncompress;
use crate::util::sqlite::open_read_only;

/// Every table the reader queries; a file without them is some other SQLite database, or a collection file (`.qhc`), which holds no pages.
const REQUIRED_TABLES: [&str; 5] = ["NamespaceTable", "FolderTable", "FileNameTable", "FileDataTable", "ContentsTable"];

/// A file of the book's folder, in the order `qhelpgenerator` stored it, which is the order of the project's file list.
#[derive(Debug, Clone)]
pub(super) struct FileEntry {
	pub name: String,
	pub file_id: i64,
}

/// The tables are as `helpgenerator.cpp` in qttools creates them. Every column but an `INTEGER PRIMARY KEY` is declared without `NOT NULL`, so each is read as optional.
pub(super) struct QchDatabase {
	connection: Connection,
	has_metadata: bool,
}

impl QchDatabase {
	/// Read-only and immutable, as Qt's reader opens it.
	pub fn open(path: &str) -> Result<Self> {
		let connection = open_read_only(path).with_context(|| format!("Failed to open Qt help file: {path}"))?;
		let present: HashSet<String> = connection
			.prepare("SELECT name FROM sqlite_master WHERE type = 'table'")
			.and_then(|mut statement| {
				statement.query_map([], |row| row.get::<_, Option<String>>(0))?.collect::<rusqlite::Result<Vec<_>>>()
			})
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
	pub fn namespace(&self) -> Result<String> {
		self.connection
			.query_row("SELECT Name FROM NamespaceTable ORDER BY Id LIMIT 1", [], |row| row.get::<_, Option<String>>(0))
			.optional()?
			.flatten()
			.filter(|name| !name.is_empty())
			.context("Qt help file names no namespace")
	}

	/// Qt's reader takes the folder with id 1, the only one `qhelpgenerator` writes.
	pub fn virtual_folder(&self) -> Result<(i64, String)> {
		self.connection
			.query_row("SELECT Id, Name FROM FolderTable ORDER BY Id LIMIT 1", [], |row| {
				Ok((row.get::<_, i64>(0)?, row.get::<_, Option<String>>(1)?))
			})
			.optional()?
			.and_then(|(id, name)| Some((id, name.filter(|name| !name.is_empty())?)))
			.context("Qt help file has no virtual folder")
	}

	/// Files made before Qt 4.4 have no metadata table, and so no value. Values are bound from a `QVariant`, so text, or whatever else a project's metadata held.
	pub fn metadata(&self, name: &str) -> Result<Option<String>> {
		if !self.has_metadata {
			return Ok(None);
		}
		let mut statement = self.connection.prepare("SELECT Value FROM MetaDataTable WHERE Name = ?1")?;
		let values =
			statement.query_map([name], |row| row.get::<_, Value>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
		// Qt answers only when the name is there once.
		Ok(match values.as_slice() {
			[Value::Text(value)] => Some(value.clone()),
			[Value::Blob(value)] => Some(String::from_utf8_lossy(value).into_owned()),
			_ => None,
		})
	}

	/// Only names in the folder count, as in Qt's reader, which leaves out the nameless page `qhelpgenerator` points missing links at.
	pub fn files(&self, folder_id: i64) -> Result<Vec<FileEntry>> {
		let mut statement = self.connection.prepare(
			"SELECT n.Name, n.FileId FROM FileNameTable n JOIN FolderTable f ON n.FolderId = f.Id WHERE f.Id = ?1 ORDER BY n.FileId",
		)?;
		let rows = statement
			.query_map([folder_id], |row| Ok((row.get::<_, Option<String>>(0)?, row.get::<_, Option<i64>>(1)?)))?
			.collect::<rusqlite::Result<Vec<_>>>()?;
		let mut seen = HashSet::new();
		Ok(rows
			.into_iter()
			.filter_map(|(name, file_id)| {
				let name = name.filter(|name| !name.is_empty())?;
				let name = name.strip_prefix("./").map(str::to_string).unwrap_or(name);
				seen.insert(name.clone()).then_some(FileEntry { name, file_id: file_id? })
			})
			.collect())
	}

	/// In order, without the copies a project gets when two filter sections list the same contents. Each is a `QDataStream` of a filter section's table of contents.
	pub fn contents(&self) -> Result<Vec<Vec<u8>>> {
		let mut statement = self.connection.prepare("SELECT Data FROM ContentsTable ORDER BY Id")?;
		let rows =
			statement.query_map([], |row| row.get::<_, Option<Vec<u8>>>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
		let mut seen = HashSet::new();
		Ok(rows.into_iter().flatten().filter(|data| !data.is_empty() && seen.insert(data.clone())).collect())
	}

	/// Each file as `qCompress` leaves it: a big-endian length, then a zlib stream.
	pub fn file_data(&self, file_id: i64) -> Result<Vec<u8>> {
		let data = self
			.connection
			.query_row("SELECT Data FROM FileDataTable WHERE Id = ?1", [file_id], |row| {
				row.get::<_, Option<Vec<u8>>>(0)
			})
			.optional()?
			.with_context(|| format!("Qt help file has no data for file {file_id}"))?
			.unwrap_or_default();
		q_uncompress(&data).with_context(|| format!("file {file_id} of the Qt help file is unreadable"))
	}

	/// For a picture, named as the parser named it; `qhelpgenerator` may have kept a leading `./`, which Qt's reader also tries. Names are matched case-sensitively, as Qt does.
	pub fn file_data_by_name(&self, name: &str) -> Result<Vec<u8>> {
		let (folder_id, _) = self.virtual_folder()?;
		let file_id = self
			.connection
			.query_row(
				"SELECT n.FileId FROM FileNameTable n JOIN FolderTable f ON n.FolderId = f.Id WHERE f.Id = ?1 AND (n.Name = ?2 OR n.Name = ?3) LIMIT 1",
				rusqlite::params![folder_id, name, format!("./{name}")],
				|row| row.get::<_, Option<i64>>(0),
			)
			.optional()?
			.flatten()
			.with_context(|| format!("Qt help file has no file named {name}"))?;
		self.file_data(file_id)
	}
}
