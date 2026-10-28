import SwiftUI

struct DocumentInfoSheet: View {
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		VStack(spacing: 0) {
			// TRANSLATORS: Title of the sheet showing metadata and statistics about the current document
			Text(t("Document Info"))
				.font(.headline)
				.padding(.vertical, 16)
				.accessibilityAddTraits(.isHeader)
			Divider()
			ScrollView {
				VStack(spacing: 0) {
					if let session = viewModel.activeSession {
						let title = session.title().isEmpty
							? (viewModel.activeTab?.title ?? "—")
							: session.title()
						let author = session.author()
						let stats = session.getStatsFfi()
						// TRANSLATORS: Row label for the document's title in the Document Info sheet
						infoRow(t("Title"), value: title)
						if !author.isEmpty {
							Divider().padding(.leading, 16)
							// TRANSLATORS: Row label for the document's author in the Document Info sheet
							infoRow(t("Author"), value: author)
						}
						Divider().padding(.leading, 16)
						// TRANSLATORS: Label for the document's file name in the Document Information dialog
						infoRow(t("File Name"), value: viewModel.activeTab?.url.lastPathComponent ?? "—")
						// Text mode has no reading bar to show this on, so it is here for either mode.
						if let percent = viewModel.reading.progressPercent() {
							Divider().padding(.leading, 16)
							// TRANSLATORS: Row label in the Document Info sheet for how far through the document the reader has got
							infoRow(t("Progress"), value: t("{}% through").replacingOccurrences(of: "{}", with: "\(percent)"))
						}
						Divider().padding(.leading, 16)
						// TRANSLATORS: Row label for the document's total word count
						infoRow(t("Words"), value: stats.wordCount.formatted())
						Divider().padding(.leading, 16)
						// TRANSLATORS: Row label for the document's total line count
						infoRow(t("Lines"), value: stats.lineCount.formatted())
						Divider().padding(.leading, 16)
						// TRANSLATORS: Row label for the document's total character count (including spaces)
						infoRow(t("Characters"), value: stats.charCount.formatted())
						Divider().padding(.leading, 16)
						// TRANSLATORS: Row label for the document's character count with whitespace excluded
						infoRow(t("Characters (excluding spaces)"), value: stats.charCountNoWhitespace.formatted())
						if stats.audioFileCount > 0 {
							Divider().padding(.leading, 16)
							// TRANSLATORS: Row label for the number of audio files in the document
							infoRow(t("Number of files"), value: stats.audioFileCount.formatted())
							Divider().padding(.leading, 16)
							// TRANSLATORS: Row label for the total playback duration across all of the document's audio files
							infoRow(t("Total duration"), value: formatDuration(stats.audioTotalDurationMs))
							Divider().padding(.leading, 16)
							// TRANSLATORS: Row label for the average playback duration per audio file in the document
							infoRow(t("Average duration"), value: formatDuration(stats.audioTotalDurationMs / stats.audioFileCount))
						}
					} else {
						// TRANSLATORS: Row label for the document's title in the Document Info sheet, shown when no document is open
						infoRow(t("Title"), value: viewModel.activeTab?.title ?? "—")
						Divider().padding(.leading, 16)
						// TRANSLATORS: Label for the document's file name in the Document Information dialog
						infoRow(t("File Name"), value: viewModel.activeTab?.url.lastPathComponent ?? "—")
					}
				}
				.background(Color(.secondarySystemBackground))
				.clipShape(RoundedRectangle(cornerRadius: 10))
				.padding()
			}
			Divider()
			// TRANSLATORS: Button that dismisses the Document Info sheet
			Button(t("OK")) { dismiss() }
				.padding()
		}
		.presentationDetents([.medium, .large])
		.sheetAccessibilityFocus(title: t("Document Info"))
	}

	private func infoRow(_ label: String, value: String) -> some View {
		HStack {
			Text(label)
			Spacer()
			Text(value)
				.foregroundStyle(.secondary)
				.multilineTextAlignment(.trailing)
		}
		.padding(.horizontal, 16)
		.padding(.vertical, 12)
		.accessibilityElement(children: .combine)
		.accessibilityLabel("\(label), \(value)")
	}
}
