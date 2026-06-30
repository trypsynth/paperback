import SwiftUI

struct DocumentInfoSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		VStack(spacing: 0) {
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
						infoRow(t("Title"), value: title)
						if !author.isEmpty {
							Divider().padding(.leading, 16)
							infoRow(t("Author"), value: author)
						}
						Divider().padding(.leading, 16)
						infoRow(t("File"), value: viewModel.activeTab?.url.lastPathComponent ?? "—")
						Divider().padding(.leading, 16)
						infoRow(t("Words"), value: stats.wordCount.formatted())
						Divider().padding(.leading, 16)
						infoRow(t("Lines"), value: stats.lineCount.formatted())
						Divider().padding(.leading, 16)
						infoRow(t("Characters"), value: stats.charCount.formatted())
						Divider().padding(.leading, 16)
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
