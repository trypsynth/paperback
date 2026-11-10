use std::{
	env, fs,
	io::Write,
	path::{Path, PathBuf},
};

use flate2::{Compression, write::GzEncoder};

fn main() {
	shipfitter::build::embed_commit_info("PAPERBACK");
	pack_braille_tables();
}

/// Packs the liblouis tables into one gzipped file the parser includes, as a name, its contents, and their lengths, so the binary carries them on every platform.
fn pack_braille_tables() {
	let dir = Path::new("liblouis/tables");
	println!("cargo::rerun-if-changed={}", dir.display());
	let mut names: Vec<PathBuf> = fs::read_dir(dir)
		.expect("read liblouis/tables")
		.map(|entry| entry.expect("list liblouis/tables").path())
		.collect();
	names.sort();
	let mut packed = GzEncoder::new(Vec::new(), Compression::best());
	for path in names {
		let name = path.file_name().and_then(|name| name.to_str()).expect("table names are UTF-8");
		let contents = fs::read(&path).expect("read a liblouis table");
		for field in [name.as_bytes(), &contents] {
			packed
				.write_all(&u32::try_from(field.len()).expect("table under 4 GiB").to_le_bytes())
				.expect("pack tables");
			packed.write_all(field).expect("pack tables");
		}
	}
	let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set")).join("liblouis-tables.gz");
	fs::write(out, packed.finish().expect("pack tables")).expect("write packed tables");
}
