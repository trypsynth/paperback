import Foundation

/// Worded here rather than by the core, whose own translations aren't loaded on iOS: "Chapter 3, 42%, 1,850 words left", or "Chapter 3, 4:10 of 12:00, 7:50 left" where the chapter is measured by its recording.
func chapterProgressText(_ progress: ChapterProgressFfi) -> String {
	var parts: [String] = []
	let title = progress.title.trimmingCharacters(in: .whitespacesAndNewlines)
	if !title.isEmpty { parts.append(title) }
	if progress.wordsLeft >= 0 {
		parts.append("\(progress.percent)%")
		// TRANSLATORS: Shown and announced with how far through a book's chapter the reader is: the number of words left in it; {} is the number. Which of the three forms is used depends on the language's plural rule, and the "many" form's trailing character isn't a typo.
		let words = nt(
			t("{} word left"),
			t("{} words left"),
			t("{} words left⁣"),
			Int(progress.wordsLeft)
		)
		parts.append(words.replacingOccurrences(of: "{}", with: progress.wordsLeft.formatted()))
	} else {
		// TRANSLATORS: Announced for how far into an audiobook the reader is: the first {} is the time played so far and the second how long it runs, both written like 1:02:03
		let played = t("{} of {}")
		if let first = played.range(of: "{}") {
			let withElapsed = played.replacingCharacters(in: first, with: formatDuration(progress.elapsedMs))
			parts.append(withElapsed.replacingOccurrences(of: "{}", with: formatDuration(progress.totalMs)))
		}
		// TRANSLATORS: Announced with how far into an audiobook's chapter the reader is: the time left in it; {} is written like 12:05
		parts.append(t("{} left").replacingOccurrences(of: "{}", with: formatDuration(progress.totalMs - progress.elapsedMs)))
	}
	return parts.joined(separator: ", ")
}
