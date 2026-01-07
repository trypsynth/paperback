#include "parser.hpp"
#include "document_data.hpp"
#include "libpaperback/src/bridge.rs.h"
#include "utils.hpp"
#include <algorithm>
#include <memory>
#include <optional>
#include <set>
#include <sstream>
#include <string>
#include <string_view>
#include <utility>
#include <vector>
#include <wx/msgdlg.h>
#include <wx/string.h>
#include <wx/translation.h>

constexpr std::string_view PASSWORD_REQUIRED_PREFIX = "[password_required]";
parser_exception make_parser_exception(const std::exception& e, const wxString& path) {
	const std::string message = e.what();
	if (message.rfind(PASSWORD_REQUIRED_PREFIX, 0) == 0) {
		std::string_view trimmed{message};
		trimmed.remove_prefix(PASSWORD_REQUIRED_PREFIX.size());
		const wxString localized = trimmed.empty() ? _("Password required or incorrect.") : wxString::FromUTF8(trimmed.data(), trimmed.size());
		return parser_exception(localized, path, error_severity::error, parser_error_code::password_required);
	}
	return parser_exception(wxString::FromUTF8(message.c_str()), path);
}

bool initialize_parser_registry() {
	try {
		// Touch the backend to surface any parser initialization errors early.
		[[maybe_unused]] const auto parser_infos = get_available_parsers();
		return true;
	} catch (const std::exception& e) {
		wxMessageBox(e.what(), _("Error"), wxICON_ERROR);
		return false;
	}
}

bool is_parser_supported(const wxString& extension) {
	if (extension.IsEmpty()) {
		return false;
	}
	const wxString normalized = extension.Lower();
	const std::string ext_utf8 = std::string(normalized.ToUTF8());
	return parser_supports_extension(ext_utf8);
}

wxString get_supported_wildcards() {
	return to_wxstring(parser_supported_wildcards());
}
