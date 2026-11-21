import SwiftUI

struct TtsModeView: View {
	@Environment(AppViewModel.self) private var viewModel

	var body: some View {
		VStack(spacing: 16) {
			Spacer()
			ScrollView {
				Text(
					viewModel.reading.currentSegmentText.isEmpty
						// TRANSLATORS: Placeholder shown in the read-aloud view before the user has started playback
						? t("Press play to start listening.")
						: viewModel.reading.currentSegmentText
				)
				.readabilityStyle(viewModel)
				.padding(.horizontal, 24)
			}
			.frame(maxHeight: 400)
			.readingBackground(viewModel)
			if let lineText = positionLine {
				Text(lineText)
					.font(.caption)
					.foregroundStyle(.secondary)
					.lineLimit(1)
					.padding(.horizontal, 24)
					// The text above is the real content, so VoiceOver reading both would say the same words twice.
					.accessibilityHidden(true)
			}
			if let note = viewModel.reading.bookmarkHere?.note, !note.isEmpty {
				noteCard(note)
			}
			// A recording moves on between the clip changes the view is told about, so its share is read again every second while it plays.
			TimelineView(.periodic(from: .now, by: 1)) { _ in
				let reading = viewModel.reading
				let chapter = reading.showsChapterProgress ? reading.chapterProgress() : nil
				if let chapter {
					progressBlock(percent: Int(chapter.percent), lines: [chapterProgressText(chapter)], showsChapter: true)
				} else if let percent = reading.progressPercent() {
					// TRANSLATORS: How far through the document the reader has got, shown under the read-aloud progress bar; {} is a whole-number percentage
					let through = t("{}% through").replacingOccurrences(of: "{}", with: "\(percent)")
					progressBlock(percent: percent, lines: [through, reading.audioTimeText()].compactMap { $0 }, showsChapter: false)
				}
			}
			if let remaining = viewModel.reading.sleepTimerRemaining {
				Text(String(format: "Sleep timer: %d:%02d", remaining / 60, remaining % 60))
					.font(.caption)
					.foregroundStyle(.secondary)
			}
			Spacer()
		}
	}

	private func noteCard(_ note: String) -> some View {
		Button {
			viewModel.navigation.bookmarkNote = viewModel.reading.noteTargetHere()
		} label: {
			Label(note, systemImage: "note.text")
				.font(.callout)
				.lineLimit(4)
				.frame(maxWidth: .infinity, alignment: .leading)
				.padding(12)
				.background(Color.accentColor.opacity(0.12), in: RoundedRectangle(cornerRadius: 10))
		}
		.buttonStyle(.plain)
		.padding(.horizontal, 24)
		// TRANSLATORS: VoiceOver label for the note shown under the passage being read; {} is the note's text
		.accessibilityLabel(t("Note: {}").replacingOccurrences(of: "{}", with: note))
		// TRANSLATORS: VoiceOver hint on the note shown under the passage being read
		.accessibilityHint(t("Double-tap to edit."))
	}

	/// The book's progress, or the chapter's once tapped; tapping switches between the two wherever the chapter can be measured.
	@ViewBuilder
	private func progressBlock(percent: Int, lines: [String], showsChapter: Bool) -> some View {
		let block = VStack(spacing: 16) {
			// The text below says the same number, so the bar is for the eye only.
			ProgressView(value: Double(percent), total: 100)
				.accessibilityHidden(true)
			ForEach(lines, id: \.self) { line in
				Text(line)
					.font(.caption)
					.foregroundStyle(.secondary)
					.multilineTextAlignment(.center)
			}
		}
		.padding(.horizontal, 24)
		.contentShape(Rectangle())
		if viewModel.reading.canShowChapterProgress {
			Button { viewModel.reading.toggleChapterProgress() } label: { block }
				.buttonStyle(.plain)
				// TRANSLATORS: What tapping the progress under the read-aloud bar does: it switches between how far through the whole book the reader is, and how far through the current chapter with how much of it is left
				.accessibilityHint(showsChapter ? t("Show book progress") : t("Show chapter progress"))
		} else {
			block
		}
	}

	/// Nil when the spoken text already contains it, as in a one-line paragraph, so the same words don't appear twice.
	private var positionLine: String? {
		guard let session = viewModel.activeSession else { return nil }
		let line = session.getLineText(position: viewModel.reading.ttsPosition)
			.trimmingCharacters(in: .whitespacesAndNewlines)
		if line.isEmpty || viewModel.reading.currentSegmentText.contains(line) {
			return nil
		}
		return line
	}
}
