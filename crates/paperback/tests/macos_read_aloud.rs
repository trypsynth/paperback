//! macOS read aloud: a paragraph spoken by the system's default voice, with its words where they're spoken. The synthesizer delivers its audio on the main thread's run loop, as the app's event loop turns it, so this runs on the main thread and turns its run loop while another thread waits for the paragraph.

#[cfg(target_os = "macos")]
#[allow(unused_imports)]
#[path = "../src/read_aloud/avspeech.rs"]
mod avspeech;

#[cfg(target_os = "macos")]
fn main() {
	use std::{
		sync::mpsc,
		thread,
		time::{Duration, Instant},
	};

	use objc2_foundation::{NSDate, NSDefaultRunLoopMode, NSRunLoop};
	use paperback_core::speech::{SpeechEngine, SpeechRequest};

	let engine = avspeech::AvSpeech;
	if engine.voices().expect("listing voices").is_empty() {
		println!("Skipped macOS read aloud checks: no voices");
		return;
	}
	let text = "Hello there, wide world.";
	let (sender, receiver) = mpsc::channel();
	thread::spawn(move || {
		let request = SpeechRequest { text: text.into(), voice: None, rate_percent: 50, pitch_percent: 50 };
		let spoken = engine.synthesize(&request).and_then(|mut synthesis| {
			let sample_rate = synthesis.sample_rate();
			synthesis.next_chunk().expect("a paragraph's audio").map(|chunk| (sample_rate, chunk))
		});
		sender.send(spoken.map_err(|error| error.to_string())).unwrap();
	});
	let start = Instant::now();
	let (sample_rate, chunk) = loop {
		if let Ok(spoken) = receiver.try_recv() {
			break spoken.expect("the paragraph is spoken");
		}
		assert!(start.elapsed() < Duration::from_secs(30), "the paragraph never came");
		unsafe {
			NSRunLoop::currentRunLoop()
				.runMode_beforeDate(NSDefaultRunLoopMode, &NSDate::dateWithTimeIntervalSinceNow(0.02));
		}
	};
	assert!(sample_rate >= 8000, "{sample_rate} Hz");
	assert!(chunk.samples.len() > usize::try_from(sample_rate).unwrap() / 2, "under half a second of speech");
	let offsets: Vec<usize> = chunk.words.iter().map(|word| word.text_offset).collect();
	assert_eq!(offsets, [0, 6, 13, 18], "{:?}", chunk.words);
	assert!(chunk.words.windows(2).all(|pair| pair[0].sample < pair[1].sample), "{:?}", chunk.words);
	assert!(chunk.words.iter().all(|word| word.sample < chunk.samples.len()), "{:?}", chunk.words);
	println!("Passed macOS read aloud checks");
}

#[cfg(not(target_os = "macos"))]
fn main() {}
