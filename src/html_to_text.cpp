#include <cctype>
#include "html_to_text.hpp"
#include <Poco/String.h>
#include <sstream>
#include <unordered_map>

using namespace Poco::XML;

html_to_text::html_to_text() :locator{nullptr}, in_paragraph{false}, in_body{false} {}

void html_to_text::setDocumentLocator(const Locator* loc) {
	locator = loc;
}

void html_to_text::startDocument() {}

void html_to_text::endDocument() {
	if (!line.empty()) {
		add_line(line);
		line = "";
	}
}

void html_to_text::startElement(const XMLString& uri, const XMLString& localName, const XMLString& qname, const Attributes& attributes) {
	if (localName == "body") in_body = true;
	if (localName == "p" || localName == "div") in_paragraph = true;
}

void html_to_text::endElement(const XMLString& uri, const XMLString& localName, const XMLString& qname) {
	if (localName == "p" || localName == "h1" || localName == "h2" || localName == "h3" || localName == "h4" || localName == "h5" || localName == "h6" || localName == "br" || localName == "div") {
		add_line(line);
		line = "";
	}
	in_paragraph = false;
}

void html_to_text::characters(const XMLChar ch[], int start, int length) {
	if (!in_body) return;
	std::string chars(ch + start, length);
	chars = Poco::trimLeftInPlace(chars);
	if (chars.empty()) return;
	if (in_paragraph) chars = collapse_whitespace(chars);
	line += chars;
}

void html_to_text::ignorableWhitespace(const XMLChar ch[], int start, int length) {
	std::string chars(ch + start, length);
	line += chars;
}

void html_to_text::processingInstruction(const XMLString& target, const XMLString& data) {}

void html_to_text::startPrefixMapping(const XMLString& prefix, const XMLString& uri) {}

void html_to_text::endPrefixMapping(const XMLString& prefix) {}

void html_to_text::skippedEntity(const XMLString& name) {
	static const std::unordered_map<std::string, std::string> entity_map = {
		{"rsquo", "’"},
		{"lsquo", "‘"},
		{"ldquo", "“"},
		{"rdquo", "”"},
		{"mdash", "—"},
		{"ndash", "–"},
		{"nbsp", " "}
	};
	auto it = entity_map.find(name);
	if (it != entity_map.end())
		line += it->second;
}

inline void html_to_text::add_line(const std::string& line) {
	lines.push_back(line);
}

std::string html_to_text::collapse_whitespace(const std::string& input) {
	std::ostringstream oss;
	bool in_space = false;
	for (char ch : input) {
		if (std::isspace(static_cast<unsigned char>(ch))) {
			if (!in_space) {
				oss << ' ';
				in_space = true;
			}
		} else {
			oss << ch;
			in_space = false;
		}
	}
	return oss.str();
}
