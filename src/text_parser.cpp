#include "text_parser.hpp"
#include <wx/filename.h>
#include <wx/txtstrm.h>
#include <wx/wfstream.h>

std::unique_ptr<document> text_parser::load(const wxString& path) const {
	wxFileInputStream file_stream(path);
	if (!file_stream.IsOk()) return nullptr;
	wxBufferedInputStream bs(file_stream);
	wxTextInputStream text_stream(bs);
	wxString content;
	while (!bs.Eof())
		content += text_stream.ReadLine() + "\n";
	auto doc = std::make_unique<document>();
	doc->title = wxFileName(path).GetName();
	doc->author = "Unknown";
	doc->text_content = content;
	doc->flags = document_flags::none;
	return doc;
}
