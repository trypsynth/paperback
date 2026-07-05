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
					Picker(t("Mode"), selection: $mode) {
						Text(t("Line")).tag(GoToMode.line)
						if hasPages {
							Text(t("Page")).tag(GoToMode.page)
						}
						Text(t("Percent")).tag(GoToMode.percent)
					}
					.pickerStyle(.wheel)
					.labelsHidden()
					.frame(height: 120)
				}
				Section {
					switch mode {
					case .line:
						TextField(t("Line number"), text: $lineValue)
							.keyboardType(.numberPad)
							.focused($fieldFocused)
					case .page:
						TextField(t("Page number"), text: $pageValue)
							.keyboardType(.numberPad)
							.focused($fieldFocused)
					case .percent:
						HStack {
							Slider(value: $percentValue, in: 0...100, step: 1)
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
			.navigationTitle(t("Go To"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					Button(t("Cancel")) { dismiss() }
				}
				ToolbarItem(placement: .confirmationAction) {
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
