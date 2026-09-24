import SwiftUI
import UIKit

/// The system share sheet, wrapped so a SwiftUI view can present it.
///
/// Export used to hand its file straight to the Files "move to" picker, which meant a document
/// could only be saved somewhere on the device. The share sheet reaches everything the phone can
/// send a file to, a message or another app, and still offers Save to Files
/// (<https://github.com/trypsynth/paperback/issues/928>).
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
	/// Presents the share sheet for `url` while it is set, clearing it when the sheet closes.
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
