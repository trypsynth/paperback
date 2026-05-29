import SwiftUI

struct TextModeView: View {
	@EnvironmentObject var viewModel: AppViewModel
	let tab: DocumentTab

	var body: some View {
		ScrollViewReader { proxy in
			ScrollView {
				LazyVStack(alignment: .leading, spacing: 0) {
					if let session = tab.session {
						let lineCount = session.lineCount()
						ForEach(0..<Int(lineCount), id: \.self) { i in
							let line = session.getLineText(position: Int64(i))
							Text(line.isEmpty ? "\n" : line)
								.font(.body)
								.padding(.horizontal)
								.padding(.vertical, 2)
								.frame(maxWidth: .infinity, alignment: .leading)
								.id(i)
						}
					} else {
						Text("No document open.")
							.padding()
							.foregroundStyle(.secondary)
					}
				}
			}
			.onAppear {
				proxy.scrollTo(Int(tab.lineScrollIndex), anchor: .top)
			}
		}
	}
}
