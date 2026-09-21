import SwiftUI

struct TocView: View {
	@Environment(AppViewModel.self) private var viewModel

	var body: some View {
		Group {
			if let session = viewModel.activeSession {
				let entries = session.getToc()
				if entries.isEmpty {
					emptyView
				} else {
					TocTree(entries: entries, activeIndex: activeIndex(in: entries))
				}
			} else {
				emptyView
			}
		}
		// TRANSLATORS: Navigation title of the table of contents screen
		.navigationTitle(t("Contents"))
		.navigationBarTitleDisplayMode(.inline)
	}

	// A position before the first entry counts as being in it, so there is always a row to land on.
	private func activeIndex(in entries: [TocEntry]) -> Int {
		entries.lastIndex(where: { $0.position <= viewModel.reading.ttsPosition }) ?? 0
	}

	@ViewBuilder private var emptyView: some View {
		ContentUnavailableView(
			// TRANSLATORS: Title shown when the current document has no table of contents
			t("No Table of Contents"),
			systemImage: "list.bullet",
			// TRANSLATORS: Description shown below the "No Table of Contents" title
			description: Text(t("This document has no table of contents."))
		)
		.sheetAccessibilityFocus(title: "Contents")
	}
}

private struct TocTree: View {
	let entries: [TocEntry]
	let activeIndex: Int
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss
	@State private var expanded: Set<Int>
	@AccessibilityFocusState private var focusedIndex: Int?
	private let chevronWidth: CGFloat = 24
	private let indentPerLevel: CGFloat = 16

	init(entries: [TocEntry], activeIndex: Int) {
		self.entries = entries
		self.activeIndex = activeIndex
		_expanded = State(initialValue: ancestors(of: activeIndex, in: entries))
	}

	var body: some View {
		ScrollViewReader { proxy in
			List(visibleIndices, id: \.self) { index in
				row(index)
			}
			.onAppear {
				proxy.scrollTo(activeIndex, anchor: .center)
				// Same delay as sheetAccessibilityFocus, so the push has settled before VoiceOver moves.
				DispatchQueue.main.asyncAfter(deadline: .now() + 0.35) {
					focusedIndex = activeIndex
				}
			}
		}
	}

	private func row(_ index: Int) -> some View {
		let entry = entries[index]
		let isActive = index == activeIndex
		let isParent = hasChildren(index)
		let isExpanded = expanded.contains(index)
		// TRANSLATORS: VoiceOver value of a table of contents entry, saying whether its children are shown
		let expansionState = isParent ? (isExpanded ? t("Expanded") : t("Collapsed")) : ""
		return HStack(spacing: 8) {
			if isParent {
				Button {
					toggle(index)
				} label: {
					Image(systemName: isExpanded ? "chevron.down" : "chevron.right")
						.frame(width: chevronWidth)
				}
				.buttonStyle(.borderless)
				// VoiceOver gets expand and collapse as actions on the row, so this would only be a duplicate stop.
				.accessibilityHidden(true)
			} else {
				Spacer()
					.frame(width: chevronWidth)
			}
			Button {
				viewModel.reading.goToPosition(entry.position, announce: true)
				dismiss()
			} label: {
				HStack {
					Text(entry.title)
						.fontWeight(isActive ? .semibold : .regular)
						.foregroundStyle(isActive ? Color.accentColor : Color.primary)
					Spacer()
				}
				.contentShape(Rectangle())
			}
			.buttonStyle(.plain)
			.accessibilityAddTraits(isActive ? .isSelected : [])
			.accessibilityValue(expansionState)
			.accessibilityActions {
				if isParent {
					// TRANSLATORS: VoiceOver action on a table of contents entry that shows or hides its children
					Button(isExpanded ? t("Collapse") : t("Expand")) {
						toggle(index)
					}
				}
			}
			.accessibilityFocused($focusedIndex, equals: index)
		}
		.padding(.leading, CGFloat(entry.level) * indentPerLevel)
		.id(index)
	}

	// Entries under a collapsed parent are skipped until the level climbs back to the parent's.
	private var visibleIndices: [Int] {
		var result: [Int] = []
		var collapsedLevel: Int32?
		for index in entries.indices {
			let level = entries[index].level
			if let collapsedLevel, level > collapsedLevel {
				continue
			}
			collapsedLevel = nil
			result.append(index)
			if hasChildren(index) && !expanded.contains(index) {
				collapsedLevel = level
			}
		}
		return result
	}

	private func hasChildren(_ index: Int) -> Bool {
		index + 1 < entries.count && entries[index + 1].level > entries[index].level
	}

	private func toggle(_ index: Int) {
		expanded.formSymmetricDifference([index])
	}
}

// The entries the given one sits under, which have to be expanded for it to be visible when the list opens.
private func ancestors(of index: Int, in entries: [TocEntry]) -> Set<Int> {
	var result: Set<Int> = []
	var level = entries[index].level
	for candidate in stride(from: index - 1, through: 0, by: -1) where entries[candidate].level < level {
		result.insert(candidate)
		level = entries[candidate].level
	}
	return result
}
