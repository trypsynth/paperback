#pragma once
#include "document.hpp"
#include <wx/treectrl.h>
#include <wx/wx.h>

class document_info_dialog : public wxDialog {
public:
	document_info_dialog(wxWindow* parent, const document* doc);

private:
	wxTextCtrl* info_text_ctrl = nullptr;
};

class find_dialog : public wxDialog {
public:
	find_dialog(wxWindow* parent);
	wxString get_find_text() const;
	bool get_match_case() const noexcept;
	bool get_match_whole_word() const noexcept;
	bool get_use_regex() const noexcept;
	void set_find_text(const wxString& text);
	void add_to_history(const wxString& text);
	void focus_find_text();

private:
	wxComboBox* find_what_combo{nullptr};
	wxCheckBox* match_case_check{nullptr};
	wxCheckBox* match_whole_word_check{nullptr};
	wxCheckBox* use_regex_check{nullptr};
	wxButton* find_previous_btn{nullptr};
	wxButton* find_next_btn{nullptr};
	wxButton* cancel_btn{nullptr};

	void create_controls();
	void bind_events();
	void on_find_previous(wxCommandEvent& event);
	void on_find_next(wxCommandEvent& event);
	void on_cancel(wxCommandEvent& event);
	void on_find_text_enter(wxCommandEvent& event);
	void on_close(wxCloseEvent& event);
};

class go_to_dialog : public wxDialog {
public:
	go_to_dialog(wxWindow* parent, wxTextCtrl* text_ctrl);
	long get_position() const;

private:
	wxTextCtrl* textbox = nullptr;
	wxTextCtrl* input_ctrl = nullptr;

	void on_key_down(wxKeyEvent& event);
	void on_char(wxKeyEvent& event);
	void adjust_line_number(int delta);
	long get_max_line() const;
};

class go_to_page_dialog : public wxDialog {
public:
	go_to_page_dialog(wxWindow* parent, document* doc, int current_page = 1);
	int get_page_number() const;

private:
	document* doc_ = nullptr;
	wxTextCtrl* input_ctrl = nullptr;

	void on_key_down(wxKeyEvent& event);
	void on_char(wxKeyEvent& event);
	void adjust_page_number(int delta);
	int get_max_page() const;
};

class toc_tree_item_data : public wxTreeItemData {
public:
	toc_tree_item_data(int offset_) : offset{offset_} {}

	int offset;
};

class toc_dialog : public wxDialog {
public:
	toc_dialog(wxWindow* parent, const document* doc, int current_offset = -1);
	int get_selected_offset() const { return selected_offset; }

private:
	wxTreeCtrl* tree;
	int selected_offset;

	void populate_tree(const std::vector<std::unique_ptr<toc_item>>& items, const wxTreeItemId& parent);
	void find_and_select_item(const wxTreeItemId& parent, int offset);
	void on_tree_selection_changed(wxTreeEvent& event);
	void on_tree_item_activated(wxTreeEvent& event);
	void on_ok(wxCommandEvent& event);
};
