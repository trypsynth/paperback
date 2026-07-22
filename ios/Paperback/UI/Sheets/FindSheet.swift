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
					// TRANSLATORS: Placeholder text shown in the empty search field of the Find sheet
					TextField(t("Search…"), text: $query)
						.autocorrectionDisabled()
						.textInputAutocapitalization(.never)
						.focused($queryFocused)
						.onSubmit { search() }
				}
				// TRANSLATORS: Section header grouping the Find sheet's match-case/whole-word/regex toggles
				Section(t("Options")) {
					// TRANSLATORS: Toggle label; when on, search matching is case-sensitive
					Toggle(t("Match Case"), isOn: $matchCase)
					// TRANSLATORS: Toggle label; when on, search only matches whole words
					Toggle(t("Whole Word"), isOn: $wholeWord)
					// TRANSLATORS: Toggle label; when on, the search query is treated as a regular expression
					Toggle(t("Regular Expression"), isOn: $useRegex)
				}
				if viewModel.activeSearchQuery != nil {
					Section {
						Button { viewModel.findPrev() } label: {
							// TRANSLATORS: Button that jumps to the previous search match
							Label(t("Previous result"), systemImage: "chevron.up")
						}
						Button { viewModel.findNext() } label: {
							// TRANSLATORS: Button that jumps to the next search match
							Label(t("Next result"), systemImage: "chevron.down")
						}
					}
				}
			}
			// TRANSLATORS: Navigation title of the Find sheet
			.navigationTitle(t("Find"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					// TRANSLATORS: Button that dismisses the Find sheet and clears the active search
					Button(t("Cancel")) {
						viewModel.clearSearch()
						dismiss()
					}
				}
				ToolbarItem(placement: .confirmationAction) {
					// TRANSLATORS: Button that starts the search using the entered query
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
