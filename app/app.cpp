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
#include "libpaperback/src/bridge.rs.h"
#include "parser.hpp"
#include "translation_manager.hpp"
#include "utils.hpp"
#include <cstdint>
#include <string>
#include <thread>
#include <utility>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/stdpaths.h>
#include <wx/utils.h>

namespace {
struct update_result_payload {
	UpdateStatus status{UpdateStatus::InternalError};
	int http_status{0};
	std::string latest_version;
	std::string download_url;
	std::string release_notes;
	std::string error_message;
};

update_result_payload convert_result(const UpdateResult& native_result) {
	update_result_payload payload;
	payload.status = native_result.status;
	payload.http_status = native_result.http_status;
	payload.latest_version = std::string(native_result.latest_version);
	payload.download_url = std::string(native_result.download_url);
	payload.release_notes = std::string(native_result.release_notes);
	payload.error_message = std::string(native_result.error_message);
	return payload;
}

bool is_installer_distribution() {
	wxFileName exe_path(wxStandardPaths::Get().GetExecutablePath());
	const wxString uninstaller_path = exe_path.GetPath() + wxFileName::GetPathSeparator() + "unins000.exe";
	return wxFileName::FileExists(uninstaller_path);
}

void present_update_result(const update_result_payload& payload, bool silent) {
	switch (payload.status) {
		case UpdateStatus::Available: {
			const wxString latest_version = payload.latest_version.empty() ? APP_VERSION : wxString::FromUTF8(payload.latest_version.c_str());
			const wxString release_notes = payload.release_notes.empty() ? _("No release notes were provided.") : wxString::FromUTF8(payload.release_notes.c_str());
			const wxString message = wxString::Format(_("There is an update available.\nYour version: %s\nLatest version: %s\nDescription:\n%s\nDo you want to open the direct download link?"), APP_VERSION, latest_version, release_notes);
			const int res = wxMessageBox(message, _("Update available"), wxYES_NO | wxICON_INFORMATION);
			if (res == wxYES && !payload.download_url.empty()) {
				wxLaunchDefaultBrowser(wxString::FromUTF8(payload.download_url.c_str()));
			}
			break;
		}
		case UpdateStatus::UpToDate:
			if (!silent) {
				wxMessageBox(_("No updates available."), _("Info"), wxICON_INFORMATION);
			}
			break;
		default:
			if (silent) {
				break;
			}
			wxString details;
			if (!payload.error_message.empty()) {
				details = wxString::FromUTF8(payload.error_message.c_str());
			} else {
				details = _("Error checking for updates.");
			}
			if (payload.status == UpdateStatus::HttpError && payload.http_status > 0) {
				details = wxString::Format(_("Failed to check for updates. HTTP status: %d"), payload.http_status);
			}
			wxMessageBox(details, _("Error"), wxICON_ERROR);
			break;
	}
}
} // namespace

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
	if (!initialize_parser_registry()) {
		return false;
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
	frame = new main_window();
	if (config_mgr.get(config_manager::restore_previous_documents)) {
		restore_previous_documents();
	}
	if (argc > 1) {
		parse_command_line();
	}
	frame->Show(true);
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
	const wxString path = file_path.GetFullPath();
	if (!wxFileName::FileExists(path)) {
		wxMessageBox(wxString::Format(_("File not found: %s"), path), _("Error"), wxICON_ERROR);
		return;
	}
	auto* doc_manager = frame->get_doc_manager();
	if (!doc_manager->open_file(path)) {
		wxMessageBox(_("Failed to load document."), _("Error"), wxICON_ERROR);
	}
}

void app::restore_previous_documents() {
	const wxArrayString opened_docs = config_mgr.get_all_opened_documents();
	auto* doc_manager = frame->get_doc_manager();
	const wxString active_doc = config_mgr.get(config_manager::active_document);
	for (const auto& path : opened_docs) {
		if (!wxFileName::FileExists(path)) {
			continue;
		}
		const int existing_tab = doc_manager->find_tab_by_path(path);
		if (existing_tab >= 0) {
			continue;
		}
		const auto* parser = find_parser_by_extension(wxFileName(path).GetExt());
		if (parser == nullptr) {
			parser = get_parser_for_unknown_file(path, config_mgr);
			if (parser == nullptr) {
				continue;
			}
		}
		if (!doc_manager->create_document_tab(path, parser, false, false)) {
			continue;
		}
	}
	doc_manager->update_ui();
	if (!active_doc.IsEmpty() && wxFileName::FileExists(active_doc)) {
		const int active_tab = doc_manager->find_tab_by_path(active_doc);
		if (active_tab >= 0) {
			frame->get_notebook()->SetSelection(active_tab);
			auto* const text_ctrl = doc_manager->get_active_text_ctrl();
			if (text_ctrl != nullptr) {
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

void app::open_file(const wxString& filename) {
	if (filename == IPC_COMMAND_ACTIVATE) {
		if (frame != nullptr) {
			frame->Show(true);
			frame->Iconize(false);
			frame->Raise();
			frame->CallAfter([frm = frame] {
				frm->restore_focus_to_text();
			});
		}
		return;
	}
	if (!wxFileName::FileExists(filename)) {
		wxMessageBox(wxString::Format(_("File not found: %s"), filename), _("Error"), wxICON_ERROR);
		return;
	}
	auto* doc_manager = frame->get_doc_manager();
	if (!doc_manager->open_file(filename)) {
		wxMessageBox(_("Failed to load document."), _("Error"), wxICON_ERROR);
		return;
	}
	frame->Raise();
	frame->RequestUserAttention();
}

void app::check_for_updates(bool silent) {
	const bool installer_build = is_installer_distribution();
	const std::string current_version = std::string(APP_VERSION.ToUTF8());
	std::thread([silent, installer_build, current_version]() {
		update_result_payload payload;
		try {
			UpdateResult result = ::check_for_updates(current_version, installer_build);
			payload = convert_result(result);
		} catch (const std::exception& e) {
			payload.status = UpdateStatus::InternalError;
			payload.error_message = std::string(e.what());
		}
		auto* wx_app = wxTheApp;
		if (wx_app == nullptr || !wx_app->IsMainLoopRunning()) {
			return;
		}
		wx_app->CallAfter([silent, payload = std::move(payload)]() {
			present_update_result(payload, silent);
		});
	}).detach();
}

wxIMPLEMENT_APP(app);
