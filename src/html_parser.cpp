#include "html_parser.hpp"
#include "html_to_text.hpp"
#include <Poco/FileStream.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/SAX/SAXParser.h>
#include <sstream>

using namespace Poco;
using namespace Poco::XML;

std::unique_ptr<document> html_parser::load(const wxString& path) {
	try {
		FileInputStream stream(path.ToStdString());
		if (!stream.good()) return nullptr;
		InputSource src(stream);
		SAXParser parser;
		parser.setFeature(XMLReader::FEATURE_NAMESPACES, false);
		parser.setFeature(XMLReader::FEATURE_NAMESPACE_PREFIXES, false);
		html_to_text handler;
		parser.setContentHandler(&handler);
		parser.parse(&src);
		std::ostringstream oss;
		for (const auto& line : handler.lines)
			oss << line << "\n";
		auto doc = std::make_unique<document>();
		doc->text_content = oss.str();
		return doc;
	} catch (const Poco::Exception& e) {
		return nullptr;
	}
}
