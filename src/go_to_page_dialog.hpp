#pragma once
#include "document.hpp"
#include <wx/wx.h>

class go_to_page_dialog : public wxDialog {
public:
	go_to_page_dialog(wxWindow* parent, document* doc, int current_page = 1);
	int get_page_number() const;

private:
	document* doc_ = nullptr;
	wxTextCtrl* input_ctrl = nullptr;

	void on_key_down(wxKeyEvent& event);
	void on_char(wxKeyEvent& event);
	void adjust_page_number(int delta);
	int get_max_page() const;
};
