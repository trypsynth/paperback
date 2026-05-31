import SwiftUI

private struct SheetFocusModifier: ViewModifier {
	let title: String
	@AccessibilityFocusState private var focused: Bool

	func body(content: Content) -> some View {
		content
			.overlay(alignment: .top) {
				Text(title)
					.frame(height: 0)
					.opacity(0)
					.accessibilityFocused($focused)
			}
			.onAppear {
				DispatchQueue.main.asyncAfter(deadline: .now() + 0.35) {
					focused = true
				}
			}
	}
}

extension View {
	func sheetAccessibilityFocus(title: String) -> some View {
		modifier(SheetFocusModifier(title: title))
	}
}
