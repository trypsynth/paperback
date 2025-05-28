#pragma once

#include "document.hpp"
#include <wx/wx.h>
#include <wx/notebook.h>

enum {
	ID_EXPORT = wxID_HIGHEST + 1,
	ID_FIND_NEXT,
	ID_FIND_PREVIOUS,
	ID_GO_TO,
	ID_TABLE_OF_CONTENTS,
	ID_WORD_COUNT,
	ID_CHECK_FOR_UPDATES,
};

constexpr int doc_command_ids[] = {
	wxID_CLOSE,
	wxID_CLOSE_ALL,
	ID_EXPORT,
	wxID_FIND,
	ID_FIND_NEXT,
	ID_FIND_PREVIOUS,
	ID_GO_TO,
	ID_TABLE_OF_CONTENTS,
	ID_WORD_COUNT,
};

class main_window : public wxFrame {
public:
	main_window();
	wxTextCtrl* active_text_ctrl();
	void open_document(const wxString& path, std::unique_ptr<document> doc);

private:
	wxNotebook* notebook;
	void update_doc_commands(wxUpdateUIEvent& event);
	void on_open(wxCommandEvent& event);
	void on_close(wxCommandEvent& event);
	void on_close_all(wxCommandEvent& event);
	void on_export(wxCommandEvent& event);
	void on_exit(wxCommandEvent& event);
	void on_go_to(wxCommandEvent& event);
	void on_word_count(wxCommandEvent& event);
	void on_about(wxCommandEvent& event);
};
