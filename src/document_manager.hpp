/* document_manager.hpp - document management header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "config_manager.hpp"
#include "document.hpp"
#include "utils.hpp"
#include <memory>
#include <wx/clntdata.h>
#include <wx/string.h>

class parser;
class wxNotebook;
class wxTextCtrl;
class wxPanel;
class wxWindow;
class wxMenu;
class main_window;

struct document_tab : public wxClientData {
	wxTextCtrl* text_ctrl{nullptr};
	std::unique_ptr<document> doc;
	wxString file_path;
	wxPanel* panel{nullptr};
	const parser* parser{nullptr};

	document_tab() = default;
	~document_tab() = default;
	document_tab(const document_tab&) = delete;
	document_tab& operator=(const document_tab&) = delete;
	document_tab(document_tab&&) = default;
	document_tab& operator=(document_tab&&) = default;
};

class document_manager {
public:
	explicit document_manager(wxNotebook* nbk, config_manager& cfg, main_window& win);
	~document_manager();
	document_manager(const document_manager&) = delete;
	document_manager& operator=(const document_manager&) = delete;
	document_manager(document_manager&&) = delete;
	document_manager& operator=(document_manager&&) = delete;
	[[nodiscard]] bool open_file(const wxString& path, bool add_to_recent = true);
	[[nodiscard]] bool create_document_tab(const wxString& path, const parser* parser, bool set_focus = true);
	void update_ui();
	void close_document(int index);
	void close_all_documents();
	[[nodiscard]] bool export_document(int index, const wxString& export_path) const;
	[[nodiscard]] document_tab* get_tab(int index) const;
	[[nodiscard]] document_tab* get_active_tab() const;
	[[nodiscard]] document* get_active_document() const;
	[[nodiscard]] wxTextCtrl* get_active_text_ctrl() const;
	[[nodiscard]] const parser* get_active_parser() const;
	[[nodiscard]] int get_tab_count() const;
	[[nodiscard]] int get_active_tab_index() const;
	[[nodiscard]] bool has_documents() const { return get_tab_count() > 0; }
	void go_to_position(long position) const;
	void go_to_previous_section() const;
	void go_to_next_section() const;
	void go_to_previous_heading();
	void go_to_next_heading();
	void go_to_previous_heading(int level);
	void go_to_next_heading(int level);
	void go_to_previous_page() const;
	void go_to_next_page() const;
	void go_to_previous_bookmark();
	void go_to_next_bookmark();
	void go_to_previous_link() const;
	void go_to_next_link() const;
	void activate_current_link() const;
	void toggle_bookmark();
	void show_bookmark_dialog(wxWindow* parent);
	void show_table_of_contents(wxWindow* parent) const;
	void show_document_info(wxWindow* parent) const;
	void save_document_position(const wxString& path, long position) const;
	[[nodiscard]] long load_document_position(const wxString& path) const;
	void save_current_tab_position() const;
	void save_all_tab_positions() const;
	[[nodiscard]] wxString get_status_text() const;
	[[nodiscard]] wxString get_window_title(const wxString& app_name) const;
	[[nodiscard]] long find_text(const wxString& query, long start_pos, find_options options) const;
	void apply_word_wrap(bool word_wrap);
	[[nodiscard]] int find_tab_by_path(const wxString& path) const;
	static void create_heading_menu(wxMenu* menu);

private:
	wxNotebook* notebook{nullptr};
	config_manager& config;
	main_window& main_win;

	static void setup_text_ctrl(wxTextCtrl* text_ctrl, const wxString& content);
<<<<<<< HEAD
	void restore_document_position(document_tab* tab) const;
=======
	void restore_document_position(document_tab* tab);
>>>>>>> multyple_windows
	wxPanel* create_tab_panel(const wxString& content, document_tab* tab_data);
	void navigate_to_heading(bool next, int specific_level = -1) const;
};
