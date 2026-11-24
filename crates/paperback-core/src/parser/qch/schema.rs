//! The tables `qhelpgenerator` writes, as `helpgenerator.cpp` in qttools creates them. Only those the reader needs are declared.
//!
//! Every column but an `INTEGER PRIMARY KEY` is declared without `NOT NULL`, so each is nullable here. Diesel needs a primary key for every table; the ones that have none in the file name the column rows are looked up by, which is only a name to Diesel and never enforced.

diesel::table! {
	/// SQLite's own catalogue, to tell a Qt help file from another SQLite database and an old file from a new one before any query fails.
	#[sql_name = "sqlite_master"]
	sqlite_master (name) {
		#[sql_name = "type"]
		kind -> Nullable<Text>,
		name -> Nullable<Text>,
	}
}

diesel::table! {
	/// One row, the namespace that `qthelp://` links name the book by, such as `org.qt-project.qtcore.680`.
	#[sql_name = "NamespaceTable"]
	namespaces (id) {
		#[sql_name = "Id"]
		id -> BigInt,
		#[sql_name = "Name"]
		name -> Nullable<Text>,
	}
}

diesel::table! {
	/// The virtual folder every page lives under, such as `qtcore`; Qt's reader takes the one with id 1.
	#[sql_name = "FolderTable"]
	folders (id) {
		#[sql_name = "Id"]
		id -> BigInt,
		#[sql_name = "Name"]
		name -> Nullable<Text>,
		#[sql_name = "NamespaceID"]
		namespace_id -> Nullable<BigInt>,
	}
}

diesel::table! {
	/// Absent from files made before Qt 4.4. Values are bound from a `QVariant`, so text, or whatever else a project's metadata held.
	#[sql_name = "MetaDataTable"]
	metadata (name) {
		#[sql_name = "Name"]
		name -> Nullable<Text>,
		#[sql_name = "Value"]
		value -> Nullable<Binary>,
	}
}

diesel::table! {
	/// Names are paths relative to the folder, cleaned by `QDir::cleanPath`, and matched case-sensitively.
	#[sql_name = "FileNameTable"]
	file_names (file_id) {
		#[sql_name = "FolderId"]
		folder_id -> Nullable<BigInt>,
		#[sql_name = "Name"]
		name -> Nullable<Text>,
		#[sql_name = "FileId"]
		file_id -> Nullable<BigInt>,
		#[sql_name = "Title"]
		title -> Nullable<Text>,
	}
}

diesel::table! {
	/// Each file as `qCompress` leaves it: a big-endian length, then a zlib stream.
	#[sql_name = "FileDataTable"]
	file_data (id) {
		#[sql_name = "Id"]
		id -> BigInt,
		#[sql_name = "Data"]
		data -> Nullable<Binary>,
	}
}

diesel::table! {
	/// One row per filter section of the project, each a `QDataStream` of the section's table of contents.
	#[sql_name = "ContentsTable"]
	contents (id) {
		#[sql_name = "Id"]
		id -> BigInt,
		#[sql_name = "NamespaceId"]
		namespace_id -> Nullable<BigInt>,
		#[sql_name = "Data"]
		data -> Nullable<Binary>,
	}
}

diesel::allow_tables_to_appear_in_same_query!(file_names, folders);
