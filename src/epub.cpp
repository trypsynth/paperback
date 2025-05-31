#include "epub.hpp"
#include <memory>
#include <Poco/AutoPtr.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/NamedNodeMap.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/SAX/SAXParser.h>
#include <Poco/Zip/ZipStream.h>
#include <unordered_map>

using namespace Poco::XML;
using namespace Poco::Zip;

epub_content_handler::epub_content_handler(epub_section& section)
	:section{section},
	locator{nullptr},
	in_paragraph{false},
	in_body{false},
	max_line_length{0} {}

void epub_content_handler::setDocumentLocator(const Locator* loc) {
	locator = loc;
}

void epub_content_handler::startDocument() {}

void epub_content_handler::endDocument() {
	if (!line.empty()) {
		add_line(line);
		line = "";
	}
}

void epub_content_handler::startElement(const XMLString& uri, const XMLString& localName, const XMLString& qname, const Attributes& attributes) {
	if (localName == "body") {
		in_body = true;
		ignore_whitespace = true;
	}
	if (localName == "p" || localName == "div")
		in_paragraph = true;
}

void epub_content_handler::endElement(const XMLString& uri, const XMLString& localName, const XMLString& qname) {
	if (localName == "p" || localName == "h1" || localName == "h2" || localName == "h3" || localName == "h4" || localName == "h5" || localName == "h6" || localName == "br" || localName == "div") {
		add_line(line);
		line = "";
		ignore_whitespace = true;
	}
	in_paragraph = false;
}

void epub_content_handler::characters(const XMLChar ch[], int start, int length) {
	if (!in_body) return;
	std::string chars(ch + start, length);
	if (ignore_whitespace) {
		ltrim(chars);
		if (chars.empty()) return;
		ignore_whitespace = false;
	}
	line += chars;
}

void epub_content_handler::ignorableWhitespace(const XMLChar ch[], int start, int length) {
	std::string chars(ch + start, length);
	line += chars;
}

void epub_content_handler::processingInstruction(const XMLString& target, const XMLString& data) {}

void epub_content_handler::startPrefixMapping(const XMLString& prefix, const XMLString& uri) {}

void epub_content_handler::endPrefixMapping(const XMLString& prefix) {}

void epub_content_handler::skippedEntity(const XMLString& name) {
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

void epub_content_handler::add_line(std::string line) {
	if (max_line_length > 0) {
		while (line.length() > max_line_length) {
			section.lines.push_back(line.substr(0, max_line_length));
			line = line.substr(max_line_length);
		}
	}
	section.lines.push_back(line);
}

void epub_content_handler::set_line_length(int n) {
	max_line_length = n;
}

void epub_content_handler::ltrim(std::string& s) {
	s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char c) {
		return !std::isspace(c);
	}));
}

bool epub::load(const std::string& fname) {
	fp.open(fname, std::ios::binary);
	if (fp.fail()) return false;
	archive = std::make_unique<ZipArchive>(fp);
	auto header = archive->findHeader("META-INF/container.xml");
	if (header == archive->headerEnd()) return false;
	ZipInputStream zis(fp, header->second, true);
	InputSource src(zis);
	DOMParser parser;
	auto doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("container", "urn:oasis:names:tc:opendocument:xmlns:container");
	auto*node = doc->getNodeByPathNS("container:container/container:rootfiles/container:rootfile", nsmap);
	if (node == nullptr) return false;
	std::string name = static_cast<Element*>(node)->getAttribute("full-path");
	// Load the OPF file
	opf_path = Poco::Path(name, Poco::Path::PATH_UNIX).makeParent();
	parse_opf(name);
	return true;
}

void epub::parse_opf(const std::string& filename) {
	auto header = archive->findHeader(filename);
	if (header == archive->headerEnd()) throw parse_error{"No OPF file found"};
	ZipInputStream zis(fp, header->second, true);
	InputSource src(zis);
	DOMParser parser;
	auto doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("opf", "http://www.idpf.org/2007/opf");
	nsmap.declarePrefix("dc", "http://purl.org/dc/elements/1.1/");
	auto* metadata = doc->getNodeByPathNS("opf:package/opf:metadata", nsmap);
	if (metadata) {
		auto children = metadata->childNodes();
		unsigned int len = children->length();
		for (unsigned int i = 0; i < len; i++) {
			auto* node = children->item(i);
			if (node->nodeType() != Node::ELEMENT_NODE) continue;
			auto* e = static_cast<Element*>(node);
			std::string localName = e->localName();
			if (localName == "title" && title_.empty())
				title_ = e->innerText();
			else if (localName == "creator" && author_.empty())
				author_ = e->innerText();
		}
	}
	auto* manifest = doc->getNodeByPathNS("opf:package/opf:manifest", nsmap);
	if (!manifest) throw parse_error{"No manifest"};
	auto children = manifest->childNodes();
	unsigned int len = children->length();
	for (unsigned int i = 0; i < len; i++) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* e = static_cast<Element*>(node);
		std::string href = e->getAttribute("href");
		Poco::Path filePath(opf_path);
		filePath.append(href);
		std::string id = e->getAttribute("id");
		manifest_items.insert(std::make_pair(id, filePath.toString(Poco::Path::PATH_UNIX)));
	}
	auto* spine = doc->getNodeByPathNS("opf:package/opf:spine", nsmap);
	if (!spine) throw parse_error{"No spine"};
	children = spine->childNodes();
	len = children->length();
	for (unsigned int i = 0; i < len; i++) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		std::string idref = element->getAttribute("idref");
		spine_items.push_back(idref);
	}
}

int epub::get_num_sections() const {
	return spine_items.size();
}

epub_section epub::parse_section(unsigned int n, std::vector<std::string>* lines, unsigned int line_length) {
	std::string id = spine_items[n];
	auto it = manifest_items.find(id);
	if (it == manifest_items.end()) throw parse_error{("Unknown id: " + id).c_str()};
	std::string href = it->second;
	auto header = archive->findHeader(href);
	if (header == archive->headerEnd()) throw parse_error{("File not found: " + href).c_str()};
	ZipInputStream zis(fp, header->second, true);
	InputSource src(zis);
	auto parser = SAXParser();
	epub_section section;
	auto handler = std::make_unique<epub_content_handler>(section);
	handler->set_line_length(line_length);
	parser.setContentHandler(handler.get());
	parser.parse(&src);
	return section;
}

std::string epub::get_section_text(epub_section& section) {
	std::string data;
	for (auto it = section.lines.begin(); it != section.lines.end(); it++) {
		if (it->empty()) continue;
		data += *it + "\n\n";
	}
	return data;
}
