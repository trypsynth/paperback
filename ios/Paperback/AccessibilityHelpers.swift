import SwiftUI
import UIKit

extension View {
	// Posts a VoiceOver screenChanged notification after the sheet animation
	// settles so focus lands on the first element instead of nowhere.
	func sheetAccessibilityFocus() -> some View {
		onAppear {
			DispatchQueue.main.asyncAfter(deadline: .now() + 0.6) {
				UIAccessibility.post(notification: .screenChanged, argument: nil)
			}
		}
	}
}
