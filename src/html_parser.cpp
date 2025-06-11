#include "html_parser.hpp"
#include "html_to_text.hpp"
#include <wx/txtstrm.h>
#include <wx/wfstream.h>

std::unique_ptr<document> html_parser::load(const wxString& path) {
	wxFileInputStream file_stream(path);
	if (!file_stream.IsOk()) return nullptr;
	wxBufferedInputStream bs(file_stream);
	wxTextInputStream text_stream(bs);
	wxString content;
	while (!file_stream.Eof()) content += text_stream.ReadLine();
	html_to_text converter;
	std::string html_content = content.ToStdString();
	if (!converter.convert(html_content)) return nullptr;
	auto doc = std::make_unique<document>();
	doc->text_content = converter.get_text();
	return doc;
}
