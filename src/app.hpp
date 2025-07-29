#pragma once
#include "config_manager.hpp"
#include "main_window.hpp"
#include <wx/wx.h>

class app : public wxApp {
public:
	bool OnInit() override;
	int OnExit() override;
	void parse_command_line();
	config_manager& get_config_manager() { return config_mgr; }

private:
	main_window* frame = nullptr;
	config_manager config_mgr;
	void load_default_config();
};

wxDECLARE_APP(app);
