import SwiftUI
import UniformTypeIdentifiers

struct ReaderView: View {
	@EnvironmentObject var viewModel: AppViewModel
	@State private var showFilePicker = false
	@State private var isScreenDimmed = false

	var body: some View {
		ZStack {
			mainContent
				.navigationTitle(viewModel.activeTab?.title ?? "Paperback")
				.navigationBarTitleDisplayMode(.inline)
				.toolbar { readerToolbar }
				.safeAreaInset(edge: .bottom) { bottomBar }
			if isScreenDimmed {
				Color.black
					.ignoresSafeArea()
					.onTapGesture { isScreenDimmed = false }
					// TRANSLATORS: Accessibility label for the black overlay shown when the sleep timer dims the screen; tapping it wakes the screen
					.accessibilityLabel(t("Screen dimmed by sleep timer. Tap to wake."))
			}
		}
		.safeAreaInset(edge: .top, spacing: 0) {
			if !viewModel.tabs.isEmpty {
				TabStripView().environmentObject(viewModel)
			}
		}
		.onReceive(viewModel.$sleepTimerRemaining) { remaining in
			if remaining == 0 { isScreenDimmed = true }
		}
		.navigationDestination(isPresented: $viewModel.showToc) {
			TocView().environmentObject(viewModel)
		}
		.navigationDestination(isPresented: $viewModel.showFind) {
			FindView().environmentObject(viewModel)
		}
		.sheet(isPresented: $viewModel.showGoTo) {
			GoToSheet().environmentObject(viewModel)
		}
		.sheet(isPresented: $viewModel.showSettings) {
			SettingsSheet().environmentObject(viewModel)
		}
		.sheet(isPresented: $viewModel.showRecents) {
			RecentDocumentsSheet().environmentObject(viewModel)
		}
		// TRANSLATORS: Title of the alert dialog showing the current document's word count
		.alert(t("Word Count"), isPresented: $viewModel.showWordCount) {
			// TRANSLATORS: Button dismissing the word count alert
			Button(t("OK"), role: .cancel) { }
		} message: {
			if let stats = viewModel.activeSession?.getStatsFfi() {
				Text("This document contains \(stats.wordCount.formatted()) words.")
			}
		}
		.sheet(isPresented: $viewModel.showDocumentInfo) {
			DocumentInfoSheet().environmentObject(viewModel)
		}
		.navigationDestination(isPresented: $viewModel.showSleepTimer) {
			SleepTimerView().environmentObject(viewModel)
		}
		.navigationDestination(isPresented: $viewModel.showElements) {
			ElementsView().environmentObject(viewModel)
		}
		.sheet(
			isPresented: Binding(
				get: { viewModel.passwordPromptUrl != nil },
				set: { if !$0 { viewModel.passwordPromptUrl = nil } }
			)
		) {
			PasswordSheet().environmentObject(viewModel)
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
			if viewModel.isTextMode {
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
		if !viewModel.isTextMode, viewModel.activeTab != nil {
			if #available(iOS 26, *) {
				// Floats as a Liquid Glass pill inset from the edges, matching Safari's bottom
				// toolbar, instead of a flat bar spanning the full width.
				TtsControlBar()
					.environmentObject(viewModel)
					.glassEffect(.regular, in: RoundedRectangle(cornerRadius: 26))
					.padding(.horizontal, 12)
					.padding(.bottom, 8)
			} else {
				TtsControlBar()
					.environmentObject(viewModel)
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
			DocumentMenu().environmentObject(viewModel)
		}
	}
}
