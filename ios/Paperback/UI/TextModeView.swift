import SwiftUI

struct TextModeView: View {
	@EnvironmentObject var viewModel: AppViewModel
	let tab: DocumentTab
	@State private var visibleLineIndices: Set<Int> = []

	private var firstVisibleLine: Int { visibleLineIndices.min() ?? 0 }

	var body: some View {
		ScrollViewReader { proxy in
			ScrollView {
				LazyVStack(alignment: .leading, spacing: 0) {
					if let session = tab.session {
						let lineCount = session.lineCount()
						ForEach(0..<Int(lineCount), id: \.self) { i in
							let pos = session.positionFromLine(line: Int64(i))
							let line = session.getLineText(position: pos)
							Text(line.isEmpty ? "\n" : line)
								.font(.body)
								.padding(.horizontal)
								.padding(.vertical, 2)
								.frame(maxWidth: .infinity, alignment: .leading)
								.id(i)
								.onAppear { visibleLineIndices.insert(i) }
								.onDisappear { visibleLineIndices.remove(i) }
						}
					} else {
						Text("No document open.")
							.padding()
							.foregroundStyle(.secondary)
					}
				}
			}
			.onAppear {
				proxy.scrollTo(tab.lineScrollIndex, anchor: .top)
			}
		}
		.onChange(of: visibleLineIndices) { indices in
			if let min = indices.min() {
				viewModel.textModeFirstLine = min
			}
		}
	}
}
