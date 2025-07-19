#include "app.hpp"
#include "constants.hpp"
#include "parser.hpp"
#include <wx/config.h>
#include <wx/filename.h>
#include <wx/stdpaths.h>

bool app::OnInit() {
	wxString exePath = wxStandardPaths::Get().GetExecutablePath();
	wxString exeDir = wxFileName(exePath).GetPath();
	wxString confpath = exeDir + wxFileName::GetPathSeparator() + APP_NAME + ".ini";
	conf = std::make_unique<wxFileConfig>(APP_NAME, "", confpath);
	wxConfigBase::Set(conf.get());
	load_default_config();
	frame = new main_window();
	if (argc > 1) parse_command_line();
	frame->Show(true);
	return true;
}

int app::OnExit() {
	if (conf) conf->Flush();
	return wxApp::OnExit();
}

void app::parse_command_line() {
	wxString path = wxString(argv[1]);
	if (!wxFileName::FileExists(path)) {
		wxMessageBox("File not found: " + path, "Error", wxICON_ERROR);
		return;
	}
	auto* par = find_parser_by_extension(wxFileName(path).GetExt());
	if (!par) {
		const bool open_as_txt = wxMessageBox("No suitable parser was found for " + path + ". Would you like to treat it as plain text?", "Warning", wxICON_WARNING | wxYES_NO) == wxYES;
		if (!open_as_txt) return;
		par = find_parser_by_extension("txt");
	}
	frame->open_document(path, par);
}

void app::load_default_config() {
	if (!conf->Exists("test")) conf->Write("Test", 1);
	conf->Flush();
}

wxIMPLEMENT_APP(app);
