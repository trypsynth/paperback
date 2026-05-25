package dev.paperback.mobile.tts

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.media.session.MediaSession
import android.os.Build
import android.os.IBinder
import androidx.core.app.NotificationCompat
import dev.paperback.mobile.MainActivity
import dev.paperback.mobile.R

class PlaybackService : Service() {

	companion object {
		const val CHANNEL_ID = "paperback_tts_channel"
		const val NOTIFICATION_ID = 1

		const val ACTION_PLAY = "dev.paperback.mobile.ACTION_PLAY"
		const val ACTION_PAUSE = "dev.paperback.mobile.ACTION_PAUSE"
		const val ACTION_NEXT = "dev.paperback.mobile.ACTION_NEXT"
		const val ACTION_PREV = "dev.paperback.mobile.ACTION_PREV"
		const val ACTION_STOP = "dev.paperback.mobile.ACTION_STOP"
		const val EXTRA_SESSION_TOKEN = "EXTRA_SESSION_TOKEN"
		const val EXTRA_IS_PLAYING = "EXTRA_IS_PLAYING"
		const val EXTRA_TITLE = "EXTRA_TITLE"

		// We store the session locally so we don't need to pass it via Intent every time.
		var activeMediaSession: MediaSession? = null
	}

	override fun onCreate() {
		super.onCreate()
		createNotificationChannel()
	}

	override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
		if (intent == null) return START_NOT_STICKY

		when (intent.action) {
			ACTION_PLAY -> activeMediaSession?.controller?.transportControls?.play()
			ACTION_PAUSE -> activeMediaSession?.controller?.transportControls?.pause()
			ACTION_NEXT -> activeMediaSession?.controller?.transportControls?.skipToNext()
			ACTION_PREV -> activeMediaSession?.controller?.transportControls?.skipToPrevious()
			ACTION_STOP -> {
				activeMediaSession?.controller?.transportControls?.stop()
				stopForeground(STOP_FOREGROUND_REMOVE)
				stopSelf()
				return START_NOT_STICKY
			}
		}

		val isPlaying = intent.getBooleanExtra(EXTRA_IS_PLAYING, false)
		val title = intent.getStringExtra(EXTRA_TITLE) ?: "Paperback TTS"

		val notification = buildNotification(isPlaying, title)
		
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
			startForeground(NOTIFICATION_ID, notification, android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK)
		} else {
			startForeground(NOTIFICATION_ID, notification)
		}

		return START_STICKY
	}

	private fun buildNotification(isPlaying: Boolean, title: String): Notification {
		val mainIntent = Intent(this, MainActivity::class.java)
		val pendingMainIntent = PendingIntent.getActivity(
			this, 0, mainIntent, PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT
		)

		val builder = NotificationCompat.Builder(this, CHANNEL_ID)
			.setSmallIcon(android.R.drawable.ic_media_play) // Fallback icon
			.setContentTitle(title)
			.setContentText(if (isPlaying) "Playing" else "Paused")
			.setContentIntent(pendingMainIntent)
			.setVisibility(NotificationCompat.VISIBILITY_PUBLIC)
			.setOngoing(isPlaying)

		// Add Media actions
		builder.addAction(
			android.R.drawable.ic_media_previous, "Previous",
			PendingIntent.getService(this, 1, Intent(this, PlaybackService::class.java).setAction(ACTION_PREV), PendingIntent.FLAG_IMMUTABLE)
		)

		if (isPlaying) {
			builder.addAction(
				android.R.drawable.ic_media_pause, "Pause",
				PendingIntent.getService(this, 2, Intent(this, PlaybackService::class.java).setAction(ACTION_PAUSE), PendingIntent.FLAG_IMMUTABLE)
			)
		} else {
			builder.addAction(
				android.R.drawable.ic_media_play, "Play",
				PendingIntent.getService(this, 2, Intent(this, PlaybackService::class.java).setAction(ACTION_PLAY), PendingIntent.FLAG_IMMUTABLE)
			)
		}

		builder.addAction(
			android.R.drawable.ic_media_next, "Next",
			PendingIntent.getService(this, 3, Intent(this, PlaybackService::class.java).setAction(ACTION_NEXT), PendingIntent.FLAG_IMMUTABLE)
		)

		activeMediaSession?.sessionToken?.let { token ->
			val mediaStyle = androidx.media.app.NotificationCompat.MediaStyle()
				.setShowActionsInCompactView(0, 1, 2)
				.setMediaSession(android.support.v4.media.session.MediaSessionCompat.Token.fromToken(token))
			builder.setStyle(mediaStyle)
		}

		return builder.build()
	}

	private fun createNotificationChannel() {
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
			val channel = NotificationChannel(
				CHANNEL_ID,
				"TTS Playback",
				NotificationManager.IMPORTANCE_LOW
			).apply {
				description = "Controls for background TTS playback"
				setShowBadge(false)
			}
			val manager = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
			manager.createNotificationChannel(channel)
		}
	}

	override fun onBind(intent: Intent?): IBinder? = null
}
