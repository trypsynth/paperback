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

/**
 * Reading the active document aloud and moving through it: where the reader is, what unit previous and next move by, and the speech or recording that follows them.
 *
 * Kept apart from the open documents themselves, which [MainScreenViewModel] owns, so the two can change without each having to be read through the other. It sees the active document only through [activeTab].
 */
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
	// Narrates DAISY audiobooks' recorded audio in place of synthesized TTS (see
	// DocumentTabState.hasAudio), re-attached to whichever tab is active.
	private val narration = RecordedNarration(application, config, scope)

	private val recordedNarrator = RecordedNarrator(narration)
	private val spokenNarrator = SpokenNarrator(ttsManager) { speakCurrentSegment() }

	/**
	 * Whatever is reading the active document aloud: its own recording for a DAISY audiobook, the
	 * synthesizer for everything else. Worked out per call rather than held, so switching tabs
	 * needs no bookkeeping of its own.
	 */
	private val narrator: Narrator
		get() = if (activeTab()?.hasAudio == true) recordedNarrator else spokenNarrator

	private val _currentNavUnit = MutableStateFlow<NavUnit>(NavUnit.Segment(SegmentTypeFfi.PARAGRAPH))
	val currentNavUnit: StateFlow<NavUnit> = _currentNavUnit.asStateFlow()

	// The source whose position was last announced after an audio seek, so a seek that stays in
	// the same file doesn't repeat its name every time.
	private var lastAnnouncedAudioSource: Int? = null

	private val _ttsPosition = MutableStateFlow(0L)
	val ttsPosition: StateFlow<Long> = _ttsPosition.asStateFlow()

	private val _currentSegmentText = MutableStateFlow("")
	val currentSegmentText: StateFlow<String> = _currentSegmentText.asStateFlow()

	val sleepTimer = SleepTimer(scope) { pauseTts() }

	/** The position whose paragraph playback already read to the end while Find was the unit.
	 * Comparing against the live position rather than clearing a flag means a move of any kind
	 * invalidates it on its own. */
	private var spokenThroughPosition: Long? = null

	init {
		// Both queued audio and completion callbacks follow the Find playback preference.
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
				// The cursor stays on the match, so remember that its paragraph was read right
				// through. Without this, play would find the same paragraph under the cursor and
				// speak it again, for ever.
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
			// Same reasoning as the TTS auto-advance callbacks above: natural playback tracking
			// would otherwise keep dragging the tracked position forward (mid-clip, off the exact
			// match) while browsing Find, racing with the next Find Previous/Next press. This
			// doesn't extend to saving the audio position, which RecordedNarration does on every
			// clip change whatever the nav unit: unlike desktop's on-close save, Android can kill
			// this process with no lifecycle callback at all, so a kill mid-Find would otherwise
			// resume from before the jump on relaunch.
			if (_currentNavUnit.value !is NavUnit.Find) {
				_ttsPosition.value = position
				refreshSegmentPreview()
				saveTtsPositionToConfig(position)
			}
		}
	}

	/** Moves the reading position without saving it or moving playback, for a document just made active, which carries its own saved position. */
	internal fun setPosition(position: Long) {
		_ttsPosition.value = position
	}

	internal fun clearSegment() {
		_currentSegmentText.value = ""
	}

	/** Points the media session and the recorded narration at [tab], or at nothing once no document is open. */
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
		// Seek amounts are a global preference rather than per-document, and share desktop's
		// setting so the two agree about what "forward" means.
		if (unit is NavUnit.Time) {
			scope.launch(Dispatchers.IO) {
				config.setAppInt(AUDIO_SEEK_AMOUNT_KEY, unit.seconds)
				config.flush()
			}
		}
	}

	/** The navigation units `tab` can offer. A document whose text spine is only there to anchor
	 * audio has nothing to step through but the recording itself, so it gets seek amounts plus
	 * Section (each underlying audio file is its own section); a DAISY book with real prose gets
	 * seek amounts and every supported segment type, seek amounts first. */
	fun navUnitsFor(tab: DocumentTabState): List<NavUnit> {
		val segments = tab.session.getSupportedSegmentTypesFfi().map { NavUnit.Segment(it) }
		if (!tab.hasAudio) return segments
		val times = AUDIO_SEEK_AMOUNTS_SECONDS.map { NavUnit.Time(it) }
		if (tab.isAudioOnly) {
			return times + segments.filter { it.type == SegmentTypeFfi.SECTION }
		}
		return times + segments
	}

	/** Falls back to a unit the newly active document actually supports, preferring the saved
	 * seek amount where seek amounts are on offer. */
	fun ensureNavUnitSupported(units: List<NavUnit>) {
		if (units.isEmpty() || units.contains(_currentNavUnit.value)) return
		val savedSeconds = config.getAppInt(AUDIO_SEEK_AMOUNT_KEY, DEFAULT_AUDIO_SEEK_SECONDS)
		_currentNavUnit.value = units.firstOrNull { it == NavUnit.Time(savedSeconds) } ?: units.first()
	}

	private fun navSegmentType(): SegmentTypeFfi =
		(_currentNavUnit.value as? NavUnit.Segment)?.type ?: SegmentTypeFfi.PARAGRAPH

	private fun shouldStopAfterFindMatch(): Boolean =
		_currentNavUnit.value is NavUnit.Find && settings.stopAfterFindMatch.state.value

	/** Handles previous/next for a document being navigated by elapsed time rather than by text
	 * unit. False when that isn't what's happening, leaving the ordinary text path to run. */
	private fun seekAudioByNavUnit(forward: Boolean): Boolean {
		val unit = _currentNavUnit.value
		if (unit !is NavUnit.Time || activeTab()?.hasAudio != true) return false
		val deltaMs = unit.seconds * 1000L
		narration.seekRelativeMs(if (forward) deltaMs else -deltaMs)
		return true
	}

	/** Handles previous/next for a document being navigated by Find match instead of by text
	 * unit or elapsed time. False when that isn't what's happening, leaving the ordinary text
	 * path to run. Always returns true once "Find" is the active unit, even with no query or no
	 * more matches, since there is nothing else for prev/next to fall back to in that case. */
	private fun navigateByFind(forward: Boolean, speak: Boolean, announce: Boolean): Boolean {
		if (_currentNavUnit.value !is NavUnit.Find) return false
		val query = search.query.value
		val options = search.options.value
		val tab = activeTab() ?: return true
		if (query == null || options == null) return true
		// Forward search is inclusive of the start position, so searching from the current
		// match's own start would just re-find it; nudge past it first. Backward search is
		// already exclusive of the start position, so it needs no such adjustment.
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

	/** Speaks where an audio seek landed. An audiobook that is a bundle of narration files has
	 * no meaningful document-wide elapsed time (its clips carry placeholder durations), so its
	 * position reads as an offset into the file now playing, named whenever the file changes. */
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
		// An audio-only book's buffer is one placeholder space per file with no newlines
		// anywhere, so asking for the paragraph enclosing a position collapses to the whole
		// buffer and reports it as starting at 0. Deriving the label from that would pin it to
		// the first file's name for the life of the book, however far playback had moved; the
		// section (that is, the file) holding the current position is the only label there is.
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

	/** Seeks the recorded narration to `segment`'s start, resumes playback there when `speak` says
	 * the reader was already going, and announces `announceText` where that is the only sign
	 * anything moved. */
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
		// Stepping by section moves between whole narration files, so name where the jump
		// landed the way a time seek names the file it crossed into: unconditionally, because
		// with playback resuming there is otherwise no cue at all that anything moved, and in
		// full rather than announceNavigationCue's five-word prefix, since here the name is the
		// whole message rather than the opening of a paragraph being previewed.
		val bySection = (_currentNavUnit.value as? NavUnit.Segment)?.type == SegmentTypeFfi.SECTION
		if (bySection && announceText.isNotBlank()) {
			announcer(announceText)
		} else if (!speak && announce) {
			announceNavigationCue(announceText)
		}
	}

	/** A segment's own text, falling back to its enclosing section's TOC title when blank — the
	 * case for a plain-audio DAISY section, whose buffer content is just a placeholder space. */
	private fun displayTextFor(tab: DocumentTabState, segment: TextSegmentFfi): String {
		if (segment.text.isNotBlank()) return segment.text
		return sectionTitleAt(tab.toc, segment.startPos)
	}

	/** Jumps straight to `pos` (a freshly found Find match) and, if `resume` says the reader was
	 * already going, speaks/plays from exactly there. Deliberately does not go through
	 * `updateTtsPosition`/`speakCurrentSegment`, which re-derive the *enclosing paragraph* of a
	 * position and snap to its start — fine for ordinary navigation, but it would silently move
	 * a Find jump off the match it just found and back to that paragraph's beginning. */
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
			// Was paused mid-utterance elsewhere; clear that stale state so a later resume
			// doesn't play the old paragraph's audio instead of the new position.
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

	/** Says where a Find match landed, for a reader whose focus is still in the Find dialog. */
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

	fun updateTtsPosition(pos: Long) {
		_ttsPosition.value = pos
		refreshSegmentPreview()
		saveTtsPositionToConfig(pos)
		narrator.moveTo(pos)
	}

	/**
	 * Seeks the recording to `percent` of its running time, reporting whether it applied.
	 *
	 * False for a document with no audio, and for one whose file lengths are not all known, so
	 * the caller maps the percentage through the text instead. A
	 * percentage through the text of an audiobook counts blank lines, one per file, so it treats
	 * a two minute file and an hour long one as equal shares of the book.
	 *
	 * The reading position follows on its own: the player reports the clip it lands in.
	 */
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
