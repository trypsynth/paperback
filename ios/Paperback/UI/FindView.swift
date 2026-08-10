import SwiftUI

struct FindView: View {
	@EnvironmentObject var viewModel: AppViewModel
	@State private var query = ""
	@State private var matchCase = false
	@State private var wholeWord = false
	@State private var useRegex = false
	@FocusState private var queryFocused: Bool

	var body: some View {
		Form {
			Section {
				// TRANSLATORS: Placeholder text shown in the empty search field of the Find screen
				TextField(t("Search…"), text: $query)
					.autocorrectionDisabled()
					.textInputAutocapitalization(.never)
					.focused($queryFocused)
					.onSubmit { find(forward: true) }
			}
			Section {
				// TRANSLATORS: Toggle label; when on, search matching is case-sensitive
				Toggle(t("Match Case"), isOn: $matchCase)
				// TRANSLATORS: Toggle label; when on, search only matches whole words
				Toggle(t("Whole Word"), isOn: $wholeWord)
				// TRANSLATORS: Toggle label; when on, the search query is treated as a regular expression
				Toggle(t("Regular Expression"), isOn: $useRegex)
			}
		}
		.safeAreaInset(edge: .bottom) {
			HStack {
				Button { find(forward: false) } label: {
					// TRANSLATORS: Button that searches backward and jumps to the previous match
					Label(t("Find Previous"), systemImage: "chevron.up")
						.labelStyle(.titleAndIcon)
						.frame(maxWidth: .infinity)
				}
				Button { find(forward: true) } label: {
					// TRANSLATORS: Button that searches forward and jumps to the next match
					Label(t("Find Next"), systemImage: "chevron.down")
						.labelStyle(.titleAndIcon)
						.frame(maxWidth: .infinity)
				}
			}
			.buttonStyle(.bordered)
			.controlSize(.large)
			.disabled(query.trimmingCharacters(in: .whitespaces).isEmpty)
			.padding()
			.background(.bar)
		}
		// TRANSLATORS: Navigation title of the Find screen
		.navigationTitle(t("Find"))
		.navigationBarTitleDisplayMode(.inline)
		.onAppear {
			query = viewModel.activeSearchQuery ?? ""
			matchCase = viewModel.searchOptions.matchCase
			wholeWord = viewModel.searchOptions.wholeWord
			useRegex = viewModel.searchOptions.regex
			queryFocused = true
		}
		.sheetAccessibilityFocus(title: "Find")
	}

	private func find(forward: Bool) {
		let trimmed = query.trimmingCharacters(in: .whitespaces)
		guard !trimmed.isEmpty else { return }
		viewModel.startSearch(
			query: trimmed,
			options: SearchOptions(matchCase: matchCase, wholeWord: wholeWord, regex: useRegex),
			forward: forward
		)
	}
}
