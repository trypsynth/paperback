/* dialogs.hpp - dialog header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "document.hpp"
#include <wx/treectrl.h>
#include <wx/wx.h>

class document_info_dialog : public wxDialog {
public:
	document_info_dialog(wxWindow* parent, const document* doc);
	~document_info_dialog() = default;
	document_info_dialog(const document_info_dialog&) = delete;
	document_info_dialog& operator=(const document_info_dialog&) = delete;
	document_info_dialog(document_info_dialog&&) = delete;
	document_info_dialog& operator=(document_info_dialog&&) = delete;

private:
	wxTextCtrl* info_text_ctrl = nullptr;
};

class find_dialog : public wxDialog {
public:
	find_dialog(wxWindow* parent);
	~find_dialog() = default;
	find_dialog(const find_dialog&) = delete;
	find_dialog& operator=(const find_dialog&) = delete;
	find_dialog(find_dialog&&) = delete;
	find_dialog& operator=(find_dialog&&) = delete;
	[[nodiscard]] wxString get_find_text() const;
	[[nodiscard]] bool get_match_case() const noexcept;
	[[nodiscard]] bool get_match_whole_word() const noexcept;
	[[nodiscard]] bool get_use_regex() const noexcept;
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

	void on_find_previous(wxCommandEvent& event);
	void on_find_next(wxCommandEvent& event);
	void on_cancel(wxCommandEvent& event);
	void on_find_text_enter(wxCommandEvent& event);
	void on_close(wxCloseEvent& event);
};

class go_to_dialog : public wxDialog {
public:
	go_to_dialog(wxWindow* parent, wxTextCtrl* text_ctrl);
	~go_to_dialog() = default;
	go_to_dialog(const go_to_dialog&) = delete;
	go_to_dialog& operator=(const go_to_dialog&) = delete;
	go_to_dialog(go_to_dialog&&) = delete;
	go_to_dialog& operator=(go_to_dialog&&) = delete;
	[[nodiscard]] long get_position() const;

private:
	wxTextCtrl* textbox = nullptr;
	wxTextCtrl* input_ctrl = nullptr;

	void on_key_down(wxKeyEvent& event);
	void on_char(wxKeyEvent& event);
	void adjust_line_number(int delta);
	[[nodiscard]] long get_max_line() const;
};

class go_to_page_dialog : public wxDialog {
public:
	go_to_page_dialog(wxWindow* parent, document* doc, int current_page = 1);
	~go_to_page_dialog() = default;
	go_to_page_dialog(const go_to_page_dialog&) = delete;
	go_to_page_dialog& operator=(const go_to_page_dialog&) = delete;
	go_to_page_dialog(go_to_page_dialog&&) = delete;
	go_to_page_dialog& operator=(go_to_page_dialog&&) = delete;
	[[nodiscard]] int get_page_number() const;

private:
	document* doc_ = nullptr;
	wxTextCtrl* input_ctrl = nullptr;

	void on_key_down(wxKeyEvent& event);
	void on_char(wxKeyEvent& event);
	void adjust_page_number(int delta);
	[[nodiscard]] int get_max_page() const;
};

class options_dialog : public wxDialog {
public:
	options_dialog(wxWindow* parent);
	~options_dialog() = default;
	options_dialog(const options_dialog&) = delete;
	options_dialog& operator=(const options_dialog&) = delete;
	options_dialog(options_dialog&&) = delete;
	options_dialog& operator=(options_dialog&&) = delete;
	bool get_restore_previous_documents() const;
	void set_restore_previous_documents(bool restore);

private:
	wxCheckBox* restore_docs_check = nullptr;

	void on_ok(wxCommandEvent& event);
	void on_cancel(wxCommandEvent& event);
};

class toc_tree_item_data : public wxTreeItemData {
public:
	toc_tree_item_data(int offset_) : offset{offset_} {}

	int offset;
};

class toc_dialog : public wxDialog {
public:
	toc_dialog(wxWindow* parent, const document* doc, int current_offset = -1);
	~toc_dialog() = default;
	toc_dialog(const toc_dialog&) = delete;
	toc_dialog& operator=(const toc_dialog&) = delete;
	toc_dialog(toc_dialog&&) = delete;
	toc_dialog& operator=(toc_dialog&&) = delete;
	[[nodiscard]] int get_selected_offset() const { return selected_offset; }

private:
	wxTreeCtrl* tree;
	int selected_offset;

	void populate_tree(const std::vector<std::unique_ptr<toc_item>>& items, const wxTreeItemId& parent);
	void find_and_select_item(const wxTreeItemId& parent, int offset);
	void on_tree_selection_changed(wxTreeEvent& event);
	void on_tree_item_activated(wxTreeEvent& event);
	void on_ok(wxCommandEvent& event);
};
