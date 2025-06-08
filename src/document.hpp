#pragma once

#include <memory>
#include <vector>
#include <wx/string.h>

struct toc_item {
	wxString name;
	wxString ref;
	std::vector<std::unique_ptr<toc_item>> children;
	int offset;
};

struct document {
	wxString title;
	wxString author;
	wxString text_content;
};
