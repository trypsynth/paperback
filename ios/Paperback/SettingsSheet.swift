import SwiftUI

struct SettingsSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss
	@State private var restore = true
	@State private var showTtsConfig = false

	var body: some View {
		NavigationStack {
			Form {
				Section {
					Toggle("Restore last open documents", isOn: $restore)
				}
				Section {
					Button("TTS Settings…") { showTtsConfig = true }
				}
			}
			.navigationTitle("Settings")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					Button("Done") {
						viewModel.restorePreviousDocuments = restore
						dismiss()
					}
				}
			}
			.sheet(isPresented: $showTtsConfig) {
				TtsConfigSheet().environmentObject(viewModel)
			}
			.onAppear {
				restore = viewModel.restorePreviousDocuments
			}
		}
		.sheetAccessibilityFocus(title: "Settings")
	}
}
