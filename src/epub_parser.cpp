#include "text_parser.hpp"
#include <wx/filename.h>
#include <wx/wfstream.h>

std::unique_ptr<document> text_parser::load(const wxString& path) const {
	wxFFileInputStream file_stream(path);
	if (!file_stream.IsOk()) return nullptr;
	constexpr size_t buffer_size = 1024 * 32;
	wxCharBuffer buffer(buffer_size);
	wxString content;
	while (!file_stream.Eof()) {
		file_stream.Read(buffer.data(), buffer_size);
		size_t bytes_read = file_stream.LastRead();
		if (bytes_read > 0) content.append(wxString::FromUTF8(buffer.data(), bytes_read));
	}
	auto doc = std::make_unique<document>();
	doc->set_title(wxFileName(path).GetName());
	doc->set_author("Unknown");
	doc->set_text_content(content);
	return doc;
}
