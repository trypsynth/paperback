#pragma once

#include "document.hpp"
#include <wx/treectrl.h>
#include <wx/wx.h>

class toc_tree_item_data : public wxTreeItemData {
public:
	toc_tree_item_data(int offset_) : offset{ offset_ } {}

	int offset;
};

class toc_dialog : public wxDialog {
public:
	toc_dialog(wxWindow* parent, const document* doc);
	int get_selected_offset() const { return selected_offset; }

private:
	wxTreeCtrl* tree = nullptr;
	int selected_offset;

	void populate_tree(const std::vector<std::unique_ptr<toc_item>>& items, const wxTreeItemId& parent);
	void on_tree_selection_changed(wxTreeEvent& event);
	void on_tree_item_activated(wxTreeEvent& event);
	void on_ok(wxCommandEvent& event);
};
