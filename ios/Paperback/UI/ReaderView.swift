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
					.accessibilityLabel("Screen dimmed by sleep timer. Tap to wake.")
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
		.onChange(of: viewModel.isTextMode) { entering in
			if entering {
				viewModel.enterTextMode()
			} else {
				viewModel.exitTextMode()
			}
		}
		.sheet(isPresented: $viewModel.showToc) {
			TocSheet().environmentObject(viewModel)
		}
		.sheet(isPresented: $viewModel.showFind) {
			FindSheet().environmentObject(viewModel)
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
		.alert("Word Count", isPresented: $viewModel.showWordCount) {
			Button("OK", role: .cancel) { }
		} message: {
			if let stats = viewModel.activeSession?.getStatsFfi() {
				Text("This document contains \(stats.wordCount.formatted()) words.")
			}
		}
		.sheet(isPresented: $viewModel.showDocumentInfo) {
			DocumentInfoSheet().environmentObject(viewModel)
		}
		.sheet(isPresented: $viewModel.showSleepTimer) {
			SleepTimerSheet().environmentObject(viewModel)
		}
		.sheet(isPresented: $viewModel.showElements) {
			ElementsSheet().environmentObject(viewModel)
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
		.alert("Open Error", isPresented: Binding(
			get: { viewModel.debugMessage != nil },
			set: { if !$0 { viewModel.debugMessage = nil } }
		)) {
			Button("OK") { viewModel.debugMessage = nil }
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
			TtsControlBar()
				.environmentObject(viewModel)
				.background {
					Rectangle()
						.fill(.bar)
						.ignoresSafeArea(edges: .bottom)
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
			.accessibilityLabel("Open document")
			if viewModel.activeTab != nil {
				DocumentMenu().environmentObject(viewModel)
			}
		}
	}
}
