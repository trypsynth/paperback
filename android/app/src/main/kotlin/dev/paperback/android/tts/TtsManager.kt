package dev.paperback.android.tts

import android.app.PendingIntent
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.media.AudioAttributes
import android.media.AudioFocusRequest
import android.media.AudioManager
import android.media.MediaPlayer
import android.os.Build
import android.os.Bundle
import android.os.IBinder
import android.speech.tts.TextToSpeech
import android.speech.tts.UtteranceProgressListener
import android.speech.tts.Voice
import androidx.annotation.OptIn
import androidx.media3.common.util.UnstableApi
import androidx.media3.session.MediaSession
import dev.paperback.android.MainActivity
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.cancel
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import uniffi.paperback.ConfigManagerFfi
import java.io.File
import java.util.Locale

class TtsManager(
	private val context: Context,
	private val config: ConfigManagerFfi
) : TextToSpeech.OnInitListener {
	private var tts: TextToSpeech? = null
	private var mediaSession: MediaSession? = null
	private var ttsPlayer: TtsPlayer? = null
	private var serviceConnection: ServiceConnection? = null
	private var mediaPlayer: MediaPlayer? = null
	private var nextMediaPlayer: MediaPlayer? = null
	private var isNextMediaPlayerPrepared = false
	private var currentTempFile: File? = null
	private var nextTempFile: File? = null
	private var precachedText: String? = null
	private var fileCounter = 0

	// Identifies the synthesis request that isSpeaking/media-player state should currently track.
	// Seeking quickly (e.g. paragraph-by-paragraph) cancels in-flight synthesis/precache requests;
	// without this, a stale callback for an abandoned request could flip isSpeaking off after the
	// next segment already started, causing TalkBack to briefly announce "Play" then "Pause".
	private var currentContentUtteranceId: String? = null
	private var currentPrecacheUtteranceId: String? = null
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

	// Speech playback always uses the same audio attributes; shared to avoid rebuilding
	// an identical AudioAttributes instance at every call site.
	private fun speechAudioAttributes(): AudioAttributes =
		AudioAttributes
			.Builder()
			.setUsage(AudioAttributes.USAGE_MEDIA)
			.setContentType(AudioAttributes.CONTENT_TYPE_SPEECH)
			.build()

	private fun requestAudioFocus() {
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
			val request = AudioFocusRequest
				.Builder(AudioManager.AUDIOFOCUS_GAIN)
				.setAudioAttributes(speechAudioAttributes())
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
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
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

	private val ttsScope = CoroutineScope(Dispatchers.Main)
	private var stopSpeakingJob: Job? = null

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
		val player = TtsPlayer(
			onPlayCommand = { onPlayCommand?.invoke() },
			onPauseCommand = { onPauseCommand?.invoke() },
			onNextCommand = { onNextCommand?.invoke() },
			onPrevCommand = { onPrevCommand?.invoke() }
		)
		ttsPlayer = player

		val sessionActivityIntent = Intent(context, MainActivity::class.java)
		val sessionActivityPendingIntent = PendingIntent.getActivity(
			context,
			0,
			sessionActivityIntent,
			PendingIntent.FLAG_IMMUTABLE
		)

		mediaSession = MediaSession
			.Builder(context, player)
			.setSessionActivity(sessionActivityPendingIntent)
			.build()

		PlaybackService.activeMediaSession = mediaSession
		updateMediaMetadata()

		// Binding (rather than Context.startForegroundService()) keeps PlaybackService
		// alive and its Media3 internals already observing the player before playback
		// ever begins. Starting it with a plain Intent instead races the 5-second
		// startForeground() deadline against Media3's async notification/session wiring
		// and intermittently crashes with ForegroundServiceDidNotStartInTimeException —
		// see https://github.com/androidx/media/issues/167, where the Media3 maintainers
		// confirm a bound controller/client is the supported way to avoid it.
		val connection = object : ServiceConnection {
			override fun onServiceConnected(
				name: ComponentName,
				binder: IBinder?
			) {
			}

			override fun onServiceDisconnected(name: ComponentName) {}
		}
		serviceConnection = connection
		context.bindService(Intent(context, PlaybackService::class.java), connection, Context.BIND_AUTO_CREATE)
	}

	private fun updatePlaybackState(isPlaying: Boolean) {
		ttsPlayer?.updatePlaybackState(isPlaying)
	}

	private fun updateMediaMetadata() {
		ttsPlayer?.updateMetadata(currentDocumentTitle, currentDocumentAuthor)
	}

	fun precache(text: String) {
		if (text.isBlank() || text == precachedText) return
		fileCounter++
		precachedText = text
		isNextMediaPlayerPrepared = false
		val precacheUtteranceId = "TTS_PRECACHE_ID_$fileCounter"
		currentPrecacheUtteranceId = precacheUtteranceId
		nextTempFile = File(context.cacheDir, "paperback_tts_next_$fileCounter.wav")
		nextTempFile?.takeIf { it.exists() }?.delete()

		val params = Bundle()
		tts?.synthesizeToFile(text, params, nextTempFile, precacheUtteranceId)
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
		tts?.setAudioAttributes(speechAudioAttributes())
	}

	override fun onInit(status: Int) {
		if (status == TextToSpeech.SUCCESS) {
			tts?.setOnUtteranceProgressListener(object : UtteranceProgressListener() {
				override fun onStart(utteranceId: String?) {
					if (utteranceId == currentContentUtteranceId && !_isPaused.value && mediaPlayer == null) {
						stopSpeakingJob?.cancel()
						_isSpeaking.value = true
						updatePlaybackState(true)
					}
				}

				override fun onDone(utteranceId: String?) {
					val isCurrentContent = utteranceId != null && utteranceId == currentContentUtteranceId
					val isCurrentPrecache = utteranceId != null && utteranceId == currentPrecacheUtteranceId
					if (isCurrentContent && currentTempFile != null) {
						ttsScope.launch(Dispatchers.IO) {
							try {
								val player = MediaPlayer().apply {
									setAudioAttributes(speechAudioAttributes())
									setDataSource(currentTempFile!!.absolutePath)

									setOnPreparedListener { mp ->
										ttsScope.launch(Dispatchers.Main) {
											mediaPlayer = mp
											if (_isSpeaking.value && !_isPaused.value) {
												mp.start()
											}
											nextMediaPlayer?.let {
												try {
													mp.setNextMediaPlayer(it)
												} catch (_: Exception) {
												}
											}
											setupCompletionListener(mp, utteranceId)
										}
									}

									setOnErrorListener { _, _, _ ->
										ttsScope.launch(Dispatchers.Main) {
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
					} else if (isCurrentPrecache && nextTempFile != null) {
						ttsScope.launch(Dispatchers.IO) {
							try {
								val nextPlayer = MediaPlayer().apply {
									setAudioAttributes(speechAudioAttributes())
									setDataSource(nextTempFile!!.absolutePath)

									setOnPreparedListener { nextMp ->
										ttsScope.launch(Dispatchers.Main) {
											nextMediaPlayer = nextMp
											isNextMediaPlayerPrepared = true
											mediaPlayer?.let {
												try {
													it.setNextMediaPlayer(nextMp)
												} catch (_: Exception) {
												}
											}
										}
									}
									prepareAsync()
								}
							} catch (e: Exception) {
								e.printStackTrace()
							}
						}
					} else if (isCurrentContent) {
						// Current content utterance finished without going through the media-player
						// path above (e.g. a sample preview). A stale/superseded content or precache
						// callback never reaches here, so it can no longer flip isSpeaking off late.
						if (_isPaused.value) return
						stopSpeakingJob?.cancel()
						stopSpeakingJob = ttsScope.launch {
							delay(400)
							_isSpeaking.value = false
							updatePlaybackState(false)
						}
						if (utteranceId?.startsWith("TTS_CONTENT_ID") == true) {
							onUtteranceCompleted?.invoke()
						}
					}
					// else: stale/cancelled utterance (superseded by a newer speak/precache) - ignore.
				}

				@Deprecated("Deprecated in Java")
				override fun onError(utteranceId: String?) {
					if (utteranceId == currentContentUtteranceId) {
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
					if (utteranceId == currentContentUtteranceId) {
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
					} catch (_: Exception) {
						null
					}
				}
			}
			_isInitialized.value = true
		}
	}

	private fun setupCompletionListener(
		mp: MediaPlayer,
		utteranceId: String?
	) {
		mp.setOnCompletionListener { _ ->
			if (nextMediaPlayer != null && isNextMediaPlayerPrepared) {
				val oldMp = mediaPlayer
				mediaPlayer = nextMediaPlayer
				nextMediaPlayer = null
				isNextMediaPlayerPrepared = false
				precachedText = null

				try {
					oldMp?.release()
				} catch (_: Exception) {
				}
				try {
					currentTempFile?.delete()
				} catch (_: Exception) {
				}
				currentTempFile = nextTempFile
				nextTempFile = null

				onSegmentTransition?.invoke()

				mediaPlayer?.let { setupCompletionListener(it, "TTS_CONTENT_ID") }
			} else {
				stopSpeakingJob?.cancel()
				stopSpeakingJob = ttsScope.launch {
					delay(400)
					_isSpeaking.value = false
					updatePlaybackState(false)
				}
				if (utteranceId?.startsWith("TTS_CONTENT_ID") == true) {
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
			stopSpeakingJob?.cancel()
			fileCounter++
			val utteranceId = if (isSample) "TTS_SAMPLE_ID_$fileCounter" else "TTS_CONTENT_ID_$fileCounter"
			currentContentUtteranceId = utteranceId
			cleanupPlayer()
			tts?.stop()

			try {
				currentTempFile = File(context.cacheDir, "paperback_tts_$fileCounter.wav")
				currentTempFile?.takeIf { it.exists() }?.delete()

				_isSpeaking.value = true
				_isPaused.value = false
				updatePlaybackState(true)

				val params = Bundle()
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
				} catch (_: Exception) {
				}
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
				} catch (_: Exception) {
				}
			}
		}
	}

	private fun cleanupPlayer() {
		try {
			mediaPlayer?.release()
		} catch (_: Exception) {
		}
		mediaPlayer = null

		try {
			nextMediaPlayer?.release()
		} catch (_: Exception) {
		}
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
		} catch (_: Exception) {
		}
	}

	fun stop() {
		tts?.stop()
		cleanupPlayer()
		stopSpeakingJob?.cancel()
		currentContentUtteranceId = null
		currentPrecacheUtteranceId = null
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
		} catch (_: Exception) {
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

	@OptIn(UnstableApi::class)
	fun shutdown() {
		stop()
		tts?.shutdown()

		mediaSession?.release()
		mediaSession = null
		ttsPlayer?.release()
		ttsPlayer = null
		PlaybackService.activeMediaSession = null

		// Unbind rather than force-stopping the service — Media3's own lifecycle
		// handling decides when it's actually safe for the service to go away.
		serviceConnection?.let { context.unbindService(it) }
		serviceConnection = null

		// Last, so nothing torn down above can leave work queued: a late onDone callback
		// would otherwise launch on this scope after shutdown and build a MediaPlayer for a
		// temp file that no longer exists, with nothing left to release it.
		ttsScope.cancel()
	}
}
