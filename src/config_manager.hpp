#pragma once
#include <memory>
#include <wx/fileconf.h>
#include <wx/wx.h>

class config_manager {
public:
	config_manager() = default;
	~config_manager();
	bool initialize();
	void flush();
	void load_defaults();
	wxFileConfig* get_config() const { return conf.get(); }
	wxString get_string(const wxString& key, const wxString& default_value = "");
	bool get_bool(const wxString& key, bool default_value = false);
	int get_int(const wxString& key, int default_value = 0);
	void set_string(const wxString& key, const wxString& value);
	void set_bool(const wxString& key, bool value);
	void set_int(const wxString& key, int value);

private:
	std::unique_ptr<wxFileConfig> conf;
	wxString get_config_path();
};
