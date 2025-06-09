#pragma once

#include "main_window.hpp"
#include <wx/fileconf.h>
#include <wx/wx.h>

class app : public wxApp {
public:
	bool OnInit() override;
	int OnExit() override;
	void parse_command_line();
	wxFileConfig* config() {return conf.get();}

private:
	main_window* frame = nullptr;
	std::unique_ptr<wxFileConfig> conf;

	void load_default_config();
};

wxDECLARE_APP(app);
