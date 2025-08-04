#include "markdown_parser.hpp"
#include "html_to_text.hpp"
#include <maddy/parser.h>
#include <memory>
#include <wx/txtstrm.h>
#include <wx/wfstream.h>

std::unique_ptr<document> markdown_parser::load(const wxString& path) const {
	wxFileInputStream file_stream(path);
	if (!file_stream.IsOk()) return nullptr;
	wxBufferedInputStream bs(file_stream);
	wxTextInputStream text_stream(bs);
	wxString content;
	while (!bs.Eof()) content += text_stream.ReadLine() + "\n";
	std::shared_ptr<maddy::Parser> parser = std::make_shared<maddy::Parser>();
	std::istringstream iss(content.ToStdString());
	std::string html = parser->Parse(iss);
	html_to_text converter;
	if (!converter.convert(html)) return nullptr;
	auto doc = std::make_unique<document>();
	doc->text_content = converter.get_text();
	doc->flags = document_flags::supports_toc;
	return doc;
}
