#include "app.hpp"
#include "main_window.hpp"
#include "parser.hpp"
#include <wx/filename.h>

bool app::OnInit() {
	main_window* frame = new main_window();
	if (argc > 1) {
		wxString path = wxString(argv[1]);
		if (wxFileName::FileExists(path)) {
			parser* par = find_parser_by_extension(wxFileName(path).GetExt());
			if (!par)
				wxMessageBox("No suitable parser found for " + path, "Error", wxICON_ERROR);
			else {
				std::unique_ptr<document> doc = par->load(path);
				if (!doc)
					wxMessageBox("Failed to load the document: " + path, "Error", wxICON_ERROR);
				else
					frame->open_document(path, std::move(doc));
			}
		} else
			wxMessageBox("File not found: " + path, "Error", wxICON_ERROR);
	}
	frame->Show(true);
	return true;
}

wxIMPLEMENT_APP(app);
