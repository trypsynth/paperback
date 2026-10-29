import SwiftUI

/// Downloads a document from a link into Downloads in the app's own folder, which the Files app shows, and opens it there.
struct OpenFromURLSheet: View {
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss

	private enum Stage {
		case entering
		case connecting
		case confirming(LinkDownload, warning: String)
		case downloading(LinkDownload, name: String)
	}

	@State private var link = ""
	@State private var stage = Stage.entering
	@State private var progress = 0.0
	@State private var failure: String?
	/// A reference, since a cancelled sheet's own state is gone by the time the server answers.
	@State private var cancellation = Cancellation()
	@FocusState private var isFocused: Bool

	private static var folder: URL { URL.documentsDirectory.appending(path: "Downloads", directoryHint: .isDirectory) }

	var body: some View {
		NavigationStack {
			Form {
				switch stage {
				case .entering, .confirming:
					Section {
						TextField(t("Link:"), text: $link)
							.keyboardType(.URL)
							.textContentType(.URL)
							.textInputAutocapitalization(.never)
							.autocorrectionDisabled()
							.focused($isFocused)
							.onSubmit { connect() }
					}
				case .connecting:
					ProgressView(t("Connecting..."))
				case .downloading(_, let name):
					ProgressView(t("Downloading %s").replacingOccurrences(of: "%s", with: name), value: progress)
				}
			}
			.navigationTitle(t("Open from URL"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					Button(t("Cancel")) { cancel() }
				}
				ToolbarItem(placement: .confirmationAction) {
					Button(t("Open")) { connect() }
						.disabled(!isEntering || link.trimmingCharacters(in: .whitespaces).isEmpty)
				}
			}
			.alert(t("Security Warning"), isPresented: Binding(get: { warning != nil }, set: { if !$0 { stage = .entering } })) {
				Button(withoutAccelerator(t("&Yes"))) { if case .confirming(let download, _) = stage { save(download) } }
				Button(withoutAccelerator(t("&No")), role: .cancel) { stage = .entering }
			} message: {
				Text(warning ?? "")
			}
			.alert(t("Open from URL"), isPresented: Binding(get: { failure != nil }, set: { if !$0 { failure = nil } })) {
				Button(t("OK")) { failure = nil }
			} message: {
				Text(failure ?? "")
			}
			.onAppear { isFocused = true }
		}
		.interactiveDismissDisabled(!isEntering)
	}

	private var isEntering: Bool {
		if case .entering = stage { return true }
		return false
	}

	private var warning: String? {
		if case .confirming(_, let warning) = stage { return warning }
		return nil
	}

	/// The & marks desktop's keyboard accelerator, which a button here has no use for.
	private func withoutAccelerator(_ label: String) -> String {
		label.replacingOccurrences(of: "&", with: "")
	}

	/// The core blocks while it talks to the server, so it runs off the main thread.
	private func connect() {
		guard isEntering else { return }
		let link = link
		stage = .connecting
		Task {
			do {
				let download = try await Task.detached { try LinkDownload(link: link) }.value
				guard !cancellation.isCancelled else { return }
				let info = download.info()
				if info.warning.isEmpty {
					save(download)
				} else {
					stage = .confirming(download, warning: info.warning)
				}
			} catch {
				fail(error)
			}
		}
	}

	private func save(_ download: LinkDownload) {
		stage = .downloading(download, name: download.info().fileName)
		progress = 0
		let folder = Self.folder.path(percentEncoded: false)
		let watcher = Task {
			while !Task.isCancelled {
				let now = download.progress()
				if now.total > 0 { progress = min(Double(now.done) / Double(now.total), 1) }
				try? await Task.sleep(for: .milliseconds(250))
			}
		}
		Task {
			defer { watcher.cancel() }
			do {
				let path = try await Task.detached { try download.save(folder: folder) }.value
				dismiss()
				viewModel.openDocument(url: URL(filePath: path))
			} catch LinkDownloadError.Cancelled {
				stage = .entering
			} catch {
				fail(error)
			}
		}
	}

	private func fail(_ error: Error) {
		stage = .entering
		if case LinkDownloadError.Failed(let message) = error {
			failure = message
		} else {
			failure = "\(error)"
		}
	}

	private func cancel() {
		cancellation.isCancelled = true
		if case .downloading(let download, _) = stage { download.cancel() }
		dismiss()
	}
}

private final class Cancellation {
	var isCancelled = false
}
