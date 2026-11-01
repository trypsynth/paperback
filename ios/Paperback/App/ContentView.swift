import SwiftUI
import UIKit

struct ContentView: View {
	// Not created here: the scene delegate needs the same instance to route keyboard shortcuts and documents from other apps.
	let viewModel: AppViewModel

	var body: some View {
		// On the stack, not ReaderView: navigationDestination content only inherits what the stack itself carries.
		NavigationStack {
			ReaderView()
		}
		.environment(viewModel)
		.preferredColorScheme(viewModel.appearanceChoice.preferredColorSchemeChoice)
	}
}
