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

	// Which paragraphs the playback queue holds. A stale callback for a request that has been replaced names neither, and is ignored.
	private var currentContentUtteranceId: String? = null
	private var currentPrecacheUtteranceId: String? = null

	// Kept so a rate or pitch change can render the paragraph being read again, since its audio was made at the old settings.
	private var currentText: String? = null

	// The rate each queued paragraph was made at, so carrying on after a rate change can scale how far in the listener was.
	private var currentRate = 1f
	private var precacheRate = 1f

	// Set while paused after a rate or pitch change, so resuming renders the paragraph again instead of playing the old audio (#1067).
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

		/** Where synthesized audio is written once it has already been handed over as it was made. */
		private val DISCARDED_AUDIO = File("/dev/null")
	}

	private val _currentEngineName = MutableStateFlow<String?>(null)
	val currentEngineName: StateFlow<String?> = _currentEngineName.asStateFlow()

	private val _isInitialized = MutableStateFlow(false)
	val isInitialized: StateFlow<Boolean> = _isInitialized.asStateFlow()

	/** Text asked for before the engine finished starting, spoken once it has.
	 *
	 * An engine reports itself ready through `onInit`, and every call made before that fails
	 * and returns ERROR. Engines differ by a lot in how long they take: a device's built-in one
	 * is usually up before the reader can press play, while a third-party engine that loads
	 * dictionaries of its own may not be, so the press landed on nothing at all. Only the most
	 * recent request is kept; an earlier one is no longer what the reader is waiting for. */
	private var pendingSpeak: Pair<String, Boolean>? = null

	private val ttsScope = CoroutineScope(Dispatchers.Main)
	private var stopSpeakingJob: Job? = null

	private val _isSpeaking = MutableStateFlow(false)
	override val isSpeaking: StateFlow<Boolean> = _isSpeaking.asStateFlow()

	private val _isPaused = MutableStateFlow(false)
	override val isPaused: StateFlow<Boolean> = _isPaused.asStateFlow()

	var onUtteranceCompleted: (() -> Unit)? = null
	var onSegmentTransition: (() -> Unit)? = null

	// Consulted when linking and consuming queued audio, so a changed Find preference
	// takes effect at the next paragraph boundary as well as on the next play press.
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
		// Binding (rather than Context.startForegroundService()) keeps PlaybackService
		// alive and its Media3 internals already observing the player before playback
		// ever begins. Starting it with a plain Intent instead races the 5-second
		// startForeground() deadline against Media3's async notification/session wiring
		// and intermittently crashes with ForegroundServiceDidNotStartInTimeException —
		// see https://github.com/androidx/media/issues/167, where the Media3 maintainers
		// confirm a bound controller/client is the supported way to avoid it.
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

	/**
	 * Throws away the paragraph synthesized ahead and synthesizes it again at the current settings.
	 *
	 * The next paragraph is made while the current one is still talking, so without this a rate or pitch change is not heard until that already-made paragraph has played, one paragraph later than expected.
	 */
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

	/**
	 * Has the engine make [text], with its audio streamed to [playback] through the progress listener.
	 *
	 * The engine only reports audio to the app while making it for a file or for its own playback, and its own playback cannot be paused part-way through. So the file is one that keeps nothing.
	 */
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
			// Nothing will ever speak through this engine. Say so rather than leaving the
			// reader pressing play against an engine that never answers.
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

	/** A paragraph has been heard to its end, pause after it included. */
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

	/** The engine gave up on a paragraph. One made ahead is dropped, so reaching it asks for it afresh rather than skipping it. */
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
		// Every call before the engine reports itself ready fails and returns ERROR, so hold the text instead of handing it over to be dropped.
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

	/** Reflects a non-TTS playback engine's (e.g. DaisyAudioPlayer) play/pause state into
	 * isSpeaking/isPaused and the MediaSession, so the UI, notification, and lock-screen
	 * controls behave the same regardless of which engine is narrating. */
	fun setExternalPlaybackState(isPlaying: Boolean) {
		_isSpeaking.value = isPlaying
		_isPaused.value = !isPlaying
		updatePlaybackState(isPlaying)
	}

	override fun resume() {
		if (!_isPaused.value) return
		val text = currentText
		if (currentAudioStale || (text != null && playback.isIdle())) {
			// Nothing left to carry on from, such as after the engine failed, so the paragraph is made again rather than resuming into silence.
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

	/**
	 * Takes up the system default engine's rate and pitch: the reader's own, once they have set one here, and otherwise Android's.
	 *
	 * Android's are only shown, not applied, since leaving them alone is what follows them. Applying the slider's default instead would change the speed of every reader who never touched it, because the middle of the slider is faster than Android's normal rate.
	 */
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

	/** One of Android's own speech settings, which it stores as a hundredth of the multiplier. */
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

	/** Makes a rate or pitch change heard in the paragraph being read, not only from the next one: straight away while playing, and on resume while paused. */
	private fun applyToCurrentParagraph() {
		if (currentText == null) return
		if (_isPaused.value) {
			currentAudioStale = true
			return
		}
		if (!_isSpeaking.value) return
		rerenderJob?.cancel()
		// A slider swipe or drag sends a run of changes; rendering the paragraph once they stop avoids a stutter of restarts.
		rerenderJob = ttsScope.launch {
			delay(RERENDER_DELAY_MS)
			rerenderCurrentParagraph()
		}
	}

	/**
	 * Makes the paragraph again at the current settings and carries on from about where the old audio was.
	 *
	 * The new audio is not known in full until it is made, so how far in to start is scaled from the old by the change in rate, which is what sets how long the same words take.
	 */
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
		// Unbind rather than force-stopping the service — Media3's own lifecycle
		// handling decides when it's actually safe for the service to go away.
		serviceConnection?.let { context.unbindService(it) }
		serviceConnection = null
		playback.close()
		// Last, so nothing torn down above can leave work queued: a late callback would otherwise launch on this scope after shutdown.
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

/** The slider percentage nearest an engine speech rate, the inverse of [speechRateFor]. */
internal fun speechRatePercentageFor(rate: Float): Int =
	((rate - 0.1f) / 2.9f * 100f).roundToInt().coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE)

/** The slider percentage nearest an engine pitch, the inverse of [pitchFor]. */
internal fun pitchPercentageFor(pitch: Float): Int =
	((pitch - 0.1f) / 1.9f * 100f).roundToInt().coerceIn(MIN_SPEECH_PERCENTAGE, MAX_SPEECH_PERCENTAGE)
