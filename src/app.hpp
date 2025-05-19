#pragma once

#include <wx/wx.h>

class app : public wxApp {
public:
	bool OnInit() override;
};

wxDECLARE_APP(app);
