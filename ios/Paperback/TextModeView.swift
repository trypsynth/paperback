import SwiftUI

struct TextModeView: View {
	@EnvironmentObject var viewModel: AppViewModel
	let tab: DocumentTab

	var body: some View {
		ScrollViewReader { proxy in
			ScrollView {
				LazyVStack(alignment: .leading, spacing: 0) {
					// TODO: replace with real lines from DocumentSession once UniFFI is wired up
					Text("Text mode: document content will appear here line by line.")
						.padding()
						.foregroundStyle(.secondary)
				}
			}
		}
	}
}
