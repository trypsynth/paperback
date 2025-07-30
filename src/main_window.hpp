#pragma once
#include "document_manager.hpp"
#include <memory>
#include <wx/fdrepdlg.h>
#include <wx/wx.h>

enum {
	ID_EXPORT = wxID_HIGHEST + 1,
	ID_FIND_NEXT,
	ID_FIND_PREVIOUS,
	ID_GO_TO,
	ID_PREVIOUS_SECTION,
	ID_NEXT_SECTION,
	ID_WORD_COUNT,
	ID_DOC_INFO,
	ID_TABLE_OF_CONTENTS,
	ID_CHECK_FOR_UPDATES,
};

class main_window : public wxFrame {
public:
	main_window();
	document_manager* get_doc_manager() const { return doc_manager.get(); }

private:
	std::unique_ptr<document_manager> doc_manager;
	wxNotebook* notebook = nullptr;
	wxStatusBar* status_bar = nullptr;
	wxTimer* position_save_timer = nullptr;
	wxFindReplaceDialog* find_dialog = nullptr;
	wxFindReplaceData find_data;

	void create_menus();
	wxMenu* create_file_menu();
	wxMenu* create_go_menu();
	wxMenu* create_tools_menu();
	wxMenu* create_help_menu();
	void bind_events();
	void update_ui();
	void update_title();
	void update_status_bar();
	void on_open(wxCommandEvent& event);
	void on_close(wxCommandEvent& event);
	void on_close_all(wxCommandEvent& event);
	void on_export(wxCommandEvent& event);
	void on_exit(wxCommandEvent& event);
	void on_find(wxCommandEvent& event);
	void on_find_next(wxCommandEvent& event);
	void on_find_previous(wxCommandEvent& event);
	void on_go_to(wxCommandEvent& event);
	void on_previous_section(wxCommandEvent& event);
	void on_next_section(wxCommandEvent& event);
	void on_word_count(wxCommandEvent& event);
	void on_doc_info(wxCommandEvent& event);
	void on_toc(wxCommandEvent& event);
	void on_about(wxCommandEvent& event);
	void on_notebook_page_changed(wxBookCtrlEvent& event);
	void on_find_dialog(wxFindDialogEvent& event);
	void on_find_close(wxFindDialogEvent& event);
	void on_text_cursor_changed(wxEvent& event);
	void on_close_window(wxCloseEvent& event);
	void on_position_save_timer(wxTimerEvent& event);
};
