import AVFoundation

/// One recording, played through AVAudioEngine because AVAudioPlayer stops at 2x and the engine's time-pitch unit goes well past it without raising the voice.
@MainActor
final class SourcePlayer {
	private let engine = AVAudioEngine()
	private let node = AVAudioPlayerNode()
	private let timePitch = AVAudioUnitTimePitch()
	private let file: AVAudioFile
	private let sampleRate: Double
	/// The frame the scheduled stretch starts at, which the node counts on from.
	private var startFrame: AVAudioFramePosition = 0
	/// Where playback stood when it last stopped moving, since a node that isn't playing reports no time.
	private var heldFrame: AVAudioFramePosition = 0
	/// Bumped on every reschedule, so the end of a stretch that was cut short isn't taken for the end of the file.
	private var generation = 0
	/// The node's last render when it was last rescheduled: until it renders again it still reports the old stretch's time.
	private var staleRender: AVAudioFramePosition?
	var onFinished: (() -> Void)?

	init(url: URL) throws {
		file = try AVAudioFile(forReading: url)
		sampleRate = file.processingFormat.sampleRate
		engine.attach(node)
		engine.attach(timePitch)
		engine.connect(node, to: timePitch, format: file.processingFormat)
		engine.connect(timePitch, to: engine.mainMixerNode, format: file.processingFormat)
		schedule(from: 0)
	}

	var duration: TimeInterval { Double(file.length) / sampleRate }

	var rate: Float {
		get { timePitch.rate }
		set { timePitch.rate = newValue }
	}

	var currentTime: TimeInterval {
		get { Double(currentFrame) / sampleRate }
		set {
			let wasPlaying = node.isPlaying
			schedule(from: min(max(AVAudioFramePosition(newValue * sampleRate), 0), file.length))
			if wasPlaying { _ = play() }
		}
	}

	private var currentFrame: AVAudioFramePosition {
		guard node.isPlaying, let nodeTime = node.lastRenderTime, staleRender.map({ nodeTime.sampleTime > $0 }) ?? true, let playerTime = node.playerTime(forNodeTime: nodeTime) else { return heldFrame }
		return min(startFrame + max(playerTime.sampleTime, 0), file.length)
	}

	/// The engine is started here rather than on creation, and again after an interruption has stopped it.
	func play() -> Bool {
		if !engine.isRunning {
			do { try engine.start() } catch { return false }
		}
		guard heldFrame < file.length || node.isPlaying else {
			// Seeked to the very end: nothing is scheduled, so the file is over as soon as it is asked to play.
			let mine = generation
			Task { @MainActor [weak self] in
				guard let self, self.generation == mine else { return }
				self.onFinished?()
			}
			return true
		}
		node.play()
		return true
	}

	func pause() {
		heldFrame = currentFrame
		node.pause()
	}

	func stop() {
		generation += 1
		node.stop()
		engine.stop()
	}

	private func schedule(from frame: AVAudioFramePosition) {
		generation += 1
		let mine = generation
		staleRender = node.lastRenderTime?.sampleTime
		node.stop()
		startFrame = frame
		heldFrame = frame
		let remaining = AVAudioFrameCount(clamping: file.length - frame)
		guard remaining > 0 else { return }
		node.scheduleSegment(file, startingFrame: frame, frameCount: remaining, at: nil, completionCallbackType: .dataPlayedBack) { [weak self] _ in
			Task { @MainActor [weak self] in
				guard let self, self.generation == mine else { return }
				self.heldFrame = self.file.length
				self.onFinished?()
			}
		}
	}
}
