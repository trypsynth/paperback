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
					// TRANSLATORS: Placeholder text for the password entry field
					SecureField(t("Password"), text: $password)
						.focused($isFocused)
						.onSubmit { submit() }
				} footer: {
					if let url = viewModel.passwordPromptUrl {
						// TRANSLATORS: Footer text naming the file that needs a password; {} is the file name
						Text(t("Enter password for {}").replacingOccurrences(of: "{}", with: url.lastPathComponent))
					}
				}
			}
			// TRANSLATORS: Navigation title of the sheet prompting for a document's password
			.navigationTitle(t("Password Required"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					// TRANSLATORS: Button that dismisses the password prompt without opening the document
					Button(t("Cancel")) { dismiss() }
				}
				ToolbarItem(placement: .confirmationAction) {
					// TRANSLATORS: Button that submits the entered password and opens the document
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
