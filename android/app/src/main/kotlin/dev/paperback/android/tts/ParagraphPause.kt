package dev.paperback.android.tts

/** The ends and step of the pause setting, matching iOS. */
internal const val MIN_PARAGRAPH_PAUSE_MS = 0
internal const val MAX_PARAGRAPH_PAUSE_MS = 1000
internal const val PARAGRAPH_PAUSE_STEP_MS = 10

/** A stored pause brought inside the setting's range and onto its steps. */
internal fun paragraphPauseFor(ms: Int): Int {
	val clamped = ms.coerceIn(MIN_PARAGRAPH_PAUSE_MS, MAX_PARAGRAPH_PAUSE_MS)
	return (clamped + PARAGRAPH_PAUSE_STEP_MS / 2) / PARAGRAPH_PAUSE_STEP_MS * PARAGRAPH_PAUSE_STEP_MS
}
