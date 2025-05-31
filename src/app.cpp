#include "app.hpp"
#include "constants.hpp"
#include "parser.hpp"
#include <wx/filename.h>

bool app::OnInit() {
	config = std::make_unique<wxFileConfig>(APP_NAME, "Quinware", "config.ini", "", wxCONFIG_USE_LOCAL_FILE);
	frame = new main_window();
	if (argc > 1) parse_command_line();
	frame->Show(true);
	return true;
}

int app::OnExit() {
	config->Write("Test", 1);
	config->Flush();
	return 0;
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
	std::unique_ptr<document> doc = par->load(path);
	if (!doc) {
		wxMessageBox("Failed to load the document: " + path, "Error", wxICON_ERROR);
		return;
	}
	frame->open_document(path, std::move(doc));
}

wxIMPLEMENT_APP(app);
