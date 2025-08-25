/* app.cpp - wxApp implementation code.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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
	if (argc > 1)
		parse_command_line();
	else if (config_mgr.get_restore_previous_documents())
		restore_previous_documents();
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

void app::restore_previous_documents() {
	wxArrayString opened_docs = config_mgr.get_opened_documents();
	for (const auto& path : opened_docs) {
		if (!wxFileName::FileExists(path)) continue;
		auto* par = find_parser_by_extension(wxFileName(path).GetExt());
		if (!par) {
			if (!should_open_as_txt(path)) continue;
			par = find_parser_by_extension("txt");
		}
		if (!frame->get_doc_manager()->open_document(path, par)) continue;
	}
}

wxIMPLEMENT_APP(app);
