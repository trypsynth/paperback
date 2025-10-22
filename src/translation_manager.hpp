/* translation_manager.hpp - Translation management header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include <vector>
#include <wx/string.h>
#include <wx/translation.h>

struct language_info {
	wxString code;
	wxString name;
	wxString native_name;

	language_info(const wxString& c, const wxString& n, const wxString& nn) : code{c}, name{n}, native_name{nn} {}
};

class translation_manager {
public:
	static translation_manager& instance();
	bool initialize();
	bool set_language(const wxString& language_code);
	wxString get_current_language() const;
	std::vector<language_info> get_available_languages() const;
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
