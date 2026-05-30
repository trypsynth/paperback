import SwiftUI

extension Notification.Name {
	static let pbMagicTap = Notification.Name("dev.paperback.magicTap")
}

@main
struct PaperbackApp: App {
	var body: some Scene {
		WindowGroup {
			ContentView()
		}
	}
}
