import SwiftUI

struct DocumentMenu: View {
	@Environment(AppViewModel.self) private var viewModel

	var body: some View {
		if viewModel.activeTab != nil {
			fullMenu
		} else {
			emptyMenu
		}
	}

	private var fullMenuAccessibilityBase: some View {
		fullMenuButton
			// TRANSLATORS: VoiceOver accessibility label for the "..." button that opens this document actions menu
			.accessibilityLabel(t("More options"))
			.accessibilityRemoveTraits(.isButton)
			.accessibilityAction(named: t("Settings")) { viewModel.navigation.showSettings = true }
			.accessibilityAction(named: t("Help")) { viewModel.openHelpDocument() }
			.accessibilityAction(named: t("Sleep Timer")) { viewModel.navigation.showSleepTimer = true }
			.accessibilityAction(named: t("Document Info")) { viewModel.navigation.showDocumentInfo = true }
			.accessibilityAction(named: t("Word Count")) { viewModel.navigation.showWordCount = true }
	}

	private var fullMenu: some View {
		fullMenuAccessibilityBase
			.accessibilityAction(named: t("Recent Documents")) { viewModel.navigation.showRecents = true }
			.accessibilityAction(named: t("Go To")) { viewModel.navigation.showGoTo = true }
			.accessibilityAction(named: t("Find")) { viewModel.navigation.showFind = true }
			.accessibilityAction(named: t("Elements")) { viewModel.navigation.showElements = true }
			.accessibilityAction(named: t("Table of Contents")) { viewModel.navigation.showToc = true }
			.accessibilityAction(named: t("Bookmarks")) { viewModel.navigation.showBookmarks = true }
			.accessibilityActions {
				if speaksWithTts {
					Button(t("Speech for This Book")) { viewModel.navigation.showBookSpeech = true }
				}
			}
			.accessibilityAction(named: toggleModeActionName) {
				viewModel.reading.toggleTextMode()
			}
			.accessibilityActions {
				if showsReadAloud {
					Button(readAloudTitle) { viewModel.reading.togglePlayPause() }
				}
			}
	}

	/// A recording reads itself, so voice settings mean nothing to it.
	private var speaksWithTts: Bool {
		!viewModel.reading.hasAudio
	}

	// Text mode has no reading bar, so this is the way to start reading aloud without leaving it.
	private var showsReadAloud: Bool {
		viewModel.reading.isTextMode && !viewModel.reading.isAudioOnly
	}

	// TRANSLATORS: Menu item / accessibility action toggling text-to-speech playback; label names the action that tapping it performs
	private var readAloudTitle: String {
		viewModel.reading.isPlayingNow ? t("Pause Read Aloud") : t("Read Aloud")
	}

	private var emptyMenu: some View {
		emptyMenuButton
			// TRANSLATORS: VoiceOver accessibility label for the "..." button that opens this document actions menu
			.accessibilityLabel(t("More options"))
			.accessibilityRemoveTraits(.isButton)
			.accessibilityAction(named: t("Settings")) { viewModel.navigation.showSettings = true }
			.accessibilityAction(named: t("Help")) { viewModel.openHelpDocument() }
	}

	// TRANSLATORS: VoiceOver custom action name for toggling between TTS and text reading mode; same strings as the menu item above
	private var toggleModeActionName: String {
		viewModel.reading.isTextMode ? t("Switch to TTS Mode") : t("Switch to Text Mode")
	}

	private var fullMenuButton: some View {
		Menu {
			modeToggleItem
			if showsReadAloud {
				Button { viewModel.reading.togglePlayPause() } label: {
					Label(readAloudTitle, systemImage: viewModel.reading.isPlayingNow ? "pause" : "play")
				}
			}
			Divider()
			navigationItems
			Divider()
			infoItems
			Divider()
			sleepTimerItem
			if speaksWithTts {
				Button { viewModel.navigation.showBookSpeech = true } label: {
					Label(t("Speech for This Book"), systemImage: "person.wave.2")
				}
			}
			if let layout = viewModel.activeDocumentPageLayout {
				pageLayoutPicker(layout)
			}
			Divider()
			helpAndSettingsItems
		} label: {
			Image(systemName: "ellipsis.circle")
		}
	}

	private var emptyMenuButton: some View {
		Menu {
			helpAndSettingsItems
		} label: {
			Image(systemName: "ellipsis.circle")
		}
	}

	@ViewBuilder private var navigationItems: some View {
		Button { viewModel.navigation.showToc = true } label: {
			// TRANSLATORS: Menu item to open the document's table of contents
			Label(t("Table of Contents"), systemImage: "list.bullet")
		}
		Button { viewModel.navigation.showElements = true } label: {
			// TRANSLATORS: Menu item to open the list of headings and links found in the document
			Label(t("Elements"), systemImage: "list.dash")
		}
		Button { viewModel.navigation.showBookmarks = true } label: {
			// TRANSLATORS: Menu item / accessibility action opening the list of the document's bookmarks and notes
			Label(t("Bookmarks"), systemImage: "bookmark")
		}
		Divider()
		Button { viewModel.navigation.showFind = true } label: {
			// TRANSLATORS: Menu item to open find-in-document search
			Label(t("Find"), systemImage: "magnifyingglass")
		}
		Button {
			viewModel.navigation.showGoTo = true
		} label: {
			// TRANSLATORS: Menu item to jump to a specific page, line, or percentage in the document
			Label(t("Go To..."), systemImage: "arrow.right.circle")
		}
	}

	@ViewBuilder private var infoItems: some View {
		Button { viewModel.navigation.showRecents = true } label: {
			// TRANSLATORS: Menu item to open the list of recently opened documents
			Label(t("Recent Documents"), systemImage: "clock")
		}
		Divider()
		Button { viewModel.navigation.showWordCount = true } label: {
			// TRANSLATORS: Menu item to show word/character/line count statistics for the document
			Label(t("Word Count"), systemImage: "textformat.123")
		}
		Button { viewModel.navigation.showDocumentInfo = true } label: {
			// TRANSLATORS: Menu item to show document metadata (title, author, format, etc.)
			Label(t("Document Info"), systemImage: "info.circle")
		}
		Button { viewModel.navigation.showExportDocument = true } label: {
			// TRANSLATORS: Menu item to save the current document in another file format
			Label(t("Export Document"), systemImage: "square.and.arrow.up")
		}
		.disabled(viewModel.supportedExportFormats.isEmpty)
	}

	private var modeToggleItem: some View {
		Button { viewModel.reading.toggleTextMode() } label: {
			// TRANSLATORS: Menu item toggling between spoken (TTS) reading mode and visual text reading mode; label reflects the mode it will switch TO
			let title: String = viewModel.reading.isTextMode ? t("Switch to TTS Mode") : t("Switch to Text Mode")
			let icon: String = viewModel.reading.isTextMode ? "speaker.wave.2" : "text.alignleft"
			Label(title, systemImage: icon)
		}
	}

	private var sleepTimerItem: some View {
		Button { viewModel.navigation.showSleepTimer = true } label: {
			// TRANSLATORS: Menu item to open the sleep timer; the "(active)" variant is shown while a timer is currently counting down
			let title: String = viewModel.reading.sleepTimerRemaining != nil ? t("Sleep Timer (active)") : t("Sleep Timer")
			Label(title, systemImage: "timer")
		}
	}

	// Left out of the More options button's actions: its four choices would crowd the rotor, and the menu already says which one is selected.
	private func pageLayoutPicker(_ layout: PageLayout?) -> some View {
		Picker(selection: Binding(get: { layout }, set: { viewModel.setActiveDocumentPageLayout($0) })) {
			// TRANSLATORS: Item in the Page Layout submenu of the document actions menu: the open PDF or XPS document follows the page layout chosen for all documents in Settings
			Text(t("Same as Other Documents")).tag(PageLayout?.none)
			// TRANSLATORS: Item in the Page Layout submenu of the document actions menu: the open document's pages are read in the order the document gives, from its tags where it has them, with wrapped lines joined into paragraphs
			Text(t("Automatic")).tag(PageLayout?.some(.automatic))
			// TRANSLATORS: Item in the Page Layout submenu of the document actions menu: the open document's own reading order is ignored and the columns of each page are found from where the text sits, for documents read out of order
			Text(t("By Columns")).tag(PageLayout?.some(.columns))
			// TRANSLATORS: Item in the Page Layout submenu of the document actions menu: like By Columns, but every line is kept apart instead of joined into paragraphs, for code, poetry and lists of clauses
			Text(t("By Columns, Every Line Apart")).tag(PageLayout?.some(.lines))
		} label: {
			// TRANSLATORS: Submenu of the document actions menu choosing how the open PDF or XPS document's pages are put in reading order and paragraphs
			Label(t("Page Layout"), systemImage: "doc.text")
		}
		.pickerStyle(.menu)
	}

	@ViewBuilder private var helpAndSettingsItems: some View {
		Button { viewModel.openHelpDocument() } label: {
			// TRANSLATORS: Menu item to open the in-app help document
			Label(t("Help"), systemImage: "questionmark.circle")
		}
		Button { viewModel.navigation.showSettings = true } label: {
			// TRANSLATORS: Menu item to open the app's settings
			Label(t("Settings"), systemImage: "gear")
		}
	}
}
