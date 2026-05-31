import SwiftUI
import UIKit

extension Notification.Name {
	static let pbMagicTap = Notification.Name("dev.paperback.magicTap")
}

class AppDelegate: NSObject, UIApplicationDelegate {
	func application(_ application: UIApplication, configurationForConnecting connectingSceneSession: UISceneSession, options: UIScene.ConnectionOptions) -> UISceneConfiguration {
		let config = UISceneConfiguration(name: nil, sessionRole: connectingSceneSession.role)
		config.delegateClass = SceneDelegate.self
		return config
	}
}

class SceneDelegate: NSObject, UIWindowSceneDelegate {
	var window: UIWindow?

	func scene(_ scene: UIScene, willConnectTo session: UISceneSession, options: UIScene.ConnectionOptions) {
		guard let windowScene = scene as? UIWindowScene else { return }
		let window = MagicTapWindow(windowScene: windowScene)
		window.rootViewController = UIHostingController(rootView: ContentView())
		self.window = window
		window.makeKeyAndVisible()
	}
}

class MagicTapWindow: UIWindow {
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
			EmptyView()
		}
	}
}
