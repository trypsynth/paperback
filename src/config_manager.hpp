#pragma once
#include <memory>
#include <wx/fileconf.h>
#include <wx/string.h>

class config_manager {
public:
	config_manager() = default;
	~config_manager();
	config_manager(const config_manager&) = delete;
	config_manager& operator=(const config_manager&) = delete;
	config_manager(config_manager&&) = default;
	config_manager& operator=(config_manager&&) = default;
	bool initialize();
	void flush();
	wxString get_string(const wxString& key, const wxString& default_value = "") const;
	bool get_bool(const wxString& key, bool default_value = false) const;
	int get_int(const wxString& key, int default_value = 0) const;
	void set_string(const wxString& key, const wxString& value);
	void set_bool(const wxString& key, bool value);
	void set_int(const wxString& key, int value);
	wxFileConfig* get_config() const { return config.get(); }
	bool is_initialized() const { return config != nullptr; }

private:
	std::unique_ptr<wxFileConfig> config;

	wxString get_config_path() const;
	void load_defaults();
};
