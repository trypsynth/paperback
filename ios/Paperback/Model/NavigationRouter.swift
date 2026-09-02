import Foundation

// Which screen or dialog the reader is showing. This lives outside the views because two
// non-SwiftUI layers drive navigation: the UIKeyCommand handlers in KeyboardShortcuts and the
// VoiceOver custom actions on DocumentMenu. Both need something to set, so per-view @State
// would not reach.
@MainActor
@Observable
final class NavigationRouter {
	var showToc = false
	var showFind = false
	var showGoTo = false
	var goToInitialMode: GoToMode = .line
	var showSettings = false
	var showRecents = false
	var showWordCount = false
	var showDocumentInfo = false
	var showSleepTimer = false
	var showElements = false
	var passwordPromptUrl: URL? = nil
}
