import SwiftUI
import UIKit

struct SettingsSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss
	@State private var restore = true
	@State private var showTtsConfig = false

	var body: some View {
		NavigationStack {
			Form {
				Section("Behaviour") {
					Toggle("Restore last open documents", isOn: $restore)
				}
				Section("Text-to-Speech") {
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
				DispatchQueue.main.asyncAfter(deadline: .now() + 0.6) {
					UIAccessibility.post(notification: .screenChanged, argument: nil)
				}
			}
		}
	}
}
