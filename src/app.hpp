#pragma once

#include "main_window.hpp"
#include <wx/fileconf.h>
#include <wx/wx.h>

class app : public wxApp {
public:
	bool OnInit() override;
	int OnExit() override;
	void parse_command_line();
	wxFileConfig* config() {return config_;}

private:
	main_window* frame = nullptr;
	wxFileConfig* config_ = nullptr;

	void load_default_config();
};

wxDECLARE_APP(app);
