package dev.paperback.android.ui.state

import android.app.Application
import dev.paperback.android.t
import dev.paperback.android.tts.Narrator
import dev.paperback.android.tts.RecordedNarration
import dev.paperback.android.tts.RecordedNarrator
import dev.paperback.android.tts.SpokenNarrator
import dev.paperback.android.tts.TtsManager
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.DocumentSession
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi
import uniffi.paperback.TextSegmentFfi

private const val AUDIO_SEEK_AMOUNT_KEY = "audio_seek_amount_seconds"
private const val DEFAULT_AUDIO_SEEK_SECONDS = 10

class ReadingController(
	application: Application,
	private val config: ConfigManagerFfi,
	private val ttsManager: TtsManager,
	private val settings: ReaderSettings,
	private val search: DocumentSearch,
	private val scope: CoroutineScope,
	private val activeTab: () -> DocumentTabState?,
	private val announcer: (String) -> Unit
) {
	private val narration = RecordedNarration(application, config, scope)

	private val recordedNarrator = RecordedNarrator(narration)

	val audioSpeed: StateFlow<Float> get() = narration.speed

	fun setAudioSpeed(speed: Float) = narration.setSpeed(speed)

	private val spokenNarrator = SpokenNarrator(ttsManager) { speakCurrentSegment() }

	/** Worked out per call rather than held, so switching tabs needs no bookkeeping. */
	private val narrator: Narrator
		get() = if (activeTab()?.hasAudio == true) recordedNarrator else spokenNarrator

	private val _currentNavUnit = MutableStateFlow<NavUnit>(NavUnit.Segment(SegmentTypeFfi.PARAGRAPH))
	val currentNavUnit: StateFlow<NavUnit> = _currentNavUnit.asStateFlow()

	// So a seek that stays in the same file doesn't repeat its name every time.
	private var lastAnnouncedAudioSource: Int? = null

	private val _ttsPosition = MutableStateFlow(0L)
	val ttsPosition: StateFlow<Long> = _ttsPosition.asStateFlow()

	private val _currentSegmentText = MutableStateFlow("")
	val currentSegmentText: StateFlow<String> = _currentSegmentText.asStateFlow()

	val sleepTimer = SleepTimer(scope) { pauseTts() }

	/** Compared against the live position rather than cleared as a flag, so a move of any kind invalidates it on its own. */
	private var spokenThroughPosition: Long? = null

	init {
		ttsManager.shouldAutoAdvance = { !shouldStopAfterFindMatch() }
		ttsManager.onSegmentTransition = {
			if (!shouldStopAfterFindMatch()) {
				transitionToNextContinuousSegment()
			}
		}
		ttsManager.onUtteranceCompleted = {
			if (!shouldStopAfterFindMatch()) {
				playNextContinuousSegment()
			} else {
				// The cursor stays on the match; without this, play would find the same paragraph and speak it again forever.
				spokenThroughPosition = _ttsPosition.value
				ttsManager.stop()
			}
		}
		ttsManager.onPlayCommand = { resumeTts() }
		ttsManager.onPauseCommand = { pauseTts() }
		ttsManager.onNextCommand = { playNextSegment() }
		ttsManager.onPrevCommand = { playPrevSegment() }
		narration.onPlayingChanged = { isPlaying -> ttsManager.setExternalPlaybackState(isPlaying) }
		narration.onSeekLanded = { elapsedMs -> announceAudioSeek(elapsedMs) }
		narration.onDetached = { lastAnnouncedAudioSource = null }
		narration.onClipChanged = { position ->
			// While browsing Find, tracking playback would drag the position off the exact match and race the next Find press; RecordedNarration still saves the audio position on every clip change.
			if (_currentNavUnit.value !is NavUnit.Find) {
				_ttsPosition.value = position
				refreshSegmentPreview()
				saveTtsPositionToConfig(position)
			}
		}
	}

	/** Doesn't save or move playback: a document just made active carries its own saved position. */
	internal fun setPosition(position: Long) {
		_ttsPosition.value = position
	}

	internal fun clearSegment() {
		_currentSegmentText.value = ""
	}

	internal fun follow(tab: DocumentTabState?) {
		if (tab != null) {
			ttsManager.currentDocumentTitle = tab.title.ifBlank { tab.fileName }
			ttsManager.currentDocumentAuthor = tab.author.ifBlank { "Unknown Author" }
			narration.attach(tab)
		} else {
			ttsManager.currentDocumentTitle = "Paperback"
			ttsManager.currentDocumentAuthor = "Unknown"
			narration.detach()
		}
	}

	internal fun shutdown() {
		narration.detach()
		narration.shutdown()
		ttsManager.shutdown()
	}

	fun savePosition(session: DocumentSession, documentUri: String, scrollIndex: Int) {
		val position = session.positionFromLine((scrollIndex + 1).toLong())
		_ttsPosition.value = position
		scope.launch(Dispatchers.IO) {
			config.setDocumentPosition(documentUri, position)
			config.flush()
		}
		if (!narration.isNarrating(documentUri) && ttsManager.isPaused.value) {
			ttsManager.stop()
		}
	}

	fun setNavUnit(unit: NavUnit) {
		_currentNavUnit.value = unit
		// Global rather than per-document, and shared with desktop so the two agree about what "forward" means.
		if (unit is NavUnit.Time) {
			scope.launch(Dispatchers.IO) {
				config.setAppInt(AUDIO_SEEK_AMOUNT_KEY, unit.seconds)
				config.flush()
			}
		}
	}

	/** An audio-only book's text only anchors the audio, so it gets seek amounts plus Section (one per audio file). */
	fun navUnitsFor(tab: DocumentTabState): List<NavUnit> {
		val segments = tab.session.getSupportedSegmentTypesFfi().map { NavUnit.Segment(it) }
		if (!tab.hasAudio) return segments
		val times = AUDIO_SEEK_AMOUNTS_SECONDS.map { NavUnit.Time(it) }
		if (tab.isAudioOnly) {
			return times + segments.filter { it.type == SegmentTypeFfi.SECTION }
		}
		return times + segments
	}

	fun ensureNavUnitSupported(units: List<NavUnit>) {
		if (units.isEmpty() || units.contains(_currentNavUnit.value)) return
		val savedSeconds = config.getAppInt(AUDIO_SEEK_AMOUNT_KEY, DEFAULT_AUDIO_SEEK_SECONDS)
		_currentNavUnit.value = units.firstOrNull { it == NavUnit.Time(savedSeconds) } ?: units.first()
	}

	private fun navSegmentType(): SegmentTypeFfi =
		(_currentNavUnit.value as? NavUnit.Segment)?.type ?: SegmentTypeFfi.PARAGRAPH

	private fun shouldStopAfterFindMatch(): Boolean =
		_currentNavUnit.value is NavUnit.Find && settings.stopAfterFindMatch.state.value

	private fun seekAudioByNavUnit(forward: Boolean): Boolean {
		val unit = _currentNavUnit.value
		if (unit !is NavUnit.Time || activeTab()?.hasAudio != true) return false
		val deltaMs = unit.seconds * 1000L
		narration.seekRelativeMs(if (forward) deltaMs else -deltaMs)
		return true
	}

	/** True whenever Find is the unit, even with no query or no more matches, since prev/next have nothing else to fall back to. */
	private fun navigateByFind(forward: Boolean, speak: Boolean, announce: Boolean): Boolean {
		if (_currentNavUnit.value !is NavUnit.Find) return false
		val query = search.query.value
		val options = search.options.value
		val tab = activeTab() ?: return true
		if (query == null || options == null) return true
		// Forward search includes the start position and would re-find the current match; backward search already excludes it.
		val searchPos = if (forward) _ttsPosition.value + 1L else _ttsPosition.value
		val res = tab.session.searchFfi(query, searchPos, options.copy(forward = forward))
		if (!res.found) {
			// TRANSLATORS: Announced when stepping to the next/previous Find match runs off the end of the document
			announcer(t("No more matches."))
			return true
		}
		_ttsPosition.value = res.position
		val segment = tab.session.getTextSegment(res.position, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		val text = displayTextFor(tab, segment)
		_currentSegmentText.value = text
		saveTtsPositionToConfig(res.position)
		if (tab.hasAudio) {
			narration.seekToPosition(res.position)
			if (speak) {
				narration.play()
			} else if (announce) {
				announceNavigationCue(text)
			}
		} else if (speak) {
			ttsManager.stop()
			ttsManager.speak(text)
			precacheNextContinuousSegment()
		} else if (announce) {
			if (ttsManager.isPaused.value) {
				ttsManager.stop()
			}
			announceNavigationCue(text)
		}
		return true
	}

	/** An audio-only book's clips carry placeholder durations, so it has no book-wide elapsed time and reads as an offset into the current file. */
	private fun announceAudioSeek(elapsedMs: Long) {
		val tab = activeTab() ?: return
		val cursor = tab.session.audioCursorAtElapsedFfi(elapsedMs)
		if (!cursor.found) return
		val clip = tab.session.audioClipFfi(cursor.clipIndex)
		if (!clip.found) return
		val time = formatDuration(if (tab.isAudioOnly) cursor.seekMs else elapsedMs)
		val fileChanged = lastAnnouncedAudioSource != clip.source
		lastAnnouncedAudioSource = clip.source
		val sectionTitle = sectionTitleAt(tab.toc, clip.start)
		announcer(if (fileChanged && sectionTitle.isNotBlank()) "$sectionTitle, $time" else time)
	}

	fun togglePlayPause() {
		if (narrator.isPlaying()) narrator.pause() else narrator.play()
	}

	private fun saveTtsPositionToConfig(pos: Long) {
		val docUri = activeTab()?.documentUri ?: return
		scope.launch(Dispatchers.IO) {
			config.setDocumentPosition(docUri, pos)
			config.flush()
		}
	}

	fun refreshSegmentPreview() {
		val tab = activeTab() ?: return
		// An audio-only buffer has no newlines, so its "paragraph" is the whole buffer and would pin the label to the first file's name.
		if (tab.isAudioOnly) {
			_currentSegmentText.value = sectionTitleAt(tab.toc, _ttsPosition.value)
			return
		}
		val current = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		_currentSegmentText.value = displayTextFor(tab, current).ifBlank {
			val next = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT)
			displayTextFor(tab, next)
		}
	}

	private fun speakCurrentSegment() {
		// Already heard this one through, so play means carry on rather than repeat it.
		if (spokenThroughPosition == _ttsPosition.value) {
			playNextContinuousSegment()
			return
		}
		val tab = activeTab() ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		if (segment.text.isNotBlank()) {
			// Keep the exact match position so Find Next cannot land on that match again.
			if (_currentNavUnit.value !is NavUnit.Find) _ttsPosition.value = segment.startPos
			_currentSegmentText.value = segment.text
			saveTtsPositionToConfig(_ttsPosition.value)
			ttsManager.stop()
			ttsManager.speak(segment.text)
			precacheNextContinuousSegment()
		} else {
			playNextSegment()
		}
	}

	private fun navigateNarrationToSegment(
		segment: TextSegmentFfi,
		announceText: String,
		speak: Boolean,
		announce: Boolean
	) {
		narration.seekToPosition(segment.startPos)
		if (speak) {
			narration.play()
		}
		// A section jump names the file it lands in, always and in full: with playback resuming there is otherwise no cue that anything moved.
		val bySection = (_currentNavUnit.value as? NavUnit.Segment)?.type == SegmentTypeFfi.SECTION
		if (bySection && announceText.isNotBlank()) {
			announcer(announceText)
		} else if (!speak && announce) {
			announceNavigationCue(announceText)
		}
	}

	/** A plain-audio DAISY section's text is just a placeholder space, hence the TOC title fallback. */
	private fun displayTextFor(tab: DocumentTabState, segment: TextSegmentFfi): String {
		if (segment.text.isNotBlank()) return segment.text
		return sectionTitleAt(tab.toc, segment.startPos)
	}

	/** Bypasses updateTtsPosition/speakCurrentSegment, which snap to the enclosing paragraph's start and would move a Find jump off its match. */
	fun jumpToFoundPosition(pos: Long, resume: Boolean) {
		val tab = activeTab() ?: return
		_ttsPosition.value = pos
		val segment = tab.session.getTextSegment(pos, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		val text = displayTextFor(tab, segment)
		_currentSegmentText.value = text
		saveTtsPositionToConfig(pos)
		if (tab.hasAudio) {
			narration.seekToPosition(pos)
			if (resume) {
				narration.play()
			}
			return
		}
		if (resume) {
			ttsManager.stop()
			ttsManager.speak(text)
			precacheNextContinuousSegment()
		} else if (ttsManager.isPaused.value) {
			// Otherwise a later resume would play the old paragraph's paused audio instead of the new position.
			ttsManager.stop()
		}
	}

	fun playNextSegment(speak: Boolean = true, announce: Boolean = false) {
		if (navigateByFind(forward = true, speak = speak, announce = announce)) return
		if (seekAudioByNavUnit(forward = true)) return
		val tab = activeTab() ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, navSegmentType(), SegmentDirectionFfi.NEXT)
		if (segment.found) {
			val text = displayTextFor(tab, segment)
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = text
			saveTtsPositionToConfig(segment.startPos)
			if (tab.hasAudio) {
				navigateNarrationToSegment(segment, text, speak, announce)
				return
			}
			if (speak) {
				ttsManager.speak(text)
				precacheNextContinuousSegment()
			} else {
				if (ttsManager.isPaused.value) {
					ttsManager.stop()
				}
				if (announce) {
					announceNavigationCue(text)
				}
			}
		}
	}

	fun announceFoundMatch(pos: Long) {
		val tab = activeTab() ?: return
		val segment = tab.session.getTextSegment(pos, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		announceNavigationCue(displayTextFor(tab, segment))
	}

	private fun announceNavigationCue(text: String) {
		val cue = text.trim().split(WHITESPACE_REGEX).take(5).joinToString(" ")
		announcer(cue)
	}

	private fun playNextContinuousSegment() {
		val tab = activeTab() ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT)
		if (segment.text.isNotBlank()) {
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = segment.text
			saveTtsPositionToConfig(segment.startPos)
			ttsManager.speak(segment.text)
			precacheNextContinuousSegment()
		}
	}

	private fun transitionToNextContinuousSegment() {
		val tab = activeTab() ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT)
		if (segment.text.isNotBlank()) {
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = segment.text
			saveTtsPositionToConfig(segment.startPos)
			precacheNextContinuousSegment()
		}
	}

	private fun precacheNextContinuousSegment() {
		if (shouldStopAfterFindMatch()) return
		val tab = activeTab() ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT)
		if (segment.text.isNotBlank()) {
			ttsManager.precache(segment.text)
		}
	}

	fun playPrevSegment(speak: Boolean = true, announce: Boolean = false) {
		if (navigateByFind(forward = false, speak = speak, announce = announce)) return
		if (seekAudioByNavUnit(forward = false)) return
		val tab = activeTab() ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, navSegmentType(), SegmentDirectionFfi.PREVIOUS)
		if (segment.found) {
			val text = displayTextFor(tab, segment)
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = text
			saveTtsPositionToConfig(segment.startPos)
			if (tab.hasAudio) {
				navigateNarrationToSegment(segment, text, speak, announce)
				return
			}
			if (speak) {
				ttsManager.speak(text)
				precacheNextContinuousSegment()
			} else {
				if (ttsManager.isPaused.value) {
					ttsManager.stop()
				}
				if (announce) {
					announceNavigationCue(text)
				}
			}
		}
	}

	fun pauseTts() {
		narrator.pause()
	}

	fun navigateByType(type: SegmentTypeFfi, direction: SegmentDirectionFfi) {
		val tab = activeTab() ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, type, direction)
		if (segment.found) {
			val text = displayTextFor(tab, segment)
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = text
			saveTtsPositionToConfig(segment.startPos)
			if (tab.hasAudio) {
				navigateNarrationToSegment(segment, text, speak = true, announce = false)
				return
			}
			ttsManager.stop()
			ttsManager.speak(text)
			precacheNextContinuousSegment()
		}
	}

	fun resumeTts() {
		narrator.play()
	}

	/** An audio-only book is measured through its recording, since its text is one placeholder line per file and would weigh a two minute file the same as an hour long one. */
	fun progressPercent(): Int? {
		val tab = activeTab() ?: return null
		val session = tab.session
		val position = _ttsPosition.value
		if (!tab.isAudioOnly) return session.getStatusInfo(position).percentage
		val point = session.audioPointForPositionFfi(position)
		if (!point.found) return null
		val percent = session.audioProgressPercentFfi(point.timeMs)
		return if (percent >= 0) percent else null
	}

	fun updateTtsPosition(pos: Long) {
		_ttsPosition.value = pos
		refreshSegmentPreview()
		saveTtsPositionToConfig(pos)
		narrator.moveTo(pos)
	}

	/** False when file lengths aren't all known, so the caller maps through the text instead; the reading position follows from the clip the player reports. */
	fun seekAudioToPercent(percent: Int): Boolean {
		val tab = activeTab() ?: return false
		val targetMs = tab.session.audioElapsedForPercentFfi(percent)
		if (targetMs < 0) return false
		return narration.seekToMs(targetMs)
	}

	private companion object {
		val WHITESPACE_REGEX = "\\s+".toRegex()
	}
}
