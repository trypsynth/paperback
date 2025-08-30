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
#include <memory>
#include <wx/bookctrl.h>
#include <wx/wx.h>

class main_window : public wxFrame {
public:
	main_window();
	~main_window() = default;
	main_window(const main_window&) = delete;
	main_window& operator=(const main_window&) = delete;
	main_window(main_window&&) = delete;
	main_window& operator=(main_window&&) = delete;
	[[nodiscard]] document_manager* get_doc_manager() const noexcept { return doc_manager.get(); }

private:
	std::unique_ptr<document_manager> doc_manager;
	wxNotebook* notebook{nullptr};
	wxStatusBar* status_bar{nullptr};
	wxTimer* position_save_timer{nullptr};
	find_dialog* find_dlg{nullptr};
	wxMenu* recent_documents_menu{nullptr};

	void create_menus();
	wxMenu* create_file_menu();
	wxMenu* create_go_menu();
	wxMenu* create_tools_menu();
	wxMenu* create_help_menu();
	void bind_events();
	void update_ui();
	void update_title();
	void update_status_bar();
	void on_open(wxCommandEvent&);
	void on_close(wxCommandEvent&);
	void on_close_all(wxCommandEvent&);
	void on_export(wxCommandEvent&);
	void on_exit(wxCommandEvent&);
	void on_find(wxCommandEvent&);
	void on_find_next(wxCommandEvent&);
	void on_find_previous(wxCommandEvent&);
	void on_go_to(wxCommandEvent&);
	void on_go_to_page(wxCommandEvent&);
	void on_previous_section(wxCommandEvent&);
	void on_next_section(wxCommandEvent&);
	void on_previous_heading(wxCommandEvent&);
	void on_next_heading(wxCommandEvent&);
	void on_previous_heading_1(wxCommandEvent&);
	void on_next_heading_1(wxCommandEvent&);
	void on_previous_heading_2(wxCommandEvent&);
	void on_next_heading_2(wxCommandEvent&);
	void on_previous_heading_3(wxCommandEvent&);
	void on_next_heading_3(wxCommandEvent&);
	void on_previous_heading_4(wxCommandEvent&);
	void on_next_heading_4(wxCommandEvent&);
	void on_previous_heading_5(wxCommandEvent&);
	void on_next_heading_5(wxCommandEvent&);
	void on_previous_heading_6(wxCommandEvent&);
	void on_next_heading_6(wxCommandEvent&);
	void on_previous_page(wxCommandEvent&);
	void on_next_page(wxCommandEvent&);
	void on_word_count(wxCommandEvent&);
	void on_doc_info(wxCommandEvent&);
	void on_toc(wxCommandEvent&);
	void on_options(wxCommandEvent&);
	void on_about(wxCommandEvent&);
	void on_help(wxCommandEvent&);
	void on_help_internal(wxCommandEvent&);
	void on_notebook_page_changed(wxBookCtrlEvent& event);
	void on_text_cursor_changed(wxEvent& event);
	void on_close_window(wxCloseEvent& event);
	void on_position_save_timer(wxTimerEvent&);
	void on_recent_document(wxCommandEvent& event);
	void do_find(bool forward);
	void update_recent_documents_menu();
};
