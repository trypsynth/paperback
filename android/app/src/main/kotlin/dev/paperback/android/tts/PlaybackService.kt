package dev.paperback.android.tts

import androidx.annotation.OptIn
import androidx.media3.common.util.UnstableApi
import androidx.media3.session.DefaultMediaNotificationProvider
import androidx.media3.session.MediaSession
import androidx.media3.session.MediaSessionService
import dev.paperback.android.R

@OptIn(UnstableApi::class)
class PlaybackService : MediaSessionService() {
	companion object {
		private const val CHANNEL_ID = "paperback_tts_channel"

		// TtsManager owns the session; this service only hosts it for the OS.
		var activeMediaSession: MediaSession? = null
	}

	override fun onCreate() {
		super.onCreate()
		setMediaNotificationProvider(
			DefaultMediaNotificationProvider
				.Builder(this)
				.setChannelId(CHANNEL_ID)
				.setChannelName(R.string.tts_notification_channel_name)
				.build()
		)
		// A plain bindService() never connects a MediaController through onGetSession(), so Media3's notification and startForeground() wiring has to be requested explicitly.
		activeMediaSession?.let { addSession(it) }
	}

	override fun onGetSession(controllerInfo: MediaSession.ControllerInfo): MediaSession? = activeMediaSession

	override fun onDestroy() {
		activeMediaSession = null
		super.onDestroy()
	}
}
