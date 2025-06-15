#include "toc_dialog.hpp"

toc_dialog::toc_dialog(wxWindow* parent, const document* doc) 
	: wxDialog(parent, wxID_ANY, "Table of Contents"), selected_offset(-1) {
	auto* main_sizer = new wxBoxSizer(wxVERTICAL);
	tree = new wxTreeCtrl(this, wxID_ANY, wxDefaultPosition, wxDefaultSize, wxTR_HIDE_ROOT);
	wxTreeItemId root = tree->AddRoot("Root");
	populate_tree(doc->toc_items, root);
	tree->Expand(root);
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
		if (!item->children.empty()) populate_tree(item->children, item_id);
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
	if (selected_offset >= 0) EndModal(wxID_OK);
}

void toc_dialog::on_ok(wxCommandEvent& event) {
	if (selected_offset >= 0) EndModal(wxID_OK);
	else wxMessageBox("Please select a section from the table of contents.", "No Selection", wxOK | wxICON_INFORMATION, this);
}
