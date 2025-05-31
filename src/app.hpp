#pragma once

#include "main_window.hpp"
#include <wx/wx.h>
#include <wx/fileconf.h>

class app : public wxApp {
public:
	bool OnInit() override;
	int OnExit() override;
	void parse_command_line();


private:
	main_window* frame = nullptr;
	std::unique_ptr<wxFileConfig> config;
};

wxDECLARE_APP(app);
