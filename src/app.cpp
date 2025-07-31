#include "app.hpp"
#include "constants.hpp"
#include "parser.hpp"
#include "utils.hpp"

bool app::OnInit() {
	if (!config_mgr.initialize()) {
		wxMessageBox("Failed to initialize configuration", "Error", wxICON_ERROR);
		return false;
	}
	frame = new main_window();
	if (argc > 1) parse_command_line();
	frame->Show(true);
	return true;
}

int app::OnExit() {
	config_mgr.flush();
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
		if (!should_open_as_txt(path)) return;
		par = find_parser_by_extension("txt");
	}
	if (!frame->get_doc_manager()->open_document(path, par))
		wxMessageBox("Failed to load document.", "Error", wxICON_ERROR);
}

wxIMPLEMENT_APP(app);
