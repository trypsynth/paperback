import SwiftUI

// The same keys and history length Android's Find dialog uses, so the options and the history
// mean the same thing on both apps.
private let matchCaseKey = "find_match_case"
private let wholeWordKey = "find_whole_word"
private let useRegexKey = "find_use_regex"
private let historyLength: Int32 = 10

struct FindView: View {
	@Environment(AppViewModel.self) private var viewModel
	@State private var query = ""
	@State private var matchCase = false
	@State private var wholeWord = false
	@State private var useRegex = false
	@State private var history: [String] = []
	@FocusState private var queryFocused: Bool

	var body: some View {
		Form {
			Section {
				// TRANSLATORS: Placeholder text shown in the empty search field of the Find screen
				TextField(t("Search..."), text: $query)
					.autocorrectionDisabled()
					.textInputAutocapitalization(.never)
					.focused($queryFocused)
					// Labels the keyboard's return key "Search" rather than "return", which is
					// what the field's submit action actually does.
					.submitLabel(.search)
					.onSubmit { find(forward: true) }
				if !history.isEmpty {
					Menu {
						ForEach(history, id: \.self) { term in
							Button(term) { query = term }
						}
					} label: {
						// TRANSLATORS: Button that opens a dropdown of previously used search terms
						Text(t("Search History"))
					}
				}
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
			let config = viewModel.configManager
			query = viewModel.reading.activeSearchQuery ?? ""
			matchCase = config.getAppBool(key: matchCaseKey, defaultValue: false)
			wholeWord = config.getAppBool(key: wholeWordKey, defaultValue: false)
			useRegex = config.getAppBool(key: useRegexKey, defaultValue: false)
			history = config.getFindHistory()
			queryFocused = true
		}
		.sheetAccessibilityFocus(title: t("Find"))
	}

	private func find(forward: Bool) {
		let trimmed = query.trimmingCharacters(in: .whitespaces)
		guard !trimmed.isEmpty else { return }
		let config = viewModel.configManager
		config.setAppBool(key: matchCaseKey, value: matchCase)
		config.setAppBool(key: wholeWordKey, value: wholeWord)
		config.setAppBool(key: useRegexKey, value: useRegex)
		config.addFindHistory(text: trimmed, maxLen: historyLength)
		history = config.getFindHistory()
		viewModel.reading.startSearch(
			query: trimmed,
			options: SearchOptions(matchCase: matchCase, wholeWord: wholeWord, regex: useRegex),
			forward: forward
		)
	}
}
