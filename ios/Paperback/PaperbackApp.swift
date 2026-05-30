import SwiftUI
import UIKit

extension Notification.Name {
	static let pbMagicTap = Notification.Name("dev.paperback.magicTap")
}

class AppDelegate: UIResponder, UIApplicationDelegate {
	override func accessibilityPerformMagicTap() -> Bool {
		NotificationCenter.default.post(name: .pbMagicTap, object: nil)
		return true
	}
}

@main
struct PaperbackApp: App {
	@UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

	var body: some Scene {
		WindowGroup {
			ContentView()
		}
	}
}
