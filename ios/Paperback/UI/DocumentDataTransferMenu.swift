import SwiftUI
import UniformTypeIdentifiers

private extension UTType {
	static let paperbackData = UTType(filenameExtension: "paperback") ?? .data
}

// Long-press affordance on the "Open Book" button to import/export a document's
// saved settings and bookmarks as a .paperback sidecar file, matching Android's
// long-press menu on its Open Book button.
struct DocumentDataTransferMenu: ViewModifier {
	@EnvironmentObject var viewModel: AppViewModel
	@State private var exportURL: URL? = nil
	@State private var showImportPicker = false
	@State private var resultMessage: String? = nil

	func body(content: Content) -> some View {
		if viewModel.activeTab != nil {
			withPickersAndFeedback(content)
				.accessibilityAction(named: "Import Document Data") { showImportPicker = true }
				.accessibilityAction(named: "Export Document Data") { beginExport() }
		} else {
			withPickersAndFeedback(content)
		}
	}

	private func withPickersAndFeedback(_ content: Content) -> some View {
		content
			.contextMenu {
				Button { beginExport() } label: {
					// TRANSLATORS: Long-press menu item to export a document's saved settings and bookmarks to a .paperback file
					Label(t("Export Document Data"), systemImage: "square.and.arrow.up")
				}
				.disabled(viewModel.activeTab == nil)
				Button { showImportPicker = true } label: {
					// TRANSLATORS: Long-press menu item to import a document's saved settings and bookmarks from a .paperback file
					Label(t("Import Document Data"), systemImage: "square.and.arrow.down")
				}
				.disabled(viewModel.activeTab == nil)
			}
			.fileMover(
				isPresented: Binding(get: { exportURL != nil }, set: { if !$0 { exportURL = nil } }),
				file: exportURL
			) { result in
				switch result {
				case .success:
					// TRANSLATORS: Confirmation shown after exporting a document's settings and bookmarks to a .paperback file
					resultMessage = t("Settings exported")
				case .failure:
					resultMessage = t("Failed to export settings")
				}
				exportURL = nil
			}
			.fileImporter(
				isPresented: $showImportPicker,
				allowedContentTypes: [.paperbackData],
				allowsMultipleSelection: false
			) { result in
				switch result {
				case .success(let urls):
					if let url = urls.first, viewModel.importActiveDocumentSettings(from: url) {
						// TRANSLATORS: Confirmation shown after importing a document's settings and bookmarks from a .paperback file
						resultMessage = t("Settings imported")
					} else {
						resultMessage = t("Failed to import settings")
					}
				case .failure:
					resultMessage = t("Failed to import settings")
				}
			}
			// TRANSLATORS: Title of the alert reporting the result of importing/exporting a document's settings and bookmarks
			.alert(t("Document Data"), isPresented: Binding(
				get: { resultMessage != nil },
				set: { if !$0 { resultMessage = nil } }
			)) {
				Button(t("OK")) { resultMessage = nil }
			} message: {
				Text(resultMessage ?? "")
			}
	}

	private func beginExport() {
		exportURL = viewModel.exportActiveDocumentSettings()
		if exportURL == nil {
			resultMessage = t("Failed to export settings")
		}
	}
}

extension View {
	func documentDataTransferMenu() -> some View {
		modifier(DocumentDataTransferMenu())
	}
}
