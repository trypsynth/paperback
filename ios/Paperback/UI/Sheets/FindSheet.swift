import SwiftUI

struct FindSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss
	@State private var query = ""
	@State private var matchCase = false
	@State private var wholeWord = false
	@State private var useRegex = false
	@FocusState private var queryFocused: Bool

	var body: some View {
		NavigationStack {
			Form {
				Section {
					TextField(t("Search…"), text: $query)
						.autocorrectionDisabled()
						.textInputAutocapitalization(.never)
						.focused($queryFocused)
						.onSubmit { search() }
				}
				Section(t("Options")) {
					Toggle(t("Match Case"), isOn: $matchCase)
					Toggle(t("Whole Word"), isOn: $wholeWord)
					Toggle(t("Regular Expression"), isOn: $useRegex)
				}
				if viewModel.activeSearchQuery != nil {
					Section {
						Button { viewModel.findPrev() } label: {
							Label(t("Previous result"), systemImage: "chevron.up")
						}
						Button { viewModel.findNext() } label: {
							Label(t("Next result"), systemImage: "chevron.down")
						}
					}
				}
			}
			.navigationTitle(t("Find"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					Button(t("Cancel")) {
						viewModel.clearSearch()
						dismiss()
					}
				}
				ToolbarItem(placement: .confirmationAction) {
					Button(t("Search")) { search() }
						.disabled(query.trimmingCharacters(in: .whitespaces).isEmpty)
				}
			}
			.onAppear {
				query = viewModel.activeSearchQuery ?? ""
				matchCase = viewModel.searchOptions.matchCase
				wholeWord = viewModel.searchOptions.wholeWord
				useRegex = viewModel.searchOptions.regex
				queryFocused = true
			}
		}
	}

	private func search() {
		let trimmed = query.trimmingCharacters(in: .whitespaces)
		guard !trimmed.isEmpty else { return }
		viewModel.startSearch(
			query: trimmed,
			options: SearchOptions(matchCase: matchCase, wholeWord: wholeWord, regex: useRegex)
		)
		dismiss()
	}
}
