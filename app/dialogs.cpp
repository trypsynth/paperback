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
