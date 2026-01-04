#pragma once
#include <vector>
#include <wx/string.h>
#include <wx/translation.h>

struct language_info {
	wxString code;
	wxString name;
	wxString native_name;

	language_info(const wxString& c, const wxString& n, const wxString& nn) : code{c}, name{n}, native_name{nn} {
	}
};

class translation_manager {
public:
	static translation_manager& instance();
	bool initialize();
	bool set_language(const wxString& language_code);
	wxString get_current_language() const;
	const std::vector<language_info>& get_available_languages() const;
	wxString get_language_display_name(const wxString& language_code) const;
	bool is_language_available(const wxString& language_code) const;

private:
	translation_manager();
	~translation_manager() = default;
	translation_manager(const translation_manager&) = delete;
	translation_manager& operator=(const translation_manager&) = delete;
	translation_manager(translation_manager&&) = delete;
	translation_manager& operator=(translation_manager&&) = delete;
	void scan_available_languages();
	static wxString get_system_language();

	wxTranslations* translations{nullptr};
	wxString current_language;
	std::vector<language_info> available_languages;
	bool initialized{false};
};
