#if DEBUG

import Foundation

// A fixed state for App Store screenshots, driven by ios/Scripts/capture-screenshots.sh. Debug only, so no shipped build reacts to these launch arguments.
@MainActor
enum ScreenshotMode {
	static let isActive = ProcessInfo.processInfo.arguments.contains("-PaperbackScreenshotMode")

	enum Screen: String {
		case reader, text, toc, recents, settings
	}

	static let screen: Screen = {
		let args = ProcessInfo.processInfo.arguments
		guard let flag = args.firstIndex(of: "-PaperbackScreenshotScreen"), flag + 1 < args.count, let screen = Screen(rawValue: args[flag + 1]) else { return .reader }
		return screen
	}()

	// Named by title because Recent Documents shows file names.
	private static let books: [(file: String, opensAt: String)] = [
		("Pride and Prejudice.epub", "I"),
		("Alice's Adventures in Wonderland.epub", "I: Down the Rabbit-Hole"),
		("The Adventures of Sherlock Holmes.epub", "A Scandal in Bohemia"),
	]

	private static var frontBook: Int {
		switch screen {
		case .reader, .recents, .settings: return 0
		case .text: return 1
		case .toc: return 2
		}
	}

	static func stage(_ viewModel: AppViewModel) {
		// Tabs restored from the previous launch would otherwise change the order.
		for tab in viewModel.tabs {
			viewModel.closeTab(tab)
		}
		let folder = URL.documentsDirectory.appending(path: "Screenshots")
		for book in books {
			viewModel.openDocument(url: folder.appending(path: book.file))
			guard let toc = viewModel.activeSession?.getToc() else { continue }
			if let entry = toc.first(where: { $0.title == book.opensAt }) ?? toc.first(where: { $0.title.hasPrefix(book.opensAt) }) {
				viewModel.reading.goToPosition(entry.position)
			}
		}
		guard viewModel.tabs.indices.contains(frontBook) else { return }
		viewModel.setActiveTab(viewModel.tabs[frontBook])
		if screen == .reader {
			// Past the chapter number, so the screen shows a sentence rather than "I".
			_ = viewModel.reading.playNextSegment(speak: false)
			return
		}
		// Text mode's scroll to the reading position and pushed destinations are both lost if they happen before the reader has appeared.
		DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
			switch screen {
			case .text:
				viewModel.reading.toggleTextMode()
				// The line text mode opens on sits under the tab strip, which would cut the chapter title in half.
				viewModel.activeLineScrollIndex = max(0, viewModel.activeLineScrollIndex - 1)
			case .toc: viewModel.navigation.showToc = true
			case .recents: viewModel.navigation.showRecents = true
			case .settings: viewModel.navigation.showSettings = true
			case .reader: break
			}
		}
	}
}

#endif
