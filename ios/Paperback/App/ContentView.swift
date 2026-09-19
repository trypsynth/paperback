import SwiftUI
import UIKit

struct ContentView: View {
	@State private var viewModel = AppViewModel()

	var body: some View {
		// The environment object goes on the stack, not on ReaderView: content pushed via
		// navigationDestination(for:) is hosted by the stack, so it only inherits what the
		// stack itself carries.
		NavigationStack {
			ReaderView()
		}
		.environment(viewModel)
		.onAppear {
			if let scene = UIApplication.shared.connectedScenes.first,
			   let sd = scene.delegate as? SceneDelegate {
				sd.appViewModel = viewModel
				// A launch straight from another app's "Open in" left the book here, because
				// the scene connected before this view model existed.
				if let pending = sd.takePendingDocument() {
					viewModel.openDocument(url: pending)
				}
			}
		}
	}
}
