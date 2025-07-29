#include "config_manager.hpp"
#include "constants.hpp"
#include <wx/filename.h>
#include <wx/stdpaths.h>

config_manager::~config_manager() {
	if (conf) conf->Flush();
}

bool config_manager::initialize() {
	wxString confpath = get_config_path();
	conf = std::make_unique<wxFileConfig>(APP_NAME, "", confpath);
	if (!conf) return false;
	wxConfigBase::Set(conf.get());
	load_defaults();
	return true;
}

void config_manager::flush() {
	if (conf) conf->Flush();
}

void config_manager::load_defaults() {
}

wxString config_manager::get_config_path() {
	wxString exePath = wxStandardPaths::Get().GetExecutablePath();
	wxString exeDir = wxFileName(exePath).GetPath();
	return exeDir + wxFileName::GetPathSeparator() + APP_NAME + ".ini";
}

wxString config_manager::get_string(const wxString& key, const wxString& default_value) {
	return conf ? conf->Read(key, default_value) : default_value;
}

bool config_manager::get_bool(const wxString& key, bool default_value) {
	return conf ? conf->ReadBool(key, default_value) : default_value;
}

int config_manager::get_int(const wxString& key, int default_value) {
	return conf ? conf->ReadLong(key, default_value) : default_value;
}

void config_manager::set_string(const wxString& key, const wxString& value) {
	if (conf) conf->Write(key, value);
}

void config_manager::set_bool(const wxString& key, bool value) {
	if (conf) conf->Write(key, value);
}

void config_manager::set_int(const wxString& key, int value) {
	if (conf) conf->Write(key, value);
}
