/* update_checker.cpp - Update checking functionality implementation.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "update_checker.hpp"
#include "constants.hpp"
#include <Poco/JSON/Object.h>
#include <Poco/JSON/Parser.h>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/stdpaths.h>
#include <wx/utils.h>
#include <wx/webrequest.h>
#include <sstream>

void check_for_updates(bool silent) {
	try {
		wxWebRequestSync request = wxWebSessionSync::GetDefault().CreateRequest("https://api.github.com/repos/trypsynth/paperback/releases/latest");
		if (!request.IsOk()) {
			if (!silent) {
				wxMessageBox(_("Failed to create update request."), _("Error"), wxICON_ERROR);
			}
			return;
		}
		request.SetHeader("Accept", "application/vnd.github.v3+json");
		request.SetHeader("User-Agent", APP_NAME.ToStdString());
		auto result = request.Execute();
		if (!request.GetResponse().IsOk()) {
			if (!silent) {
				wxMessageBox(_("error checking for updates."), _("Error"), wxICON_ERROR);
			}
			return;
		}
		int status = request.GetResponse().GetStatus();
		if (status != 200) {
			if (!silent) {
				wxMessageBox(wxString::Format(_("Failed to check for updates. HTTP status: %d"), status), _("Error"), wxICON_ERROR);
			}
			return;
		}
		wxString response_body = request.GetResponse().AsString();
		Poco::JSON::Parser parser;
		auto json_result = parser.parse(response_body.ToStdString());
		auto json_object = json_result.extract<Poco::JSON::Object::Ptr>();
		const std::string latest_version = json_object->getValue<std::string>("tag_name");
		const std::string release_body = json_object->getValue<std::string>("body");
		if (APP_VERSION.ToStdString() >= latest_version) {
			if (!silent) {
				wxMessageBox(_("No updates available."), _("Info"), wxICON_INFORMATION);
			}
			return;
		}
		wxFileName exe_path(wxStandardPaths::Get().GetExecutablePath());
		const wxString exe_dir = exe_path.GetPath();
		const wxString uninstaller_path = exe_dir + wxFileName::GetPathSeparator() + "unins000.exe";
		const bool is_installer = wxFileName::FileExists(uninstaller_path);
		wxString download_url;
		auto assets = json_object->getArray("assets");
		for (size_t i = 0; i < assets->size(); ++i) {
			auto asset = assets->getObject(i);
			const std::string asset_name = asset->getValue<std::string>("name");
			if (is_installer && asset_name == "paperback_setup.exe") {
				download_url = wxString::FromUTF8(asset->getValue<std::string>("browser_download_url"));
				break;
			}
			if (!is_installer && asset_name == "paperback.zip") {
				download_url = wxString::FromUTF8(asset->getValue<std::string>("browser_download_url"));
				break;
			}
		}
		if (download_url.IsEmpty()) {
			if (!silent) {
				wxMessageBox(_("Update is available but download link could not be found."), _("Error"), wxICON_ERROR);
			}
			return;
		}
		const wxString message = wxString::Format(_("There is an update available.\nYour version: %s\nLatest version: %s\nDescription:\n%s\nDo you want to open the direct download link?"), APP_VERSION, wxString::FromUTF8(latest_version), wxString::FromUTF8(release_body));
		const int result_dialog = wxMessageBox(message, _("Update available"), wxYES_NO | wxICON_INFORMATION);
		if (result_dialog == wxYES) {
			wxLaunchDefaultBrowser(download_url);
		}
	} catch (const std::exception& e) {
		if (!silent) {
			wxMessageBox(wxString::Format(_("Error checking for updates: %s"), wxString::FromUTF8(e.what())), _("Error"), wxICON_ERROR);
		}
	}
}
