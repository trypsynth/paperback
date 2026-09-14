package dev.paperback.android.tts

import android.app.PendingIntent
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.media.MediaPlayer
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
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import uniffi.paperback.ConfigManagerFfi
import java.io.File
import java.util.Locale

class TtsManager(
	private val context: Context,
	private val config: ConfigManagerFfi
) : TextToSpeech.OnInitListener,
	SpeechEngine {
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

	companion object {
		const val SYSTEM_DEFAULT = "system_default"
		const val KEY_ENGINE = "tts_engine"
		const val KEY_VOICE = "tts_voice"
		const val KEY_RATE = "tts_rate"
		const val KEY_PITCH = "tts_pitch"
	}

	private val _currentEngineName = MutableStateFlow<String?>(null)
	val currentEngineName: StateFlow<String?> = _currentEngineName.asStateFlow()

	private val _isInitialized = MutableStateFlow(false)
	val isInitialized: StateFlow<Boolean> = _isInitialized.asStateFlow()

	private val ttsScope = CoroutineScope(Dispatchers.Main)
	private var stopSpeakingJob: Job? = null

	private val _isSpeaking = MutableStateFlow(false)
	override val isSpeaking: StateFlow<Boolean> = _isSpeaking.asStateFlow()

	private val _isPaused = MutableStateFlow(false)
	override val isPaused: StateFlow<Boolean> = _isPaused.asStateFlow()

	var onUtteranceCompleted: (() -> Unit)? = null
	var onSegmentTransition: (() -> Unit)? = null
	var onPlayCommand: (() -> Unit)? = null
	var onPauseCommand: (() -> Unit)? = null
	var onNextCommand: (() -> Unit)? = null
	var onPrevCommand: (() -> Unit)? = null

	private val audioFocus =
		AudioFocusHolder(
			context,
			isPlaying = { _isSpeaking.value },
			onPause = { onPauseCommand?.invoke() },
			onResume = { onPlayCommand?.invoke() }
		)

	private val _currentSpeechRate = MutableStateFlow(50)
	val currentSpeechRate: StateFlow<Int> = _currentSpeechRate.asStateFlow()

	private val _currentPitch = MutableStateFlow(50)
	val currentPitch: StateFlow<Int> = _currentPitch.asStateFlow()

	private val _currentVoice = MutableStateFlow<Voice?>(null)
	val currentVoice: StateFlow<Voice?> = _currentVoice.asStateFlow()

	private val _availableVoices = MutableStateFlow<List<Voice>>(emptyList())
	val availableVoices: StateFlow<List<Voice>> = _availableVoices.asStateFlow()

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
				audioFocus.request()
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

	override fun pause() {
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

	/** Reflects a non-TTS playback engine's (e.g. DaisyAudioPlayer) play/pause state into
	 * isSpeaking/isPaused and the MediaSession, so the UI, notification, and lock-screen
	 * controls behave the same regardless of which engine is narrating. */
	fun setExternalPlaybackState(isPlaying: Boolean) {
		_isSpeaking.value = isPlaying
		_isPaused.value = !isPlaying
		updatePlaybackState(isPlaying)
	}

	override fun resume() {
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

	override fun stop() {
		tts?.stop()
		cleanupPlayer()
		stopSpeakingJob?.cancel()
		currentContentUtteranceId = null
		currentPrecacheUtteranceId = null
		_isSpeaking.value = false
		_isPaused.value = false
		updatePlaybackState(false)
		audioFocus.abandon()
	}

	fun setSpeechRate(ratePercentage: Int) {
		val percentage = ratePercentage.coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE)
		_currentSpeechRate.value = percentage
		val engine = _currentEngineName.value
		if (engine != null && engine != SYSTEM_DEFAULT) {
			config.setAppString("${KEY_RATE}_$engine", percentage.toString())
			config.flush()
			tts?.setSpeechRate(speechRateFor(percentage))
		}
	}

	fun setPitch(pitchPercentage: Int) {
		val percentage = pitchPercentage.coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE)
		_currentPitch.value = percentage
		val engine = _currentEngineName.value
		if (engine != null && engine != SYSTEM_DEFAULT) {
			config.setAppString("${KEY_PITCH}_$engine", percentage.toString())
			config.flush()
			tts?.setPitch(pitchFor(percentage))
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

/** The ends of the rate and pitch sliders, which every stored value is brought back inside: a
 * config written by hand or by another version can hold anything at all. */
internal const val MIN_SPEECH_PERCENTAGE = 0
internal const val MAX_SPEECH_PERCENTAGE = 100

/**
 * The engine speech rate a slider percentage means. The slider runs 0 to 100 with 50 in the
 * middle, and the engine takes a multiplier, so this spreads it over 0.1x to 3.0x: slow enough to
 * follow an unfamiliar word, fast enough for a practised listener.
 */
internal fun speechRateFor(percentage: Int): Float =
	0.1f + (percentage.coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE) / 100f) * 2.9f

/** The engine pitch a slider percentage means, over a narrower 0.1x to 2.0x: past that a voice
 * stops being understandable rather than just sounding different. */
internal fun pitchFor(percentage: Int): Float =
	0.1f + (percentage.coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE) / 100f) * 1.9f
