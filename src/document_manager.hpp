#pragma once
#include "document.hpp"
#include "parser.hpp"
#include <memory>
#include <vector>
#include <wx/notebook.h>
#include <wx/textctrl.h>
#include <wx/wx.h>

// Represents a single document tab with its associated data.
struct document_tab : public wxClientData {
	wxTextCtrl* text_ctrl = nullptr;
	std::unique_ptr<document> doc;
	wxString file_path;
	wxPanel* panel = nullptr;
};

// Manages all open documents and their associated UI components.
class document_manager {
public:
	explicit document_manager(wxNotebook* notebook);
	~document_manager();
	bool open_document(const wxString& path, const parser* parser);
	void close_document(int index);
	void close_all_documents();
	bool export_document(int index, const wxString& export_path);
	document_tab* get_tab(int index) const;
	document_tab* get_active_tab() const;
	document* get_active_document() const;
	wxTextCtrl* get_active_text_ctrl() const;
	int get_tab_count() const;
	int get_active_tab_index() const;
	bool has_documents() const { return get_tab_count() > 0; }
	bool active_document_supports_sections() const;
	bool active_document_supports_toc() const;
	void go_to_position(long position);
	void go_to_previous_section();
	void go_to_next_section();
	void show_table_of_contents(wxWindow* parent);
	int get_word_count() const;
	void show_document_info(wxWindow* parent);
	void save_document_position(const wxString& path, long position);
	long load_document_position(const wxString& path);
	void save_current_tab_position();
	void save_all_tab_positions();
	wxString get_status_text() const;
	wxString get_window_title(const wxString& app_name) const;
	long find_text(const wxString& query, long start_pos, bool forward, bool match_case) const;

private:
	wxNotebook* notebook_;

	void setup_text_ctrl(wxTextCtrl* text_ctrl, const wxString& content);
	void restore_document_position(document_tab* tab);
	wxPanel* create_tab_panel(const wxString& content, document_tab* tab_data);
};
