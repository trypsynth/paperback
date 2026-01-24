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

enum class bookmark_filter {
	all,
	bookmarks_only,
	notes_only
};

class bookmark_dialog : public dialog {
public:
	bookmark_dialog(wxWindow* parent, session_document* session_doc, wxTextCtrl* text_ctrl, config_manager& config, const wxString& file_path, long current_pos = -1, bookmark_filter initial_filter = bookmark_filter::all);
	~bookmark_dialog() override = default;
	bookmark_dialog(const bookmark_dialog&) = delete;
	bookmark_dialog& operator=(const bookmark_dialog&) = delete;
	bookmark_dialog(bookmark_dialog&&) = delete;
	bookmark_dialog& operator=(bookmark_dialog&&) = delete;

	[[nodiscard]] long get_selected_position() const {
		return selected_position;
	}

private:
	wxChoice* filter_choice{nullptr};
	wxListBox* bookmark_list{nullptr};
	std::vector<bookmark> bookmark_positions;
	long selected_position;
	config_manager& config;
	wxString file_path;
	wxTextCtrl* text_ctrl;
	session_document* session_doc_{nullptr};
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

class note_entry_dialog : public dialog {
public:
	note_entry_dialog(wxWindow* parent, const wxString& title, const wxString& message, const wxString& existing_note);
	~note_entry_dialog() override = default;
	note_entry_dialog(const note_entry_dialog&) = delete;
	note_entry_dialog& operator=(const note_entry_dialog&) = delete;
	note_entry_dialog(note_entry_dialog&&) = delete;
	note_entry_dialog& operator=(note_entry_dialog&&) = delete;

	[[nodiscard]] wxString get_note() const;

private:
	wxTextCtrl* note_ctrl{nullptr};
	void on_key_down(wxKeyEvent& event);
};


class password_dialog : public dialog {
public:
	explicit password_dialog(wxWindow* parent);
	~password_dialog() override = default;
	password_dialog(const password_dialog&) = delete;
	password_dialog& operator=(const password_dialog&) = delete;
	password_dialog(password_dialog&&) = delete;
	password_dialog& operator=(password_dialog&&) = delete;

	[[nodiscard]] wxString get_password() const;

private:
	wxTextCtrl* password_ctrl{nullptr};
};

class sleep_timer_dialog : public dialog {
public:
	sleep_timer_dialog(wxWindow* parent, int initial_duration);
	~sleep_timer_dialog() override = default;
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
	explicit toc_tree_item_data(int offset_) : offset{offset_} {
	}

	int offset{0};
};

class view_note_dialog : public dialog {
public:
	view_note_dialog(wxWindow* parent, const wxString& note_text);
	~view_note_dialog() override = default;
	view_note_dialog(const view_note_dialog&) = delete;
	view_note_dialog& operator=(const view_note_dialog&) = delete;
	view_note_dialog(view_note_dialog&&) = delete;
	view_note_dialog& operator=(view_note_dialog&&) = delete;

private:
	wxTextCtrl* note_ctrl{nullptr};
};

class web_view_dialog : public wxDialog {
public:
	web_view_dialog(wxWindow* parent, const wxString& title, const wxString& url_or_content, bool is_url = false, std::function<bool(const wxString&)> navigation_handler = nullptr);

private:
	wxWebView* web_view;
	std::function<bool(const wxString&)> navigation_handler_;

	void on_webview_loaded(wxWebViewEvent& event);
	void on_webview_navigating(wxWebViewEvent& event);
	void on_script_message(wxWebViewEvent& event);
	void simulate_click();
};
