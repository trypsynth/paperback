//! macOS's own voices, the ones System Settings lists under Spoken Content, through `AVSpeechSynthesizer`.
//!
//! The synthesizer hands over its audio, and says which word it has reached, on the main thread's run loop, so a paragraph is synthesized on another thread while the UI's event loop delivers it.

use std::{ptr::NonNull, sync::mpsc, time::Duration};

use anyhow::{Context, Result, bail};
use block2::RcBlock;
use objc2::{AllocAnyThread, DefinedClass, define_class, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_avf_audio::{
	AVAudioBuffer, AVAudioCommonFormat, AVAudioPCMBuffer, AVSpeechSynthesisVoice, AVSpeechSynthesizer,
	AVSpeechSynthesizerDelegate, AVSpeechUtterance,
};
use objc2_foundation::{NSObject, NSObjectProtocol, NSRange, NSString};
use paperback_core::speech::{
	AudioChunk, SpeechEngine, SpeechRequest, Synthesis, Voice, WholeSynthesis, WordBoundary, utf16_to_char_offset,
};

/// How long a paragraph may take to synthesize, since a synthesizer that fails says nothing.
const TIMEOUT: Duration = Duration::from_secs(60);

pub struct AvSpeech;

/// What the synthesizer writes, in the order it writes it, sent from the main thread to the one waiting for the paragraph.
enum Written {
	Audio {
		sample_rate: u32,
		samples: Vec<i16>,
	},
	/// The synthesizer has reached this stretch of the text, in UTF-16 units, which starts where the audio written so far ends.
	Word {
		utf16_start: usize,
		utf16_len: usize,
	},
	Done,
}

define_class!(
	/// The synthesizer's delegate, which is told of each word as its audio is written. Its markers would say where too, but macOS sends none.
	#[unsafe(super(NSObject))]
	#[name = "PaperbackSpeechWords"]
	#[ivars = mpsc::Sender<Written>]
	struct WordReporter;

	unsafe impl NSObjectProtocol for WordReporter {}

	unsafe impl AVSpeechSynthesizerDelegate for WordReporter {
		#[unsafe(method(speechSynthesizer:willSpeakRangeOfSpeechString:utterance:))]
		fn will_speak(&self, _synthesizer: &AVSpeechSynthesizer, range: NSRange, _utterance: &AVSpeechUtterance) {
			let _ = self.ivars().send(Written::Word { utf16_start: range.location, utf16_len: range.length });
		}
	}
);

impl WordReporter {
	fn new(sender: mpsc::Sender<Written>) -> Retained<Self> {
		let this = Self::alloc().set_ivars(sender);
		unsafe { msg_send![super(this), init] }
	}
}

impl SpeechEngine for AvSpeech {
	fn id(&self) -> &'static str {
		"avspeech"
	}

	fn voices(&self) -> Result<Vec<Voice>> {
		let voices = unsafe { AVSpeechSynthesisVoice::speechVoices() };
		Ok(voices
			.iter()
			.map(|voice| unsafe {
				Voice {
					id: voice.identifier().to_string(),
					name: voice.name().to_string(),
					language: voice.language().to_string(),
				}
			})
			.collect())
	}

	fn synthesize(&self, request: &SpeechRequest) -> Result<Box<dyn Synthesis>> {
		let (sender, receiver) = mpsc::channel();
		let words = WordReporter::new(sender.clone());
		let on_audio = RcBlock::new(move |buffer: NonNull<AVAudioBuffer>| {
			// The synthesizer lends each buffer for the duration of the callback.
			let buffer = unsafe { buffer.as_ref() };
			let written = buffer.downcast_ref::<AVAudioPCMBuffer>().map_or(Written::Done, |buffer| {
				let (sample_rate, samples) = unsafe { mono_samples(buffer) };
				if samples.is_empty() { Written::Done } else { Written::Audio { sample_rate, samples } }
			});
			let _ = sender.send(written);
		});
		// Kept until the paragraph is written, as the synthesizer only holds its delegate weakly.
		let synthesizer;
		unsafe {
			let utterance = AVSpeechUtterance::speechUtteranceWithString(&NSString::from_str(&request.text));
			if let Some(id) = &request.voice {
				let voice = AVSpeechSynthesisVoice::voiceWithIdentifier(&NSString::from_str(id))
					.with_context(|| format!("the voice {id} isn't installed"))?;
				utterance.setVoice(Some(&voice));
			}
			// The book's own voice and rate, not VoiceOver's.
			utterance.setPrefersAssistiveTechnologySettings(false);
			utterance.setRate(speech_rate(request.rate_percent));
			utterance.setPitchMultiplier(pitch_multiplier(request.pitch_percent));
			synthesizer = AVSpeechSynthesizer::new();
			synthesizer.setDelegate(Some(ProtocolObject::from_ref(&*words)));
			synthesizer.writeUtterance_toBufferCallback(&utterance, RcBlock::as_ptr(&on_audio));
		}
		let (mut sample_rate, mut samples, mut boundaries) = (0, Vec::new(), Vec::new());
		loop {
			match receiver.recv_timeout(TIMEOUT).context("the voice stopped speaking")? {
				Written::Audio { sample_rate: rate, samples: more } => {
					sample_rate = rate;
					samples.extend(more);
				}
				Written::Word { utf16_start, utf16_len } => {
					let text_offset = utf16_to_char_offset(&request.text, utf16_start);
					let text_len = utf16_to_char_offset(&request.text, utf16_start + utf16_len) - text_offset;
					// Punctuation is reported as a word of its own, which the caret has no reason to stop at.
					if request.text.chars().skip(text_offset).take(text_len).any(char::is_alphanumeric) {
						boundaries.push(WordBoundary { sample: samples.len(), text_offset, text_len });
					}
				}
				Written::Done => break,
			}
		}
		drop((synthesizer, words));
		if sample_rate == 0 {
			bail!("the voice wrote no speech");
		}
		Ok(Box::new(WholeSynthesis::new(sample_rate, AudioChunk { samples, words: boundaries })))
	}
}

/// The buffer's first channel as 16-bit samples, which is all of it, as speech is mono.
///
/// # Safety
///
/// `buffer` must be the synthesizer's, lent to its callback.
unsafe fn mono_samples(buffer: &AVAudioPCMBuffer) -> (u32, Vec<i16>) {
	unsafe {
		let format = buffer.format();
		#[allow(
			clippy::cast_possible_truncation,
			clippy::cast_sign_loss,
			reason = "speech is sampled well under 4 GHz"
		)]
		let sample_rate = format.sampleRate() as u32;
		let frames = buffer.frameLength() as usize;
		if frames == 0 {
			return (sample_rate, Vec::new());
		}
		let samples = match format.commonFormat() {
			AVAudioCommonFormat::PCMFormatFloat32 => {
				let channels = buffer.floatChannelData();
				if channels.is_null() {
					Vec::new()
				} else {
					#[allow(clippy::cast_possible_truncation, reason = "clamped to i16's range first")]
					std::slice::from_raw_parts((*channels).as_ptr(), frames)
						.iter()
						.map(|&sample| (sample * 32_768.0).round().clamp(-32_768.0, 32_767.0) as i16)
						.collect()
				}
			}
			AVAudioCommonFormat::PCMFormatInt16 => {
				let channels = buffer.int16ChannelData();
				if channels.is_null() {
					Vec::new()
				} else {
					std::slice::from_raw_parts((*channels).as_ptr(), frames).to_vec()
				}
			}
			_ => Vec::new(),
		};
		(sample_rate, samples)
	}
}

/// From the slowest to the fastest the voices speak, with 50 their normal rate.
fn speech_rate(percent: u8) -> f32 {
	f32::from(percent.min(100)) / 100.0
}

/// From half to twice the voice's pitch, with 50 its own.
fn pitch_multiplier(percent: u8) -> f32 {
	((f32::from(percent.min(100)) - 50.0) / 50.0).exp2()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_rate_and_pitch_have_their_normal_at_fifty() {
		assert!((speech_rate(50) - 0.5).abs() < f32::EPSILON);
		assert!((pitch_multiplier(50) - 1.0).abs() < f32::EPSILON);
		assert!((pitch_multiplier(0) - 0.5).abs() < f32::EPSILON);
		assert!((pitch_multiplier(100) - 2.0).abs() < f32::EPSILON);
	}
}
