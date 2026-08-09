package dev.paperback.android.tts

import androidx.media3.session.DefaultMediaNotificationProvider
import androidx.media3.session.MediaSession
import androidx.media3.session.MediaSessionService
import dev.paperback.android.R

class PlaybackService : MediaSessionService() {
	companion object {
		private const val CHANNEL_ID = "paperback_tts_channel"

		// TtsManager owns the session's lifecycle (creates it in initMediaSession(),
		// releases it in shutdown()); this service just hosts it for the OS, so it's
		// shared via this field rather than passed through an Intent.
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
		// addSession() is what wires the session into Media3's own notification/foreground
		// machinery (it creates an internal MediaController that observes the player and
		// calls startForeground() when appropriate). That's normally triggered automatically
		// when a real MediaController connects through onGetSession(), but TtsManager starts
		// this service with a plain bindService() call (just to keep it alive before playback
		// begins), which never goes through that path — so it has to be requested explicitly.
		// activeMediaSession is always set before TtsManager binds to this service, so it's
		// guaranteed non-null here.
		activeMediaSession?.let { addSession(it) }
	}

	override fun onGetSession(controllerInfo: MediaSession.ControllerInfo): MediaSession? = activeMediaSession

	override fun onDestroy() {
		activeMediaSession = null
		super.onDestroy()
	}
}
