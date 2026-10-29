package dev.paperback.android.tts

import android.app.PendingIntent
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.os.IBinder
import android.provider.Settings
import android.speech.tts.TextToSpeech
import android.speech.tts.UtteranceProgressListener
import android.speech.tts.Voice
import android.util.Log
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
import kotlin.math.roundToInt

private const val TAG = "PaperbackTts"

class TtsManager(private val context: Context, private val config: ConfigManagerFfi) :
	TextToSpeech.OnInitListener,
	SpeechEngine {
	private var tts: TextToSpeech? = null
	private var mediaSession: MediaSession? = null
	private var ttsPlayer: TtsPlayer? = null
	private var serviceConnection: ServiceConnection? = null
	private val playback = StreamedPlayback(speechAudioAttributes(), ::onSegmentPlayed)
	private var precachedText: String? = null
	private var utteranceCounter = 0

	// A stale callback for a replaced request names neither of these, and is ignored.
	private var currentContentUtteranceId: String? = null
	private var currentPrecacheUtteranceId: String? = null

	// Kept so a rate or pitch change can render the current paragraph again at the new settings.
	private var currentText: String? = null

	// So carrying on after a rate change can scale how far in the listener was.
	private var currentRate = 1f
	private var precacheRate = 1f

	// Set by a rate or pitch change while paused, so resuming renders again instead of playing the old audio (#1067).
	private var currentAudioStale = false
	private var rerenderJob: Job? = null

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
		const val KEY_PARAGRAPH_PAUSE = "tts_paragraph_pause_ms"
		private const val RERENDER_DELAY_MS = 400L
		private const val SAMPLE_PREFIX = "TTS_SAMPLE_ID_"
		private const val SYSTEM_SPEECH_SCALE = 100

		/** The audio already arrives through the progress listener, so the file keeps nothing. */
		private val DISCARDED_AUDIO = File("/dev/null")
	}

	private val _currentEngineName = MutableStateFlow<String?>(null)
	val currentEngineName: StateFlow<String?> = _currentEngineName.asStateFlow()

	private val _isInitialized = MutableStateFlow(false)
	val isInitialized: StateFlow<Boolean> = _isInitialized.asStateFlow()

	/** Calls before `onInit` fail, and a third-party engine loading its dictionaries may not be ready when play is pressed, so the press landed on nothing. */
	private var pendingSpeak: Pair<String, Boolean>? = null

	private val ttsScope = CoroutineScope(Dispatchers.Main)
	private var stopSpeakingJob: Job? = null

	private val _isSpeaking = MutableStateFlow(false)
	override val isSpeaking: StateFlow<Boolean> = _isSpeaking.asStateFlow()

	private val _isPaused = MutableStateFlow(false)
	override val isPaused: StateFlow<Boolean> = _isPaused.asStateFlow()

	var onUtteranceCompleted: (() -> Unit)? = null
	var onSegmentTransition: (() -> Unit)? = null

	// Read at each paragraph boundary, so a changed Find preference takes effect there and not only on the next play press.
	var shouldAutoAdvance: () -> Boolean = { true }

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

	private val _paragraphPauseMs = MutableStateFlow(MIN_PARAGRAPH_PAUSE_MS)
	val paragraphPauseMs: StateFlow<Int> = _paragraphPauseMs.asStateFlow()

	private val _currentVoice = MutableStateFlow<Voice?>(null)
	val currentVoice: StateFlow<Voice?> = _currentVoice.asStateFlow()

	private val _availableVoices = MutableStateFlow<List<Voice>>(emptyList())
	val availableVoices: StateFlow<List<Voice>> = _availableVoices.asStateFlow()

	fun loadConfigAndInit() {
		val savedEngine = config.getAppString(KEY_ENGINE, SYSTEM_DEFAULT)
		config.getAppString(KEY_PARAGRAPH_PAUSE, "").toIntOrNull()?.let { _paragraphPauseMs.value = paragraphPauseFor(it) }
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
		// Bound rather than started: startForegroundService() races the 5-second deadline against Media3's async wiring and intermittently throws ForegroundServiceDidNotStartInTimeException (androidx/media#167).
		val connection = object : ServiceConnection {
			override fun onServiceConnected(name: ComponentName, binder: IBinder?) {
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

	/** The next paragraph is made while the current one talks, so without this a rate or pitch change arrives one paragraph late. */
	private fun resynthesizePrecache() {
		val text = precachedText ?: return
		currentPrecacheUtteranceId?.let { playback.remove(it) }
		currentPrecacheUtteranceId = null
		precachedText = null
		precache(text)
	}

	fun precache(text: String) {
		if (text.isBlank() || text == precachedText) return
		currentPrecacheUtteranceId?.let { playback.remove(it) }
		val utteranceId = "TTS_PRECACHE_ID_${++utteranceCounter}"
		precachedText = text
		precacheRate = speechRateFor(_currentSpeechRate.value)
		currentPrecacheUtteranceId = utteranceId
		playback.enqueue(utteranceId, { _paragraphPauseMs.value })
		synthesize(text, utteranceId)
	}

	/** The engine only hands audio over while making a file or playing it itself, and its own playback can't pause mid-paragraph, hence a file that keeps nothing. */
	private fun synthesize(text: String, utteranceId: String): Boolean {
		val queued = tts?.synthesizeToFile(text, null, DISCARDED_AUDIO, utteranceId)
		if (queued != TextToSpeech.SUCCESS) {
			Log.e(TAG, "TTS engine refused to synthesize (result $queued)")
			playback.done(utteranceId)
			return false
		}
		return true
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
		if (status != TextToSpeech.SUCCESS) {
			// Otherwise the reader is left pressing play against an engine that never answers.
			Log.e(TAG, "TTS engine ${_currentEngineName.value ?: SYSTEM_DEFAULT} failed to start (status $status)")
			pendingSpeak = null
			_isSpeaking.value = false
			updatePlaybackState(false)
			return
		}
		if (status == TextToSpeech.SUCCESS) {
			tts?.setOnUtteranceProgressListener(object : UtteranceProgressListener() {
				override fun onStart(utteranceId: String?) {}

				override fun onBeginSynthesis(utteranceId: String?, sampleRateInHz: Int, audioFormat: Int, channelCount: Int) {
					utteranceId?.let { playback.begin(it, sampleRateInHz, audioFormat, channelCount) }
				}

				override fun onAudioAvailable(utteranceId: String?, audio: ByteArray?) {
					if (utteranceId != null && audio != null) playback.audio(utteranceId, audio)
				}

				override fun onDone(utteranceId: String?) {
					utteranceId ?: return
					if (utteranceId.startsWith(SAMPLE_PREFIX)) {
						ttsScope.launch { finishSample(utteranceId) }
					} else {
						playback.done(utteranceId)
					}
				}

				@Deprecated("Deprecated in Java")
				override fun onError(utteranceId: String?) {
					utteranceId ?: return
					playback.done(utteranceId)
					ttsScope.launch { onSynthesisFailed(utteranceId) }
				}

				override fun onStop(utteranceId: String?, interrupted: Boolean) {
					utteranceId?.let { playback.done(it) }
				}
			})
			val langResult = tts?.setLanguage(Locale.getDefault()) ?: TextToSpeech.LANG_NOT_SUPPORTED
			if (langResult == TextToSpeech.LANG_MISSING_DATA || langResult == TextToSpeech.LANG_NOT_SUPPORTED) {
				_availableVoices.value = emptyList()
				_currentVoice.value = null
				_isInitialized.value = true
				return
			}
			val engine = _currentEngineName.value
			if (engine == null || engine == SYSTEM_DEFAULT) {
				_availableVoices.value = emptyList()
				_currentVoice.value = null
				loadSystemDefaultSpeech()
			} else {
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
			speakPending()
		}
	}

	private fun speakPending() {
		val (text, isSample) = pendingSpeak ?: return
		pendingSpeak = null
		speak(text, isSample)
	}

	private fun onSegmentPlayed(utteranceId: String) {
		if (utteranceId != currentContentUtteranceId) return
		val next = currentPrecacheUtteranceId
		if (next != null && shouldAutoAdvance() && precachedText != null) {
			currentContentUtteranceId = next
			currentText = precachedText
			currentRate = precacheRate
			currentPrecacheUtteranceId = null
			precachedText = null
			onSegmentTransition?.invoke()
			return
		}
		next?.let { playback.remove(it) }
		currentPrecacheUtteranceId = null
		precachedText = null
		stopSpeakingJob?.cancel()
		stopSpeakingJob = ttsScope.launch {
			delay(400)
			_isSpeaking.value = false
			updatePlaybackState(false)
		}
		onUtteranceCompleted?.invoke()
	}

	/** A failed paragraph made ahead is dropped, so reaching it asks for it afresh rather than skipping it. */
	private fun onSynthesisFailed(utteranceId: String) {
		if (utteranceId == currentPrecacheUtteranceId) {
			playback.remove(utteranceId)
			currentPrecacheUtteranceId = null
			precachedText = null
		} else if (utteranceId == currentContentUtteranceId && !_isPaused.value) {
			stopSpeakingJob?.cancel()
			_isSpeaking.value = false
			updatePlaybackState(false)
		}
	}

	private fun finishSample(utteranceId: String) {
		if (utteranceId != currentContentUtteranceId || _isPaused.value) return
		stopSpeakingJob?.cancel()
		stopSpeakingJob = ttsScope.launch {
			delay(400)
			_isSpeaking.value = false
			updatePlaybackState(false)
		}
	}

	fun speak(text: String, isSample: Boolean = false) {
		speakFrom(text, isSample, skipMs = 0)
	}

	private fun speakFrom(text: String, isSample: Boolean, skipMs: Long) {
		if (text.isBlank()) return
		// Every call before the engine is ready fails, so hold the text rather than have it dropped.
		if (!_isInitialized.value) {
			pendingSpeak = text to isSample
			return
		}
		if (!isSample) {
			audioFocus.request()
		}
		stopSpeakingJob?.cancel()
		rerenderJob?.cancel()
		tts?.stop()
		playback.clear()
		currentPrecacheUtteranceId = null
		precachedText = null
		currentText = if (isSample) null else text
		currentAudioStale = false
		_isSpeaking.value = true
		_isPaused.value = false
		updatePlaybackState(true)
		if (isSample) {
			// A sample is a one-off with nothing to pause or carry on from, so the engine plays it itself.
			val utteranceId = "$SAMPLE_PREFIX${++utteranceCounter}"
			currentContentUtteranceId = utteranceId
			tts?.speak(text, TextToSpeech.QUEUE_FLUSH, null, utteranceId)
			return
		}
		val utteranceId = "TTS_CONTENT_ID_${++utteranceCounter}"
		currentContentUtteranceId = utteranceId
		currentRate = speechRateFor(_currentSpeechRate.value)
		playback.enqueue(utteranceId, { _paragraphPauseMs.value }, skipMs)
		playback.play()
		if (!synthesize(text, utteranceId)) {
			// The return value is the only report of a refused request, and without acting on it the bar says playing while nothing ever will.
			playback.clear()
			_isSpeaking.value = false
			updatePlaybackState(false)
		}
	}

	override fun pause() {
		if (_isSpeaking.value && !_isPaused.value) {
			_isPaused.value = true
			_isSpeaking.value = false
			stopSpeakingJob?.cancel()
			if (currentText == null) tts?.stop() else playback.pause()
			updatePlaybackState(false)
		}
	}

	/** So the UI, notification and lock-screen controls behave the same while DAISY audio narrates. */
	fun setExternalPlaybackState(isPlaying: Boolean) {
		_isSpeaking.value = isPlaying
		_isPaused.value = !isPlaying
		updatePlaybackState(isPlaying)
	}

	override fun resume() {
		if (!_isPaused.value) return
		val text = currentText
		if (currentAudioStale || (text != null && playback.isIdle())) {
			// Nothing left to carry on from (say the engine failed), so render again rather than resume into silence.
			rerenderCurrentParagraph()
			return
		}
		_isPaused.value = false
		_isSpeaking.value = true
		updatePlaybackState(true)
		playback.play()
	}

	override fun stop() {
		tts?.stop()
		playback.clear()
		stopSpeakingJob?.cancel()
		rerenderJob?.cancel()
		currentText = null
		currentAudioStale = false
		currentContentUtteranceId = null
		currentPrecacheUtteranceId = null
		precachedText = null
		_isSpeaking.value = false
		_isPaused.value = false
		updatePlaybackState(false)
		audioFocus.abandon()
	}

	/** Android's own rate and pitch are only shown, not applied: applying the slider's middle would speed up every reader who never touched it. */
	private fun loadSystemDefaultSpeech() {
		val savedRate = config.getAppString("${KEY_RATE}_$SYSTEM_DEFAULT", "").toIntOrNull()
		if (savedRate != null) {
			setSpeechRate(savedRate)
		} else {
			_currentSpeechRate.value = speechRatePercentageFor(systemSpeechSetting(Settings.Secure.TTS_DEFAULT_RATE))
		}
		val savedPitch = config.getAppString("${KEY_PITCH}_$SYSTEM_DEFAULT", "").toIntOrNull()
		if (savedPitch != null) {
			setPitch(savedPitch)
		} else {
			_currentPitch.value = pitchPercentageFor(systemSpeechSetting(Settings.Secure.TTS_DEFAULT_PITCH))
		}
	}

	/** Android stores these as a hundredth of the multiplier. */
	private fun systemSpeechSetting(name: String): Float =
		Settings.Secure.getInt(context.contentResolver, name, SYSTEM_SPEECH_SCALE) / SYSTEM_SPEECH_SCALE.toFloat()

	fun setSpeechRate(ratePercentage: Int) {
		val percentage = ratePercentage.coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE)
		_currentSpeechRate.value = percentage
		config.setAppString("${KEY_RATE}_${getCurrentEngine()}", percentage.toString())
		config.flush()
		tts?.setSpeechRate(speechRateFor(percentage))
		resynthesizePrecache()
		applyToCurrentParagraph()
	}

	fun setPitch(pitchPercentage: Int) {
		val percentage = pitchPercentage.coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE)
		_currentPitch.value = percentage
		config.setAppString("${KEY_PITCH}_${getCurrentEngine()}", percentage.toString())
		config.flush()
		tts?.setPitch(pitchFor(percentage))
		resynthesizePrecache()
		applyToCurrentParagraph()
	}

	private fun applyToCurrentParagraph() {
		if (currentText == null) return
		if (_isPaused.value) {
			currentAudioStale = true
			return
		}
		if (!_isSpeaking.value) return
		rerenderJob?.cancel()
		// A slider swipe sends a run of changes; waiting for them to stop avoids a stutter of restarts.
		rerenderJob = ttsScope.launch {
			delay(RERENDER_DELAY_MS)
			rerenderCurrentParagraph()
		}
	}

	/** The new audio's length isn't known until it is made, so the resume point is scaled from the old by the change in rate. */
	private fun rerenderCurrentParagraph() {
		val text = currentText ?: return
		val played = playback.playedMsOfCurrent() ?: 0
		val skip = (played * currentRate / speechRateFor(_currentSpeechRate.value)).toLong()
		val next = precachedText
		speakFrom(text, isSample = false, skipMs = skip)
		next?.let { precache(it) }
	}

	fun setParagraphPause(ms: Int) {
		val pause = paragraphPauseFor(ms)
		if (pause == _paragraphPauseMs.value) return
		_paragraphPauseMs.value = pause
		config.setAppString(KEY_PARAGRAPH_PAUSE, pause.toString())
		config.flush()
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
		// Unbound rather than stopped: Media3 decides when the service can safely go.
		serviceConnection?.let { context.unbindService(it) }
		serviceConnection = null
		playback.close()
		// Last, or a late callback from something torn down above could launch on this scope after shutdown.
		ttsScope.cancel()
	}
}

/** Stored values are clamped to these, since a hand-edited or older config can hold anything. */
internal const val MIN_SPEECH_PERCENTAGE = 0
internal const val MAX_SPEECH_PERCENTAGE = 100

/** 0.1x to 3.0x: slow enough to follow an unfamiliar word, fast enough for a practised listener. */
internal fun speechRateFor(percentage: Int): Float =
	0.1f + (percentage.coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE) / 100f) * 2.9f

/** Only up to 2.0x: past that a voice stops being understandable. */
internal fun pitchFor(percentage: Int): Float =
	0.1f + (percentage.coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE) / 100f) * 1.9f

internal fun speechRatePercentageFor(rate: Float): Int =
	((rate - 0.1f) / 2.9f * 100f).roundToInt().coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE)

internal fun pitchPercentageFor(pitch: Float): Int =
	((pitch - 0.1f) / 1.9f * 100f).roundToInt().coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE)
