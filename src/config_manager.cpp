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
