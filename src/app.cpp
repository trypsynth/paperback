#include "app.hpp"
#include "constants.hpp"
#include "parser.hpp"
#include <wx/filename.h>

bool app::OnInit() {
	wxString config_path = wxGetCwd() + wxFileName::GetPathSeparator() + APP_NAME + ".ini";
	config_ = new wxFileConfig(APP_NAME, "", config_path);
	wxConfigBase::Set(config_);
	load_default_config();
	frame = new main_window();
	if (argc > 1) parse_command_line();
	frame->Show(true);
	return true;
}

int app::OnExit() {
	if (config_) {
		config_->Flush();
		delete config_;
	}
	return wxApp::OnExit();
}

void app::parse_command_line() {
	wxString path = wxString(argv[1]);
	if (!wxFileName::FileExists(path)) {
		wxMessageBox("File not found: " + path, "Error", wxICON_ERROR);
		return;
	}
	parser* par = find_parser_by_extension(wxFileName(path).GetExt());
	if (!par) {
		wxMessageBox("No suitable parser found for " + path, "Error", wxICON_ERROR);
		return;
	}
	frame->open_document(path, par);
}

void app::load_default_config() {
	if (!config_->Exists("test"))
		config_->Write("Test", 1);
	config_->Flush();
}

wxIMPLEMENT_APP(app);
