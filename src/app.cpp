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

const wxString app::IPC_SERVICE = "paperback_ipc_service";

bool paperback_connection::OnExec(const wxString& topic, const wxString& data) {
	if (topic == "open_file") {
		wxGetApp().open_file(data);
		return true;
	}
	return false;
}

wxConnectionBase* paperback_server::OnAcceptConnection(const wxString& topic) {
	if (topic == "open_file") return new paperback_connection();
	return nullptr;
}

bool app::OnInit() {
	if (!config_mgr.initialize()) {
		wxMessageBox("Failed to initialize configuration", "Error", wxICON_ERROR);
		return false;
	}
	single_instance_checker = new wxSingleInstanceChecker("paperback_running");
	if (single_instance_checker->IsAnotherRunning()) {
		if (argc > 1) {
			paperback_client client;
			wxConnectionBase* connection = client.MakeConnection("localhost", IPC_SERVICE, "open_file");
			if (connection) {
				connection->Execute(wxString(argv[1]));
				connection->Disconnect();
				delete connection;
			}
		} else {
			paperback_client client;
			wxConnectionBase* connection = client.MakeConnection("localhost", IPC_SERVICE, "open_file");
			if (connection) {
				connection->Execute("ACTIVATE");
				connection->Disconnect();
				delete connection;
			}
		}
		delete single_instance_checker;
		single_instance_checker = nullptr;
		return false;
	}
	ipc_server = new paperback_server();
	if (!ipc_server->Create(IPC_SERVICE)) wxMessageBox("Failed to create IPC server", "Warning", wxICON_WARNING);
	frame = new main_window();
	if (config_mgr.get_restore_previous_documents())
		restore_previous_documents();
	if (argc > 1)
		parse_command_line();
	frame->Show(true);
	return true;
}

int app::OnExit() {
	if (ipc_server) {
		delete ipc_server;
		ipc_server = nullptr;
	}
	if (single_instance_checker) {
		delete single_instance_checker;
		single_instance_checker = nullptr;
	}
	config_mgr.shutdown();
	return wxApp::OnExit();
}

void app::parse_command_line() {
	wxString path = wxString(argv[1]);
	if (!wxFileName::FileExists(path)) {
		wxMessageBox("File not found: " + path, "Error", wxICON_ERROR);
		return;
	}
	auto* doc_manager = frame->get_doc_manager();
	const int existing_tab = doc_manager->find_tab_by_path(path);
	if (existing_tab >= 0) {
		frame->get_notebook()->SetSelection(existing_tab);
		auto* const text_ctrl = doc_manager->get_active_text_ctrl();
		if (text_ctrl) text_ctrl->SetFocus();
		return;
	}
	auto* par = find_parser_by_extension(wxFileName(path).GetExt());
	if (!par) {
		if (!should_open_as_txt(path)) return;
		par = find_parser_by_extension("txt");
	}
	if (!doc_manager->create_document_tab(path, par))
		wxMessageBox("Failed to load document.", "Error", wxICON_ERROR);
	doc_manager->update_ui();
}

void app::restore_previous_documents() {
	wxArrayString opened_docs = config_mgr.get_all_opened_documents();
	auto* doc_manager = frame->get_doc_manager();
	wxString active_doc = config_mgr.get_active_document();
	for (const auto& path : opened_docs) {
		if (!wxFileName::FileExists(path)) continue;
		const int existing_tab = doc_manager->find_tab_by_path(path);
		if (existing_tab >= 0) continue;
		auto* par = find_parser_by_extension(wxFileName(path).GetExt());
		if (!par) {
			if (!should_open_as_txt(path)) continue;
			par = find_parser_by_extension("txt");
		}
		if (!doc_manager->create_document_tab(path, par, false)) continue;
	}
	doc_manager->update_ui();
	if (!active_doc.IsEmpty() && wxFileName::FileExists(active_doc)) {
		const int active_tab = doc_manager->find_tab_by_path(active_doc);
		if (active_tab >= 0) {
			frame->get_notebook()->SetSelection(active_tab);
			auto* const text_ctrl = doc_manager->get_active_text_ctrl();
			if (text_ctrl) text_ctrl->SetFocus();
		}
	} else if (doc_manager->has_documents()) {
		auto* const text_ctrl = doc_manager->get_active_text_ctrl();
		if (text_ctrl) text_ctrl->SetFocus();
	}
}

void app::open_file(const wxString& filename) {
	if (filename == "ACTIVATE") {
		if (frame) frame->Raise();
		return;
	}
	if (!wxFileName::FileExists(filename)) {
		wxMessageBox("File not found: " + filename, "Error", wxICON_ERROR);
		return;
	}
	auto* doc_manager = frame->get_doc_manager();
	const int existing_tab = doc_manager->find_tab_by_path(filename);
	if (existing_tab >= 0) {
		frame->get_notebook()->SetSelection(existing_tab);
		auto* const text_ctrl = doc_manager->get_active_text_ctrl();
		if (text_ctrl) text_ctrl->SetFocus();
		frame->Raise();
		frame->RequestUserAttention();
		return;
	}
	auto* par = find_parser_by_extension(wxFileName(filename).GetExt());
	if (!par) {
		if (!should_open_as_txt(filename)) return;
		par = find_parser_by_extension("txt");
	}
	if (!doc_manager->create_document_tab(filename, par))
		wxMessageBox("Failed to load document.", "Error", wxICON_ERROR);
	else {
		frame->Raise();
		frame->RequestUserAttention();
	}
}

wxIMPLEMENT_APP(app);
