use std::io::Read;

use anyhow::{Context, Result, bail};
use flate2::read::ZlibDecoder;

/// A length the file claims is only trusted this far for the first allocation; the stream itself decides how much there is.
const MAX_PREALLOCATION: usize = 16 * 1024 * 1024;

/// Undoes `qCompress`: a big-endian `u32` of the uncompressed length, then a zlib stream.
///
/// An empty blob is the page `qhelpgenerator` writes for links to files the project lacks, and four zero bytes is how `qCompress` stores an empty file; both are empty, as `qUncompress` has them.
pub(super) fn q_uncompress(data: &[u8]) -> Result<Vec<u8>> {
	if data.is_empty() {
		return Ok(Vec::new());
	}
	let Some((header, stream)) = data.split_first_chunk::<4>() else {
		bail!("compressed data is {} bytes, too short for its length header", data.len());
	};
	let expected = u32::from_be_bytes(*header) as usize;
	if stream.is_empty() {
		if expected == 0 {
			return Ok(Vec::new());
		}
		bail!("compressed data claims {expected} bytes but holds no stream");
	}
	let mut output = Vec::with_capacity(expected.min(MAX_PREALLOCATION));
	ZlibDecoder::new(stream).read_to_end(&mut output).context("compressed data is corrupt")?;
	if output.len() != expected {
		tracing::debug!(expected, actual = output.len(), "qch file data inflated to a length other than its header's");
	}
	Ok(output)
}

#[cfg(test)]
pub(super) fn q_compress(data: &[u8]) -> Vec<u8> {
	use std::io::Write;
	if data.is_empty() {
		return vec![0; 4];
	}
	let mut output = u32::try_from(data.len()).unwrap().to_be_bytes().to_vec();
	let mut encoder = flate2::write::ZlibEncoder::new(&mut output, flate2::Compression::default());
	encoder.write_all(data).unwrap();
	encoder.finish().unwrap();
	output
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case(b"")]
	#[case(b"<html><title>A</title></html>")]
	fn round_trips_what_qcompress_writes(#[case] data: &[u8]) {
		assert_eq!(q_uncompress(&q_compress(data)).unwrap(), data);
	}

	#[test]
	fn the_missing_file_placeholder_is_empty() {
		assert!(q_uncompress(&[]).unwrap().is_empty());
	}

	#[rstest]
	#[case(&[0, 0, 1])]
	#[case(&[0, 0, 0, 9])]
	#[case(&[0, 0, 0, 9, 1, 2, 3])]
	fn refuses_corrupt_data(#[case] data: &[u8]) {
		assert!(q_uncompress(data).is_err());
	}

	#[test]
	fn a_wrong_length_header_does_not_lose_the_file() {
		let mut data = q_compress(b"hello world");
		data[..4].copy_from_slice(&3u32.to_be_bytes());
		assert_eq!(q_uncompress(&data).unwrap(), b"hello world");
	}
}
