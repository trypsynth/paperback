#include "epub.hpp"
#include "html_to_text.hpp"
#include <memory>
#include <Poco/AutoPtr.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/NamedNodeMap.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/SAX/SAXParser.h>
#include <Poco/String.h>
#include <Poco/Zip/ZipStream.h>

using namespace Poco;
using namespace Poco::XML;
using namespace Poco::Zip;

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
	auto name = static_cast<Element*>(node)->getAttribute("full-path");
	// Load the OPF file
	opf_path = Path(name, Path::PATH_UNIX).makeParent();
	parse_opf(name);
	return true;
}

void epub::parse_opf(const std::string& filename) {
	auto header = archive->findHeader(filename);
	if (header == archive->headerEnd()) throw parse_error("No OPF file found");
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
		size_t len = children->length();
		for (size_t i = 0; i < len; i++) {
			auto* node = children->item(i);
			if (node->nodeType() != Node::ELEMENT_NODE) continue;
			auto* e = static_cast<Element*>(node);
			auto localName = e->localName();
			if (localName == "title" && title_.empty())
				title_ = e->innerText();
			else if (localName == "creator" && author_.empty())
				author_ = e->innerText();
		}
	}
	auto* manifest = doc->getNodeByPathNS("opf:package/opf:manifest", nsmap);
	if (!manifest) throw parse_error("No manifest");
	auto children = manifest->childNodes();
	size_t len = children->length();
	for (size_t i = 0; i < len; i++) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* e = static_cast<Element*>(node);
		const auto href = e->getAttribute("href");
		Path filePath(opf_path);
		filePath.append(href);
		const auto id = e->getAttribute("id");
		manifest_items.insert(std::make_pair(id, filePath.toString(Path::PATH_UNIX)));
	}
	auto* spine = doc->getNodeByPathNS("opf:package/opf:spine", nsmap);
	if (!spine) throw parse_error("No spine");
	children = spine->childNodes();
	len = children->length();
	for (size_t i = 0; i < len; i++) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		const auto idref = element->getAttribute("idref");
		spine_items.push_back(idref);
	}
}

int epub::get_num_sections() const {
	return spine_items.size();
}

epub_section epub::parse_section(size_t n) {
	const auto id = spine_items[n];
	auto it = manifest_items.find(id);
	if (it == manifest_items.end()) throw parse_error("Unknown id: " + id);
	const auto href = it->second;
	auto header = archive->findHeader(href);
	if (header == archive->headerEnd()) throw parse_error("File not found: " + href);
	ZipInputStream zis(fp, header->second, true);
	InputSource src(zis);
	auto parser = SAXParser();
	epub_section section;
	auto handler = std::make_unique<html_to_text>();
	parser.setContentHandler(handler.get());
	parser.parse(&src);
	section.lines = handler.get()->lines();
	return section;
}

std::string epub::get_section_text(epub_section& section) {
	std::string data;
	for (auto& line : section.lines) {
		line = trimInPlace(line);
		if (line.empty()) continue;
		data += line + "\n";
	}
	return data;
}
