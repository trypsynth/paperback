#include "html_parser.hpp"
#include "html_to_text.hpp"
#include <wx/txtstrm.h>
#include <wx/wfstream.h>

std::unique_ptr<document> html_parser::load(const wxString& path) const {
	wxFileInputStream file_stream(path);
	if (!file_stream.IsOk()) return nullptr;
	wxBufferedInputStream bs(file_stream);
	wxTextInputStream text_stream(bs);
	wxString content, line;
	while (!bs.Eof()) content += text_stream.ReadLine() + "\n";
	html_to_text converter;
	if (!converter.convert(content.ToStdString())) return nullptr;
	auto doc = std::make_unique<document>();
	doc->text_content = converter.get_text();
	return doc;
}
