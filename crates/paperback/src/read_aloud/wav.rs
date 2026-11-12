//! The WAV files engines synthesize into, as mono 16-bit samples.

use anyhow::{Context, Result, bail};

/// A WAV file's sample rate and samples, mixed down to mono, since the narrator plays one channel.
pub fn mono_samples(bytes: &[u8]) -> Result<(u32, Vec<i16>)> {
	if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
		bail!("not a WAV file");
	}
	let mut format = None;
	let mut rest = &bytes[12..];
	while rest.len() >= 8 {
		let (id, size) = (&rest[..4], u32::from_le_bytes(rest[4..8].try_into().expect("four bytes")) as usize);
		let body = rest.get(8..8 + size).unwrap_or_else(|| &rest[8..]);
		match id {
			b"fmt " => {
				let field = |at: usize| body.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
				let tag = field(0).context("short format chunk")?;
				let channels = field(2).context("short format chunk")?;
				let rate = body
					.get(4..8)
					.map(|b| u32::from_le_bytes(b.try_into().expect("four bytes")))
					.context("short format chunk")?;
				let bits = field(14).context("short format chunk")?;
				// 0xFFFE is the extensible format, which engines use for plain PCM too.
				if !matches!(tag, 1 | 0xFFFE) || bits != 16 || channels == 0 {
					bail!("unsupported WAV format: tag {tag}, {bits} bits, {channels} channels");
				}
				format = Some((rate, usize::from(channels)));
			}
			b"data" => {
				let (rate, channels) = format.context("the data comes before its format")?;
				let samples = body
					.chunks_exact(2 * channels)
					.map(|frame| {
						let sum: i32 = frame.as_chunks::<2>().0.iter().map(|&s| i32::from(i16::from_le_bytes(s))).sum();
						i16::try_from(sum / i32::try_from(channels).unwrap_or(1)).unwrap_or_default()
					})
					.collect();
				return Ok((rate, samples));
			}
			_ => {}
		}
		// Chunks are padded to an even length.
		let next = 8 + size + size % 2;
		if next > rest.len() {
			break;
		}
		rest = &rest[next..];
	}
	bail!("the WAV file has no audio")
}

#[cfg(test)]
mod tests {
	use super::*;

	fn wav(rate: u32, channels: u16, samples: &[i16]) -> Vec<u8> {
		let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
		let mut bytes = b"RIFF\0\0\0\0WAVE".to_vec();
		bytes.extend_from_slice(b"fmt \x10\0\0\0");
		bytes.extend_from_slice(&1u16.to_le_bytes());
		bytes.extend_from_slice(&channels.to_le_bytes());
		bytes.extend_from_slice(&rate.to_le_bytes());
		bytes.extend_from_slice(&(rate * u32::from(channels) * 2).to_le_bytes());
		bytes.extend_from_slice(&(channels * 2).to_le_bytes());
		bytes.extend_from_slice(&16u16.to_le_bytes());
		// An odd-sized chunk engines may add, which must be skipped with its padding byte.
		bytes.extend_from_slice(b"LIST\x03\0\0\0abc\0");
		bytes.extend_from_slice(b"data");
		bytes.extend_from_slice(&u32::try_from(data.len()).unwrap().to_le_bytes());
		bytes.extend_from_slice(&data);
		bytes
	}

	#[test]
	fn mono_samples_come_out_as_they_went_in() {
		assert_eq!(mono_samples(&wav(24_000, 1, &[1, -2, 300])).unwrap(), (24_000, vec![1, -2, 300]));
	}

	#[test]
	fn stereo_is_mixed_down_to_mono() {
		assert_eq!(mono_samples(&wav(44_100, 2, &[100, 300, -10, -30])).unwrap(), (44_100, vec![200, -20]));
	}

	#[test]
	fn anything_else_is_refused() {
		assert!(mono_samples(b"not a wav").is_err());
		let mut eight_bit = wav(8000, 1, &[0]);
		eight_bit[34] = 8;
		assert!(mono_samples(&eight_bit).is_err());
	}
}
