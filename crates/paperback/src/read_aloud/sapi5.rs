//! SAPI 5 voices, such as those of Eloquence, Vocalizer or Mikropuhe, through COM.
//!
//! Paperback is a 64-bit app, so it finds only voices installed for 64-bit programs.

use anyhow::{Context, Result};
use paperback_core::speech::{AudioChunk, SpeechEngine, SpeechRequest, Synthesis, Voice, WordBoundary};
use windows::{
	Win32::{
		Foundation::HGLOBAL,
		Globalization::LCIDToLocaleName,
		Media::{
			Audio::{WAVE_FORMAT_PCM, WAVEFORMATEX},
			Speech::{
				ISpObjectToken, ISpObjectTokenCategory, ISpStream, ISpVoice, SPCAT_VOICES, SPEI_RESERVED1,
				SPEI_RESERVED2, SPEI_WORD_BOUNDARY, SPEVENT, SPF_IS_NOT_XML, SPF_IS_XML, SpObjectToken,
				SpObjectTokenCategory, SpStream, SpVoice,
			},
		},
		System::Com::{
			CLSCTX_ALL, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree, STREAM_SEEK_SET,
			StructuredStorage::CreateStreamOnHGlobal,
		},
	},
	core::{GUID, HSTRING, PCWSTR, PWSTR, w},
};

use super::WholeSynthesis;

/// SAPI's identifier for audio described by a `WAVEFORMATEX`, which the windows crate doesn't define.
const SPDFID_WAVE_FORMAT_EX: GUID = GUID::from_u128(0xc31a_dbae_527f_4ff5_a230_f62b_b61f_f70c);
/// The rate SAPI is asked for. It converts each voice's own rate to this, and the result is resampled to the device's rate after.
const SAMPLE_RATE: u32 = 22_050;

pub struct Sapi5;

impl SpeechEngine for Sapi5 {
	fn id(&self) -> &'static str {
		"sapi5"
	}

	fn voices(&self) -> Result<Vec<Voice>> {
		join_apartment();
		unsafe {
			let category: ISpObjectTokenCategory = CoCreateInstance(&SpObjectTokenCategory, None, CLSCTX_ALL)?;
			category.SetId(SPCAT_VOICES, false)?;
			let tokens = category.EnumTokens(PCWSTR::null(), PCWSTR::null())?;
			let mut count = 0;
			tokens.GetCount(&raw mut count)?;
			let mut voices = Vec::new();
			for index in 0..count {
				let Ok(token) = tokens.Item(index) else { continue };
				let (Ok(id), Ok(name)) =
					(take_string(token.GetId()), take_string(token.GetStringValue(PCWSTR::null())))
				else {
					continue;
				};
				let language = token
					.OpenKey(w!("Attributes"))
					.and_then(|attributes| take_string(attributes.GetStringValue(w!("Language"))))
					.map(|lcids| language_tag(&lcids))
					.unwrap_or_default();
				voices.push(Voice { id, name, language });
			}
			Ok(voices)
		}
	}

	fn synthesize(&self, request: &SpeechRequest) -> Result<Box<dyn Synthesis>> {
		join_apartment();
		let markup = Markup::new(&request.text, request.pitch_percent);
		let spoken = unsafe { speak(request, &markup) }.context("speaking with SAPI 5")?;
		let samples = spoken.audio.as_chunks::<2>().0.iter().map(|&pair| i16::from_le_bytes(pair)).collect();
		let words = spoken
			.words
			.into_iter()
			.map(|word| WordBoundary {
				sample: usize::try_from(word.byte_offset / 2).unwrap_or(usize::MAX),
				text_offset: markup.to_text_offset(word.position),
				text_len: word.len,
			})
			.collect();
		Ok(Box::new(WholeSynthesis::new(SAMPLE_RATE, AudioChunk { samples, words })))
	}
}

/// What SAPI spoke into memory.
struct Spoken {
	/// 16-bit mono at [`SAMPLE_RATE`].
	audio: Vec<u8>,
	words: Vec<SpokenWord>,
}

struct SpokenWord {
	byte_offset: u64,
	/// Where the word starts in the markup, in UTF-16 units.
	position: usize,
	len: usize,
}

unsafe fn speak(request: &SpeechRequest, markup: &Markup) -> Result<Spoken> {
	unsafe {
		let voice: ISpVoice = CoCreateInstance(&SpVoice, None, CLSCTX_ALL)?;
		if let Some(id) = &request.voice {
			let token: ISpObjectToken = CoCreateInstance(&SpObjectToken, None, CLSCTX_ALL)?;
			token
				.SetId(PCWSTR::null(), &HSTRING::from(id), false)
				.with_context(|| format!("the voice {id} isn't installed"))?;
			voice.SetVoice(&token)?;
		}
		let format = WAVEFORMATEX {
			wFormatTag: u16::try_from(WAVE_FORMAT_PCM).unwrap_or(1),
			nChannels: 1,
			nSamplesPerSec: SAMPLE_RATE,
			nAvgBytesPerSec: SAMPLE_RATE * 2,
			nBlockAlign: 2,
			wBitsPerSample: 16,
			cbSize: 0,
		};
		let memory = CreateStreamOnHGlobal(HGLOBAL::default(), true)?;
		let stream: ISpStream = CoCreateInstance(&SpStream, None, CLSCTX_ALL)?;
		stream.SetBaseStream(&memory, &SPDFID_WAVE_FORMAT_EX, &raw const format)?;
		voice.SetOutput(&stream, true)?;
		voice.SetRate(sapi_scale(request.rate_percent))?;
		// SAPI insists on its two reserved bits alongside the events asked for.
		let interest = (1u64 << SPEI_WORD_BOUNDARY.0) | (1u64 << SPEI_RESERVED1.0) | (1u64 << SPEI_RESERVED2.0);
		voice.SetInterest(interest, interest)?;
		let flags = if markup.is_xml { SPF_IS_XML.0 } else { SPF_IS_NOT_XML.0 };
		voice.Speak(&HSTRING::from(&markup.text), u32::try_from(flags).unwrap_or(0), None)?;
		let mut words = Vec::new();
		loop {
			let mut event = SPEVENT::default();
			let mut fetched = 0;
			if voice.GetEvents(1, &raw mut event, &raw mut fetched).is_err() || fetched == 0 {
				break;
			}
			// The low word is the event, the high word what its lParam holds; a word boundary's is a plain number.
			if event._bitfield & 0xFFFF == SPEI_WORD_BOUNDARY.0 {
				let position = usize::try_from(event.lParam.0).unwrap_or(0);
				words.push(SpokenWord { byte_offset: event.ullAudioStreamOffset, position, len: event.wParam.0 });
			}
		}
		let mut bytes = Vec::new();
		memory.Seek(0, STREAM_SEEK_SET, None)?;
		let mut buffer = vec![0u8; 64 * 1024];
		loop {
			let mut read = 0;
			let result = memory.Read(
				buffer.as_mut_ptr().cast(),
				u32::try_from(buffer.len()).unwrap_or(u32::MAX),
				Some(&raw mut read),
			);
			if result.is_err() || read == 0 {
				break;
			}
			bytes.extend_from_slice(&buffer[..read as usize]);
		}
		Ok(Spoken { audio: bytes, words })
	}
}

/// A percentage of the range, 50 being normal, on SAPI's scale from -10 to 10.
fn sapi_scale(percent: u8) -> i32 {
	(i32::from(percent.min(100)) - 50) / 5
}

/// The text as SAPI is given it. Pitch can only be set with SAPI's markup, so text spoken at another pitch is escaped and wrapped in a tag, and word positions, which SAPI counts in the markup, are mapped back to the text.
struct Markup {
	text: String,
	is_xml: bool,
	/// Where each char of the original text starts in `text`, in UTF-16 units as SAPI counts.
	starts: Vec<usize>,
}

impl Markup {
	fn new(text: &str, pitch_percent: u8) -> Self {
		let pitch = sapi_scale(pitch_percent);
		if pitch == 0 {
			let mut starts = Vec::with_capacity(text.len());
			let mut at = 0;
			for c in text.chars() {
				starts.push(at);
				at += c.len_utf16();
			}
			return Self { text: text.to_owned(), is_xml: false, starts };
		}
		let mut markup = format!("<pitch absmiddle=\"{pitch}\">");
		let mut at = markup.encode_utf16().count();
		let mut starts = Vec::with_capacity(text.len());
		for c in text.chars() {
			starts.push(at);
			let piece = match c {
				'&' => "&amp;",
				'<' => "&lt;",
				'>' => "&gt;",
				_ => {
					markup.push(c);
					at += c.len_utf16();
					continue;
				}
			};
			markup.push_str(piece);
			at += piece.len();
		}
		markup.push_str("</pitch>");
		Self { text: markup, is_xml: true, starts }
	}

	/// The char of the original text that a position in the markup falls in.
	fn to_text_offset(&self, position: usize) -> usize {
		self.starts.partition_point(|&start| start <= position).saturating_sub(1)
	}
}

/// SAPI lists a voice's languages as hexadecimal locale numbers, such as `409;9`, the first being its own.
fn language_tag(lcids: &str) -> String {
	let Some(lcid) = lcids.split(';').next().and_then(|first| u32::from_str_radix(first.trim(), 16).ok()) else {
		return String::new();
	};
	let mut name = [0u16; 85];
	let len = unsafe { LCIDToLocaleName(lcid, Some(&mut name), 0) };
	usize::try_from(len)
		.ok()
		.filter(|&len| len > 1)
		.map(|len| String::from_utf16_lossy(&name[..len - 1]))
		.unwrap_or_default()
}

/// Takes a string SAPI allocated, and frees it.
fn take_string(result: windows::core::Result<PWSTR>) -> windows::core::Result<String> {
	let pointer = result?;
	let text = unsafe { pointer.to_string() }.unwrap_or_default();
	unsafe { CoTaskMemFree(Some(pointer.0.cast())) };
	Ok(text)
}

/// SAPI is reached through COM, which a thread must join before using it. Joining twice is harmless.
fn join_apartment() {
	let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rate_and_pitch_go_on_sapis_scale() {
		assert_eq!(sapi_scale(0), -10);
		assert_eq!(sapi_scale(50), 0);
		assert_eq!(sapi_scale(100), 10);
	}

	#[test]
	fn plain_text_goes_as_it_is() {
		let markup = Markup::new("Tom & Jerry", 50);
		assert!(!markup.is_xml);
		assert_eq!(markup.text, "Tom & Jerry");
		assert_eq!(markup.to_text_offset(6), 6);
	}

	#[test]
	fn another_pitch_wraps_escaped_text_and_word_positions_map_back() {
		let markup = Markup::new("Tom & Jerry <3", 100);
		assert!(markup.is_xml);
		assert_eq!(markup.text, "<pitch absmiddle=\"10\">Tom &amp; Jerry &lt;3</pitch>");
		let in_markup = |word: &str| markup.text.find(word).unwrap();
		assert_eq!(markup.to_text_offset(in_markup("Tom")), 0);
		assert_eq!(markup.to_text_offset(in_markup("Jerry")), "Tom & ".len());
		assert_eq!(markup.to_text_offset(in_markup("&lt;")), "Tom & Jerry ".len());
	}

	#[test]
	fn languages_become_tags() {
		assert_eq!(language_tag("409;9"), "en-US");
		assert_eq!(language_tag("80A"), "es-MX");
		assert_eq!(language_tag(""), "");
	}

	/// Needs at least one 64-bit SAPI voice, which a build server may not have.
	#[test]
	fn a_paragraph_is_spoken_with_its_words_in_place() {
		let engine = Sapi5;
		if engine.voices().expect("listing voices").is_empty() {
			return;
		}
		let text = "Hello there, wide world.";
		for pitch in [50, 80] {
			let request = SpeechRequest { text: text.into(), voice: None, rate_percent: 50, pitch_percent: pitch };
			let mut synthesis = engine.synthesize(&request).unwrap();
			let chunk = synthesis.next_chunk().unwrap().unwrap();
			assert!(
				chunk.samples.len() > usize::try_from(SAMPLE_RATE).unwrap() / 2,
				"under half a second at pitch {pitch}"
			);
			let offsets: Vec<usize> = chunk.words.iter().map(|word| word.text_offset).collect();
			assert_eq!(offsets, [0, 6, 13, 18], "pitch {pitch}: {:?}", chunk.words);
		}
	}
}
