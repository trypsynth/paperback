#pragma once

#include "main_window.hpp"
#include <wx/wx.h>

class app : public wxApp {
public:
	bool OnInit() override;

private:
	main_window* frame;
	void parse_command_line();
};

wxDECLARE_APP(app);
