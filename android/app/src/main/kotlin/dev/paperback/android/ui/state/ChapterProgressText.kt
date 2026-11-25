package dev.paperback.android.ui.state

import dev.paperback.android.nt
import dev.paperback.android.t
import uniffi.paperback.ChapterProgressFfi
import java.text.NumberFormat

/** Worded here rather than by the core, whose own translations aren't loaded on Android: "Chapter 3, 42%, 1,850 words left", or "Chapter 3, 4:10 of 12:00, 7:50 left" where the chapter is measured by its recording. */
fun chapterProgressText(progress: ChapterProgressFfi): String {
	val parts = mutableListOf<String>()
	if (progress.title.isNotBlank()) parts.add(progress.title)
	if (progress.wordsLeft >= 0) {
		parts.add("${progress.percent}%")
		// TRANSLATORS: Shown and announced with how far through a book's chapter the reader is: the number of words left in it; {} is
		// replaced with the number. Which of the three forms is used depends on the target language's own plural rule. The "many"
		// form's trailing character isn't a typo — see PLURAL_MANY_⁣ER in Translations.kt.
		val words = nt(t("{} word left"), t("{} words left"), t("{} words left⁣"), progress.wordsLeft)
		parts.add(words.replaceFirst("{}", NumberFormat.getIntegerInstance().format(progress.wordsLeft)))
	} else {
		// TRANSLATORS: Announced for how far into an audiobook the reader is: the first {} is the time played so far and the second how long it runs, both written like 1:02:03
		parts.add(t("{} of {}", formatDuration(progress.elapsedMs), formatDuration(progress.totalMs)))
		// TRANSLATORS: Announced with how far into an audiobook's chapter the reader is: the time left in it; {} is written like 12:05
		parts.add(t("{} left", formatDuration(progress.totalMs - progress.elapsedMs)))
	}
	return parts.joinToString(", ")
}
