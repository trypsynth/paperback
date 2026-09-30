use std::io::{Cursor, Write};

use zip::{ZipWriter, write::FileOptions};

mod daisy2;
mod daisy3;
mod plain_audio;

fn write_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
	let mut buf = Vec::new();
	{
		let cursor = Cursor::new(&mut buf);
		let mut writer = ZipWriter::new(cursor);
		for (name, data) in entries {
			writer.start_file(*name, FileOptions::<()>::default()).unwrap();
			writer.write_all(data).unwrap();
		}
		writer.finish().unwrap();
	}
	buf
}

/// A minimal valid mono 8-bit PCM WAV file with `num_samples` samples at `sample_rate`, for
/// exercising real audio-duration probing without shipping a binary fixture. Its duration is
/// exactly `num_samples / sample_rate` seconds; the sample bytes themselves are silence.
fn make_wav(sample_rate: u32, num_samples: u32) -> Vec<u8> {
	let block_align: u16 = 1; // mono, 8 bits per sample
	let byte_rate = sample_rate * u32::from(block_align);
	let data_size = num_samples * u32::from(block_align);
	let mut wav = Vec::new();
	wav.extend_from_slice(b"RIFF");
	wav.extend_from_slice(&(36 + data_size).to_le_bytes());
	wav.extend_from_slice(b"WAVE");
	wav.extend_from_slice(b"fmt ");
	wav.extend_from_slice(&16u32.to_le_bytes());
	wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
	wav.extend_from_slice(&1u16.to_le_bytes()); // mono
	wav.extend_from_slice(&sample_rate.to_le_bytes());
	wav.extend_from_slice(&byte_rate.to_le_bytes());
	wav.extend_from_slice(&block_align.to_le_bytes());
	wav.extend_from_slice(&8u16.to_le_bytes()); // bits per sample
	wav.extend_from_slice(b"data");
	wav.extend_from_slice(&data_size.to_le_bytes());
	wav.extend(std::iter::repeat_n(128u8, data_size as usize));
	wav
}
