package dev.paperback.mobile.tts

import android.content.Context
import android.media.AudioFocusRequest
import android.media.AudioManager
import android.media.session.MediaSession
import android.media.session.PlaybackState
import android.speech.tts.TextToSpeech
import android.speech.tts.UtteranceProgressListener
import android.speech.tts.Voice
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import uniffi.paperback.ConfigManagerFfi
import java.util.Locale

class TtsManager(
	private val context: Context,
	private val config: ConfigManagerFfi
) : TextToSpeech.OnInitListener {
	private var tts: TextToSpeech? = null
	private var mediaSession: MediaSession? = null
	private val audioManager = context.getSystemService(Context.AUDIO_SERVICE) as AudioManager
	private var audioFocusRequest: AudioFocusRequest? = null

	private val audioFocusChangeListener = AudioManager.OnAudioFocusChangeListener { focusChange ->
		when (focusChange) {
			AudioManager.AUDIOFOCUS_LOSS,
			AudioManager.AUDIOFOCUS_LOSS_TRANSIENT -> {
				onPauseCommand?.invoke()
			}
		}
	}

	private fun requestAudioFocus() {
		if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
			val request = AudioFocusRequest
				.Builder(AudioManager.AUDIOFOCUS_GAIN)
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
	}

	private val _currentEngineName = MutableStateFlow<String?>(null)
	val currentEngineName: StateFlow<String?> = _currentEngineName

	private val _isInitialized = MutableStateFlow(false)
	val isInitialized: StateFlow<Boolean> = _isInitialized

	private val _isSpeaking = MutableStateFlow(false)
	val isSpeaking: StateFlow<Boolean> = _isSpeaking

	var onUtteranceCompleted: (() -> Unit)? = null
	var onPlayCommand: (() -> Unit)? = null
	var onPauseCommand: (() -> Unit)? = null
	var onNextCommand: (() -> Unit)? = null
	var onPrevCommand: (() -> Unit)? = null

	private val _currentSpeechRate = MutableStateFlow(50)
	val currentSpeechRate: StateFlow<Int> = _currentSpeechRate

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
		mediaSession = MediaSession(context, "PaperbackTtsSession")
		mediaSession?.setCallback(object : MediaSession.Callback() {
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
		updatePlaybackState(false)
	}

	private fun updatePlaybackState(isPlaying: Boolean) {
		val state = if (isPlaying) PlaybackState.STATE_PLAYING else PlaybackState.STATE_PAUSED
		val playbackState = PlaybackState
			.Builder()
			.setActions(
				PlaybackState.ACTION_PLAY or PlaybackState.ACTION_PAUSE or PlaybackState.ACTION_PLAY_PAUSE or
					PlaybackState.ACTION_SKIP_TO_NEXT or
					PlaybackState.ACTION_SKIP_TO_PREVIOUS
			).setState(state, PlaybackState.PLAYBACK_POSITION_UNKNOWN, 1.0f)
			.build()
		mediaSession?.setPlaybackState(playbackState)
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
	}

	override fun onInit(status: Int) {
		if (status == TextToSpeech.SUCCESS) {
			tts?.setOnUtteranceProgressListener(object : UtteranceProgressListener() {
				override fun onStart(utteranceId: String?) {
					_isSpeaking.value = true
					updatePlaybackState(true)
				}

				override fun onDone(utteranceId: String?) {
					_isSpeaking.value = false
					updatePlaybackState(false)
					if (utteranceId == "TTS_CONTENT_ID") {
						onUtteranceCompleted?.invoke()
					}
				}

				@Deprecated("Deprecated in Java")
				override fun onError(utteranceId: String?) {
					_isSpeaking.value = false
					updatePlaybackState(false)
				}

				override fun onStop(
					utteranceId: String?,
					interrupted: Boolean
				) {
					_isSpeaking.value = false
					updatePlaybackState(false)
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

	fun speak(
		text: String,
		isSample: Boolean = false
	) {
		if (text.isNotBlank()) {
			if (!isSample) {
				requestAudioFocus()
			}
			val utteranceId = if (isSample) "TTS_SAMPLE_ID" else "TTS_CONTENT_ID"
			tts?.speak(text, TextToSpeech.QUEUE_FLUSH, null, utteranceId)
		}
	}

	fun stop() {
		tts?.stop()
		_isSpeaking.value = false
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
		tts?.shutdown()
		mediaSession?.release()
	}
}
