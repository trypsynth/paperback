import SwiftUI
import UIKit

extension Notification.Name {
	static let pbMagicTap = Notification.Name("dev.paperback.magicTap")
}

class AppDelegate: UIResponder, UIApplicationDelegate {
	func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil) -> Bool {
		application.beginReceivingRemoteControlEvents()
		return true
	}

	func application(_ application: UIApplication, configurationForConnecting connectingSceneSession: UISceneSession, options: UIScene.ConnectionOptions) -> UISceneConfiguration {
		let config = UISceneConfiguration(name: nil, sessionRole: connectingSceneSession.role)
		config.delegateClass = SceneDelegate.self
		return config
	}

	// Belt-and-suspenders: in UIKit apps AppDelegate is at the end of the responder
	// chain, but with SwiftUI's @UIApplicationDelegateAdaptor this may never fire.
	// UIWindow (below) is the reliable catch for in-app magic tap.
	override func accessibilityPerformMagicTap() -> Bool {
		NotificationCenter.default.post(name: .pbMagicTap, object: nil)
		return true
	}
}

// UIWindow is always in the responder chain as the key window, making it the
// reliable place to catch VoiceOver magic tap while the app is foregrounded.
class MagicTapWindow: UIWindow {
	override func accessibilityPerformMagicTap() -> Bool {
		NotificationCenter.default.post(name: .pbMagicTap, object: nil)
		return true
	}
}

class SceneDelegate: NSObject, UIWindowSceneDelegate {
	var window: UIWindow?
	weak var appViewModel: AppViewModel?

	func scene(_ scene: UIScene, willConnectTo session: UISceneSession, options: UIScene.ConnectionOptions) {
		guard let windowScene = scene as? UIWindowScene else { return }
		let window = MagicTapWindow(windowScene: windowScene)
		window.rootViewController = UIHostingController(rootView: ContentView())
		self.window = window
		window.makeKeyAndVisible()
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
