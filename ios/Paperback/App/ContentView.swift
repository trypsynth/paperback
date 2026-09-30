import SwiftUI
import UIKit

struct ContentView: View {
	// Handed in by the scene that owns it, rather than created here: the scene delegate needs
	// the same instance to route keyboard shortcuts and documents opened from other apps.
	let viewModel: AppViewModel

	var body: some View {
		// The environment object goes on the stack, not on ReaderView: content pushed via
		// navigationDestination(for:) is hosted by the stack, so it only inherits what the
		// stack itself carries.
		NavigationStack {
			ReaderView()
		}
		.environment(viewModel)
		.preferredColorScheme(viewModel.appearanceChoice.preferredColorSchemeChoice)
	}
}
