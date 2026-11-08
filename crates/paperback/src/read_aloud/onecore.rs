//! Windows' `OneCore` voices, the ones Windows lists under Speech in Settings, through `Windows.Media.SpeechSynthesis`.

use anyhow::{Context, Result};
use paperback_core::speech::{
	AudioChunk, SpeechEngine, SpeechRequest, Synthesis, Voice, WordBoundary, utf16_to_char_offset,
};
use windows::{
	Media::{Core::SpeechCue, SpeechSynthesis::SpeechSynthesizer},
	Storage::Streams::{Buffer, DataReader, InputStreamOptions},
	Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx},
	core::{HSTRING, Interface},
};

use super::wav;

/// Ticks of a `TimeSpan`, which counts in 100 nanoseconds.
const TICKS_PER_SECOND: i64 = 10_000_000;

pub struct OneCore;

impl SpeechEngine for OneCore {
	fn id(&self) -> &'static str {
		"onecore"
	}

	fn voices(&self) -> Result<Vec<Voice>> {
		join_apartment();
		let voices = SpeechSynthesizer::AllVoices().context("listing the OneCore voices")?;
		Ok(voices
			.into_iter()
			.filter_map(|voice| {
				Some(Voice {
					id: voice.Id().ok()?.to_string(),
					name: voice.DisplayName().ok()?.to_string(),
					language: voice.Language().ok()?.to_string(),
				})
			})
			.collect())
	}

	fn synthesize(&self, request: &SpeechRequest) -> Result<Box<dyn Synthesis>> {
		join_apartment();
		let synthesizer = SpeechSynthesizer::new().context("starting OneCore speech")?;
		if let Some(id) = &request.voice {
			let voice = SpeechSynthesizer::AllVoices()?
				.into_iter()
				.find(|voice| voice.Id().is_ok_and(|voice_id| voice_id == id.as_str()))
				.with_context(|| format!("the voice {id} isn't installed"))?;
			synthesizer.SetVoice(&voice)?;
		}
		let options = synthesizer.Options()?;
		options.SetIncludeWordBoundaryMetadata(true)?;
		options.SetSpeakingRate(speaking_rate(request.rate_percent))?;
		options.SetAudioPitch(f64::from(request.pitch_percent.min(100)) / 50.0)?;
		let stream =
			synthesizer.SynthesizeTextToStreamAsync(&HSTRING::from(&request.text))?.join().context("synthesizing")?;
		let size = u32::try_from(stream.Size()?).context("the speech is over 4 GB")?;
		let buffer = stream.ReadAsync(&Buffer::Create(size)?, size, InputStreamOptions::None)?.join()?;
		let mut bytes = vec![0; buffer.Length()? as usize];
		DataReader::FromBuffer(&buffer)?.ReadBytes(&mut bytes)?;
		let (sample_rate, samples) = wav::mono_samples(&bytes)?;
		let mut words = Vec::new();
		for track in stream.TimedMetadataTracks()? {
			for cue in track.Cues()? {
				let Ok(cue) = cue.cast::<SpeechCue>() else { continue };
				let (Ok(start), Ok(position)) = (cue.StartTime(), cue.StartPositionInInput().and_then(|p| p.Value()))
				else {
					continue;
				};
				let sample = start.Duration.max(0) * i64::from(sample_rate) / TICKS_PER_SECOND;
				let text_offset = utf16_to_char_offset(&request.text, usize::try_from(position).unwrap_or(0));
				let text_len = cue.Text().map_or(0, |text| text.to_string().chars().count());
				words.push(WordBoundary { sample: usize::try_from(sample).unwrap_or(0), text_offset, text_len });
			}
		}
		words.sort_by_key(|word| word.sample);
		Ok(Box::new(Synthesized { sample_rate, chunk: Some(AudioChunk { samples, words }) }))
	}
}

/// `OneCore` gives the whole paragraph at once, so it's handed over as one chunk.
struct Synthesized {
	sample_rate: u32,
	chunk: Option<AudioChunk>,
}

impl Synthesis for Synthesized {
	fn sample_rate(&self) -> u32 {
		self.sample_rate
	}

	fn next_chunk(&mut self) -> Option<Result<AudioChunk>> {
		self.chunk.take().map(Ok)
	}
}

/// As Windows' own speed slider has it: 50 is normal, the lower half slows to half speed, and the upper half speeds up to `OneCore`'s six times.
fn speaking_rate(percent: u8) -> f64 {
	let percent = f64::from(percent.min(100));
	if percent <= 50.0 { 0.5 + percent / 100.0 } else { 1.0 + (percent - 50.0) / 10.0 }
}

/// `OneCore` is reached through COM, which a thread must join before using it. Joining twice is harmless.
fn join_apartment() {
	let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_rate_follows_windows_speed_slider() {
		assert!((speaking_rate(0) - 0.5).abs() < f64::EPSILON);
		assert!((speaking_rate(50) - 1.0).abs() < f64::EPSILON);
		assert!((speaking_rate(100) - 6.0).abs() < f64::EPSILON);
	}

	/// Needs at least one `OneCore` voice, which a build server may not have.
	#[test]
	fn a_paragraph_is_spoken_with_its_words_in_place() {
		let engine = OneCore;
		if engine.voices().expect("listing voices").is_empty() {
			return;
		}
		let text = "Hello there, wide world.";
		let request = SpeechRequest { text: text.into(), voice: None, rate_percent: 50, pitch_percent: 50 };
		let mut synthesis = engine.synthesize(&request).unwrap();
		let chunk = synthesis.next_chunk().unwrap().unwrap();
		assert!(synthesis.sample_rate() >= 8000);
		assert!(
			chunk.samples.len() > usize::try_from(synthesis.sample_rate()).unwrap() / 2,
			"under half a second of speech"
		);
		let offsets: Vec<usize> = chunk.words.iter().map(|word| word.text_offset).collect();
		assert_eq!(offsets, [0, 6, 13, 18], "{:?}", chunk.words);
		assert!(chunk.words.windows(2).all(|pair| pair[0].sample < pair[1].sample));
	}
}
