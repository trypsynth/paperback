import SwiftUI

struct ContentView: View {
	@StateObject private var viewModel = AppViewModel()

	var body: some View {
		NavigationStack {
			ReaderView()
				.environmentObject(viewModel)
		}
	}
}
