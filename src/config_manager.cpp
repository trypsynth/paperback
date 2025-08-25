#include "config_manager.hpp"
#include "constants.hpp"
#include <wx/filename.h>
#include <wx/stdpaths.h>

config_manager::~config_manager() {
	flush();
}

bool config_manager::initialize() {
	const wxString config_path = get_config_path();
	config = std::make_unique<wxFileConfig>(APP_NAME, "", config_path);
	if (!config) return false;
	wxConfigBase::Set(config.get());
	load_defaults();
	return true;
}

void config_manager::flush() {
	if (config) config->Flush();
}

void config_manager::load_defaults() {
	config->SetPath("/");
	if (!config->HasEntry("RestorePreviousDocuments")) set_bool("RestorePreviousDocuments", true);
}

wxString config_manager::get_config_path() const {
	const wxString exe_path = wxStandardPaths::Get().GetExecutablePath();
	const wxString exe_dir = wxFileName(exe_path).GetPath();
	return exe_dir + wxFileName::GetPathSeparator() + APP_NAME + ".ini";
}

wxString config_manager::get_string(const wxString& key, const wxString& default_value) const {
	return config ? config->Read(key, default_value) : default_value;
}

bool config_manager::get_bool(const wxString& key, bool default_value) const {
	return config ? config->ReadBool(key, default_value) : default_value;
}

int config_manager::get_int(const wxString& key, int default_value) const {
	return config ? config->ReadLong(key, default_value) : default_value;
}

void config_manager::set_string(const wxString& key, const wxString& value) {
	if (config) config->Write(key, value);
}

void config_manager::set_bool(const wxString& key, bool value) {
	if (config) config->Write(key, value);
}

void config_manager::set_int(const wxString& key, int value) {
	if (config) config->Write(key, value);
}

void config_manager::add_recent_document(const wxString& path) {
	if (!config) return;
	wxArrayString recent = get_recent_documents();
	int existing_index = recent.Index(path);
	if (existing_index != wxNOT_FOUND) recent.RemoveAt(existing_index);
	recent.Insert(path, 0);
	const int max_recent = 10;
	while (recent.GetCount() > max_recent) recent.RemoveAt(recent.GetCount() - 1);
	config->DeleteGroup("RecentDocuments");
	config->SetPath("/RecentDocuments");
	for (size_t i = 0; i < recent.GetCount(); ++i) config->Write(wxString::Format("File%zu", i), recent[i]);
	config->SetPath("/");
}

wxArrayString config_manager::get_recent_documents() const {
	wxArrayString result;
	if (!config) return result;
	config->SetPath("/RecentDocuments");
	wxString key;
	long index;
	bool cont = config->GetFirstEntry(key, index);
	wxArrayString temp_list;
	while (cont) {
		wxString path = config->Read(key, "");
		if (!path.IsEmpty() && wxFileName::FileExists(path)) temp_list.Add(path);
		cont = config->GetNextEntry(key, index);
	}
	config->SetPath("/");
	for (int i = 0; i < 10 && i < temp_list.GetCount(); ++i) {
		wxString key_name = wxString::Format("File%d", i);
		for (const auto& path : temp_list) {
			if (config->Read("/RecentDocuments/" + key_name, "") == path) {
				result.Add(path);
				break;
			}
		}
	}
	return result;
}

void config_manager::clear_recent_documents() {
	if (config) config->DeleteGroup("RecentDocuments");
}

bool config_manager::get_restore_previous_documents() const {
	if (config) {
		config->SetPath("/");
		return config->ReadBool("RestorePreviousDocuments", true);
	}
	return true;
}

void config_manager::set_restore_previous_documents(bool restore) {
	if (config) {
		config->SetPath("/");
		config->Write("RestorePreviousDocuments", restore);
	}
}
