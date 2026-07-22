import SwiftUI

struct GoToSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss
	@Environment(\.accessibilityVoiceOverEnabled) private var voiceOverEnabled
	@State private var mode: GoToMode = .line
	@State private var lineValue = ""
	@State private var pageValue = ""
	@State private var percentValue: Double = 50
	@FocusState private var fieldFocused: Bool

	private var session: DocumentSession? { viewModel.activeSession }
	private var hasPages: Bool { (session?.pageCountFfi() ?? 0) > 0 }

	var body: some View {
		NavigationStack {
			Form {
				Section {
					// TRANSLATORS: Label for the wheel picker that chooses whether Go To navigates by line, page, or percent
					Picker(t("Mode"), selection: $mode) {
						// TRANSLATORS: Go To mode option: navigate to a specific line number
						Text(t("Line")).tag(GoToMode.line)
						if hasPages {
							// TRANSLATORS: Go To mode option: navigate to a specific page number
							Text(t("Page")).tag(GoToMode.page)
						}
						// TRANSLATORS: Go To mode option: navigate to a percentage through the document
						Text(t("Percent")).tag(GoToMode.percent)
					}
					.pickerStyle(.wheel)
					.labelsHidden()
					.frame(height: 120)
				}
				Section {
					switch mode {
					case .line:
						// TRANSLATORS: Placeholder for the line number input field in Go To
						TextField(t("Line number"), text: $lineValue)
							.keyboardType(.numberPad)
							.focused($fieldFocused)
					case .page:
						// TRANSLATORS: Placeholder for the page number input field in Go To
						TextField(t("Page number"), text: $pageValue)
							.keyboardType(.numberPad)
							.focused($fieldFocused)
					case .percent:
						HStack {
							Slider(value: $percentValue, in: 0...100, step: 1)
								// TRANSLATORS: VoiceOver label for the slider that picks a percentage through the document
								.accessibilityLabel(t("Percentage"))
								.accessibilityValue("\(Int(percentValue))%")
							Text("\(Int(percentValue))%")
								.monospacedDigit()
								.frame(width: 44, alignment: .trailing)
								.foregroundStyle(.secondary)
								.accessibilityHidden(true)
						}
					}
				}
			}
			// TRANSLATORS: Navigation title of the Go To sheet
			.navigationTitle(t("Go To"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					// TRANSLATORS: Button that dismisses the Go To sheet without navigating
					Button(t("Cancel")) { dismiss() }
				}
				ToolbarItem(placement: .confirmationAction) {
					// TRANSLATORS: Button that performs the Go To navigation using the entered line/page/percent
					Button(t("Go")) { go() }
						.disabled(!canGo)
				}
			}
			.onAppear { populate() }
			.onChange(of: mode) { _ in viewModel.goToInitialMode = mode }
		}
		.sheetAccessibilityFocus(title: "Go To")
	}

	private var canGo: Bool {
		switch mode {
		case .line:    return Int64(lineValue) != nil
		case .page:    return Int64(pageValue) != nil
		case .percent: return true
		}
	}

	private func populate() {
		let initialMode = viewModel.goToInitialMode
		mode = (initialMode == .page && !hasPages) ? .line : initialMode

		guard let session else { return }
		let pos = viewModel.ttsPosition
		let status = session.getStatusInfoFfi(position: pos)
		lineValue = "\(status.lineNumber)"
		percentValue = Double(status.percentage)
		if hasPages {
			pageValue = "\(session.currentPageFfi(position: pos))"
		}

		if !voiceOverEnabled { fieldFocused = mode != .percent }
	}

	private func go() {
		switch mode {
		case .line:
			guard let n = Int64(lineValue) else { dismiss(); return }
			viewModel.goToLine(n)
		case .page:
			guard let n = Int64(pageValue) else { dismiss(); return }
			viewModel.goToPage(Int32(n))
		case .percent:
			viewModel.goToPercent(Int32(percentValue))
		}
		dismiss()
	}
}
