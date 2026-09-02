package dev.paperback.android.ui

import dev.paperback.android.nt
import dev.paperback.android.t
import uniffi.paperback.SegmentTypeFfi

/**
 * Seek amounts offered as navigation units for documents with recorded audio, in seconds.
 * Mirrors desktop's `dialogs::AUDIO_SEEK_AMOUNTS_SECONDS` so the shared
 * `audio_seek_amount_seconds` setting means the same thing on both platforms.
 */
val AUDIO_SEEK_AMOUNTS_SECONDS = listOf(5, 10, 30, 60, 120, 300, 600, 1800, 3600)

/**
 * What the previous/next controls move by. Text documents step through structural units
 * (paragraph, heading, and friends); recorded audio steps through elapsed time instead, since
 * an audiobook that is just a bundle of narration files has no text spine to walk.
 */
sealed interface NavUnit {
	data class Segment(
		val type: SegmentTypeFfi
	) : NavUnit

	data class Time(
		val seconds: Int
	) : NavUnit

	/** Steps through matches of the active Find query instead of a structural unit or elapsed
	 * time. Only offered while a search is active; see `MainScreen`'s nav-unit list building. */
	data object Find : NavUnit
}

fun getSegmentTypeName(type: SegmentTypeFfi): String =
	when (type) {
		// TRANSLATORS: Name of the "paragraph" reading/navigation unit
		SegmentTypeFfi.PARAGRAPH -> t("Paragraph")
		// TRANSLATORS: Name of the "line" reading/navigation unit
		SegmentTypeFfi.LINE -> t("Line")
		// TRANSLATORS: Name of the "heading" reading/navigation unit
		SegmentTypeFfi.HEADING -> t("Heading")
		// TRANSLATORS: Name of the "link" reading/navigation unit
		SegmentTypeFfi.LINK -> t("Link")
		// TRANSLATORS: Name of the "section" reading/navigation unit
		SegmentTypeFfi.SECTION -> t("Section")
		// TRANSLATORS: Name of the "page" reading/navigation unit
		SegmentTypeFfi.PAGE -> t("Page")
		// TRANSLATORS: Name of the "list" reading/navigation unit
		SegmentTypeFfi.LIST -> t("List")
		// TRANSLATORS: Name of the "list item" reading/navigation unit
		SegmentTypeFfi.LIST_ITEM -> t("List Item")
		// TRANSLATORS: Name of the "table" reading/navigation unit
		SegmentTypeFfi.TABLE -> t("Table")
		// TRANSLATORS: Name of the "separator" reading/navigation unit
		SegmentTypeFfi.SEPARATOR -> t("Separator")
		// TRANSLATORS: Name of the "image" reading/navigation unit
		SegmentTypeFfi.IMAGE -> t("Image")
		// TRANSLATORS: Name of the "figure" reading/navigation unit
		SegmentTypeFfi.FIGURE -> t("Figure")
	}

/** Matches the labels desktop shows for the same presets in its Options dialog. */
fun getSeekAmountName(seconds: Int): String =
	when (seconds) {
		// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
		5 -> t("5 seconds")
		// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
		10 -> t("10 seconds")
		// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
		30 -> t("30 seconds")
		// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
		60 -> t("1 minute")
		// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
		120 -> t("2 minutes")
		// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
		300 -> t("5 minutes")
		// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
		600 -> t("10 minutes")
		// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
		1800 -> t("30 minutes")
		// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
		3600 -> t("1 hour")
		// TRANSLATORS: Fallback audio seek amount label for a value outside the fixed presets
		// above; {} is the number of seconds. Which of the three forms is used depends on the
		// target language's own plural rule, read from its catalogue (see nt() in
		// Translations.kt). The "many" form's trailing character isn't a typo — see
		// PLURAL_MANY_MARKER in Translations.kt.
		else -> nt(t("{} second"), t("{} seconds"), t("{} seconds⁣"), seconds.toLong()).replace("{}", seconds.toString())
	}

fun getNavUnitName(unit: NavUnit): String =
	when (unit) {
		is NavUnit.Segment -> getSegmentTypeName(unit.type)
		is NavUnit.Time -> getSeekAmountName(unit.seconds)
		// TRANSLATORS: Name of the "Find" navigation unit, which moves between search matches
		is NavUnit.Find -> t("Find")
	}

/** "1:23" or "1:02:03": the shortest form that still reads unambiguously. */
fun formatDuration(ms: Long): String {
	val totalSeconds = (ms.coerceAtLeast(0L) + 500L) / 1000L
	val hours = totalSeconds / 3600
	val minutes = (totalSeconds % 3600) / 60
	val seconds = totalSeconds % 60
	return if (hours > 0) {
		String.format(java.util.Locale.ROOT, "%d:%02d:%02d", hours, minutes, seconds)
	} else {
		String.format(java.util.Locale.ROOT, "%d:%02d", minutes, seconds)
	}
}
