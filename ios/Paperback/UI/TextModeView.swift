import SwiftUI

struct TextModeView: View {
	@Environment(AppViewModel.self) private var viewModel
	let tab: DocumentTab
	@State private var visibleLineIndices: Set<Int> = []

	private var firstVisibleLine: Int { visibleLineIndices.min() ?? 0 }

	/// The heading level a line carries, if any. A line is a heading when the core reports a
	/// heading marker on it, the same signal the Android reader uses to mark headings up.
	private func headingLevel(of markers: [LineMarker]) -> Int? {
		for marker in markers {
			switch marker.mtype {
			case .heading1: return 1
			case .heading2: return 2
			case .heading3: return 3
			case .heading4: return 4
			case .heading5: return 5
			case .heading6: return 6
			default: continue
			}
		}
		return nil
	}

	private func headingTrait(_ level: Int?) -> AccessibilityHeadingLevel {
		switch level {
		case 1: return .h1
		case 2: return .h2
		case 3: return .h3
		case 4: return .h4
		case 5: return .h5
		case 6: return .h6
		default: return .unspecified
		}
	}

	var body: some View {
		ScrollViewReader { proxy in
			ScrollView {
				LazyVStack(alignment: .leading, spacing: 0) {
					if let session = tab.session {
						let lineCount = session.lineCount()
						ForEach(0..<Int(lineCount), id: \.self) { i in
							// The core numbers lines from 1; these indices are 0-based because
							// the scroll position they feed (lineScrollIndex, textModeFirstLine)
							// is too.
							let lineNumber = Int64(i + 1)
							let pos = session.positionFromLine(line: lineNumber)
							let line = session.getLineText(position: pos)
							let heading = headingLevel(of: session.getLineMarkers(line: lineNumber))
							Text(line.isEmpty ? "\n" : line)
								.font(heading == nil ? .body : .body.weight(.bold))
								.padding(.horizontal)
								.padding(.vertical, 2)
								.frame(maxWidth: .infinity, alignment: .leading)
								.accessibilityAddTraits(heading == nil ? [] : .isHeader)
								.accessibilityHeading(headingTrait(heading))
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
		.onChange(of: visibleLineIndices) { _, indices in
			if let min = indices.min() {
				viewModel.reading.textModeFirstLine = min
			}
		}
	}
}
