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

	// Belt-and-suspenders: with @UIApplicationDelegateAdaptor this may never fire, so UIWindow below is the reliable catch.
	override func accessibilityPerformMagicTap() -> Bool {
		NotificationCenter.default.post(name: .pbMagicTap, object: nil)
		return true
	}
}

// The key window is always in the responder chain, so it reliably catches VoiceOver magic tap.
class MagicTapWindow: UIWindow {
	override func accessibilityPerformMagicTap() -> Bool {
		NotificationCenter.default.post(name: .pbMagicTap, object: nil)
		return true
	}
}

@MainActor
class SceneDelegate: NSObject, UIWindowSceneDelegate {
	var window: UIWindow?
	/// Owned here so the shortcut and incoming-document handlers reach this scene's instance without searching UIApplication for a scene.
	let appViewModel = AppViewModel()

	func scene(_ scene: UIScene, willConnectTo session: UISceneSession, options: UIScene.ConnectionOptions) {
		guard let windowScene = scene as? UIWindowScene else { return }
		let window = MagicTapWindow(windowScene: windowScene)
		window.rootViewController = UIHostingController(rootView: ContentView(viewModel: appViewModel))
		self.window = window
		window.makeKeyAndVisible()
		open(options.urlContexts)
		#if DEBUG
		if ScreenshotMode.isActive { ScreenshotMode.stage(appViewModel) }
		#endif
	}

	func scene(_ scene: UIScene, openURLContexts URLContexts: Set<UIOpenURLContext>) {
		open(URLContexts)
	}

	private func open(_ contexts: Set<UIOpenURLContext>) {
		guard let url = contexts.first?.url else { return }
		appViewModel.openDocument(url: url)
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
