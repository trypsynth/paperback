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
#include "translation_manager.hpp"
#include "update_checker.hpp"
#include "utils.hpp"
#include <wx/filename.h>

bool paperback_connection::OnExec(const wxString& topic, const wxString& data) {
	if (topic == IPC_TOPIC_OPEN_FILE) {
		wxGetApp().CallAfter([data]() {
			wxGetApp().open_file(data);
		});
		return true;
	}
	return false;
}

wxConnectionBase* paperback_server::OnAcceptConnection(const wxString& topic) {
	if (topic == IPC_TOPIC_OPEN_FILE) {
		return new paperback_connection();
	}
	return nullptr;
}

bool app::OnInit() {
	if (!config_mgr.initialize()) {
		wxMessageBox(_("Failed to initialize configuration"), _("Error"), wxICON_ERROR);
		return false;
	}
	translation_manager::instance().initialize();
	const wxString preferred_language = config_mgr.get(config_manager::language);
	if (!preferred_language.IsEmpty()) {
		translation_manager::instance().set_language(preferred_language);
	}
	single_instance_checker = std::make_unique<wxSingleInstanceChecker>(SINGLE_INSTANCE_NAME);
	if (single_instance_checker->IsAnotherRunning()) {
		if (argc > 1) {
			paperback_client client;
			const std::unique_ptr<wxConnectionBase> connection(client.MakeConnection(IPC_HOST_LOCALHOST, IPC_SERVICE, IPC_TOPIC_OPEN_FILE));
			if (connection) {
				const wxString arg_path = wxString(argv[1]);
				wxFileName file_path{arg_path};
				file_path.Normalize(wxPATH_NORM_ABSOLUTE);
				connection->Execute(file_path.GetFullPath());
				connection->Disconnect();
			}
		} else {
			paperback_client client;
			const std::unique_ptr<wxConnectionBase> connection(client.MakeConnection(IPC_HOST_LOCALHOST, IPC_SERVICE, IPC_TOPIC_OPEN_FILE));
			if (connection) {
				connection->Execute(IPC_COMMAND_ACTIVATE);
				connection->Disconnect();
			}
		}
		return false;
	}
	ipc_server = std::make_unique<paperback_server>();
	if (!ipc_server->Create(IPC_SERVICE)) {
		wxMessageBox(_("Failed to create IPC server"), _("Warning"), wxICON_WARNING);
	}
	if (config_mgr.get(config_manager::restore_previous_documents)) {
		restore_previous_documents();
	}
	if (argc > 1) {
		parse_command_line();
	}
	if (frames.empty()) {
		create_new_window();
	}
	if (config_mgr.get(config_manager::check_for_updates_on_startup)) {
		check_for_updates(true);
	}
	return true;
}

int app::OnExit() {
	config_mgr.shutdown();
	return wxApp::OnExit();
}

void app::parse_command_line() {
	const wxString arg_path = wxString(argv[1]);
	wxFileName file_path{arg_path};
	file_path.Normalize(wxPATH_NORM_ABSOLUTE);
	wxString path = file_path.GetFullPath();
	open_file(path);
}

void app::restore_previous_documents() {
	wxArrayString opened_docs = config_mgr.get_all_opened_documents();
	if (config_mgr.get(config_manager::open_in_new_window)) {
		for (const wxString& path : opened_docs) {
			if (!wxFileName::FileExists(path)) {
				continue;
			}
			create_new_window(path);
		}
	} else {
		if (opened_docs.IsEmpty()) {
			return;
		}
		main_window* frame = frames.empty() ? create_new_window() : frames.front();
		auto* doc_manager = frame->get_doc_manager();
		wxString active_doc = config_mgr.get(config_manager::active_document);
		for (const wxString& path : opened_docs) {
			if (!wxFileName::FileExists(path)) {
				continue;
			}
			const int existing_tab = doc_manager->find_tab_by_path(path);
			if (existing_tab >= 0) {
				continue;
			}
			const auto* par = find_parser_by_extension(wxFileName(path).GetExt());
			if (par == nullptr) {
				par = get_parser_for_unknown_file(path, config_mgr);
				if (par == nullptr) {
					continue;
				}
			}
			doc_manager->create_document_tab(path, par, false);
		}
		doc_manager->update_ui();
		if (!active_doc.IsEmpty() && wxFileName::FileExists(active_doc)) {
			const int active_tab = doc_manager->find_tab_by_path(active_doc);
			if (active_tab >= 0) {
				frame->get_notebook()->SetSelection(active_tab);
				auto* const text_ctrl = doc_manager->get_active_text_ctrl();
				if (text_ctrl) {
					text_ctrl->SetFocus();
				}
			}
		} else if (doc_manager->has_documents()) {
			auto* const text_ctrl = doc_manager->get_active_text_ctrl();
			if (text_ctrl != nullptr) {
				text_ctrl->SetFocus();
			}
		}
	}
}

void app::open_file(const wxString& filename, main_window* current_frame) {
	if (filename == IPC_COMMAND_ACTIVATE) {
		if (!frames.empty()) {
			frames.back()->Raise();
		}
		return;
	}
	if (!wxFileName::FileExists(filename)) {
		wxMessageBox(wxString::Format(_("File not found: %s"), filename), _("Error"), wxICON_ERROR);
		return;
	}
	for (auto* frame : frames) {
		auto* doc_manager = frame->get_doc_manager();
		if (!doc_manager) {
			continue;
		}
		const int existing_tab = doc_manager->find_tab_by_path(filename);
		if (existing_tab != wxNOT_FOUND) {
			if (auto* nb = frame->get_notebook()) {
				nb->SetSelection(existing_tab);
			}
			auto* const text_ctrl = doc_manager->get_active_text_ctrl();
			if (text_ctrl) {
				text_ctrl->SetFocus();
			}
			frame->Raise();
			frame->RequestUserAttention();
			return;
		}
	}
	if (config_mgr.get(config_manager::open_in_new_window)) {
        if (current_frame && !current_frame->get_doc_manager()->has_documents()) {
            auto* doc_manager = current_frame->get_doc_manager();
            const auto* par = find_parser_by_extension(wxFileName(filename).GetExt());
            if (par == nullptr) {
                par = get_parser_for_unknown_file(filename, config_mgr);
                if (par == nullptr) {
                    return;
                }
            }
            if (!doc_manager->create_document_tab(filename, par)) {
                wxMessageBox(_("Failed to load document."), _("Error"), wxICON_ERROR);
            } else {
                current_frame->Raise();
                current_frame->RequestUserAttention();
            }
        } else {
		    create_new_window(filename);
		}
		return;
	}
	main_window* frame = frames.empty() ? create_new_window() : frames.back();
	auto* doc_manager = frame->get_doc_manager();
	const auto* par = find_parser_by_extension(wxFileName(filename).GetExt());
	if (par == nullptr) {
		par = get_parser_for_unknown_file(filename, config_mgr);
		if (par == nullptr) {
			return;
		}
	}
	if (!doc_manager->create_document_tab(filename, par)) {
		wxMessageBox(_("Failed to load document."), _("Error"), wxICON_ERROR);
	} else {
		frame->Raise();
		frame->RequestUserAttention();
	}
}

main_window* app::create_new_window(const wxString& path) {
	main_window* frame = new main_window();
	frames.push_back(frame);
	if (!path.IsEmpty()) {
		auto* doc_manager = frame->get_doc_manager();
		const auto* par = find_parser_by_extension(wxFileName(path).GetExt());
		if (par == nullptr) {
			par = get_parser_for_unknown_file(path, config_mgr);
		}
		if (par) {
			[[maybe_unused]] bool success = doc_manager->create_document_tab(path, par);
		}
	}
	frame->Show(true);
	return frame;
}

void app::remove_window(main_window* frame) {
	frames.erase(std::remove(frames.begin(), frames.end(), frame), frames.end());
	if (frames.empty()) {
		Exit();
	}
}

void app::update_all_windows_ui() {
	for (auto* frame : frames) {
		frame->update_ui();
	}
}

wxIMPLEMENT_APP(app);
