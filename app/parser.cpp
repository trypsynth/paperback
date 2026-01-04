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

// Legacy function - no longer used, documents now created via session_new()
// Kept commented for reference during migration
/*
std::unique_ptr<document> load_document_from_rust(const wxString& path, const std::optional<std::string>& password, const wxString& forced_extension) {
	try {
		const std::string file_path = path.ToUTF8().data();
		const std::string password_value = password.value_or(std::string());
		const std::string extension = forced_extension.ToUTF8().data();
		auto handle = parse_document_handle(rust::Str(file_path), rust::Str(password_value), rust::Str(extension));
		auto doc = std::make_unique<document>();
		doc->handle = std::move(handle);
		const auto& handle_ref = **doc->handle;
		doc->title = to_wxstring(document_title(handle_ref));
		doc->author = to_wxstring(document_author(handle_ref));
		doc->content = to_wxstring(document_content(handle_ref));
		doc->stats = document_stats(handle_ref);
		populate_id_positions(*doc, document_id_positions(handle_ref));
		populate_spine_items(*doc, document_spine_items(handle_ref));
		populate_manifest_items(*doc, document_manifest_items(handle_ref));
		return doc;
	} catch (const std::exception& e) {
		throw make_parser_exception(e, path);
	}
}
*/
