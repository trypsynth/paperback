import SwiftUI
import UIKit

struct ContentView: View {
	@StateObject private var viewModel = AppViewModel()

	var body: some View {
		NavigationStack {
			ReaderView()
				.environmentObject(viewModel)
		}
		.onAppear {
			if let scene = UIApplication.shared.connectedScenes.first,
			   let sd = scene.delegate as? SceneDelegate {
				sd.appViewModel = viewModel
			}
		}
	}
}
