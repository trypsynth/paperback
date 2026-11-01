import SwiftUI
import UIKit

/// Rather than the Files picker, which could only save on the device; the share sheet still offers Save to Files (#928).
struct ShareSheet: UIViewControllerRepresentable {
	let items: [Any]
	let onFinish: () -> Void

	func makeUIViewController(context: Context) -> UIActivityViewController {
		let controller = UIActivityViewController(activityItems: items, applicationActivities: nil)
		controller.completionWithItemsHandler = { _, _, _, _ in onFinish() }
		return controller
	}

	func updateUIViewController(_ controller: UIActivityViewController, context: Context) {}
}

extension View {
	func shareSheet(for url: Binding<URL?>, onFinish: @escaping () -> Void = {}) -> some View {
		sheet(isPresented: Binding(get: { url.wrappedValue != nil }, set: { if !$0 { url.wrappedValue = nil } })) {
			if let item = url.wrappedValue {
				ShareSheet(items: [item]) {
					url.wrappedValue = nil
					onFinish()
				}
			}
		}
	}
}
