#pragma once
#include "config_manager.hpp"
#include "dialogs.hpp"
#include "document_data.hpp"
#include "utils.hpp"
#include <memory>
#include <wx/clntdata.h>
#include <wx/string.h>

class wxNotebook;
class wxTextCtrl;
class wxPanel;
class wxWindow;
class wxMenu;
class main_window;

struct document_tab : public wxClientData {
	wxTextCtrl* text_ctrl{nullptr};
	std::unique_ptr<session_document> session_doc;
	wxString file_path;
	wxPanel* panel{nullptr};

	document_tab() = default;
	~document_tab() = default;
	document_tab(const document_tab&) = delete;
	document_tab& operator=(const document_tab&) = delete;
	document_tab(document_tab&&) = default;
	document_tab& operator=(document_tab&&) = default;

	// Get the document session
	[[nodiscard]] DocumentSession* get_session() const {
		return session_doc ? &*session_doc->session : nullptr;
	}

	// Get title
	[[nodiscard]] wxString get_title() const {
		return session_doc ? session_doc->get_title() : wxString("Untitled");
	}

	// Get content
	[[nodiscard]] const wxString& get_content() const {
		static wxString empty;
		return session_doc ? session_doc->content : empty;
	}
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
	[[nodiscard]] bool create_document_tab(const wxString& path, bool set_focus = true, bool add_to_recent = true);
	void update_ui();
	void close_document(int index);
	void close_all_documents();
	[[nodiscard]] bool export_document(int index, const wxString& export_path) const;
	[[nodiscard]] document_tab* get_tab(int index) const;
	[[nodiscard]] document_tab* get_active_tab() const;
	[[nodiscard]] wxTextCtrl* get_active_text_ctrl() const;
	[[nodiscard]] int get_tab_count() const;
	[[nodiscard]] int get_active_tab_index() const;

	[[nodiscard]] bool has_documents() const {
		return get_tab_count() > 0;
	}

	void go_to_position(int position) const;
	void go_to_previous_section() const;
	void go_to_next_section() const;
	void go_to_previous_heading() const;
	void go_to_next_heading() const;
	void go_to_previous_heading(int level) const;
	void go_to_next_heading(int level) const;
	void go_to_previous_page() const;
	void go_to_next_page() const;
	void go_to_previous_bookmark() const;
	void go_to_next_bookmark() const;
	void go_to_previous_note() const;
	void go_to_next_note() const;
	void go_to_previous_link() const;
	void go_to_next_link() const;
	void go_to_previous_table();
	void go_to_next_table();
	void activate_current_table();
	void go_to_previous_list() const;
	void go_to_next_list() const;
	void go_to_previous_list_item() const;
	void go_to_next_list_item() const;
	void go_to_previous_position() const;
	void go_to_next_position() const;
	void navigate_history(bool next) const;
	void activate_current_link() const;
	void toggle_bookmark() const;
	void add_bookmark_with_note() const;
	void show_bookmark_dialog(wxWindow* parent, bookmark_filter initial_filter = bookmark_filter::all);
	void show_table_of_contents(wxWindow* parent);
	void show_document_info(wxWindow* parent);
	void save_document_position(const wxString& path, long position) const;
	void save_current_tab_position() const;
	void save_all_tab_positions() const;
	[[nodiscard]] wxString get_status_text() const;
	[[nodiscard]] wxString get_window_title(const wxString& app_name) const;
	void apply_word_wrap(bool word_wrap);
	[[nodiscard]] int find_tab_by_path(const wxString& path) const;
	static void create_heading_menu(wxMenu* menu);

private:
	wxNotebook* notebook{nullptr};
	config_manager& config;
	main_window& main_win;

	static void show_parser_error(const parser_exception& e);
	static void setup_text_ctrl(wxTextCtrl* text_ctrl, const wxString& content);
	void restore_document_position(document_tab* tab) const;
	wxPanel* create_tab_panel(const wxString& content, document_tab* tab_data);
	void navigate_to_heading(bool next, int specific_level = -1) const;
	void navigate_to_section(bool next) const;
	void navigate_to_page(bool next) const;
	void navigate_to_bookmark(bool next) const;
	void navigate_to_note(bool next) const;
	void navigate_to_bookmark_or_note(bool next, bool notes_only) const;
	void navigate_to_link(bool next) const;
	void navigate_to_table(bool next) const;
	void navigate_to_list(bool next) const;
	void navigate_to_list_item(bool next) const;
	void navigate_to_element(NavTarget target, bool next, int level_filter = 0) const;
};
