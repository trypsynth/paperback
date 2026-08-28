import SwiftUI
import UniformTypeIdentifiers

struct ReaderView: View {
	@Environment(AppViewModel.self) private var viewModel
	@State private var showFilePicker = false

	var body: some View {
		@Bindable var navigation = viewModel.navigation
		return ZStack {
			mainContent
				.navigationTitle(viewModel.activeTab?.title ?? "Paperback")
				.navigationBarTitleDisplayMode(.inline)
				.toolbar { readerToolbar }
				.safeAreaInset(edge: .bottom) { bottomBar }
		}
		.safeAreaInset(edge: .top, spacing: 0) {
			if !viewModel.tabs.isEmpty {
				TabStripView().environment(viewModel)
			}
		}
		.navigationDestination(isPresented: $navigation.showToc) {
			TocView().environment(viewModel)
		}
		.navigationDestination(isPresented: $navigation.showFind) {
			FindView().environment(viewModel)
		}
		.sheet(isPresented: $navigation.showGoTo) {
			GoToSheet().environment(viewModel)
		}
		.navigationDestination(isPresented: $navigation.showSettings) {
			SettingsView().environment(viewModel)
		}
		.navigationDestination(isPresented: $navigation.showRecents) {
			RecentDocumentsView().environment(viewModel)
		}
		// TRANSLATORS: Title of the alert dialog showing the current document's word count
		.alert(t("Word Count"), isPresented: $navigation.showWordCount) {
			// TRANSLATORS: Button dismissing the word count alert
			Button(t("OK"), role: .cancel) { }
		} message: {
			if let stats = viewModel.activeSession?.getStatsFfi() {
				Text("This document contains \(stats.wordCount.formatted()) words.")
			}
		}
		.sheet(isPresented: $navigation.showDocumentInfo) {
			DocumentInfoSheet().environment(viewModel)
		}
		.navigationDestination(isPresented: $navigation.showSleepTimer) {
			SleepTimerView().environment(viewModel)
		}
		.navigationDestination(isPresented: $navigation.showElements) {
			ElementsView().environment(viewModel)
		}
		.sheet(
			isPresented: Binding(
				get: { viewModel.navigation.passwordPromptUrl != nil },
				set: { if !$0 { viewModel.navigation.passwordPromptUrl = nil } }
			)
		) {
			PasswordSheet().environment(viewModel)
		}
		.fileImporter(
			isPresented: $showFilePicker,
			allowedContentTypes: [.item],
			allowsMultipleSelection: false
		) { result in
			guard case .success(let urls) = result, let url = urls.first else { return }
			viewModel.openDocument(url: url)
		}
		// TRANSLATORS: Title of the alert shown when a document fails to open
		.alert(t("Open Error"), isPresented: Binding(
			get: { viewModel.debugMessage != nil },
			set: { if !$0 { viewModel.debugMessage = nil } }
		)) {
			// TRANSLATORS: Button dismissing the document-open-error alert
			Button(t("OK")) { viewModel.debugMessage = nil }
		} message: {
			Text(viewModel.debugMessage ?? "")
		}
	}

	// MARK: - Main content

	@ViewBuilder
	private var mainContent: some View {
		if let tab = viewModel.activeTab {
			if viewModel.reading.isTextMode {
				TextModeView(tab: tab)
			} else {
				TtsModeView()
			}
		} else {
			EmptyStateView(onOpenFile: { showFilePicker = true })
		}
	}

	// MARK: - Bottom bar

	@ViewBuilder
	private var bottomBar: some View {
		if !viewModel.reading.isTextMode, viewModel.activeTab != nil {
			if #available(iOS 26, *) {
				// Floats as a Liquid Glass pill inset from the edges, matching Safari's bottom
				// toolbar, instead of a flat bar spanning the full width.
				TtsControlBar()
					.environment(viewModel)
					.glassEffect(.regular, in: RoundedRectangle(cornerRadius: 26))
					.padding(.horizontal, 12)
					.padding(.bottom, 8)
			} else {
				TtsControlBar()
					.environment(viewModel)
					.background {
						Rectangle()
							.fill(.bar)
							.ignoresSafeArea(edges: .bottom)
					}
			}
		}
	}

	// MARK: - Toolbar

	@ToolbarContentBuilder
	private var readerToolbar: some ToolbarContent {
		ToolbarItemGroup(placement: .topBarTrailing) {
			Button { showFilePicker = true } label: {
				Image(systemName: "folder")
			}
			// TRANSLATORS: Accessibility label for the toolbar button that opens a file picker to choose a document
			.accessibilityLabel(t("Open Book"))
			.documentDataTransferMenu()
			DocumentMenu().environment(viewModel)
		}
	}
}
