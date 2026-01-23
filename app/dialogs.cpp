#include "dialogs.hpp"
#include "app.hpp"
#include "config_manager.hpp"
#include "constants.hpp"
#include "controls.hpp"
#include "document_data.hpp"
#include "parser.hpp"
#include "translation_manager.hpp"
#include "utils.hpp"
#include <algorithm>
#include <climits>
#include <cmath>
#include <cstddef>
#include <string>
#include <vector>
#include <wx/arrstr.h>
#include <wx/combobox.h>
#include <wx/defs.h>
#include <wx/dialog.h>
#include <wx/dynarray.h>
#include <wx/event.h>
#include <wx/filedlg.h>
#include <wx/filename.h>
#include <wx/listbox.h>
#include <wx/listctrl.h>
#include <wx/msgdlg.h>
#include <wx/sizer.h>
#include <wx/stattext.h>
#include <wx/string.h>
#include <wx/textctrl.h>
#include <wx/textdlg.h>
#include <wx/translation.h>
#include <wx/timer.h>
#include <wx/uiaction.h>
#include <wx/window.h>

dialog::dialog(wxWindow* parent, const wxString& title, dialog_button_config buttons) : wxDialog(parent, wxID_ANY, title), main_sizer{new wxBoxSizer(wxVERTICAL)}, button_config{buttons} {
	SetSizer(main_sizer);
}

void dialog::set_content(wxSizer* content_sizer) {
	if (layout_finalized) return;
	main_sizer->Add(content_sizer, 1, wxEXPAND | wxALL, DIALOG_PADDING);
}

void dialog::finalize_layout() {
	if (layout_finalized) return;
	create_buttons();
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, DIALOG_PADDING);
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

bookmark_dialog::bookmark_dialog(wxWindow* parent, session_document* session_doc, wxTextCtrl* text_ctrl, config_manager& config, const wxString& file_path, long current_pos, bookmark_filter initial_filter) : dialog(parent, _("Jump to Bookmark"), dialog_button_config::ok_cancel), selected_position{-1}, config{config}, file_path{file_path}, text_ctrl{text_ctrl}, session_doc_{session_doc} {
	auto* filter_row = new wxBoxSizer(wxHORIZONTAL);
	auto* filter_label = new wxStaticText(this, wxID_ANY, _("&Filter:"));
	filter_choice = new wxChoice(this, wxID_ANY);
	filter_choice->Append(_("All"));
	filter_choice->Append(_("Bookmarks"));
	filter_choice->Append(_("Notes"));
	int initial_index{0};
	switch (initial_filter) {
		case bookmark_filter::all:
			initial_index = 0;
			break;
		case bookmark_filter::bookmarks_only:
			initial_index = 1;
			break;
		case bookmark_filter::notes_only:
			initial_index = 2;
			break;
	}
	filter_choice->SetSelection(initial_index);
	filter_row->Add(filter_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 6);
	filter_row->Add(filter_choice, 1, wxEXPAND);
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	bookmark_list = new wxListBox(this, wxID_ANY);
	content_sizer->Add(filter_row, 0, wxEXPAND | wxALL, DIALOG_PADDING);
	content_sizer->Add(bookmark_list, 1, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	auto* action_sizer = new wxStdDialogButtonSizer();
	edit_note_button = new wxButton(this, wxID_EDIT, _("&Edit Note"));
	delete_button = new wxButton(this, wxID_DELETE, _("&Delete"));
	jump_button = new wxButton(this, wxID_OK, _("&Jump"));
	auto* cancel_button = new wxButton(this, wxID_CANCEL, _("&Cancel"));
	action_sizer->AddButton(edit_note_button);
	action_sizer->AddButton(delete_button);
	action_sizer->AddButton(jump_button);
	action_sizer->AddButton(cancel_button);
	action_sizer->Realize();
	content_sizer->Add(action_sizer, 0, wxALIGN_RIGHT | wxALL, DIALOG_PADDING);
	set_content(content_sizer);
	SetSizerAndFit(main_sizer);
	CentreOnParent();
	jump_button->SetDefault();
	jump_button->Enable(false);
	delete_button->Enable(false);
	edit_note_button->Enable(false);
	repopulate_list(current_pos);
	bookmark_list->SetFocus();
	filter_choice->Bind(wxEVT_CHOICE, &bookmark_dialog::on_filter_changed, this);
	bookmark_list->Bind(wxEVT_LISTBOX, &bookmark_dialog::on_list_selection_changed, this);
	bookmark_list->Bind(wxEVT_KEY_DOWN, &bookmark_dialog::on_key_down, this);
	Bind(wxEVT_BUTTON, &bookmark_dialog::on_ok, this, wxID_OK);
	Bind(wxEVT_BUTTON, &bookmark_dialog::on_delete, this, wxID_DELETE);
	Bind(wxEVT_BUTTON, &bookmark_dialog::on_edit_note, this, wxID_EDIT);
}

void bookmark_dialog::on_list_selection_changed(wxCommandEvent& /*event*/) {
	const int selection = bookmark_list->GetSelection();
	if (selection >= 0 && static_cast<size_t>(selection) < bookmark_positions.size()) {
		selected_position = bookmark_positions[static_cast<std::size_t>(selection)].start;
		jump_button->Enable(true);
		delete_button->Enable(true);
		edit_note_button->Enable(true);
	} else {
		selected_position = -1;
		jump_button->Enable(false);
		delete_button->Enable(false);
		edit_note_button->Enable(false);
	}
}

void bookmark_dialog::on_ok(wxCommandEvent& /*event*/) {
	if (selected_position >= 0)
		EndModal(wxID_OK);
	else
		wxMessageBox(_("Please select a bookmark to jump to."), _("Error"), wxICON_ERROR);
}

void bookmark_dialog::on_key_down(wxKeyEvent& event) {
	const int key = event.GetKeyCode();
	if (key == WXK_DELETE || key == WXK_NUMPAD_DELETE) {
		const wxCommandEvent remove_event(wxEVT_BUTTON, wxID_DELETE);
		wxPostEvent(this, remove_event);
	} else {
		event.Skip();
	}
}

void bookmark_dialog::on_delete(wxCommandEvent&) {
	const int selection = bookmark_list->GetSelection();
	if (selection < 0) return;
	const bookmark& deleted_bookmark = bookmark_positions[static_cast<std::size_t>(selection)];
	config.remove_bookmark(file_path, deleted_bookmark.start, deleted_bookmark.end);
	config.flush();
	repopulate_list(text_ctrl ? text_ctrl->GetInsertionPoint() : -1);
}

void bookmark_dialog::on_edit_note(wxCommandEvent&) {
	const int selection = bookmark_list->GetSelection();
	if (selection < 0 || static_cast<size_t>(selection) >= bookmark_positions.size()) return;
	const bookmark& selected_bookmark = bookmark_positions[static_cast<std::size_t>(selection)];
	note_entry_dialog note_dialog(this, _("Bookmark Note"), _("Edit bookmark note:"), selected_bookmark.note);
	if (note_dialog.ShowModal() != wxID_OK) return;
	wxString new_note = note_dialog.get_note();
	config.update_bookmark_note(file_path, selected_bookmark.start, selected_bookmark.end, new_note);
	config.flush();
	repopulate_list(text_ctrl ? text_ctrl->GetInsertionPoint() : -1);
}

void bookmark_dialog::on_filter_changed(wxCommandEvent&) {
	repopulate_list(text_ctrl ? text_ctrl->GetInsertionPoint() : -1);
}

void bookmark_dialog::repopulate_list(long current_pos) {
	if (current_pos == -1 && text_ctrl != nullptr) current_pos = text_ctrl->GetInsertionPoint();
	const int sel = filter_choice != nullptr ? filter_choice->GetSelection() : 0;
	BookmarkFilterType filter_type = BookmarkFilterType::All;
	if (sel == 1) filter_type = BookmarkFilterType::BookmarksOnly;
	else if (sel == 2) filter_type = BookmarkFilterType::NotesOnly;
	bookmark_list->Clear();
	bookmark_positions.clear();
	const long previously_selected = selected_position;
	int closest_index = -1;
	if (session_doc_ == nullptr) {
		jump_button->Enable(false);
		delete_button->Enable(false);
		edit_note_button->Enable(false);
		selected_position = -1;
		return;
	}
	auto filtered = get_filtered_bookmark_display_items(*session_doc_->session, config.backend_for_ffi(), file_path.ToUTF8().data(), current_pos, filter_type);
	closest_index = filtered.closest_index;
	for (auto& item : filtered.items) {
		wxString text_snippet = wxString::FromUTF8(item.snippet.c_str()).Strip(wxString::both);
		if (text_snippet.IsEmpty()) text_snippet = _("blank");
		wxString display_text;
		const wxString note = wxString::FromUTF8(item.note.c_str());
		if (!note.IsEmpty())
			display_text = wxString::Format("%s - %s", note, text_snippet);
		else
			display_text = text_snippet;
		bookmark bm;
		bm.start = static_cast<long>(item.start);
		bm.end = static_cast<long>(item.end);
		bm.note = note;
		bookmark_positions.push_back(bm);
		bookmark_list->Append(display_text);
	}
	jump_button->Enable(false);
	delete_button->Enable(false);
	edit_note_button->Enable(false);
	selected_position = -1;
	if (previously_selected >= 0) {
		const auto it_sel = std::find_if(bookmark_positions.begin(), bookmark_positions.end(), [&](const bookmark& bm) {
			return bm.start == previously_selected;
		});
		if (it_sel != bookmark_positions.end()) {
			const int idx = static_cast<int>(std::distance(bookmark_positions.begin(), it_sel));
			bookmark_list->SetSelection(idx);
			selected_position = it_sel->start;
			jump_button->Enable(true);
			delete_button->Enable(true);
			edit_note_button->Enable(true);
			return;
		}
	}
	if (closest_index >= 0 && static_cast<size_t>(closest_index) < bookmark_positions.size()) {
		bookmark_list->SetSelection(closest_index);
		selected_position = bookmark_positions[static_cast<size_t>(closest_index)].start;
		jump_button->Enable(true);
		delete_button->Enable(true);
		edit_note_button->Enable(true);
	}
}

elements_dialog::elements_dialog(wxWindow* parent, session_document* session_doc, long current_pos) : dialog(parent, _("Elements")), session_doc_(session_doc), current_pos(current_pos) {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	auto* choice_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* choice_label = new wxStaticText(this, wxID_ANY, _("&View:"));
	view_choice = new wxComboBox(this, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, 0, nullptr, wxCB_READONLY);
	view_choice->Append(_("Headings"));
	view_choice->Append(_("Links"));
	view_choice->SetSelection(0);
	choice_sizer->Add(choice_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, DIALOG_PADDING);
	choice_sizer->Add(view_choice, 1, wxEXPAND);
	content_sizer->Add(choice_sizer, 0, wxEXPAND | wxALL, DIALOG_PADDING);
	headings_sizer = new wxBoxSizer(wxVERTICAL);
	headings_tree = new wxTreeCtrl(this, wxID_ANY, wxDefaultPosition, wxSize(400, 500), wxTR_DEFAULT_STYLE | wxTR_HIDE_ROOT);
	headings_sizer->Add(headings_tree, 1, wxEXPAND);
	content_sizer->Add(headings_sizer, 1, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	links_sizer = new wxBoxSizer(wxVERTICAL);
	links_list = new wxListBox(this, wxID_ANY);
	links_sizer->Add(links_list, 1, wxEXPAND);
	content_sizer->Add(links_sizer, 1, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	populate_headings();
	populate_links();
	links_sizer->Show(false);
	set_content(content_sizer);
	finalize_layout();
	Bind(wxEVT_COMBOBOX, &elements_dialog::on_view_choice_changed, this, view_choice->GetId());
	Bind(wxEVT_TREE_ITEM_ACTIVATED, &elements_dialog::on_heading_activated, this, headings_tree->GetId());
	Bind(wxEVT_BUTTON, &elements_dialog::on_ok, this, wxID_OK);
	if (view_choice->GetSelection() == 0) headings_tree->SetFocus();
	else links_list->SetFocus();
	view_choice->SetFocus();
}

void elements_dialog::populate_links() {
	if (session_doc_ == nullptr) return;
	const auto link_list = session_link_list(*session_doc_->session, current_pos);
	for (const auto& item : link_list.items) {
		links_list->Append(to_wxstring(item.text));
		links_list->SetClientData(links_list->GetCount() - 1, reinterpret_cast<void*>(item.offset));
	}
	if (links_list->IsEmpty()) return;
	if (link_list.closest_index != -1) links_list->SetSelection(link_list.closest_index);
	else links_list->SetSelection(0);
}

void elements_dialog::populate_headings() {
	if (session_doc_ == nullptr) return;
	const wxTreeItemId root = headings_tree->AddRoot(_("Root"));
	const auto tree = session_heading_tree(*session_doc_->session, current_pos);
	std::vector<wxTreeItemId> item_ids(tree.items.size());
	for (size_t i = 0; i < tree.items.size(); ++i) {
		const auto& heading_marker = tree.items[i];
		const int level = heading_marker.level;
		if (level < 1 || level > 6) continue;
		const int parent_index = heading_marker.parent_index;
		const wxTreeItemId parent_id = (parent_index >= 0 && static_cast<size_t>(parent_index) < item_ids.size()) ? item_ids[static_cast<size_t>(parent_index)] : root;
		auto text = heading_marker.text;
		const wxString heading_text = wxString::FromUTF8(text.c_str());
		const wxString display_text = heading_text.IsEmpty() ? wxString(_("Untitled")) : heading_text;
		const wxTreeItemId item_id = headings_tree->AppendItem(parent_id, display_text);
		headings_tree->SetItemData(item_id, new toc_tree_item_data(heading_marker.offset));
		item_ids[i] = item_id;
	}
	headings_tree->ExpandAll();
	if (tree.closest_index >= 0 && static_cast<size_t>(tree.closest_index) < item_ids.size() && item_ids[static_cast<size_t>(tree.closest_index)].IsOk()) {
		headings_tree->SelectItem(item_ids[static_cast<size_t>(tree.closest_index)]);
		headings_tree->EnsureVisible(item_ids[static_cast<size_t>(tree.closest_index)]);
		return;
	}
	wxTreeItemIdValue cookie;
	const wxTreeItemId first_item = headings_tree->GetFirstChild(headings_tree->GetRootItem(), cookie);
	if (first_item.IsOk()) {
		headings_tree->SelectItem(first_item);
		headings_tree->EnsureVisible(first_item);
	}
}

void elements_dialog::on_view_choice_changed(wxCommandEvent& /*event*/) {
	const int selection = view_choice->GetSelection();
	if (selection == 0) {
		headings_sizer->Show(true);
		links_sizer->Show(false);
	} else {
		headings_sizer->Show(false);
		links_sizer->Show(true);
	}
	view_choice->SetFocus();
	main_sizer->Layout();
}

void elements_dialog::on_heading_activated(wxTreeEvent& event) {
	const wxTreeItemId item = event.GetItem();
	if (item.IsOk()) {
		const auto* data = dynamic_cast<toc_tree_item_data*>(headings_tree->GetItemData(item));
		if (data != nullptr) {
			selected_offset = data->offset;
			EndModal(wxID_OK);
		}
	}
}

void elements_dialog::on_ok(wxCommandEvent& /*event*/) {
	if (view_choice->GetSelection() == 0) {
		const wxTreeItemId item = headings_tree->GetSelection();
		if (item.IsOk()) {
			const auto* data = dynamic_cast<toc_tree_item_data*>(headings_tree->GetItemData(item));
			if (data != nullptr) {
				selected_offset = data->offset;
				EndModal(wxID_OK);
			}
		}
	} else {
		const int selection = links_list->GetSelection();
		if (selection != wxNOT_FOUND) {
			selected_offset = static_cast<int>(reinterpret_cast<size_t>(links_list->GetClientData(selection)));
			EndModal(wxID_OK);
		}
	}
}

note_entry_dialog::note_entry_dialog(wxWindow* parent, const wxString& title, const wxString& message, const wxString& existing_note) : dialog(parent, title) {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	auto* message_label = new wxStaticText(this, wxID_ANY, message);
	content_sizer->Add(message_label, 0, wxALL, DIALOG_PADDING);
	note_ctrl = new wxTextCtrl(this, wxID_ANY, existing_note, wxDefaultPosition, wxSize(400, 200), wxTE_MULTILINE);
	content_sizer->Add(note_ctrl, 1, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	set_content(content_sizer);
	finalize_layout();
	note_ctrl->SetFocus();
	note_ctrl->Bind(wxEVT_KEY_DOWN, &note_entry_dialog::on_key_down, this);
}

wxString note_entry_dialog::get_note() const {
	return note_ctrl->GetValue();
}

void note_entry_dialog::on_key_down(wxKeyEvent& event) {
	if (event.GetKeyCode() == WXK_RETURN && event.ShiftDown()) note_ctrl->WriteText("\n");
	else if (event.GetKeyCode() == WXK_RETURN) EndModal(wxID_OK);
	else event.Skip();
}

options_dialog::options_dialog(wxWindow* parent) : dialog(parent, _("Options")) {
	constexpr int option_padding = 5;
	constexpr int max_recent_docs = 100;
	constexpr int default_recent_docs = 10;
	auto* general_box = new wxStaticBoxSizer(wxVERTICAL, this, _("General"));
	restore_docs_check = new wxCheckBox(this, wxID_ANY, _("&Restore previously opened documents on startup"));
	general_box->Add(restore_docs_check, 0, wxALL, option_padding);
	word_wrap_check = new wxCheckBox(this, wxID_ANY, _("&Word wrap"));
	general_box->Add(word_wrap_check, 0, wxALL, option_padding);
	minimize_to_tray_check = new wxCheckBox(this, wxID_ANY, _("&Minimize to system tray"));
	general_box->Add(minimize_to_tray_check, 0, wxALL, option_padding);
	start_maximized_check = new wxCheckBox(this, wxID_ANY, _("&Start maximized"));
	general_box->Add(start_maximized_check, 0, wxALL, option_padding);
	compact_go_menu_check = new wxCheckBox(this, wxID_ANY, _("Show compact &go menu"));
	general_box->Add(compact_go_menu_check, 0, wxALL, option_padding);
	navigation_wrap_check = new wxCheckBox(this, wxID_ANY, _("&Wrap navigation"));
	general_box->Add(navigation_wrap_check, 0, wxALL, option_padding);
	check_for_updates_on_startup_check = new wxCheckBox(this, wxID_ANY, _("Check for &updates on startup"));
	general_box->Add(check_for_updates_on_startup_check, 0, wxALL, option_padding);
	auto* recent_docs_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* recent_docs_label = new wxStaticText(this, wxID_ANY, _("Number of &recent documents to show:"));
	recent_docs_count_spin = new wxSpinCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxSP_ARROW_KEYS, 0, max_recent_docs, default_recent_docs);
	recent_docs_sizer->Add(recent_docs_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, DIALOG_PADDING);
	recent_docs_sizer->Add(recent_docs_count_spin, 0, wxALIGN_CENTER_VERTICAL);
	general_box->Add(recent_docs_sizer, 0, wxALL, option_padding);
	auto* language_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* language_label = new wxStaticText(this, wxID_ANY, _("&Language:"));
	language_combo = new wxComboBox(this, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, 0, nullptr, wxCB_READONLY);
	const auto& available_languages = translation_manager::instance().get_available_languages();
	for (const auto& lang : available_languages) language_combo->Append(lang.native_name, new wxStringClientData(lang.code));
	language_sizer->Add(language_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, DIALOG_PADDING);
	language_sizer->Add(language_combo, 0, wxALIGN_CENTER_VERTICAL);
	general_box->Add(language_sizer, 0, wxALL, option_padding);
	set_content(general_box);
	Bind(wxEVT_BUTTON, &options_dialog::on_ok, this, wxID_OK);
	Bind(wxEVT_BUTTON, &options_dialog::on_cancel, this, wxID_CANCEL);
	finalize_layout();
}

bool options_dialog::get_restore_previous_documents() const {
	return restore_docs_check != nullptr ? restore_docs_check->GetValue() : false;
}

void options_dialog::set_restore_previous_documents(bool restore) {
	if (restore_docs_check != nullptr) restore_docs_check->SetValue(restore);
}

bool options_dialog::get_word_wrap() const {
	return word_wrap_check != nullptr ? word_wrap_check->GetValue() : false;
}

void options_dialog::set_word_wrap(bool word_wrap) {
	if (word_wrap_check != nullptr) word_wrap_check->SetValue(word_wrap);
}

bool options_dialog::get_minimize_to_tray() const {
	return minimize_to_tray_check != nullptr ? minimize_to_tray_check->GetValue() : false;
}

void options_dialog::set_minimize_to_tray(bool minimize) {
	if (minimize_to_tray_check != nullptr) minimize_to_tray_check->SetValue(minimize);
}

bool options_dialog::get_start_maximized() const {
	return start_maximized_check != nullptr ? start_maximized_check->GetValue() : false;
}

void options_dialog::set_start_maximized(bool maximized) {
	if (start_maximized_check != nullptr) start_maximized_check->SetValue(maximized);
}

bool options_dialog::get_compact_go_menu() const {
	return compact_go_menu_check != nullptr ? compact_go_menu_check->GetValue() : true;
}

void options_dialog::set_compact_go_menu(bool compact) {
	if (compact_go_menu_check != nullptr) compact_go_menu_check->SetValue(compact);
}

bool options_dialog::get_navigation_wrap() const {
	return navigation_wrap_check != nullptr ? navigation_wrap_check->GetValue() : false;
}

void options_dialog::set_navigation_wrap(bool value) {
	if (navigation_wrap_check) navigation_wrap_check->SetValue(value);
}

bool options_dialog::get_check_for_updates_on_startup() const {
	return check_for_updates_on_startup_check != nullptr ? check_for_updates_on_startup_check->GetValue() : true;
}

void options_dialog::set_check_for_updates_on_startup(bool check) {
	if (check_for_updates_on_startup_check != nullptr) check_for_updates_on_startup_check->SetValue(check);
}

int options_dialog::get_recent_documents_to_show() const {
	constexpr int default_value = 10;
	return recent_docs_count_spin != nullptr ? recent_docs_count_spin->GetValue() : default_value;
}

void options_dialog::set_recent_documents_to_show(int count) {
	if (recent_docs_count_spin != nullptr) recent_docs_count_spin->SetValue(count);
}

wxString options_dialog::get_language() const {
	if (language_combo == nullptr) return {};
	const int selection = language_combo->GetSelection();
	if (selection == wxNOT_FOUND) return {};
	const auto* data = dynamic_cast<wxStringClientData*>(language_combo->GetClientObject(selection));
	return data != nullptr ? data->GetData() : wxString{};
}

void options_dialog::set_language(const wxString& language) {
	if (language_combo == nullptr) return;
	for (unsigned int i = 0; i < language_combo->GetCount(); ++i) {
		const auto* data = dynamic_cast<wxStringClientData*>(language_combo->GetClientObject(i));
		if (data != nullptr && data->GetData() == language) {
			language_combo->SetSelection(static_cast<int>(i));
			return;
		}
	}
}

void options_dialog::on_ok(wxCommandEvent& /*event*/) {
	EndModal(wxID_OK);
}

void options_dialog::on_cancel(wxCommandEvent& /*event*/) {
	EndModal(wxID_CANCEL);
}

password_dialog::password_dialog(wxWindow* parent) : dialog(parent, _("Document Password")) {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	auto* message_label = new wxStaticText(this, wxID_ANY, _("&Password"));
	content_sizer->Add(message_label, 0, wxALL, DIALOG_PADDING);
	password_ctrl = new wxTextCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxSize(300, -1), wxTE_PASSWORD);
	content_sizer->Add(password_ctrl, 0, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	set_content(content_sizer);
	finalize_layout();
	password_ctrl->SetFocus();
}

wxString password_dialog::get_password() const {
	return password_ctrl->GetValue();
}

sleep_timer_dialog::sleep_timer_dialog(wxWindow* parent, int initial_duration) : dialog(parent, _("Sleep Timer")) {
	constexpr int label_spacing = 5;
	auto* sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, _("&Minutes:"));
	input_ctrl = new wxSpinCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxSP_ARROW_KEYS, 1, 999, initial_duration);
	sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, label_spacing);
	sizer->Add(input_ctrl, 1, wxEXPAND);
	set_content(sizer);
	finalize_layout();
}

int sleep_timer_dialog::get_duration() const {
	return input_ctrl->GetValue();
}

view_note_dialog::view_note_dialog(wxWindow* parent, const wxString& note_text) : dialog(parent, _("View Note"), dialog_button_config::ok_only) {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	note_ctrl = new wxTextCtrl(this, wxID_ANY, note_text, wxDefaultPosition, wxSize(400, 200), wxTE_MULTILINE | wxTE_READONLY | wxTE_RICH2);
	content_sizer->Add(note_ctrl, 1, wxEXPAND | wxALL, DIALOG_PADDING);
	set_content(content_sizer);
	finalize_layout();
	FindWindow(wxID_OK)->SetLabel(_("Close"));
	note_ctrl->SetFocus();
}

web_view_dialog::web_view_dialog(wxWindow* parent, const wxString& title, const wxString& url_or_content, bool is_url, std::function<bool(const wxString&)> navigation_handler) : wxDialog(parent, wxID_ANY, title), navigation_handler_{navigation_handler} {
	web_view = wxWebView::New(this, wxID_ANY);
	web_view->AddScriptMessageHandler("wx");
	Bind(wxEVT_WEBVIEW_LOADED, &web_view_dialog::on_webview_loaded, this, web_view->GetId());
	Bind(wxEVT_WEBVIEW_SCRIPT_MESSAGE_RECEIVED, &web_view_dialog::on_script_message, this, web_view->GetId());
	Bind(wxEVT_WEBVIEW_NAVIGATING, &web_view_dialog::on_webview_navigating, this, web_view->GetId());
	if (is_url) {
		web_view->LoadURL(url_or_content);
	} else {
		wxString full_html;
		if (url_or_content.Lower().Contains("<html"))
			full_html = url_or_content;
		else
			full_html << "<html><head><title>" << title << "</title></head><body>" << url_or_content << "</body></html>";
		web_view->SetPage(full_html, "");
	}
	auto* sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(web_view, 1, wxEXPAND | wxALL, 5);
	auto* button_sizer = CreateStdDialogButtonSizer(wxCLOSE);
	sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 5);
	SetSizerAndFit(sizer);
	Centre();
}

void web_view_dialog::on_webview_navigating(wxWebViewEvent& event) {
	if (navigation_handler_ && !navigation_handler_(event.GetURL())) event.Veto();
}

void web_view_dialog::simulate_click() {
	wxPoint pos = web_view->GetScreenPosition();
	wxSize size = web_view->GetSize();
	int x = pos.x + size.x / 2;
	int y = pos.y + size.y / 2;
	wxUIActionSimulator sim;
	sim.MouseMove(x, y);
	sim.MouseClick();
}

void web_view_dialog::on_webview_loaded([[maybe_unused]] wxWebViewEvent& event) {
	wxTimer* timer = new wxTimer();
	timer->Bind(wxEVT_TIMER, [this, timer](wxTimerEvent&) {
		simulate_click();
		timer->Stop();
		delete timer;
	});
	timer->StartOnce(100);
	web_view->RunScript(
		"document.addEventListener('keydown', function(event) {"
		"    if (event.key === 'Escape' || event.keyCode === 27) {"
		"        window.wx.postMessage('close_dialog');"
		"    }"
		"});"
	);
}

void web_view_dialog::on_script_message(wxWebViewEvent& event) {
	if (event.GetString() == "close_dialog") EndModal(wxID_CANCEL);
}
