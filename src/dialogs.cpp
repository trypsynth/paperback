/* dialogs.cpp - all of Paperback's wxDialog implementations.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "dialogs.hpp"
#include "config_manager.hpp"
#include "constants.hpp"

dialog::dialog(wxWindow* parent, const wxString& title, dialog_button_config buttons) : wxDialog(parent, wxID_ANY, title), button_config{buttons} {
	main_sizer = new wxBoxSizer(wxVERTICAL);
	SetSizer(main_sizer);
}

void dialog::set_content(wxSizer* content_sizer) {
	if (layout_finalized) return;
	main_sizer->Add(content_sizer, 1, wxEXPAND | wxALL, 10);
}

void dialog::finalize_layout() {
	if (layout_finalized) return;
	create_buttons();
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	SetSizerAndFit(main_sizer);
	CentreOnParent();
	layout_finalized = true;
}

void dialog::create_buttons() {
	button_sizer = new wxStdDialogButtonSizer();
	auto* ok_button = new wxButton(this, wxID_OK);
	button_sizer->AddButton(ok_button);
	if (button_config == dialog_button_config::ok_cancel)
		button_sizer->AddButton(new wxButton(this, wxID_CANCEL));
	ok_button->SetDefault();
	button_sizer->Realize();
}

bookmark_dialog::bookmark_dialog(wxWindow* parent, const wxArrayLong& bookmarks, wxTextCtrl* text_ctrl, long current_pos) : dialog(parent, "Jump to Bookmark"), bookmark_positions(bookmarks), selected_position{-1} {
	bookmark_list = new wxListBox(this, wxID_ANY);
	int closest_index = -1;
	long closest_distance = LONG_MAX;
	for (size_t i = 0; i < bookmarks.GetCount(); ++i) {
		long pos = bookmarks[i];
		long line;
		text_ctrl->PositionToXY(pos, 0, &line);
		wxString line_text = text_ctrl->GetLineText(line);
		line_text = line_text.Strip(wxString::both);
		if (line_text.IsEmpty()) line_text = "blank";
		wxString bookmark_desc = wxString::Format("Bookmark %zu: %s", i + 1, line_text);
		bookmark_list->Append(bookmark_desc);
		if (current_pos >= 0) {
			long distance = std::abs(pos - current_pos);
			if (distance < closest_distance) {
				closest_distance = distance;
				closest_index = i;
			}
		}
	}
	if (closest_index >= 0) {
		bookmark_list->SetSelection(closest_index);
		selected_position = bookmarks[closest_index];
	}
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	content_sizer->Add(bookmark_list, 1, wxEXPAND);
	set_content(content_sizer);
	bookmark_list->Bind(wxEVT_LISTBOX, &bookmark_dialog::on_list_selection_changed, this);
	Bind(wxEVT_BUTTON, &bookmark_dialog::on_ok, this, wxID_OK);
	finalize_layout();
}

void bookmark_dialog::on_list_selection_changed(wxCommandEvent& event) {
	int selection = bookmark_list->GetSelection();
	if (selection >= 0 && selection < static_cast<int>(bookmark_positions.GetCount())) selected_position = bookmark_positions[selection];
}

void bookmark_dialog::on_ok(wxCommandEvent& event) {
	if (selected_position >= 0)
		EndModal(wxID_OK);
	else
		wxMessageBox("Please select a bookmark to jump to.", "error", wxICON_ERROR);
}

document_info_dialog::document_info_dialog(wxWindow* parent, const document* doc) : dialog(parent, "Document Info", dialog_button_config::ok_only) {
	info_text_ctrl = new wxTextCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxSize(600, 400), wxTE_MULTILINE | wxTE_READONLY);
	wxString info_text;
	info_text << "Title: " << doc->title << "\n";
	info_text << "Author: " << doc->author << "\n";
	info_text << "Total number of words: " << doc->stats.word_count << ".\n";
	info_text << "Total number of lines: " << doc->stats.line_count << ".\n";
	info_text << "Total number of characters: " << doc->stats.char_count << ".\n";
	info_text << "Total number of characters (excluding whitespace): " << doc->stats.char_count_no_whitespace << ".\n";
	info_text_ctrl->SetValue(info_text);
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	content_sizer->Add(info_text_ctrl, 1, wxEXPAND);
	set_content(content_sizer);
	finalize_layout();
}

find_dialog::find_dialog(wxWindow* parent) : wxDialog(parent, wxID_ANY, "Find") {
	auto* const main_sizer = new wxBoxSizer(wxVERTICAL);
	auto* const find_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* const find_label = new wxStaticText(this, wxID_ANY, "Find &what:");
	find_what_combo = new wxComboBox(this, wxID_ANY, "", wxDefaultPosition, wxSize(250, -1), 0, nullptr, wxTE_PROCESS_ENTER);
	find_sizer->Add(find_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 10);
	find_sizer->Add(find_what_combo, 1, wxEXPAND);
	auto* const options_box = new wxStaticBoxSizer(wxVERTICAL, this, "Options");
	match_case_check = new wxCheckBox(this, wxID_ANY, "&Match case");
	match_whole_word_check = new wxCheckBox(this, wxID_ANY, "Match &whole word");
	use_regex_check = new wxCheckBox(this, wxID_ANY, "Use &regular expressions");
	options_box->Add(match_case_check, 0, wxALL, 2);
	options_box->Add(match_whole_word_check, 0, wxALL, 2);
	options_box->Add(use_regex_check, 0, wxALL, 2);
	auto* const button_sizer = new wxBoxSizer(wxHORIZONTAL);
	find_previous_btn = new wxButton(this, wxID_ANY, "Find &Previous");
	find_next_btn = new wxButton(this, wxID_ANY, "Find &Next");
	cancel_btn = new wxButton(this, wxID_CANCEL, "Cancel");
	button_sizer->Add(find_previous_btn, 0, wxRIGHT, 5);
	button_sizer->Add(find_next_btn, 0, wxRIGHT, 5);
	button_sizer->AddStretchSpacer();
	button_sizer->Add(cancel_btn, 0);
	find_next_btn->SetDefault();
	main_sizer->Add(find_sizer, 0, wxEXPAND | wxALL, 10);
	main_sizer->Add(options_box, 0, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, 10);
	main_sizer->Add(button_sizer, 0, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, 10);
	SetSizer(main_sizer);
	find_previous_btn->Bind(wxEVT_BUTTON, &find_dialog::on_find_previous, this);
	find_next_btn->Bind(wxEVT_BUTTON, &find_dialog::on_find_next, this);
	cancel_btn->Bind(wxEVT_BUTTON, &find_dialog::on_cancel, this);
	find_what_combo->Bind(wxEVT_TEXT_ENTER, &find_dialog::on_find_text_enter, this);
	Bind(wxEVT_CLOSE_WINDOW, &find_dialog::on_close, this);
	find_what_combo->SetFocus();
	Fit();
	CenterOnParent();
}

wxString find_dialog::get_find_text() const {
	return find_what_combo->GetValue();
}

bool find_dialog::get_match_case() const noexcept {
	return match_case_check->GetValue();
}

bool find_dialog::get_match_whole_word() const noexcept {
	return match_whole_word_check->GetValue();
}

bool find_dialog::get_use_regex() const noexcept {
	return use_regex_check->GetValue();
}

void find_dialog::set_find_text(const wxString& text) {
	find_what_combo->SetValue(text);
	find_what_combo->SetSelection(-1, -1);
}

void find_dialog::add_to_history(const wxString& text) {
	if (text.IsEmpty()) return;
	const int existing = find_what_combo->FindString(text);
	if (existing != wxNOT_FOUND)
		find_what_combo->Delete(existing);
	find_what_combo->Insert(text, 0);
	while (find_what_combo->GetCount() > MAX_FIND_HISTORY_SIZE)
		find_what_combo->Delete(find_what_combo->GetCount() - 1);
	find_what_combo->SetValue(text);
}

void find_dialog::focus_find_text() {
	find_what_combo->SetFocus();
	find_what_combo->SetSelection(-1, -1);
}

void find_dialog::on_find_previous(wxCommandEvent& event) {
	const wxString text = get_find_text();
	if (!text.IsEmpty()) {
		add_to_history(text);
		wxCommandEvent find_event(wxEVT_COMMAND_MENU_SELECTED, ID_FIND_PREVIOUS);
		wxPostEvent(GetParent(), find_event);
	}
}

void find_dialog::on_find_next(wxCommandEvent& event) {
	const wxString text = get_find_text();
	if (!text.IsEmpty()) {
		add_to_history(text);
		wxCommandEvent find_event(wxEVT_COMMAND_MENU_SELECTED, ID_FIND_NEXT);
		wxPostEvent(GetParent(), find_event);
	}
}

void find_dialog::on_cancel(wxCommandEvent& event) {
	Hide();
}

void find_dialog::on_find_text_enter(wxCommandEvent& event) {
	on_find_next(event);
}

void find_dialog::on_close(wxCloseEvent& event) {
	Hide();
}

go_to_line_dialog::go_to_line_dialog(wxWindow* parent, wxTextCtrl* text_ctrl) : dialog(parent, "Go to Line"), textbox{text_ctrl} {
	auto* line_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, "&Line number:");
	long line;
	textbox->PositionToXY(textbox->GetInsertionPoint(), 0, &line);
	input_ctrl = new wxSpinCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxSP_ARROW_KEYS, 1, textbox->GetNumberOfLines(), line + 1);
	line_sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 5);
	line_sizer->Add(input_ctrl, 1, wxEXPAND);
	set_content(line_sizer);
	finalize_layout();
}

long go_to_line_dialog::get_position() const {
	long line = input_ctrl->GetValue();
	if (line >= 1 && line <= textbox->GetNumberOfLines())
		return textbox->XYToPosition(0, line - 1);
	return textbox->GetInsertionPoint();
}

long go_to_line_dialog::get_max_line() const {
	return textbox->GetNumberOfLines();
}

go_to_page_dialog::go_to_page_dialog(wxWindow* parent, document* doc, const parser* par, int current_page) : dialog(parent, "Go to page"), doc_{doc}, parser_{par} {
	auto* page_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, wxString::Format("Go to page (1/%d):", get_max_page()));
	input_ctrl = new wxSpinCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxSP_ARROW_KEYS, 1, get_max_page(), current_page);
	page_sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 5);
	page_sizer->Add(input_ctrl, 1, wxEXPAND);
	set_content(page_sizer);
	finalize_layout();
}

int go_to_page_dialog::get_page_number() const {
	long page = input_ctrl->GetValue();
	if (page >= 1 && page <= get_max_page())
		return static_cast<int>(page);
	return 1;
}

int go_to_page_dialog::get_max_page() const {
	if (!doc_ || !parser_) return 1;
	if (!parser_->has_flag(parser_flags::supports_pages)) return 1;
	return static_cast<int>(doc_->buffer.count_markers_by_type(marker_type::page_break));
}

go_to_percent_dialog::go_to_percent_dialog(wxWindow* parent, wxTextCtrl* text_ctrl) : dialog(parent, "Go to Percent"), textbox{text_ctrl} {
	long current_pos = textbox->GetInsertionPoint();
	long total_pos = textbox->GetLastPosition();
	int current_percent = total_pos > 0 ? static_cast<int>((current_pos * 100) / total_pos) : 0;
	auto* slider_label = new wxStaticText(this, wxID_ANY, "&Percent");
	percent_slider = new accessible_slider(this, wxID_ANY, current_percent, 0, 100);
	auto* input_label = new wxStaticText(this, wxID_ANY, "P&ercent:");
	input_ctrl = new wxSpinCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxSP_ARROW_KEYS, 0, 100, current_percent);
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	content_sizer->Add(slider_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 5);
	content_sizer->Add(percent_slider, 0, wxEXPAND | wxBOTTOM, 5);
	content_sizer->Add(input_label, 0, wxALIGN_CENTER_VERTICAL | wxLEFT, 5);
	content_sizer->Add(input_ctrl, 0, wxEXPAND);
	percent_slider->Bind(wxEVT_SLIDER, &go_to_percent_dialog::on_slider_changed, this);
	set_content(content_sizer);
	finalize_layout();
}

long go_to_percent_dialog::get_position() const {
	long percent = input_ctrl->GetValue();
	long total_chars = textbox->GetLastPosition();
	return (percent * total_chars + 100 - 1) / 100;
}

void go_to_percent_dialog::on_slider_changed(wxCommandEvent& event) {
	int slider_value = percent_slider->GetValue();
	input_ctrl->SetValue(slider_value);
}

open_as_dialog::open_as_dialog(wxWindow* parent, const wxString& path) : dialog(parent, "Open As") {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	auto* label = new wxStaticText(this, wxID_ANY, wxString::Format("No suitable parser was found for %s.\nHow would you like to open this file?", path));
	content_sizer->Add(label, 0, wxALL, 5);
	auto* format_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* format_label = new wxStaticText(this, wxID_ANY, "Open &as:");
	format_combo = new wxComboBox(this, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, 0, nullptr, wxCB_READONLY);
	format_combo->Append("Plain Text");
	format_combo->Append("HTML");
	format_combo->Append("Markdown");
	format_combo->SetSelection(0);
	format_sizer->Add(format_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 10);
	format_sizer->Add(format_combo, 1, wxEXPAND);
	content_sizer->Add(format_sizer, 0, wxEXPAND | wxALL, 5);
	set_content(content_sizer);
	finalize_layout();
	format_combo->SetFocus();
}

wxString open_as_dialog::get_selected_format() const {
	const int selection = format_combo->GetSelection();
	switch (selection) {
		case 0:
			return "txt";
		case 1:
			return "html";
		case 2:
			return "md";
		default:
			return "txt";
	}
}

options_dialog::options_dialog(wxWindow* parent) : dialog(parent, "Options") {
	auto* general_box = new wxStaticBoxSizer(wxVERTICAL, this, "General");
	restore_docs_check = new wxCheckBox(this, wxID_ANY, "&Restore previously opened documents on startup");
	general_box->Add(restore_docs_check, 0, wxALL, 5);
	word_wrap_check = new wxCheckBox(this, wxID_ANY, "&Word wrap");
	general_box->Add(word_wrap_check, 0, wxALL, 5);
	set_content(general_box);
	Bind(wxEVT_BUTTON, &options_dialog::on_ok, this, wxID_OK);
	Bind(wxEVT_BUTTON, &options_dialog::on_cancel, this, wxID_CANCEL);
	finalize_layout();
}

bool options_dialog::get_restore_previous_documents() const {
	return restore_docs_check ? restore_docs_check->GetValue() : false;
}

void options_dialog::set_restore_previous_documents(bool restore) {
	if (restore_docs_check) restore_docs_check->SetValue(restore);
}

bool options_dialog::get_word_wrap() const {
	return word_wrap_check ? word_wrap_check->GetValue() : false;
}

void options_dialog::set_word_wrap(bool word_wrap) {
	if (word_wrap_check) word_wrap_check->SetValue(word_wrap);
}

void options_dialog::on_ok(wxCommandEvent& event) {
	EndModal(wxID_OK);
}

void options_dialog::on_cancel(wxCommandEvent& event) {
	EndModal(wxID_CANCEL);
}

toc_dialog::toc_dialog(wxWindow* parent, const document* doc, int current_offset) : dialog(parent, "Table of Contents"), selected_offset{-1} {
	tree = new wxTreeCtrl(this, wxID_ANY, wxDefaultPosition, wxDefaultSize, wxTR_HIDE_ROOT);
	wxTreeItemId root = tree->AddRoot("Root");
	populate_tree(doc->toc_items, root);
	if (current_offset != -1) find_and_select_item(root, current_offset);
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	content_sizer->Add(tree, 1, wxEXPAND);
	set_content(content_sizer);
	Bind(wxEVT_TREE_SEL_CHANGED, &toc_dialog::on_tree_selection_changed, this);
	Bind(wxEVT_TREE_ITEM_ACTIVATED, &toc_dialog::on_tree_item_activated, this, wxID_ANY);
	Bind(wxEVT_BUTTON, &toc_dialog::on_ok, this, wxID_OK);
	finalize_layout();
}

void toc_dialog::populate_tree(const std::vector<std::unique_ptr<toc_item>>& items, const wxTreeItemId& parent) {
	for (const auto& item : items) {
		wxString display_text = item->name.IsEmpty() ? wxString("Untitled") : item->name;
		wxTreeItemId item_id = tree->AppendItem(parent, display_text);
		tree->SetItemData(item_id, new toc_tree_item_data(item->offset));
		if (!item->children.empty())
			populate_tree(item->children, item_id);
	}
}

void toc_dialog::find_and_select_item(const wxTreeItemId& parent, int offset) {
	wxTreeItemIdValue cookie;
	for (wxTreeItemId item_id = tree->GetFirstChild(parent, cookie); item_id.IsOk(); item_id = tree->GetNextChild(parent, cookie)) {
		auto* data = dynamic_cast<toc_tree_item_data*>(tree->GetItemData(item_id));
		if (data && data->offset == offset) {
			tree->SelectItem(item_id);
			tree->SetFocusedItem(item_id);
			tree->EnsureVisible(item_id);
			selected_offset = data->offset;
			return;
		}
		if (tree->ItemHasChildren(item_id))
			find_and_select_item(item_id, offset);
	}
}

void toc_dialog::on_tree_selection_changed(wxTreeEvent& event) {
	const wxTreeItemId item = event.GetItem();
	if (!item.IsOk()) return;
	auto* data = dynamic_cast<toc_tree_item_data*>(tree->GetItemData(item));
	if (!data) return;
	selected_offset = data->offset;
}

void toc_dialog::on_tree_item_activated(wxTreeEvent& event) {
	if (selected_offset >= 0)
		EndModal(wxID_OK);
}

void toc_dialog::on_ok(wxCommandEvent& event) {
	if (selected_offset >= 0)
		EndModal(wxID_OK);
	else
		wxMessageBox("Please select a section from the table of contents.", "No Selection", wxOK | wxICON_INFORMATION, this);
}
