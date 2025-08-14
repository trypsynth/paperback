#pragma once
#include <wx/wx.h>

class find_dialog : public wxDialog {
public:
	find_dialog(wxWindow* parent);
	wxString get_find_text() const;
	bool get_match_case() const;
	bool get_match_whole_word() const;
	void set_find_text(const wxString& text);
	void add_to_history(const wxString& text);
	void focus_find_text();

private:
	wxComboBox* find_what_combo = nullptr;
	wxCheckBox* match_case_check = nullptr;
	wxCheckBox* match_whole_word_check = nullptr;
	wxButton* find_previous_btn = nullptr;
	wxButton* find_next_btn = nullptr;
	wxButton* cancel_btn = nullptr;

	void create_controls();
	void bind_events();
	void on_find_previous(wxCommandEvent& event);
	void on_find_next(wxCommandEvent& event);
	void on_cancel(wxCommandEvent& event);
	void on_find_text_enter(wxCommandEvent& event);
	void on_close(wxCloseEvent& event);
};
