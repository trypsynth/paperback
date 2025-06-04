#pragma once

#include "parser.hpp"
#include <wx/fdrepdlg.h>
#include <wx/notebook.h>
#include <wx/wx.h>

enum {
	ID_EXPORT_PLAIN_TEXT = wxID_HIGHEST + 1,
	ID_FIND_NEXT,
	ID_FIND_PREVIOUS,
	ID_GO_TO,
	ID_PREVIOUS_SECTION,
	ID_NEXT_SECTION,
	ID_WORD_COUNT,
	ID_TABLE_OF_CONTENTS,
	ID_CHECK_FOR_UPDATES,
};

constexpr int doc_command_ids[] = {
	wxID_CLOSE,
	wxID_CLOSE_ALL,
	ID_EXPORT_PLAIN_TEXT,
	wxID_FIND,
	ID_FIND_NEXT,
	ID_FIND_PREVIOUS,
	ID_GO_TO,
	ID_PREVIOUS_SECTION,
	ID_NEXT_SECTION,
	ID_WORD_COUNT,
	ID_TABLE_OF_CONTENTS,
};

// Each tab stores a pointer to one of these.
struct user_data : public wxClientData {
	wxTextCtrl* textbox = nullptr;
	parser* par = nullptr;
};

class main_window : public wxFrame {
public:
	main_window();
	user_data* active_user_data() const;
	wxTextCtrl* active_text_ctrl() const;
	parser* active_parser() const;
	void open_document(const wxString& path, parser* par);

private:
	wxNotebook* notebook = nullptr;
	wxFindReplaceDialog* find_dialog = nullptr;
	wxFindReplaceData find_data;

	void update_doc_commands(wxUpdateUIEvent& event);
	void on_open(wxCommandEvent& event);
	void on_close(wxCommandEvent& event);
	void on_close_all(wxCommandEvent& event);
	void on_export_plain_text(wxCommandEvent& event);
	void on_exit(wxCommandEvent& event);
	void on_find(wxCommandEvent& event);
	void on_find_next(wxCommandEvent& event);
	void on_find_previous(wxCommandEvent& event);
	void on_go_to(wxCommandEvent& event);
	void on_previous_section(wxCommandEvent& event);
	void on_next_section(wxCommandEvent& event);
	void on_word_count(wxCommandEvent& event);
	void on_about(wxCommandEvent& event);
	void on_find_dialog(wxFindDialogEvent& event);
	void on_find_close(wxFindDialogEvent& event);
};
