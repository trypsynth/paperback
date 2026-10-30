import Foundation
import Observation

/// Up to 4x because audiobook listeners commonly go past 3x; desktop and Android keep to the same range.
let audioSpeedRange: ClosedRange<Float> = 0.5...4.0
let audioSpeedStep: Float = 0.1
let audioSpeedPresets: [Float] = [1, 1.25, 1.5, 2, 2.5, 3, 3.5, 4]

/// "2.5x", as desktop says it.
func audioSpeedLabel(_ speed: Float) -> String {
	var label = String(format: "%.2f", speed)
	while label.hasSuffix("0") { label.removeLast() }
	if label.hasSuffix(".") { label.removeLast() }
	return label + "x"
}

/// One player shared by every tab, since only one book narrates at a time and a decoder per tab would hold a file and memory per book.
@MainActor
@Observable
final class RecordedNarration {
	private let player: DaisyAudioPlayer
	private let config: ConfigManagerFfi

	private var attachedDocumentPath: String?

	var onPlayingChanged: ((Bool) -> Void)?
	var onSeekLanded: ((Int64) -> Void)?
	var onClipChanged: ((Int64) -> Void)?
	var onDetached: (() -> Void)?

	init(config: ConfigManagerFfi, player: DaisyAudioPlayer? = nil) {
		self.config = config
		let player = player ?? DaisyAudioPlayer()
		self.player = player
		// Saved on every pause and clip change rather than at a later checkpoint, so the book never resumes from the wrong place.
		player.onPlaybackStateChanged = { [weak self] playing in
			self?.isPlaying = playing
			self?.onPlayingChanged?(playing)
			if !playing { self?.persistPosition() }
		}
		player.onRelativeSeekLanded = { [weak self] elapsedMs in
			self?.onSeekLanded?(elapsedMs)
		}
		player.onClipChanged = { [weak self] position in
			self?.onClipChanged?(position)
			self?.persistPosition()
		}
		player.speed = Float(config.getAppString(key: Self.speedKey, defaultValue: "")).map(Self.clamped) ?? 1
		speed = player.speed
	}

	/// One speed for every recording, under the key desktop and Android use too.
	private static let speedKey = "audio_speed"

	private static func clamped(_ speed: Float) -> Float {
		(min(max(speed, audioSpeedRange.lowerBound), audioSpeedRange.upperBound) * 100).rounded() / 100
	}

	private(set) var speed: Float = 1

	func setSpeed(_ newSpeed: Float) {
		let clamped = Self.clamped(newSpeed)
		guard clamped != speed else { return }
		speed = clamped
		player.speed = clamped
		config.setAppString(key: Self.speedKey, value: "\(clamped)")
	}

	private(set) var isPlaying = false

	func attach(to tab: DocumentTab) {
		let path = tab.url.path(percentEncoded: false)
		if attachedDocumentPath == path { return }
		detach()
		guard let session = tab.session, session.hasAudioFfi() else { return }
		player.attach(session: session, docKey: config.getDocKey(path: path))
		attachedDocumentPath = path
		let savedAudioMs = config.getDocumentAudioTimeFfi(path: path)
		if savedAudioMs >= 0 {
			player.seekToMs(savedAudioMs)
		} else {
			player.seekToPosition(tab.currentPosition)
		}
	}

	func detach() {
		persistPosition()
		player.detach()
		attachedDocumentPath = nil
		onDetached?()
	}

	func isNarrating(_ path: String) -> Bool { attachedDocumentPath == path }

	func persistPosition() {
		guard let path = attachedDocumentPath, let ms = player.resumePointMs() else { return }
		config.setDocumentAudioTimeFfi(path: path, timeMs: ms)
		config.flush()
	}

	func play() { player.play() }

	func pause() { player.pause() }

	@discardableResult
	func seekToPosition(_ position: Int64) -> Bool {
		let moved = player.seekToPosition(position)
		if moved { persistPosition() }
		return moved
	}

	@discardableResult
	func seekToMs(_ elapsedMs: Int64) -> Bool { player.seekToMs(elapsedMs) }

	var elapsedMs: Int64? { player.elapsedMs }

	@discardableResult
	func seekRelativeMs(_ deltaMs: Int64) -> Bool { player.seekRelativeMs(deltaMs) }

	func shutdown() { player.detach() }
}
