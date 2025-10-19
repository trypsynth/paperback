/* translation_manager.cpp - Translation management implementation.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "translation_manager.hpp"
#include <unordered_map>
#include <wx/dir.h>
#include <wx/filename.h>
#include <wx/stdpaths.h>
#include <wx/uilocale.h>

translation_manager& translation_manager::instance() {
	static translation_manager inst;
	return inst;
}

translation_manager::translation_manager() {
	available_languages.emplace_back("en", "English", "English");
}

bool translation_manager::initialize() {
	if (initialized) {
		return true;
	}
	translations = new wxTranslations();
	wxTranslations::Set(translations);
	wxString exe_path = wxStandardPaths::Get().GetExecutablePath();
	wxFileName exe_file(exe_path);
	wxString langs_dir = exe_file.GetPath() + wxFileName::GetPathSeparator() + "langs";
	wxFileTranslationsLoader::AddCatalogLookupPathPrefix(langs_dir);
	translations->AddStdCatalog();
	translations->AddCatalog("paperback");
	scan_available_languages();
	wxString sys_lang = get_system_language();
	if (is_language_available(sys_lang)) {
		current_language = sys_lang;
	} else {
		current_language = "en";
	}
	if (current_language != "en") {
		translations->SetLanguage(current_language);
	}
	initialized = true;
	return true;
}

bool translation_manager::set_language(const wxString& language_code) {
	if (!initialized) {
		return false;
	}
	if (!is_language_available(language_code)) {
		return false;
	}
	current_language = language_code;
	translations = new wxTranslations();
	// Calling Set() deletes the previous object automatically. Remove this and we crash. Yay C++!
	wxTranslations::Set(translations);
	wxString exe_path = wxStandardPaths::Get().GetExecutablePath();
	wxFileName exe_file(exe_path);
	wxString langs_dir = exe_file.GetPath() + wxFileName::GetPathSeparator() + "langs";
	wxFileTranslationsLoader::AddCatalogLookupPathPrefix(langs_dir);
	translations->SetLanguage(language_code);
	translations->AddStdCatalog();
	if (language_code != "en") {
		translations->AddCatalog("paperback");
	}
	return true;
}

wxString translation_manager::get_current_language() const {
	return current_language;
}

std::vector<language_info> translation_manager::get_available_languages() const {
	return available_languages;
}

wxString translation_manager::get_language_display_name(const wxString& language_code) const {
	for (const auto& lang : available_languages) {
		if (lang.code == language_code) {
			return lang.native_name;
		}
	}
	return language_code;
}

bool translation_manager::is_language_available(const wxString& language_code) const {
	for (const auto& lang : available_languages) {
		if (lang.code == language_code) {
			return true;
		}
	}
	return false;
}

void translation_manager::scan_available_languages() {
	wxString exe_path = wxStandardPaths::Get().GetExecutablePath();
	wxFileName exe_file(exe_path);
	wxString langs_dir = exe_file.GetPath() + wxFileName::GetPathSeparator() + "langs";
	if (!wxDir::Exists(langs_dir)) {
		return;
	}
	wxDir dir(langs_dir);
	if (!dir.IsOpened()) {
		return;
	}
	wxString dirname;
	bool cont = dir.GetFirst(&dirname, "", wxDIR_DIRS);
	static const std::unordered_map<std::string, std::pair<std::string, std::string>> language_names = {
		{"af", {"Afrikaans", "Afrikaans"}},
		{"am", {"Amharic", "አማርኛ"}},
		{"ar", {"Arabic", "العربية"}},
		{"az", {"Azerbaijani", "Azərbaycan"}},
		{"be", {"Belarusian", "Беларуская"}},
		{"bg", {"Bulgarian", "Български"}},
		{"bn", {"Bengali", "বাংলা"}},
		{"bs", {"Bosnian", "Bosanski"}},
		{"ca", {"Catalan", "Català"}},
		{"cs", {"Czech", "Čeština"}},
		{"cy", {"Welsh", "Cymraeg"}},
		{"da", {"Danish", "Dansk"}},
		{"de", {"German", "Deutsch"}},
		{"el", {"Greek", "Ελληνικά"}},
		{"en", {"English", "English"}},
		{"eo", {"Esperanto", "Esperanto"}},
		{"es", {"Spanish", "Español"}},
		{"et", {"Estonian", "Eesti"}},
		{"eu", {"Basque", "Euskara"}},
		{"fa", {"Persian", "فارسی"}},
		{"fi", {"Finnish", "Suomi"}},
		{"fil", {"Filipino", "Filipino"}},
		{"fr", {"French", "Français"}},
		{"ga", {"Irish", "Gaeilge"}},
		{"gl", {"Galician", "Galego"}},
		{"gu", {"Gujarati", "ગુજરાતી"}},
		{"he", {"Hebrew", "עברית"}},
		{"hi", {"Hindi", "हिन्दी"}},
		{"hr", {"Croatian", "Hrvatski"}},
		{"hu", {"Hungarian", "Magyar"}},
		{"hy", {"Armenian", "Հայերեն"}},
		{"id", {"Indonesian", "Bahasa Indonesia"}},
		{"is", {"Icelandic", "Íslenska"}},
		{"it", {"Italian", "Italiano"}},
		{"ja", {"Japanese", "日本語"}},
		{"ka", {"Georgian", "ქართული"}},
		{"kk", {"Kazakh", "Қазақ"}},
		{"km", {"Khmer", "ខ្មែរ"}},
		{"kn", {"Kannada", "ಕನ್ನಡ"}},
		{"ko", {"Korean", "한국어"}},
		{"ky", {"Kyrgyz", "Кыргызча"}},
		{"lo", {"Lao", "ລາວ"}},
		{"lt", {"Lithuanian", "Lietuvių"}},
		{"lv", {"Latvian", "Latviešu"}},
		{"mk", {"Macedonian", "Македонски"}},
		{"ml", {"Malayalam", "മലയാളം"}},
		{"mn", {"Mongolian", "Монгол"}},
		{"mr", {"Marathi", "मराठी"}},
		{"ms", {"Malay", "Bahasa Melayu"}},
		{"mt", {"Maltese", "Malti"}},
		{"nb", {"Norwegian Bokmål", "Bokmål"}},
		{"ne", {"Nepali", "नेपाली"}},
		{"nl", {"Dutch", "Nederlands"}},
		{"no", {"Norwegian", "Norsk"}},
		{"pa", {"Punjabi", "ਪੰਜਾਬੀ"}},
		{"pl", {"Polish", "Polski"}},
		{"ps", {"Pashto", "پښتو"}},
		{"pt", {"Portuguese", "Português"}},
		{"ro", {"Romanian", "Română"}},
		{"ru", {"Russian", "Русский"}},
		{"si", {"Sinhala", "සිංහල"}},
		{"sk", {"Slovak", "Slovenčina"}},
		{"sl", {"Slovenian", "Slovenščina"}},
		{"sq", {"Albanian", "Shqip"}},
		{"sr", {"Serbian", "Српски"}},
		{"sv", {"Swedish", "Svenska"}},
		{"sw", {"Swahili", "Kiswahili"}},
		{"ta", {"Tamil", "தமிழ்"}},
		{"te", {"Telugu", "తెలుగు"}},
		{"th", {"Thai", "ไทย"}},
		{"tl", {"Tagalog", "Tagalog"}},
		{"tr", {"Turkish", "Türkçe"}},
		{"uk", {"Ukrainian", "Українська"}},
		{"ur", {"Urdu", "اردو"}},
		{"uz", {"Uzbek", "Oʻzbek"}},
		{"vi", {"Vietnamese", "Tiếng Việt"}},
		{"xh", {"Xhosa", "isiXhosa"}},
		{"yi", {"Yiddish", "ייִדיש"}},
		{"zh_CN", {"Chinese (Simplified)", "简体中文"}},
		{"zh_TW", {"Chinese (Traditional)", "繁體中文"}},
		{"zu", {"Zulu", "isiZulu"}},
	};
	while (cont) {
		wxString catalog_path = langs_dir + wxFileName::GetPathSeparator() + dirname + wxFileName::GetPathSeparator() + "LC_MESSAGES" + wxFileName::GetPathSeparator() + "paperback.mo";
		if (wxFileExists(catalog_path)) {
			wxString display_name = dirname;
			wxString native_name = dirname;
			auto it = language_names.find(dirname.ToStdString());
			if (it != language_names.end()) {
				display_name = wxString::FromUTF8(it->second.first);
				native_name = wxString::FromUTF8(it->second.second);
			}
			available_languages.emplace_back(dirname, display_name, native_name);
		}
		cont = dir.GetNext(&dirname);
	}
}

wxString translation_manager::get_system_language() const {
	wxUILocale locale = wxUILocale::GetCurrent();
	wxString lang_tag = locale.GetName();
	wxString lang_code = lang_tag.BeforeFirst('_').BeforeFirst('-');
	return lang_code;
}
