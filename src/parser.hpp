#pragma once

#include <vector>
#include <wx/string.h>

class parser {
public:
	virtual ~parser() = default;
	virtual wxString name() const = 0;
	virtual std::vector<wxString> extensions() const = 0;
};
