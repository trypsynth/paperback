/* main_window.hpp - primary user interface header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "dialogs.hpp"
#include "document_manager.hpp"
#include "task_bar_icon.hpp"
#include <memory>
#include <wx/bookctrl.h>
#include <wx/longlong.h>
#include <wx/wx.h>



class main_window : public wxFrame {
public:
	main_window();
	~main_window();
	main_window(const main_window&) = delete;
	main_window& operator=(const main_window&) = delete;
	main_window(main_window&&) = delete;
	main_window& operator=(main_window&&) = delete;

	[[nodiscard]] document_manager* get_doc_manager() const noexcept {
		return doc_manager.get();
	}

	[[nodiscard]] wxNotebook* get_notebook() const noexcept {
		return notebook;
	}

	[[nodiscard]] wxStaticText* get_live_region_label() const noexcept {
		return live_region_label;
	}

	void on_text_cursor_changed(wxEvent& event);
	void on_text_char(wxKeyEvent& event);
	void trigger_throttled_position_save();
	void save_position_immediately();
	void trigger_throttled_status_update();
	void update_ui();
	void update_title();
	void update_status_bar();
	void update_recent_documents_menu();
	void set_document_content(const wxString& content);
	bool is_in_single_window_mode() const { return single_window_mode; }
	wxTextCtrl* get_single_text_ctrl() const { return single_text_ctrl; }

private:
	std::unique_ptr<document_manager> doc_manager;
	wxPanel* main_panel{nullptr};
	wxNotebook* notebook{nullptr};
	wxPanel* single_doc_panel{nullptr};
	wxTextCtrl* single_text_ctrl{nullptr};
	bool single_window_mode{false};
	wxStatusBar* status_bar{nullptr};
	wxTimer* position_save_timer{nullptr};
	wxTimer* status_update_timer{nullptr};
	wxLongLong last_status_update_time{0};
	find_dialog* find_dlg{nullptr};
	wxMenu* recent_documents_menu{nullptr};
	wxStaticText* live_region_label{nullptr};
	task_bar_icon* task_bar_icon_{nullptr};
	wxTimer* sleep_timer{nullptr};
	wxTimer* sleep_status_update_timer{nullptr};
	int sleep_timer_duration_minutes{0};
	wxLongLong sleep_timer_start_time{0};

	void create_menus();
	wxMenu* create_file_menu();
	static wxMenu* create_go_menu();
	static wxMenu* create_tools_menu();
	static wxMenu* create_help_menu();
	void refresh_ui_language();
	void bind_events();
	void on_open(wxCommandEvent&);
	void on_close(wxCommandEvent&);
	void on_close_all(wxCommandEvent&);
	void on_export(wxCommandEvent&);
	void on_exit(wxCommandEvent&);
	void on_find(wxCommandEvent&);
	void on_find_next(wxCommandEvent&);
	void on_find_previous(wxCommandEvent&);
	void on_go_to_line(wxCommandEvent&);
	void on_go_to_percent(wxCommandEvent&);
	void on_go_to_page(wxCommandEvent&);
	void on_previous_section(wxCommandEvent&);
	void on_next_section(wxCommandEvent&);
	void on_previous_heading(wxCommandEvent&);
	void on_next_heading(wxCommandEvent&);
	void on_previous_page(wxCommandEvent&);
	void on_next_page(wxCommandEvent&);
	void on_next_bookmark(wxCommandEvent&);
	void on_previous_bookmark(wxCommandEvent&);
	void on_toggle_bookmark(wxCommandEvent&);
	void on_bookmark_with_note(wxCommandEvent&);
	void on_jump_to_bookmark(wxCommandEvent&);
	void on_next_link(wxCommandEvent&);
	void on_previous_link(wxCommandEvent&);
	void on_word_count(wxCommandEvent&);
	void on_doc_info(wxCommandEvent&);
	void on_toc(wxCommandEvent&);
	void on_options(wxCommandEvent&);
	void on_about(wxCommandEvent&);
	void on_help(wxCommandEvent&);
	void on_help_internal(wxCommandEvent&);
	void on_donate(wxCommandEvent&);
	void on_check_for_updates(wxCommandEvent&);
	void on_sleep_timer(wxCommandEvent&);
	void on_sleep_timer_tick(wxTimerEvent&);
	void on_sleep_status_update_timer(wxTimerEvent&);
	void on_notebook_page_changed(wxBookCtrlEvent& event);
	void on_close_window(wxCloseEvent& event);
	void on_position_save_timer(wxTimerEvent&);
	void on_status_update_timer(wxTimerEvent&);
	void on_recent_document(wxCommandEvent& event);
	void on_show_all_documents(wxCommandEvent&);
	void on_notebook_key_down(wxKeyEvent& event);
	void on_iconize(wxIconizeEvent& event);
	void on_activate(wxActivateEvent& event);
	void do_find(bool forward);
	void navigate_heading_by_level(int level, bool forward);
};