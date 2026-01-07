#pragma once
#include "libpaperback/src/bridge.rs.h"
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>
#include <wx/string.h>

using marker_type = MarkerKind;

struct toc_item {
	wxString name;
	wxString ref;
	std::vector<std::unique_ptr<toc_item>> children;
	size_t offset{0};

	toc_item() = default;
	~toc_item() = default;
	toc_item(const toc_item&) = delete;
	toc_item& operator=(const toc_item&) = delete;
	toc_item(toc_item&&) = default;
	toc_item& operator=(toc_item&&) = default;
};

struct session_document {
	rust::Box<DocumentSession> session;
	wxString content;  // Cached for wxTextCtrl
	std::vector<std::unique_ptr<toc_item>> toc_items;  // Cached TOC for dialog
	bool toc_loaded{false};

	session_document() = delete;
	explicit session_document(rust::Box<DocumentSession> sess) : session(std::move(sess)), content(wxString::FromUTF8(session_content(*session).c_str())) {}
	~session_document() = default;
	session_document(const session_document&) = delete;
	session_document& operator=(const session_document&) = delete;
	session_document(session_document&&) = default;
	session_document& operator=(session_document&&) = default;
	[[nodiscard]] wxString get_title() const { return wxString::FromUTF8(session_title(*session).c_str()); }
	[[nodiscard]] wxString get_author() const { return wxString::FromUTF8(session_author(*session).c_str()); }
	[[nodiscard]] const DocumentHandle& get_handle() const { return session_handle(*session); }
	[[nodiscard]] uint32_t get_parser_flags() const { return session_parser_flags(*session); }
	void ensure_toc_loaded();
	[[nodiscard]] size_t find_closest_toc_offset(size_t position) const { return document_find_closest_toc_offset(get_handle(), position); }
};

// Legacy type - kept for compilation compatibility during migration
// Will be fully removed once all navigation methods are updated to use DocumentSession
struct document_data;
using document = document_data;
