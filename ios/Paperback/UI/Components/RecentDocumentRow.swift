import SwiftUI

struct RecentDocumentRow: View {
	let doc: RecentDocument
	var showClosedStatus: Bool = true
	// The empty-state preview isn't a List, so it has no swipe action and this button is its only way to remove. Off, the row also leaves Remove to the swipe action VoiceOver offers.
	var showRemoveButton: Bool = true
	let onOpen: () -> Void
	let onRemove: () -> Void
	let onLocate: (() -> Void)?

	var body: some View {
		if doc.isMissing, let onLocate {
			content
				.accessibilityAction(named: t("Locate"), onLocate)
				.modifier(RemoveAction(isNeeded: showRemoveButton, onRemove: onRemove))
		} else {
			content
				.accessibilityAddTraits(.isButton)
				.accessibilityAction(.default, onOpen)
				.modifier(RemoveAction(isNeeded: showRemoveButton, onRemove: onRemove))
		}
	}

	private var content: some View {
		HStack(alignment: .top, spacing: 12) {
			VStack(alignment: .leading, spacing: 2) {
				Text(doc.title)
					.foregroundStyle(doc.isMissing ? .secondary : .primary)
					.lineLimit(1)
				statusText
			}
			.frame(maxWidth: .infinity, alignment: .leading)
			.contentShape(Rectangle())
			.onTapGesture {
				if !doc.isMissing { onOpen() }
			}
			if doc.isMissing, let onLocate {
				// TRANSLATORS: Button to pick a new file location for a recent document that can no longer be found
				Button(t("Locate"), action: onLocate)
					.font(.footnote)
					.buttonStyle(.borderless)
			}
			if showRemoveButton {
				// TRANSLATORS: Button to remove a document from the recent documents list
				Button(t("Remove"), role: .destructive, action: onRemove)
					.font(.footnote)
					.buttonStyle(.borderless)
			}
		}
		.padding(.vertical, 8)
		.accessibilityElement(children: .combine)
	}

	@ViewBuilder private var statusText: some View {
		if doc.isMissing {
			// TRANSLATORS: Status label for a recent document whose file can no longer be found
			Text(t("File Missing"))
				.font(.caption)
				.foregroundStyle(.red)
		} else if doc.isOpen {
			// TRANSLATORS: Status label for a recent document that's currently open in a tab
			Text(t("Currently Open"))
				.font(.caption)
				.foregroundStyle(.secondary)
		} else if showClosedStatus {
			// TRANSLATORS: Status label for a recent document that isn't currently open
			Text(t("Closed"))
				.font(.caption)
				.foregroundStyle(.secondary)
		}
	}
}

/// A row in a List removes by its swipe action, which VoiceOver already offers as Remove, so a second action of that name would be read twice (#1333).
private struct RemoveAction: ViewModifier {
	let isNeeded: Bool
	let onRemove: () -> Void

	func body(content: Content) -> some View {
		if isNeeded {
			content.accessibilityAction(named: t("Remove"), onRemove)
		} else {
			content
		}
	}
}
