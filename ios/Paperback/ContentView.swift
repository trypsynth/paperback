import SwiftUI

struct ContentView: View {
	var body: some View {
		NavigationStack {
			VStack(spacing: 24) {
				Image(systemName: "book.closed")
					.font(.system(size: 64))
					.foregroundStyle(.secondary)
				Text("No document open")
					.font(.title2)
					.foregroundStyle(.secondary)
			}
			.frame(maxWidth: .infinity, maxHeight: .infinity)
			.navigationTitle("Paperback")
			.toolbar {
				ToolbarItem(placement: .topBarTrailing) {
					Button {
						// open file picker
					} label: {
						Label("Open", systemImage: "folder")
					}
				}
			}
		}
	}
}

#Preview {
	ContentView()
}
