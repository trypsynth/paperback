import Foundation

// Outside the views because UIKeyCommand handlers and DocumentMenu's VoiceOver custom actions drive navigation too, and per-view @State wouldn't reach them.
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
	var showExportDocument = false
	var showSleepTimer = false
	var showElements = false
	var showFilePicker = false
	var passwordPromptUrl: URL? = nil
}
