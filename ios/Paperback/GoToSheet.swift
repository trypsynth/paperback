import SwiftUI

struct GoToSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss
	@State private var mode: GoToMode = .line
	@State private var value = ""
	@FocusState private var valueFocused: Bool

	var body: some View {
		NavigationStack {
			Form {
				Section {
					Picker("Go to", selection: $mode) {
						Text("Line").tag(GoToMode.line)
						Text("Page").tag(GoToMode.page)
						Text("Percent").tag(GoToMode.percent)
					}
					.pickerStyle(.segmented)
					.listRowBackground(Color.clear)
					.listRowInsets(.init())
					.padding(.vertical, 4)
				}
				Section {
					TextField(placeholder, text: $value)
						.keyboardType(.numberPad)
						.focused($valueFocused)
				}
			}
			.navigationTitle("Go To")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					Button("Cancel") { dismiss() }
				}
				ToolbarItem(placement: .confirmationAction) {
					Button("Go") { go() }
						.disabled(value.isEmpty)
				}
			}
			.onAppear {
				mode = viewModel.goToInitialMode
				valueFocused = true
			}
		}
	}

	private var placeholder: String {
		switch mode {
		case .line:    return "Line number"
		case .page:    return "Page number"
		case .percent: return "0 – 100"
		}
	}

	private func go() {
		guard let n = Int64(value) else { dismiss(); return }
		switch mode {
		case .line:    viewModel.goToLine(n)
		case .page:    viewModel.goToPage(Int32(n))
		case .percent: viewModel.goToPercent(Int32(min(100, max(0, n))))
		}
		dismiss()
	}
}
