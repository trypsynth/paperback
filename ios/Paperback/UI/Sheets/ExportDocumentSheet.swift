import SwiftUI

extension ExportFormat {
	var fileExtension: String {
		switch self {
		case .text: return "txt"
		case .html: return "html"
		case .markdown: return "md"
		}
	}

	var menuLabel: String {
		switch self {
		// TRANSLATORS: Export format option to save the document as a plain text file
		case .text: return t("Plain Text (.txt)")
		// TRANSLATORS: Export format option to save the document as an HTML file
		case .html: return t("HTML (.html)")
		// TRANSLATORS: Export format option to save the document as a Markdown file
		case .markdown: return t("Markdown (.md)")
		}
	}

	var systemImage: String {
		switch self {
		case .text: return "doc.text"
		case .html: return "chevron.left.forwardslash.chevron.right"
		case .markdown: return "text.alignleft"
		}
	}
}

/// Picks a format and hands the rendered document to the system file mover. Which formats
/// appear comes from the core, since what a document can be rendered as depends on the
/// document.
struct ExportDocumentSheet: View {
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss
	@State private var exportURL: URL?
	@State private var failed = false

	var body: some View {
		List {
			Section {
				ForEach(viewModel.supportedExportFormats, id: \.self) { format in
					Button { export(format) } label: {
						Label(format.menuLabel, systemImage: format.systemImage)
					}
				}
			} header: {
				// TRANSLATORS: Instruction text above the list of export format options
				Text(t("Select a format to export the current document:"))
			}
		}
		// TRANSLATORS: Navigation title of the screen for exporting the current document to another file format
		.navigationTitle(t("Export Document"))
		.navigationBarTitleDisplayMode(.inline)
		.fileMover(
			isPresented: Binding(get: { exportURL != nil }, set: { if !$0 { exportURL = nil } }),
			file: exportURL
		) { result in
			exportURL = nil
			if case .failure = result {
				failed = true
			} else {
				dismiss()
			}
		}
		// TRANSLATORS: Title of the alert shown when exporting the document to another file format fails
		.alert(t("Export Document"), isPresented: $failed) {
			// TRANSLATORS: OK button dismissing the failed document export alert
			Button(t("OK")) { failed = false }
		} message: {
			// TRANSLATORS: Message shown when exporting the document to another file format fails
			Text(t("Failed to export document"))
		}
		.sheetAccessibilityFocus(title: "Export Document")
	}

	private func export(_ format: ExportFormat) {
		guard let url = viewModel.exportActiveDocument(as: format) else {
			failed = true
			return
		}
		exportURL = url
	}
}
