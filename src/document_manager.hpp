#pragma once
#include "document.hpp"
#include "utils.hpp"
#include <memory>
#include <wx/string.h>
#include <wx/clntdata.h>

class parser;
class wxNotebook;
class wxTextCtrl;
class wxPanel;
class wxWindow;

struct document_tab : public wxClientData {
	wxTextCtrl* text_ctrl{nullptr};
	std::unique_ptr<document> doc;
	wxString file_path;
	wxPanel* panel{nullptr};
};

class document_manager {
public:
	explicit document_manager(wxNotebook* notebook);
	~document_manager();
	[[nodiscard]] bool open_document(const wxString& path, const parser* parser);
	void close_document(int index);
	void close_all_documents();
	[[nodiscard]] bool export_document(int index, const wxString& export_path);
	[[nodiscard]] document_tab* get_tab(int index) const;
	[[nodiscard]] document_tab* get_active_tab() const;
	[[nodiscard]] document* get_active_document() const;
	[[nodiscard]] wxTextCtrl* get_active_text_ctrl() const;
	[[nodiscard]] int get_tab_count() const;
	[[nodiscard]] int get_active_tab_index() const;
	[[nodiscard]] bool has_documents() const { return get_tab_count() > 0; }
	void go_to_position(long position);
	void go_to_previous_section();
	void go_to_next_section();
	void go_to_previous_page();
	void go_to_next_page();
	void show_table_of_contents(wxWindow* parent);
	[[nodiscard]] int get_word_count() const;
	void show_document_info(wxWindow* parent);
	void save_document_position(const wxString& path, long position) const;
	[[nodiscard]] long load_document_position(const wxString& path) const;
	void save_current_tab_position();
	void save_all_tab_positions();
	[[nodiscard]] wxString get_status_text() const;
	[[nodiscard]] wxString get_window_title(const wxString& app_name) const;
	[[nodiscard]] long find_text(const wxString& query, long start_pos, find_options options) const;

private:
	wxNotebook* notebook_;

	void setup_text_ctrl(wxTextCtrl* text_ctrl, const wxString& content);
	void restore_document_position(document_tab* tab);
	wxPanel* create_tab_panel(const wxString& content, document_tab* tab_data);
};
