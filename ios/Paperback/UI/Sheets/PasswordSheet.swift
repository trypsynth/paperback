import SwiftUI

struct PasswordSheet: View {
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss

	@State private var password = ""
	@State private var wasWrong = false
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
					if wasWrong {
						Text(t("Password incorrect"))
					} else if let url = viewModel.navigation.passwordPromptUrl {
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

	/// A wrong password leaves the sheet up to try again.
	private func submit() {
		guard !password.isEmpty, let url = viewModel.navigation.passwordPromptUrl else { return }
		guard viewModel.openDocument(url: url, password: password) else {
			password = ""
			wasWrong = true
			UIAccessibility.post(notification: .announcement, argument: t("Password incorrect"))
			return
		}
		dismiss()
	}
}
