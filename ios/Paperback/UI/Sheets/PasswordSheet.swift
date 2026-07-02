import SwiftUI

struct PasswordSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	@State private var password = ""
	@FocusState private var isFocused: Bool

	var body: some View {
		NavigationStack {
			Form {
				Section {
					SecureField(t("Password"), text: $password)
						.focused($isFocused)
						.onSubmit { submit() }
				} footer: {
					if let url = viewModel.passwordPromptUrl {
						Text("Enter password for \(url.lastPathComponent)")
					}
				}
			}
			.navigationTitle(t("Password Required"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					Button(t("Cancel")) { dismiss() }
				}
				ToolbarItem(placement: .confirmationAction) {
					Button(t("Open")) { submit() }
						.disabled(password.isEmpty)
				}
			}
			.onAppear { isFocused = true }
		}
	}

	private func submit() {
		guard !password.isEmpty, let url = viewModel.passwordPromptUrl else { return }
		viewModel.openDocument(url: url, password: password)
		dismiss()
	}
}
