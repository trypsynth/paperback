import Foundation

/// A DAISY audiobook's own recorded narration, moved from tab to tab as the reader switches
/// documents. One player serves every open document: only one of them can be narrating at a
/// time, and holding a decoder open per tab would keep a file handle and a chunk of memory per
/// book.
///
/// Where playback has reached is written back to the config on every clip change and every
/// pause rather than at some later checkpoint, because a book resumed from the wrong place is a
/// book the reader has to find their way back through.
@MainActor
final class RecordedNarration {
	private let player = DaisyAudioPlayer()
	private let config: ConfigManagerFfi

	/// The document the player is currently narrating, if any.
	private var attachedDocumentPath: String?

	/// Invoked whenever playback starts or stops, including at the end of the book.
	var onPlayingChanged: ((Bool) -> Void)?
	/// Invoked with where a relative seek actually landed, in document elapsed time.
	var onSeekLanded: ((Int64) -> Void)?
	/// Invoked with the text position of the clip now narrating, whenever it changes.
	var onClipChanged: ((Int64) -> Void)?
	/// Invoked when the player moves off a document, so a caller tracking what it last said
	/// about this one can forget it.
	var onDetached: (() -> Void)?

	init(config: ConfigManagerFfi) {
		self.config = config
		player.onPlaybackStateChanged = { [weak self] playing in
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
	}

	var isPlaying: Bool { player.isPlaying }

	/// Switches to narrating `tab`, picking up from its saved audio position, or from its saved
	/// reading position when it has never been listened to. Does nothing for a tab with no
	/// recording, or one that is already attached.
	func attach(to tab: DocumentTab) {
		let path = tab.url.path(percentEncoded: false)
		if attachedDocumentPath == path { return }
		detach()
		guard let session = tab.session, session.hasAudioFfi() else { return }
		player.attach(session: session, docKey: tab.id.uuidString)
		attachedDocumentPath = path
		let savedAudioMs = config.getDocumentAudioTimeFfi(path: path)
		if savedAudioMs >= 0 {
			player.seekToMs(savedAudioMs)
		} else {
			player.seekToPosition(tab.currentPosition)
		}
	}

	/// Saves where playback had reached and lets the document go, ahead of switching to another
	/// one or the app going away.
	func detach() {
		persistPosition()
		player.detach()
		attachedDocumentPath = nil
		onDetached?()
	}

	/// Whether the recording currently loaded belongs to this document.
	func isNarrating(_ path: String) -> Bool { attachedDocumentPath == path }

	func persistPosition() {
		guard let path = attachedDocumentPath, let ms = player.resumePointMs() else { return }
		config.setDocumentAudioTimeFfi(path: path, timeMs: ms)
		config.flush()
	}

	func play() { player.play() }

	func pause() { player.pause() }

	@discardableResult
	func seekToPosition(_ position: Int64) -> Bool { player.seekToPosition(position) }

	@discardableResult
	func seekToMs(_ elapsedMs: Int64) -> Bool { player.seekToMs(elapsedMs) }

	@discardableResult
	func seekRelativeMs(_ deltaMs: Int64) -> Bool { player.seekRelativeMs(deltaMs) }

	func shutdown() { player.detach() }
}
