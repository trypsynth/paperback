#pragma once
#include "config_manager.hpp"
#include "document_data.hpp"
#include "parser.hpp"
#include <wx/arrstr.h>
#include <wx/button.h>
#include <wx/checkbox.h>
#include <wx/choice.h>
#include <wx/clntdata.h>
#include <wx/combobox.h>
#include <wx/dialog.h>
#include <wx/listbox.h>
#include <wx/listctrl.h>
#include <wx/spinctrl.h>
#include <wx/srchctrl.h>
#include <wx/textctrl.h>
#include <wx/timer.h>
#include <wx/treectrl.h>
#include <functional>
#include <wx/webview.h>

enum class dialog_button_config {
	ok_only,
	ok_cancel
};

class dialog : public wxDialog {
public:
	dialog(wxWindow* parent, const wxString& title, dialog_button_config buttons = dialog_button_config::ok_cancel);
	virtual ~dialog() = default;

protected:
	void set_content(wxSizer* content_sizer);
	void finalize_layout();
	wxBoxSizer* main_sizer{nullptr};

private:
	wxStdDialogButtonSizer* button_sizer{nullptr};
	dialog_button_config button_config;
	bool layout_finalized{false};

	void create_buttons();
};

class elements_dialog : public dialog {
public:
	elements_dialog(wxWindow* parent, session_document* session_doc, long current_pos);
	~elements_dialog() override = default;
	elements_dialog(const elements_dialog&) = delete;
	elements_dialog& operator=(const elements_dialog&) = delete;
	elements_dialog(elements_dialog&&) = delete;
	elements_dialog& operator=(elements_dialog&&) = delete;

	[[nodiscard]] int get_selected_offset() const {
		return selected_offset;
	}

	[[nodiscard]] int get_selected_view() const {
		return view_choice->GetSelection();
	}

private:
	session_document* session_doc_{nullptr};
	wxComboBox* view_choice{nullptr};
	wxListBox* links_list{nullptr};
	wxTreeCtrl* headings_tree{nullptr};
	wxBoxSizer* links_sizer{nullptr};
	wxBoxSizer* headings_sizer{nullptr};
	int selected_offset{-1};
	long current_pos{-1};

	void populate_links();
	void populate_headings();
	void on_view_choice_changed(wxCommandEvent&);
	void on_heading_activated(wxTreeEvent& event);
	void on_ok(wxCommandEvent&);
};
