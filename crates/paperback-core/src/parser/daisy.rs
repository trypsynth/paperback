use std::{
	fs::File,
	io::{BufReader, Read},
	path::Path,
};

use anyhow::Result;

use crate::{
	document::{Document, ParserContext},
	parser::Parser,
};

mod daisy2;
mod loose;
mod ncx;
mod opf;
mod plain_audio;
mod smil;
mod timeline;
mod zip;

pub struct DaisyParser;

impl Parser for DaisyParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let path = Path::new(&context.file_path);
		let ext_is_zip = path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("zip"));
		let is_zip = ext_is_zip || {
			let magic_result = File::open(path).and_then(|f| {
				let mut header = [0; 4];
				let mut reader = BufReader::new(f);
				reader.read_exact(&mut header)?;
				Ok(header == [0x50, 0x4b, 0x03, 0x04])
			});
			if let Err(ref e) = magic_result {
				tracing::warn!(path = %path.display(), error = %e, "failed to read file header while checking for zip magic bytes");
			}
			magic_result.unwrap_or(false)
		};
		if ext_is_zip {
			tracing::debug!(path = %path.display(), "detected zip via file extension");
		} else if is_zip {
			tracing::debug!(path = %path.display(), "detected zip via magic bytes");
		}
		tracing::debug!(path = %path.display(), is_zip, "starting daisy parse");
		if is_zip {
			tracing::debug!("taking zip archive parse path");
			zip::parse(context, path)
		} else {
			tracing::debug!(path = %path.display(), "taking loose files parse path");
			loose::parse(context, path)
		}
	}
}

#[cfg(test)]
mod tests;
