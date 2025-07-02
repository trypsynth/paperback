#pragma once

#include "document.hpp"
#include <wx/wx.h>

class document_info_dialog : public wxDialog {
public:
	document_info_dialog(wxWindow* parent, const document* doc);

private:
	wxTextCtrl* info_text_ctrl = nullptr;
};
