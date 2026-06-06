import SwiftUI

struct DocumentInfoSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		VStack(spacing: 0) {
			Text("Document Info")
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
						infoRow("Title", value: title)
						if !author.isEmpty {
							Divider().padding(.leading, 16)
							infoRow("Author", value: author)
						}
						Divider().padding(.leading, 16)
						infoRow("File", value: viewModel.activeTab?.url.lastPathComponent ?? "—")
						Divider().padding(.leading, 16)
						infoRow("Words", value: stats.wordCount.formatted())
						Divider().padding(.leading, 16)
						infoRow("Lines", value: stats.lineCount.formatted())
						Divider().padding(.leading, 16)
						infoRow("Characters", value: stats.charCount.formatted())
						Divider().padding(.leading, 16)
						infoRow("Characters (excluding spaces)", value: stats.charCountNoWhitespace.formatted())
					} else {
						infoRow("Title", value: viewModel.activeTab?.title ?? "—")
						Divider().padding(.leading, 16)
						infoRow("File", value: viewModel.activeTab?.url.lastPathComponent ?? "—")
					}
				}
				.background(Color(.secondarySystemBackground))
				.clipShape(RoundedRectangle(cornerRadius: 10))
				.padding()
			}
			Divider()
			Button("OK") { dismiss() }
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
