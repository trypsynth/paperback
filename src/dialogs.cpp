#include "dialogs.hpp"
#include "constants.hpp"

document_info_dialog::document_info_dialog(wxWindow* parent, const document* doc) : wxDialog(parent, wxID_ANY, "Document Info") {
	auto* main_sizer = new wxBoxSizer(wxVERTICAL);
	info_text_ctrl = new wxTextCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxSize(600, 400), wxTE_MULTILINE | wxTE_READONLY);
	wxString info_text;
	info_text << "Title: " << doc->title << "\n";
	info_text << "Author: " << doc->author << "\n";
	info_text << "Total number of words: " << doc->get_word_count() << ".\n";
	info_text << "Total number of lines: " << doc->get_line_count() << ".\n";
	info_text << "Total number of characters: " << doc->get_char_count() << ".\n";
	info_text << "Total number of characters (excluding whitespace): " << doc->get_char_count_no_whitespace() << ".\n";
	info_text_ctrl->SetValue(info_text);
	main_sizer->Add(info_text_ctrl, 1, wxEXPAND | wxALL, 10);
	auto* button_sizer = new wxStdDialogButtonSizer();
	button_sizer->AddButton(new wxButton(this, wxID_OK));
	button_sizer->Realize();
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	SetSizerAndFit(main_sizer);
	SetMinSize(wxSize(350, 250));
	CentreOnParent();
}

find_dialog::find_dialog(wxWindow* parent) : wxDialog(parent, wxID_ANY, "Find") {
	create_controls();
	bind_events();
	find_what_combo->SetFocus();
	Fit();
	CenterOnParent();
}

void find_dialog::create_controls() {
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
}

void find_dialog::bind_events() {
	find_previous_btn->Bind(wxEVT_BUTTON, &find_dialog::on_find_previous, this);
	find_next_btn->Bind(wxEVT_BUTTON, &find_dialog::on_find_next, this);
	cancel_btn->Bind(wxEVT_BUTTON, &find_dialog::on_cancel, this);
	find_what_combo->Bind(wxEVT_TEXT_ENTER, &find_dialog::on_find_text_enter, this);
	Bind(wxEVT_CLOSE_WINDOW, &find_dialog::on_close, this);
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
	while (find_what_combo->GetCount() > 10)
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

go_to_dialog::go_to_dialog(wxWindow* parent, wxTextCtrl* text_ctrl) : wxDialog(parent, wxID_ANY, "Go to"), textbox{text_ctrl} {
	auto* main_sizer = new wxBoxSizer(wxVERTICAL);
	auto* line_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, "Go to:");
	input_ctrl = new wxTextCtrl(this, wxID_ANY);
	line_sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 5);
	line_sizer->Add(input_ctrl, 1, wxEXPAND);
	long line;
	textbox->PositionToXY(textbox->GetInsertionPoint(), 0, &line);
	input_ctrl->SetValue(wxString::Format("%d", line + 1));
	input_ctrl->SetSelection(-1, -1);
	input_ctrl->Bind(wxEVT_KEY_DOWN, &go_to_dialog::on_key_down, this);
	input_ctrl->Bind(wxEVT_CHAR, &go_to_dialog::on_char, this);
	auto* button_sizer = new wxStdDialogButtonSizer();
	auto* ok_button = new wxButton(this, wxID_OK);
	button_sizer->AddButton(ok_button);
	button_sizer->AddButton(new wxButton(this, wxID_CANCEL));
	ok_button->SetDefault();
	button_sizer->Realize();
	main_sizer->Add(line_sizer, 0, wxALL | wxEXPAND, 5);
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	SetSizerAndFit(main_sizer);
}

long go_to_dialog::get_position() const {
	wxString input = input_ctrl->GetValue().Trim(true).Trim(false);
	if (input.EndsWith("%")) {
		input.RemoveLast();
		long percent;
		if (input.ToLong(&percent) && percent >= 0 && percent <= 100) {
			long total_chars = textbox->GetLastPosition();
			return (percent * total_chars) / 100;
		}
	} else {
		long line;
		if (input.ToLong(&line) && line >= 1 && line <= textbox->GetNumberOfLines())
			return textbox->XYToPosition(0, line - 1);
	}
	return textbox->GetInsertionPoint();
}

void go_to_dialog::on_key_down(wxKeyEvent& event) {
	int key_code = event.GetKeyCode();
	if (key_code == WXK_UP)
		adjust_line_number(1);
	else if (key_code == WXK_DOWN)
		adjust_line_number(-1);
	else
		event.Skip();
}

void go_to_dialog::on_char(wxKeyEvent& event) {
	int key = event.GetKeyCode();
	if (key < WXK_SPACE || key == WXK_DELETE || key == WXK_BACK || key == WXK_LEFT || key == WXK_RIGHT || key == WXK_TAB) {
		event.Skip();
		return;
	}
	wxString current = input_ctrl->GetValue();
	long from, to;
	input_ctrl->GetSelection(&from, &to);
	wxChar ch = static_cast<wxChar>(key);
	if (wxIsdigit(ch))
		event.Skip();
	else if (ch == '%' && !current.Contains('%'))
		event.Skip();
	else
		wxBell();
}

void go_to_dialog::adjust_line_number(int delta) {
	wxString current_value = input_ctrl->GetValue().Trim(true).Trim(false);
	if (!current_value.EndsWith("%")) {
		long current_line;
		if (current_value.ToLong(&current_line)) {
			long new_line = current_line + delta;
			long max_line = get_max_line();
			if (new_line < 1)
				new_line = 1;
			else if (new_line > max_line)
				new_line = max_line;
			input_ctrl->SetValue(wxString::Format("%ld", new_line));
			input_ctrl->SetSelection(-1, -1);
		}
	}
}

long go_to_dialog::get_max_line() const {
	return textbox->GetNumberOfLines();
}

go_to_page_dialog::go_to_page_dialog(wxWindow* parent, document* doc, int current_page) : wxDialog(parent, wxID_ANY, "Go to page"), doc_{doc} {
	auto* main_sizer = new wxBoxSizer(wxVERTICAL);
	auto* page_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, "Go to page:");
	input_ctrl = new wxTextCtrl(this, wxID_ANY);
	page_sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 5);
	page_sizer->Add(input_ctrl, 1, wxEXPAND);
	input_ctrl->SetValue(wxString::Format("%d", current_page));
	input_ctrl->SetSelection(-1, -1);
	input_ctrl->Bind(wxEVT_KEY_DOWN, &go_to_page_dialog::on_key_down, this);
	input_ctrl->Bind(wxEVT_CHAR, &go_to_page_dialog::on_char, this);
	auto* button_sizer = new wxStdDialogButtonSizer();
	auto* ok_button = new wxButton(this, wxID_OK);
	button_sizer->AddButton(ok_button);
	button_sizer->AddButton(new wxButton(this, wxID_CANCEL));
	ok_button->SetDefault();
	button_sizer->Realize();
	main_sizer->Add(page_sizer, 0, wxALL | wxEXPAND, 5);
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	SetSizerAndFit(main_sizer);
}

int go_to_page_dialog::get_page_number() const {
	wxString input = input_ctrl->GetValue().Trim(true).Trim(false);
	long page;
	if (input.ToLong(&page) && page >= 1 && page <= get_max_page())
		return static_cast<int>(page);
	return 1;
}

void go_to_page_dialog::on_key_down(wxKeyEvent& event) {
	int key_code = event.GetKeyCode();
	if (key_code == WXK_UP)
		adjust_page_number(1);
	else if (key_code == WXK_DOWN)
		adjust_page_number(-1);
	else
		event.Skip();
}

void go_to_page_dialog::on_char(wxKeyEvent& event) {
	int key = event.GetKeyCode();
	if (key < WXK_SPACE || key == WXK_DELETE || key == WXK_BACK || key == WXK_LEFT || key == WXK_RIGHT || key == WXK_TAB) {
		event.Skip();
		return;
	}
	wxChar ch = static_cast<wxChar>(key);
	if (wxIsdigit(ch))
		event.Skip();
	else
		wxBell();
}

void go_to_page_dialog::adjust_page_number(int delta) {
	wxString current_value = input_ctrl->GetValue().Trim(true).Trim(false);
	long current_page;
	if (current_value.ToLong(&current_page)) {
		long new_page = current_page + delta;
		long max_page = get_max_page();
		if (new_page < 1)
			new_page = 1;
		else if (new_page > max_page)
			new_page = max_page;
		input_ctrl->SetValue(wxString::Format("%ld", new_page));
		input_ctrl->SetSelection(-1, -1);
	}
}

int go_to_page_dialog::get_max_page() const {
	if (!doc_ || !doc_->has_flag(document_flags::supports_pages)) return 1;
	return static_cast<int>(doc_->page_offsets.size());
}

toc_dialog::toc_dialog(wxWindow* parent, const document* doc, int current_offset) : wxDialog(parent, wxID_ANY, "Table of Contents"), selected_offset{-1} {
	auto* main_sizer = new wxBoxSizer(wxVERTICAL);
	tree = new wxTreeCtrl(this, wxID_ANY, wxDefaultPosition, wxDefaultSize, wxTR_HIDE_ROOT);
	wxTreeItemId root = tree->AddRoot("Root");
	populate_tree(doc->toc_items, root);
	if (current_offset != -1) find_and_select_item(root, current_offset);
	auto* button_sizer = new wxStdDialogButtonSizer();
	for (int id : {wxID_OK, wxID_CANCEL})
		button_sizer->AddButton(new wxButton(this, id));
	button_sizer->Realize();
	main_sizer->Add(tree, 1, wxEXPAND | wxALL, 10);
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	Bind(wxEVT_TREE_SEL_CHANGED, &toc_dialog::on_tree_selection_changed, this);
	Bind(wxEVT_TREE_ITEM_ACTIVATED, &toc_dialog::on_tree_item_activated, this, wxID_ANY);
	Bind(wxEVT_BUTTON, &toc_dialog::on_ok, this, wxID_OK);
	SetSizer(main_sizer);
	SetSize(500, 400);
	CentreOnParent();
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
