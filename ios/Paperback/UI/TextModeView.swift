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

	/// Builds the line's text with its link markers turned into real links. The marker's
	/// position is carried in the URL so the openURL handler can hand it straight back to
	/// activateLink, which is the core's own resolution of where a link goes.
	private func attributedLine(_ line: String, lineStart: Int64, markers: [LineMarker]) -> AttributedString {
		var attributed = AttributedString(line)
		for marker in markers where marker.mtype == .link {
			let startOffset = Int(marker.position - lineStart)
			let length = marker.text.count
			guard startOffset >= 0, length > 0, startOffset < line.count else { continue }
			let endOffset = min(startOffset + length, line.count)
			let characters = attributed.characters
			guard
				let start = characters.index(characters.startIndex, offsetBy: startOffset, limitedBy: characters.endIndex),
				let end = characters.index(characters.startIndex, offsetBy: endOffset, limitedBy: characters.endIndex),
				start < end,
				let url = URL(string: "\(Self.linkScheme)://\(marker.position)")
			else { continue }
			attributed[start..<end].link = url
			attributed[start..<end].underlineStyle = .single
		}
		return attributed
	}

	/// Resolves a tapped link. Internal links scroll to their target; external ones are handed
	/// to the system.
	private func activate(_ url: URL, in session: DocumentSession, scrollingWith proxy: ScrollViewProxy) -> OpenURLAction.Result {
		guard url.scheme == Self.linkScheme, let host = url.host(), let position = Int64(host) else {
			return .systemAction
		}
		let result = session.activateLink(position: position)
		guard result.found else { return .handled }
		switch result.action {
		case .internal:
			let line = session.lineFromPosition(position: result.offset)
			let index = max(0, Int(line) - 1)
			viewModel.reading.goToPosition(result.offset)
			proxy.scrollTo(index, anchor: .top)
			return .handled
		case .external:
			guard let target = URL(string: result.url) else { return .handled }
			return .systemAction(target)
		case .notFound:
			return .handled
		}
	}

	private static let linkScheme = "paperback-link"

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
							let markers = session.getLineMarkers(line: lineNumber)
							let heading = headingLevel(of: markers)
							Text(line.isEmpty ? AttributedString("\n") : attributedLine(line, lineStart: pos, markers: markers))
								.readabilityStyle(viewModel)
								.fontWeight(heading == nil ? .regular : .bold)
								.padding(.horizontal)
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
			.environment(\.openURL, OpenURLAction { url in
				guard let session = tab.session else { return .systemAction }
				return activate(url, in: session, scrollingWith: proxy)
			})
		}
		.onChange(of: visibleLineIndices) { _, indices in
			if let min = indices.min() {
				viewModel.reading.textModeFirstLine = min
			}
		}
	}
}
