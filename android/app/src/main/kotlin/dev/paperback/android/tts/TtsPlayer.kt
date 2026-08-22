package dev.paperback.android.tts

import android.os.Looper
import androidx.annotation.OptIn
import androidx.media3.common.MediaItem
import androidx.media3.common.MediaMetadata
import androidx.media3.common.Player
import androidx.media3.common.SimpleBasePlayer
import androidx.media3.common.util.UnstableApi
import com.google.common.util.concurrent.Futures
import com.google.common.util.concurrent.ListenableFuture

/**
 * Adapts TtsManager's play/pause state and document title/author into Media3's Player
 * interface, so a MediaSession (and its auto-generated notification, lock-screen
 * controls, and Bluetooth/headset button routing) can be driven by it. The actual
 * TextToSpeech-to-MediaPlayer playback pipeline lives entirely in TtsManager; this class
 * only reports its state and forwards transport commands back to it.
 */
@OptIn(UnstableApi::class)
class TtsPlayer(
	private val onPlayCommand: () -> Unit,
	private val onPauseCommand: () -> Unit,
	private val onNextCommand: () -> Unit,
	private val onPrevCommand: () -> Unit
) : SimpleBasePlayer(Looper.getMainLooper()) {
	private var playing = false
	private var mediaItemData = MediaItemData.Builder(MEDIA_ITEM_UID).build()

	private val availableCommands = Player.Commands
		.Builder()
		.addAll(
			Player.COMMAND_PLAY_PAUSE,
			Player.COMMAND_SEEK_TO_NEXT,
			Player.COMMAND_SEEK_TO_PREVIOUS,
			Player.COMMAND_GET_METADATA,
			Player.COMMAND_GET_CURRENT_MEDIA_ITEM,
			Player.COMMAND_GET_TIMELINE
		).build()

	override fun getState(): State =
		State
			.Builder()
			.setAvailableCommands(availableCommands)
			.setPlaylist(listOf(mediaItemData))
			.setCurrentMediaItemIndex(0)
			.setPlayWhenReady(playing, Player.PLAY_WHEN_READY_CHANGE_REASON_USER_REQUEST)
			.setPlaybackState(Player.STATE_READY)
			.build()

	override fun handleSetPlayWhenReady(playWhenReady: Boolean): ListenableFuture<*> {
		if (playWhenReady) onPlayCommand() else onPauseCommand()
		return Futures.immediateVoidFuture()
	}

	override fun handleSeek(
		mediaItemIndex: Int,
		positionMs: Long,
		seekCommand: Int
	): ListenableFuture<*> {
		when (seekCommand) {
			Player.COMMAND_SEEK_TO_NEXT -> onNextCommand()
			Player.COMMAND_SEEK_TO_PREVIOUS -> onPrevCommand()
			// Only next/previous are advertised in availableCommands; anything else Media3 routes
			// here is a transport we don't support, so there's nothing to do.
			else -> {}
		}
		return Futures.immediateVoidFuture()
	}

	/** Called by TtsManager whenever isSpeaking changes. */
	fun updatePlaybackState(isPlaying: Boolean) {
		playing = isPlaying
		invalidateState()
	}

	/** Called by TtsManager whenever the document title/author changes. */
	fun updateMetadata(
		title: String,
		author: String
	) {
		val metadata = MediaMetadata
			.Builder()
			.setTitle(title)
			.setArtist(author)
			.build()
		mediaItemData = MediaItemData
			.Builder(MEDIA_ITEM_UID)
			.setMediaItem(MediaItem.Builder().setMediaMetadata(metadata).build())
			.setMediaMetadata(metadata)
			.build()
		invalidateState()
	}

	companion object {
		private const val MEDIA_ITEM_UID = "paperback_tts_item"
	}
}
