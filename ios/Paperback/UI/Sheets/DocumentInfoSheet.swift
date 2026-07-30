import SwiftUI

struct DocumentInfoSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
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
						// TRANSLATORS: Row label for the document's file name/path in the Document Info sheet
						infoRow(t("File"), value: viewModel.activeTab?.url.lastPathComponent ?? "—")
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
					} else {
						infoRow(t("Title"), value: viewModel.activeTab?.title ?? "—")
						Divider().padding(.leading, 16)
						infoRow(t("File"), value: viewModel.activeTab?.url.lastPathComponent ?? "—")
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
		.sheetAccessibilityFocus(title: "Document Info")
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
