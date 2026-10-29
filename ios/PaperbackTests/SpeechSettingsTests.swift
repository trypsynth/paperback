import AVFoundation
import XCTest
@testable import Paperback

@MainActor
final class SpeechSettingsTests: XCTestCase {
	private var configPath = ""

	override func setUp() {
		super.setUp()
		configPath = FileManager.default.temporaryDirectory.appending(path: "\(UUID().uuidString).ini").path(percentEncoded: false)
	}

	private func settings() -> (SpeechSettings, TtsManager, ConfigManagerFfi) {
		let config = ConfigManagerFfi()
		_ = config.initialize(configPath: configPath)
		let tts = TtsManager()
		let speech = SpeechSettings()
		speech.start(config: config, tts: tts)
		return (speech, tts, config)
	}

	func testABookWithoutItsOwnFollowsSettings() {
		let (speech, tts, _) = settings()
		speech.load(bookAt: "/books/novel.epub")
		speech.setDefaultRate(speechRateForPercent(30))
		XCTAssertEqual(tts.speechRatePercent, 30)
	}

	func testABooksOwnRateWinsAndOnlyItsOwnBookUsesIt() {
		let (speech, tts, _) = settings()
		speech.setDefaultRate(speechRateForPercent(50))
		speech.load(bookAt: "/books/textbook.epub")
		speech.setBookRatePercent(20)
		XCTAssertEqual(tts.speechRatePercent, 20)
		speech.setDefaultRate(speechRateForPercent(60))
		XCTAssertEqual(tts.speechRatePercent, 20)
		speech.load(bookAt: "/books/novel.epub")
		XCTAssertEqual(tts.speechRatePercent, 60)
		speech.load(bookAt: "/books/textbook.epub")
		XCTAssertEqual(tts.speechRatePercent, 20)
	}

	func testTheReadingBarChangesWhateverIsInEffect() {
		let (speech, tts, _) = settings()
		speech.setDefaultRate(speechRateForPercent(50))
		speech.load(bookAt: "/books/textbook.epub")
		speech.setBookRatePercent(20)
		tts.speechRatePercent = 25
		XCTAssertEqual(speech.book.ratePercent, 25)
		XCTAssertEqual(percentForSpeechRate(speech.defaultRate), 50)
		speech.load(bookAt: "/books/novel.epub")
		tts.speechRatePercent = 70
		XCTAssertEqual(percentForSpeechRate(speech.defaultRate), 70)
	}

	func testSwitchingASettingOffFollowsSettingsAgain() {
		let (speech, tts, _) = settings()
		speech.setDefaultPitch(pitchForPercent(50))
		speech.load(bookAt: "/books/textbook.epub")
		speech.setBookPitchPercent(80)
		XCTAssertEqual(tts.pitchPercent, 80)
		speech.setBookPitchPercent(nil)
		XCTAssertEqual(tts.pitchPercent, 50)
	}

	func testABooksOwnSettingsSurviveARestart() {
		let (first, _, config) = settings()
		first.load(bookAt: "/books/textbook.epub")
		first.setBookRatePercent(15)
		first.setBookVoice("")
		config.flush()
		let (speech, tts, _) = settings()
		speech.load(bookAt: "/books/textbook.epub")
		XCTAssertEqual(speech.book.ratePercent, 15)
		XCTAssertEqual(speech.book.voice, "")
		XCTAssertEqual(tts.speechRatePercent, 15)
	}

	func testAVoiceThisDeviceDoesNotHaveFallsBackToSettings() {
		let (speech, tts, _) = settings()
		let installed = AVSpeechSynthesisVoice.speechVoices().first?.identifier
		speech.setDefaultVoice(installed)
		speech.load(bookAt: "/books/textbook.epub")
		speech.setBookVoice("com.example.voice.chosen-on-another-device")
		XCTAssertEqual(tts.selectedVoiceIdentifier, installed)
		speech.setBookVoice("")
		XCTAssertNil(tts.selectedVoiceIdentifier)
	}
}
