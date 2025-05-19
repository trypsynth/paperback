#pragma once

#include "document.hpp"
#include <memory>
#include <vector>
#include <wx/string.h>

class parser {
public:
	virtual ~parser() = default;
	virtual wxString name() const = 0;
	virtual const std::vector<wxString>& extensions() const = 0;
	virtual std::unique_ptr<document> load(const wxString& path) const = 0;
};
