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
#include <nlohmann/json.hpp>
#include <wx/app.h>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/stdpaths.h>
#include <wx/utils.h>
#include <wx/webrequest.h>

using nlohmann::json;

namespace {
struct semver {
	int major{0};
	int minor{0};
	int patch{0};
};

std::optional<semver> parse_semver(std::string v) {
	if (!v.empty() && (v[0] == 'v' || v[0] == 'V')) {
		v.erase(0, 1);
	}
	auto cut = v.find_first_of("-+");
	if (cut != std::string::npos) {
		v.erase(cut);
	}
	int M{0}, m{0}, p{0};
	char dot1{0}, dot2{0};
	std::istringstream iss(v);
	if (!(iss >> M)) {
		return std::nullopt;
	}
	if (iss.peek() == '.') {
		iss >> dot1;
		if (!(iss >> m)) {
			m = 0;
		}
	} else {
		m = 0;
	}
	if (iss.peek() == '.') {
		iss >> dot2;
		if (!(iss >> p)) {
			p = 0;
		}
	} else {
		p = 0;
	}
	return semver{M, m, p};
}

bool is_current_latest(const semver& cur, const semver& latest) {
	if (cur.major != latest.major) {
		return cur.major > latest.major;
	}
	if (cur.minor != latest.minor) {
		return cur.minor >= latest.minor && cur.major == latest.major;
	}
	return cur.patch >= latest.patch;
}
} // namespace

void check_for_updates(bool silent) {
	auto* app = wxTheApp;
	if (!app) {
		return;
	}
	wxWebRequest request = wxWebSession::GetDefault().CreateRequest(app, "https://api.github.com/repos/trypsynth/paperback/releases/latest");
	if (!request.IsOk()) {
		if (!silent) {
			app->CallAfter([]() {
				wxMessageBox(_("Failed to create update request."), _("Error"), wxICON_ERROR);
			});
		}
		return;
	}
	request.SetHeader("Accept", "application/vnd.github.v3+json");
	request.SetHeader("User-Agent", std::string(APP_NAME.mb_str()));
	const auto req_id = request.GetId();
	app->Bind(wxEVT_WEBREQUEST_STATE, [silent, req_id](wxWebRequestEvent& evt) {
		if (evt.GetId() != req_id) {
			return;
		}
		const auto state = evt.GetState();
		auto* app_local = wxTheApp;
		if (!app_local) {
			return;
		}
		auto unbind = []() {};
		if (state == wxWebRequest::State_Completed) {
			int status = evt.GetResponse().GetStatus();
			if (status != 200) {
				unbind();
				if (!silent) {
					app_local->CallAfter([status]() {
						wxMessageBox(wxString::Format(_("Failed to check for updates. HTTP status: %d"), status), _("Error"), wxICON_ERROR);
					});
				}
				return;
			}
			const wxString body = evt.GetResponse().AsString();
			auto j = json::parse(body.ToStdString(), nullptr, false);
			if (j.is_discarded()) {
				unbind();
				if (!silent) {
					app_local->CallAfter([]() {
						wxMessageBox(_("Failed to parse update response."), _("Error"), wxICON_ERROR);
					});
				}
				return;
			}
			const std::string latest_tag = j.value("tag_name", "");
			const std::string release_body = j.value("body", "");
			auto cur = parse_semver(APP_VERSION.ToStdString());
			auto latest = parse_semver(latest_tag);
			if (!cur || !latest || is_current_latest(*cur, *latest)) {
				unbind();
				if (!silent) {
					app_local->CallAfter([]() {
						wxMessageBox(_("No updates available."), _("Info"), wxICON_INFORMATION);
					});
				}
				return;
			}
			wxFileName exe_path(wxStandardPaths::Get().GetExecutablePath());
			const wxString exe_dir = exe_path.GetPath();
			const wxString uninstaller_path = exe_dir + wxFileName::GetPathSeparator() + "unins000.exe";
			const bool is_installer = wxFileName::FileExists(uninstaller_path);
			wxString download_url;
			if (j.contains("assets") && j["assets"].is_array()) {
				for (const auto& asset : j["assets"]) {
					std::string asset_name = asset.value("name", "");
					if (is_installer && asset_name == "paperback_setup.exe") {
						download_url = wxString::FromUTF8(asset.value("browser_download_url", ""));
						break;
					}
					if (!is_installer && asset_name == "paperback.zip") {
						download_url = wxString::FromUTF8(asset.value("browser_download_url", ""));
						break;
					}
				}
			}
			if (download_url.IsEmpty()) {
				unbind();
				if (!silent) {
					app_local->CallAfter([]() {
						wxMessageBox(_("Update is available but download link could not be found."), _("Error"), wxICON_ERROR);
					});
				}
				return;
			}
			const wxString message = wxString::Format(_("There is an update available.\nYour version: %s\nLatest version: %s\nDescription:\n%s\nDo you want to open the direct download link?"), APP_VERSION, wxString::FromUTF8(latest_tag), wxString::FromUTF8(release_body));
			app_local->CallAfter([message, download_url]() {
				const int res = wxMessageBox(message, _("Update available"), wxYES_NO | wxICON_INFORMATION);
				if (res == wxYES) {
					wxLaunchDefaultBrowser(download_url);
				}
			});
			unbind();
		} else if (state == wxWebRequest::State_Failed || state == wxWebRequest::State_Cancelled) {
			if (!silent) {
				if (app_local) {
					app_local->CallAfter([]() {
						wxMessageBox(_("Error checking for updates."), _("Error"), wxICON_ERROR);
					});
				}
			}
		}
	}, req_id);
	request.Start();
}
