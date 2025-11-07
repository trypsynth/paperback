/* dialogs.hpp - dialog header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "config_manager.hpp"
#include "controls.hpp"
#include "document.hpp"
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

class all_documents_dialog : public dialog {
public:
	all_documents_dialog(wxWindow* parent, config_manager& cfg_mgr, const wxArrayString& open_docs);
	~all_documents_dialog() = default;
	all_documents_dialog(const all_documents_dialog&) = delete;
	all_documents_dialog& operator=(const all_documents_dialog&) = delete;
	all_documents_dialog(all_documents_dialog&&) = delete;
	all_documents_dialog& operator=(all_documents_dialog&&) = delete;

	[[nodiscard]] wxString get_selected_path() const {
		return selected_path;
	}

private:
	wxTextCtrl* search_ctrl{nullptr};
	wxListView* doc_list{nullptr};
	wxButton* open_button{nullptr};
	config_manager& config_mgr;
	wxArrayString doc_paths;
	wxArrayString open_doc_paths;
	wxString selected_path;

	void on_open(wxCommandEvent& event);
	void on_remove(wxCommandEvent& event);
	void on_search(wxCommandEvent& event);
	void on_list_item_activated(wxListEvent& event);
	void on_list_item_selected(wxListEvent& event);
	void on_key_down(wxKeyEvent&);
	void populate_document_list(const wxString& filter = wxEmptyString);
};

enum class bookmark_filter {
	all,
	bookmarks_only,
	notes_only
};

class bookmark_dialog : public dialog {
public:
	bookmark_dialog(wxWindow* parent, const std::vector<bookmark>& bookmarks, wxTextCtrl* text_ctrl, config_manager& config, const wxString& file_path, long current_pos = -1, bookmark_filter initial_filter = bookmark_filter::all);
	~bookmark_dialog() = default;
	bookmark_dialog(const bookmark_dialog&) = delete;
	bookmark_dialog& operator=(const bookmark_dialog&) = delete;
	bookmark_dialog(bookmark_dialog&&) = delete;
	bookmark_dialog& operator=(bookmark_dialog&&) = delete;

	[[nodiscard]] long get_selected_position() const {
		return selected_position;
	}

private:
	wxChoice* filter_choice{nullptr};
	std::vector<bookmark> all_bookmarks;
	wxListBox* bookmark_list{nullptr};
	std::vector<bookmark> bookmark_positions;
	long selected_position;
	config_manager& config;
	wxString file_path;
	wxTextCtrl* text_ctrl;
	wxButton* jump_button{nullptr};
	wxButton* delete_button{nullptr};
	wxButton* edit_note_button{nullptr};

	void on_list_selection_changed(wxCommandEvent& event);
	void on_ok(wxCommandEvent& event);
	void on_key_down(wxKeyEvent&);
	void on_delete(wxCommandEvent& event);
	void on_edit_note(wxCommandEvent& event);
	void on_filter_changed(wxCommandEvent& event);
	void repopulate_list(long current_pos = -1);
};

class document_info_dialog : public dialog {
public:
	document_info_dialog(wxWindow* parent, const document* doc, const wxString& file_path, config_manager& cfg_mgr);
	~document_info_dialog() = default;
	document_info_dialog(const document_info_dialog&) = delete;
	document_info_dialog& operator=(const document_info_dialog&) = delete;
	document_info_dialog(document_info_dialog&&) = delete;
	document_info_dialog& operator=(document_info_dialog&&) = delete;

	long imported_position{-1};

private:
	wxTextCtrl* info_text_ctrl{nullptr};
	config_manager& config_mgr;
	wxString doc_path;
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

class go_to_line_dialog : public dialog {
public:
	go_to_line_dialog(wxWindow* parent, wxTextCtrl* text_ctrl);
	~go_to_line_dialog() = default;
	go_to_line_dialog(const go_to_line_dialog&) = delete;
	go_to_line_dialog& operator=(const go_to_line_dialog&) = delete;
	go_to_line_dialog(go_to_line_dialog&&) = delete;
	go_to_line_dialog& operator=(go_to_line_dialog&&) = delete;
	[[nodiscard]] long get_position() const;

private:
	wxTextCtrl* textbox{nullptr};
	wxSpinCtrl* input_ctrl{nullptr};

	[[nodiscard]] long get_max_line() const;
};

class go_to_page_dialog : public dialog {
public:
	go_to_page_dialog(wxWindow* parent, document* doc, const parser* par, int current_page = 1);
	~go_to_page_dialog() = default;
	go_to_page_dialog(const go_to_page_dialog&) = delete;
	go_to_page_dialog& operator=(const go_to_page_dialog&) = delete;
	go_to_page_dialog(go_to_page_dialog&&) = delete;
	go_to_page_dialog& operator=(go_to_page_dialog&&) = delete;
	[[nodiscard]] int get_page_number() const;

private:
	document* doc_{nullptr};
	const parser* parser_{nullptr};
	wxSpinCtrl* input_ctrl{nullptr};

	[[nodiscard]] int get_max_page() const;
};

class go_to_percent_dialog : public dialog {
public:
	go_to_percent_dialog(wxWindow* parent, wxTextCtrl* text_ctrl);
	~go_to_percent_dialog() = default;
	go_to_percent_dialog(const go_to_percent_dialog&) = delete;
	go_to_percent_dialog& operator=(const go_to_percent_dialog&) = delete;
	go_to_percent_dialog(go_to_percent_dialog&&) = delete;
	go_to_percent_dialog& operator=(go_to_percent_dialog&&) = delete;
	[[nodiscard]] long get_position() const;

private:
	wxTextCtrl* textbox{nullptr};
	accessible_slider* percent_slider{nullptr};
	wxSpinCtrl* input_ctrl{nullptr};

	void on_slider_changed(wxCommandEvent& event);
	void on_spin_changed(wxSpinEvent& event);
};

class note_entry_dialog : public dialog {
public:
	note_entry_dialog(wxWindow* parent, const wxString& title, const wxString& message, const wxString& existing_note);
	~note_entry_dialog() = default;
	note_entry_dialog(const note_entry_dialog&) = delete;
	note_entry_dialog& operator=(const note_entry_dialog&) = delete;
	note_entry_dialog(note_entry_dialog&&) = delete;
	note_entry_dialog& operator=(note_entry_dialog&&) = delete;

	[[nodiscard]] wxString get_note() const;

private:
	wxTextCtrl* note_ctrl{nullptr};
	void on_key_down(wxKeyEvent& event);
};

class open_as_dialog : public dialog {
public:
	open_as_dialog(wxWindow* parent, const wxString& path);
	~open_as_dialog() = default;
	open_as_dialog(const open_as_dialog&) = delete;
	open_as_dialog& operator=(const open_as_dialog&) = delete;
	open_as_dialog(open_as_dialog&&) = delete;
	open_as_dialog& operator=(open_as_dialog&&) = delete;
	[[nodiscard]] wxString get_selected_format() const;

private:
	wxComboBox* format_combo{nullptr};
};

class options_dialog : public dialog {
public:
	options_dialog(wxWindow* parent);
	~options_dialog() = default;
	options_dialog(const options_dialog&) = delete;
	options_dialog& operator=(const options_dialog&) = delete;
	options_dialog(options_dialog&&) = delete;
	options_dialog& operator=(options_dialog&&) = delete;
	bool get_restore_previous_documents() const;
	void set_restore_previous_documents(bool restore);
	bool get_word_wrap() const;
	void set_word_wrap(bool word_wrap);
	bool get_minimize_to_tray() const;
	void set_minimize_to_tray(bool minimize);
	bool get_compact_go_menu() const;
	void set_compact_go_menu(bool compact);
	bool get_navigation_wrap() const;
	void set_navigation_wrap(bool value);
	bool get_check_for_updates_on_startup() const;
	void set_check_for_updates_on_startup(bool check);
	int get_recent_documents_to_show() const;
	void set_recent_documents_to_show(int count);
	wxString get_language() const;
	void set_language(const wxString& language);

private:
	wxCheckBox* restore_docs_check{nullptr};
	wxCheckBox* word_wrap_check{nullptr};
	wxCheckBox* minimize_to_tray_check{nullptr};
	wxCheckBox* compact_go_menu_check{nullptr};
	wxCheckBox* navigation_wrap_check{nullptr};
	wxCheckBox* check_for_updates_on_startup_check{nullptr};
	wxSpinCtrl* recent_docs_count_spin{nullptr};
	wxComboBox* language_combo{nullptr};

	void on_ok(wxCommandEvent& event);
	void on_cancel(wxCommandEvent& event);
};

class sleep_timer_dialog : public dialog {
public:
	sleep_timer_dialog(wxWindow* parent, int initial_duration);
	~sleep_timer_dialog() = default;
	sleep_timer_dialog(const sleep_timer_dialog&) = delete;
	sleep_timer_dialog& operator=(const sleep_timer_dialog&) = delete;
	sleep_timer_dialog(sleep_timer_dialog&&) = delete;
	sleep_timer_dialog& operator=(sleep_timer_dialog&&) = delete;
	[[nodiscard]] int get_duration() const;

private:
	wxSpinCtrl* input_ctrl{nullptr};
};

class toc_tree_item_data : public wxTreeItemData {
public:
	toc_tree_item_data(int offset_) : offset{offset_} {
	}

	int offset{0};
};

class toc_dialog : public dialog {
public:
	toc_dialog(wxWindow* parent, const document* doc, int current_offset = -1);
	~toc_dialog() = default;
	toc_dialog(const toc_dialog&) = delete;
	toc_dialog& operator=(const toc_dialog&) = delete;
	toc_dialog(toc_dialog&&) = delete;
	toc_dialog& operator=(toc_dialog&&) = delete;

	[[nodiscard]] int get_selected_offset() const {
		return selected_offset;
	}

private:
	wxTreeCtrl* tree{nullptr};
	int selected_offset{0};
	wxString search_string_;
	wxTimer* search_timer_{nullptr};

	void populate_tree(const std::vector<std::unique_ptr<toc_item>>& items, const wxTreeItemId& parent);
	void find_and_select_item(const wxTreeItemId& parent, int offset);
	void on_tree_selection_changed(wxTreeEvent& event);
	void on_tree_item_activated(wxTreeEvent& event);
	void on_ok(wxCommandEvent& event);
	void on_char_hook(wxKeyEvent& event);
	void on_search_timer(wxTimerEvent& event);
	bool find_and_select_item_by_name(const wxString& name, const wxTreeItemId& parent);
};

class view_note_dialog : public dialog {
public:
	view_note_dialog(wxWindow* parent, const wxString& note_text);
	~view_note_dialog() = default;
	view_note_dialog(const view_note_dialog&) = delete;
	view_note_dialog& operator=(const view_note_dialog&) = delete;
	view_note_dialog(view_note_dialog&&) = delete;
	view_note_dialog& operator=(view_note_dialog&&) = delete;

private:
	wxTextCtrl* note_ctrl{nullptr};
};
