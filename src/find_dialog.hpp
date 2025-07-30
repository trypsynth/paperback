#pragma once
#include <wx/wx.h>
#include <wx/regex.h>

enum {
	ID_FIND_NEXT_BTN = wxID_HIGHEST + 1,
	ID_FIND_PREVIOUS_BTN,
	ID_FIND_TEXT_CTRL
};

class find_dialog : public wxDialog {
public:
	find_dialog(wxWindow* parent);
	wxString get_find_text() const;
	bool get_match_case() const;
	bool get_whole_word() const;
	bool get_use_regex() const;
	void set_find_text(const wxString& text);
	void focus_find_text();

private:
	wxTextCtrl* find_text_ctrl;
	wxCheckBox* match_case_cb;
	wxCheckBox* whole_word_cb;
	wxCheckBox* regex_cb;
	wxButton* find_next_btn;
	wxButton* find_previous_btn;
	wxButton* cancel_btn;

	void create_controls();
	void on_find_next(wxCommandEvent& event);
	void on_find_previous(wxCommandEvent& event);
	void on_cancel(wxCommandEvent& event);
	void on_text_enter(wxCommandEvent& event);
	void on_close(wxCloseEvent& event);
};
