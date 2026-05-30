import UIKit

class PaperbackApplication: UIApplication {
	override func accessibilityPerformMagicTap() -> Bool {
		NotificationCenter.default.post(name: .pbMagicTap, object: nil)
		return true
	}
}
