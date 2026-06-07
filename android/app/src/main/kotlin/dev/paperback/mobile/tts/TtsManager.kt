package dev.paperback.mobile.tts

import android.content.Context
import android.media.AudioFocusRequest
import android.media.AudioManager
import android.support.v4.media.MediaMetadataCompat
import android.support.v4.media.session.MediaSessionCompat
import android.support.v4.media.session.PlaybackStateCompat
import android.speech.tts.TextToSpeech
import android.speech.tts.UtteranceProgressListener
import android.speech.tts.Voice
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import uniffi.paperback.ConfigManagerFfi
import java.util.Locale

class TtsManager(
	private val context: Context,
	private val config: ConfigManagerFfi
) : TextToSpeech.OnInitListener {
	private var tts: TextToSpeech? = null
	private var mediaSession: MediaSessionCompat? = null
	private var mediaPlayer: android.media.MediaPlayer? = null
	private var nextMediaPlayer: android.media.MediaPlayer? = null
	private var isNextMediaPlayerPrepared = false
	private var currentTempFile: java.io.File? = null
	private var nextTempFile: java.io.File? = null
	private var precachedText: String? = null
	private var fileCounter = 0
	private val audioManager = context.getSystemService(Context.AUDIO_SERVICE) as AudioManager

	var currentDocumentTitle: String = "Paperback"
		set(value) {
			field = value
			updateMediaMetadata()
		}
	var currentDocumentAuthor: String = "Unknown"
		set(value) {
			field = value
			updateMediaMetadata()
		}
	private var audioFocusRequest: AudioFocusRequest? = null
	private var wasPlayingBeforeFocusLoss = false
	private var hasStartedService = false

	private val audioFocusChangeListener = AudioManager.OnAudioFocusChangeListener { focusChange ->
		when (focusChange) {
			AudioManager.AUDIOFOCUS_LOSS,
			AudioManager.AUDIOFOCUS_LOSS_TRANSIENT -> {
				wasPlayingBeforeFocusLoss = _isSpeaking.value
				if (_isSpeaking.value) {
					onPauseCommand?.invoke()
				}
			}
			AudioManager.AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK -> {
				// System handles ducking automatically on API 26+, or it just keeps playing on older APIs.
			}
			AudioManager.AUDIOFOCUS_GAIN -> {
				if (wasPlayingBeforeFocusLoss) {
					onPlayCommand?.invoke()
					wasPlayingBeforeFocusLoss = false
				}
			}
		}
	}

	private fun requestAudioFocus() {
		if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
			val attributes = android.media.AudioAttributes.Builder()
				.setUsage(android.media.AudioAttributes.USAGE_MEDIA)
				.setContentType(android.media.AudioAttributes.CONTENT_TYPE_SPEECH)
				.build()
			val request = AudioFocusRequest
				.Builder(AudioManager.AUDIOFOCUS_GAIN)
				.setAudioAttributes(attributes)
				.setOnAudioFocusChangeListener(audioFocusChangeListener)
				.build()
			audioFocusRequest = request
			audioManager.requestAudioFocus(request)
		} else {
			@Suppress("DEPRECATION")
			audioManager.requestAudioFocus(
				audioFocusChangeListener,
				AudioManager.STREAM_MUSIC,
				AudioManager.AUDIOFOCUS_GAIN
			)
		}
	}

	private fun abandonAudioFocus() {
		if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
			audioFocusRequest?.let { audioManager.abandonAudioFocusRequest(it) }
		} else {
			@Suppress("DEPRECATION")
			audioManager.abandonAudioFocus(audioFocusChangeListener)
		}
	}

	companion object {
		const val SYSTEM_DEFAULT = "system_default"
		const val KEY_ENGINE = "tts_engine"
		const val KEY_VOICE = "tts_voice"
		const val KEY_RATE = "tts_rate"
		const val KEY_PITCH = "tts_pitch"
	}

	private val _currentEngineName = MutableStateFlow<String?>(null)
	val currentEngineName: StateFlow<String?> = _currentEngineName

	private val _isInitialized = MutableStateFlow(false)
	val isInitialized: StateFlow<Boolean> = _isInitialized

	private val ttsScope = kotlinx.coroutines.CoroutineScope(kotlinx.coroutines.Dispatchers.Main)
	private var stopSpeakingJob: kotlinx.coroutines.Job? = null

	private val _isSpeaking = MutableStateFlow(false)
	val isSpeaking: StateFlow<Boolean> = _isSpeaking

	private val _isPaused = MutableStateFlow(false)
	val isPaused: StateFlow<Boolean> = _isPaused

	var onUtteranceCompleted: (() -> Unit)? = null
	var onSegmentTransition: (() -> Unit)? = null
	var onPlayCommand: (() -> Unit)? = null
	var onPauseCommand: (() -> Unit)? = null
	var onNextCommand: (() -> Unit)? = null
	var onPrevCommand: (() -> Unit)? = null

	private val _currentSpeechRate = MutableStateFlow(50)
	val currentSpeechRate: StateFlow<Int> = _currentSpeechRate

	private val _currentPitch = MutableStateFlow(50)
	val currentPitch: StateFlow<Int> = _currentPitch

	private val _currentVoice = MutableStateFlow<Voice?>(null)
	val currentVoice: StateFlow<Voice?> = _currentVoice

	private val _availableVoices = MutableStateFlow<List<Voice>>(emptyList())
	val availableVoices: StateFlow<List<Voice>> = _availableVoices

	fun loadConfigAndInit() {
		val savedEngine = config.getAppString(KEY_ENGINE, SYSTEM_DEFAULT)
		initTts(savedEngine)
		initMediaSession()
	}

	private fun initMediaSession() {
		val mediaButtonIntent = android.content.Intent(android.content.Intent.ACTION_MEDIA_BUTTON).apply {
			setClass(context, androidx.media.session.MediaButtonReceiver::class.java)
		}

		val pendingIntent = android.app.PendingIntent.getBroadcast(
			context,
			0,
			mediaButtonIntent,
			android.app.PendingIntent.FLAG_IMMUTABLE
		)

		mediaSession = MediaSessionCompat(context, "PaperbackTtsSession").apply {
			setMediaButtonReceiver(pendingIntent)
		}

		PlaybackService.activeMediaSession = mediaSession
		@Suppress("DEPRECATION")
		mediaSession?.setFlags(
			MediaSessionCompat.FLAG_HANDLES_MEDIA_BUTTONS or
				MediaSessionCompat.FLAG_HANDLES_TRANSPORT_CONTROLS
		)
		mediaSession?.setCallback(object : MediaSessionCompat.Callback() {
			override fun onPlay() {
				onPlayCommand?.invoke()
			}

			override fun onPause() {
				onPauseCommand?.invoke()
			}

			override fun onSkipToNext() {
				onNextCommand?.invoke()
			}

			override fun onSkipToPrevious() {
				onPrevCommand?.invoke()
			}
		})

		mediaSession?.isActive = true

		val playbackState = PlaybackStateCompat
			.Builder()
			.setActions(
				PlaybackStateCompat.ACTION_PLAY or PlaybackStateCompat.ACTION_PAUSE or PlaybackStateCompat.ACTION_PLAY_PAUSE or
					PlaybackStateCompat.ACTION_SKIP_TO_NEXT or
					PlaybackStateCompat.ACTION_SKIP_TO_PREVIOUS
			).setState(PlaybackStateCompat.STATE_PAUSED, PlaybackStateCompat.PLAYBACK_POSITION_UNKNOWN, 1.0f)
			.build()
		mediaSession?.setPlaybackState(playbackState)
		updateMediaMetadata()
	}

	private fun updatePlaybackState(isPlaying: Boolean) {
		if (mediaSession?.isActive != true && (isPlaying || hasStartedService)) {
			mediaSession?.isActive = true
		}

		val state = if (isPlaying) PlaybackStateCompat.STATE_PLAYING else PlaybackStateCompat.STATE_PAUSED
		val playbackState = PlaybackStateCompat
			.Builder()
			.setActions(
				PlaybackStateCompat.ACTION_PLAY or PlaybackStateCompat.ACTION_PAUSE or PlaybackStateCompat.ACTION_PLAY_PAUSE or
					PlaybackStateCompat.ACTION_SKIP_TO_NEXT or
					PlaybackStateCompat.ACTION_SKIP_TO_PREVIOUS
			).setState(state, PlaybackStateCompat.PLAYBACK_POSITION_UNKNOWN, 1.0f)
			.build()
		mediaSession?.setPlaybackState(playbackState)

		if (isPlaying && !hasStartedService) {
			hasStartedService = true
		}
		updateMediaMetadata()
	}

	private fun updateMediaMetadata() {
		val metadata = MediaMetadataCompat.Builder()
			.putString(MediaMetadataCompat.METADATA_KEY_TITLE, currentDocumentTitle)
			.putString(MediaMetadataCompat.METADATA_KEY_ARTIST, currentDocumentAuthor)
			.build()
		mediaSession?.setMetadata(metadata)

		if (hasStartedService) {
			val intent = android.content.Intent(context, PlaybackService::class.java).apply {
				putExtra(PlaybackService.EXTRA_IS_PLAYING, _isSpeaking.value)
				putExtra(PlaybackService.EXTRA_TITLE, currentDocumentTitle)
				putExtra(PlaybackService.EXTRA_AUTHOR, currentDocumentAuthor)
			}
			androidx.core.content.ContextCompat.startForegroundService(context, intent)
		}
	}

	fun precache(text: String) {
		if (text.isBlank() || text == precachedText) return
		fileCounter++
		precachedText = text
		isNextMediaPlayerPrepared = false
		nextTempFile = java.io.File(context.cacheDir, "paperback_tts_next_$fileCounter.wav")
		if (nextTempFile!!.exists()) nextTempFile!!.delete()

		val params = android.os.Bundle()
		tts?.synthesizeToFile(text, params, nextTempFile, "TTS_PRECACHE_ID")
	}

	private fun initTts(engineName: String?) {
		_isInitialized.value = false
		tts?.shutdown()
		_currentEngineName.value = engineName
		val actualEngine = if (engineName == SYSTEM_DEFAULT) null else engineName
		tts = if (actualEngine != null) {
			TextToSpeech(context, this, actualEngine)
		} else {
			TextToSpeech(context, this)
		}
		if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.LOLLIPOP) {
			val attributes = android.media.AudioAttributes.Builder()
				.setUsage(android.media.AudioAttributes.USAGE_MEDIA)
				.setContentType(android.media.AudioAttributes.CONTENT_TYPE_SPEECH)
				.build()
			tts?.setAudioAttributes(attributes)
		}
	}

	override fun onInit(status: Int) {
		if (status == TextToSpeech.SUCCESS) {
			tts?.setOnUtteranceProgressListener(object : UtteranceProgressListener() {
				override fun onStart(utteranceId: String?) {
					if (!_isPaused.value && mediaPlayer == null) {
						stopSpeakingJob?.cancel()
						_isSpeaking.value = true
						updatePlaybackState(true)
					}
				}

				override fun onDone(utteranceId: String?) {
					if (utteranceId == "TTS_CONTENT_ID" && currentTempFile != null) {
						ttsScope.launch(kotlinx.coroutines.Dispatchers.IO) {
							try {
								val player = android.media.MediaPlayer().apply {
									val attributes = android.media.AudioAttributes.Builder()
										.setUsage(android.media.AudioAttributes.USAGE_MEDIA)
										.setContentType(android.media.AudioAttributes.CONTENT_TYPE_SPEECH)
										.build()
									setAudioAttributes(attributes)
									setDataSource(currentTempFile!!.absolutePath)

									setOnPreparedListener { mp ->
										ttsScope.launch(kotlinx.coroutines.Dispatchers.Main) {
											mediaPlayer = mp
											if (_isSpeaking.value && !_isPaused.value) {
												mp.start()
											}
											nextMediaPlayer?.let {
												try { mp.setNextMediaPlayer(it) } catch(e: Exception){}
											}
											setupCompletionListener(mp, utteranceId)
										}
									}

									setOnErrorListener { _, _, _ ->
										ttsScope.launch(kotlinx.coroutines.Dispatchers.Main) {
											stopSpeakingJob?.cancel()
											_isSpeaking.value = false
											updatePlaybackState(false)
											cleanupPlayer()
										}
										true
									}
									prepareAsync()
								}
							} catch (e: Exception) {
								e.printStackTrace()
							}
						}
					} else if (utteranceId == "TTS_PRECACHE_ID" && nextTempFile != null) {
						ttsScope.launch(kotlinx.coroutines.Dispatchers.IO) {
							try {
								val nextPlayer = android.media.MediaPlayer().apply {
									val attributes = android.media.AudioAttributes.Builder()
										.setUsage(android.media.AudioAttributes.USAGE_MEDIA)
										.setContentType(android.media.AudioAttributes.CONTENT_TYPE_SPEECH)
										.build()
									setAudioAttributes(attributes)
									setDataSource(nextTempFile!!.absolutePath)

									setOnPreparedListener { nextMp ->
										ttsScope.launch(kotlinx.coroutines.Dispatchers.Main) {
											nextMediaPlayer = nextMp
											isNextMediaPlayerPrepared = true
											mediaPlayer?.let {
												try { it.setNextMediaPlayer(nextMp) } catch (e: Exception) {}
											}
										}
									}
									prepareAsync()
								}
							} catch (e: Exception) {
								e.printStackTrace()
							}
						}
					} else {
						if (_isPaused.value) return
						stopSpeakingJob?.cancel()
						stopSpeakingJob = ttsScope.launch {
							kotlinx.coroutines.delay(400)
							_isSpeaking.value = false
							updatePlaybackState(false)
						}
						if (utteranceId == "TTS_CONTENT_ID") {
							onUtteranceCompleted?.invoke()
						}
					}
				}

				@Deprecated("Deprecated in Java")
				override fun onError(utteranceId: String?) {
					if (currentTempFile == null) {
						if (_isPaused.value) return
						stopSpeakingJob?.cancel()
						_isSpeaking.value = false
						updatePlaybackState(false)
					}
				}

				override fun onStop(
					utteranceId: String?,
					interrupted: Boolean
				) {
					if (currentTempFile == null) {
						if (_isPaused.value) return
						stopSpeakingJob?.cancel()
						_isSpeaking.value = false
						updatePlaybackState(false)
					}
				}
			})
			val langResult = tts?.setLanguage(Locale.getDefault()) ?: TextToSpeech.LANG_NOT_SUPPORTED
			if (langResult == TextToSpeech.LANG_MISSING_DATA || langResult == TextToSpeech.LANG_NOT_SUPPORTED) {
				_availableVoices.value = emptyList()
				_currentVoice.value = null
				_isInitialized.value = true
				return
			}
			if (_currentEngineName.value == SYSTEM_DEFAULT) {
				_availableVoices.value = emptyList()
				_currentVoice.value = null
			} else {
				val engine = _currentEngineName.value!!
				val savedRate = config.getAppString("${KEY_RATE}_$engine", "50").toIntOrNull() ?: 50
				setSpeechRate(savedRate)
				val savedPitch = config.getAppString("${KEY_PITCH}_$engine", "50").toIntOrNull() ?: 50
				setPitch(savedPitch)
				_availableVoices.value = getAvailableVoicesInternal()
				val savedVoiceName = config.getAppString("${KEY_VOICE}_$engine", "")
				val matchedVoice = _availableVoices.value.find { it.name == savedVoiceName }
				if (matchedVoice != null) {
					setVoice(matchedVoice)
				} else {
					_currentVoice.value = try {
						tts?.voice
					} catch (e: Exception) {
						null
					}
				}
			}
			_isInitialized.value = true
		}
	}

	private fun setupCompletionListener(mp: android.media.MediaPlayer, utteranceId: String?) {
		mp.setOnCompletionListener { _ ->
			if (nextMediaPlayer != null && isNextMediaPlayerPrepared) {
				val oldMp = mediaPlayer
				mediaPlayer = nextMediaPlayer
				nextMediaPlayer = null
				isNextMediaPlayerPrepared = false
				precachedText = null

				try { oldMp?.release() } catch (e: Exception) {}
				try { currentTempFile?.delete() } catch(e: Exception) {}
				currentTempFile = nextTempFile
				nextTempFile = null

				onSegmentTransition?.invoke()

				mediaPlayer?.let { setupCompletionListener(it, "TTS_CONTENT_ID") }
			} else {
				stopSpeakingJob?.cancel()
				stopSpeakingJob = ttsScope.launch {
					kotlinx.coroutines.delay(400)
					_isSpeaking.value = false
					updatePlaybackState(false)
				}
				if (utteranceId == "TTS_CONTENT_ID") {
					onUtteranceCompleted?.invoke()
				}
				cleanupPlayer()
			}
		}
	}

	fun speak(
		text: String,
		isSample: Boolean = false
	) {
		if (text.isNotBlank()) {
			if (!isSample) {
				requestAudioFocus()
			}
			val utteranceId = if (isSample) "TTS_SAMPLE_ID" else "TTS_CONTENT_ID"
			cleanupPlayer()
			tts?.stop()

			try {
				fileCounter++
				currentTempFile = java.io.File(context.cacheDir, "paperback_tts_$fileCounter.wav")
				if (currentTempFile!!.exists()) {
					currentTempFile!!.delete()
				}

				_isSpeaking.value = true
				_isPaused.value = false
				updatePlaybackState(true)

				val params = android.os.Bundle()
				tts?.synthesizeToFile(text, params, currentTempFile, utteranceId)
			} catch (e: Exception) {
				e.printStackTrace()
				cleanupPlayer()
				tts?.speak(text, TextToSpeech.QUEUE_FLUSH, null, utteranceId)
			}
		}
	}



	fun pause() {
		if (_isSpeaking.value && !_isPaused.value) {
			_isPaused.value = true
			_isSpeaking.value = false
			mediaPlayer?.let {
				try {
					if (it.isPlaying) {
						it.pause()
					}
				} catch (e: Exception) {}
			} ?: run {
				if (currentTempFile == null) {
					tts?.stop()
				}
			}
			updatePlaybackState(false)
		}
	}

	fun resume() {
		if (_isPaused.value) {
			_isPaused.value = false
			_isSpeaking.value = true
			updatePlaybackState(true)
			mediaPlayer?.let {
				try {
					it.start()
				} catch (e: Exception) {}
			}
		}
	}

	private fun cleanupPlayer() {
		try {
			mediaPlayer?.release()
		} catch (e: Exception) {}
		mediaPlayer = null

		try {
			nextMediaPlayer?.release()
		} catch (e: Exception) {}
		nextMediaPlayer = null
		isNextMediaPlayerPrepared = false
		precachedText = null

		currentTempFile = null
		nextTempFile = null

		try {
			context.cacheDir.listFiles()?.forEach {
				if (it.name.startsWith("paperback_tts_")) {
					it.delete()
				}
			}
		} catch (e: Exception) {}
	}

	fun stop() {
		tts?.stop()
		cleanupPlayer()
		stopSpeakingJob?.cancel()
		_isSpeaking.value = false
		_isPaused.value = false
		updatePlaybackState(false)
		abandonAudioFocus()
	}

	fun setSpeechRate(ratePercentage: Int) {
		_currentSpeechRate.value = ratePercentage
		val engine = _currentEngineName.value
		if (engine != null && engine != SYSTEM_DEFAULT) {
			config.setAppString("${KEY_RATE}_$engine", ratePercentage.toString())
			config.flush()
			val mappedRate = 0.1f + (ratePercentage / 100f) * 2.9f
			tts?.setSpeechRate(mappedRate)
		}
	}

	fun setPitch(pitchPercentage: Int) {
		_currentPitch.value = pitchPercentage
		val engine = _currentEngineName.value
		if (engine != null && engine != SYSTEM_DEFAULT) {
			config.setAppString("${KEY_PITCH}_$engine", pitchPercentage.toString())
			config.flush()
			val mappedPitch = 0.1f + (pitchPercentage / 100f) * 1.9f
			tts?.setPitch(mappedPitch)
		}
	}

	fun getAvailableEngines(): List<TextToSpeech.EngineInfo> {
		val engines = tts?.engines?.toMutableList() ?: mutableListOf()
		if (engines.none { it.name == SYSTEM_DEFAULT }) {
			engines.add(
				0,
				TextToSpeech.EngineInfo().apply {
					name = SYSTEM_DEFAULT
					label = "System Default"
				}
			)
		}
		return engines
	}

	fun getDefaultEngine(): String? = tts?.defaultEngine

	fun getCurrentEngine(): String? = _currentEngineName.value ?: SYSTEM_DEFAULT

	fun setEngine(engineName: String) {
		if (engineName != getCurrentEngine()) {
			config.setAppString(KEY_ENGINE, engineName)
			config.flush()
			initTts(engineName)
		}
	}

	private fun getAvailableVoicesInternal(): List<Voice> {
		val ttsInstance = tts ?: return emptyList()
		return try {
			val currentLocale = Locale.getDefault()
			val voices = ttsInstance.voices ?: return emptyList()
			val filtered = voices.filter { it.locale.language == currentLocale.language }
			if (filtered.isNotEmpty()) {
				filtered.sortedBy { it.name }
			} else {
				voices.sortedBy { it.name }
			}
		} catch (e: Exception) {
			emptyList()
		}
	}

	fun getAvailableVoices(): List<Voice> = _availableVoices.value

	fun setVoice(voice: Voice) {
		val engine = _currentEngineName.value
		if (engine != null && engine != SYSTEM_DEFAULT) {
			config.setAppString("${KEY_VOICE}_$engine", voice.name)
			config.flush()
			tts?.voice = voice
			_currentVoice.value = voice
		}
	}

	fun getCurrentVoice(): Voice? = _currentVoice.value

	fun shutdown() {
		stop()
		val stopIntent = android.content.Intent(context, PlaybackService::class.java).apply {
			action = PlaybackService.ACTION_STOP
		}
		context.startService(stopIntent)

		tts?.shutdown()
		mediaSession?.release()
		PlaybackService.activeMediaSession = null
		hasStartedService = false
	}
}
