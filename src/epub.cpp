#include "epub.hpp"

epub_content_handler::epub_content_handler(epub_section& section) : locator{0}, in_paragraph{false}, in_body{false}, section{section}, max_line_length{0} {}

void epub_content_handler::setDocumentLocator(const Poco::XML::Locator* loc) {locator = loc;}

void epub_content_handler::startDocument() {}

void epub_content_handler::endDocument() {
	if (!line.empty()) {
		add_line(line);
		line = "";
	}
}

void epub_content_handler::startElement(const Poco::XML::XMLString& uri, const Poco::XML::XMLString& localName, const Poco::XML::XMLString& qname, const Poco::XML::Attributes& attributes) {
	if (localName == "body") {
		in_body = true;
		ignore_whitespace = true;
	}
	if (localName == "p" || localName == "div") in_paragraph = true;
}

void epub_content_handler::endElement(const Poco::XML::XMLString& uri, const Poco::XML::XMLString& localName, const Poco::XML::XMLString& qname) {
	if (localName == "p" || localName == "h1" || localName == "h2" || localName == "h3" || localName == "h4" || localName == "h5" || localName == "h6" || localName == "br" || localName == "div") {
		add_line(line);
		line = "";
		ignore_whitespace = true;
	}
	in_paragraph = false;
}

void epub_content_handler::characters(const Poco::XML::XMLChar ch[], int start, int length) {
	if (!in_body) return;
	std::string chars(ch + start, length);
	if (ignore_whitespace) {
		ltrim(chars);
		if (chars.empty()) return;
		ignore_whitespace = false;
	}
	line += chars;
}

void epub_content_handler::ignorableWhitespace(const Poco::XML::XMLChar ch[], int start, int length) {
	std::string chars(ch + start, length);
	line += chars;
}

void epub_content_handler::processingInstruction(const Poco::XML::XMLString& target, const Poco::XML::XMLString& data) {}

void epub_content_handler::startPrefixMapping(const Poco::XML::XMLString& prefix, const Poco::XML::XMLString& uri) {}

void epub_content_handler::endPrefixMapping(const Poco::XML::XMLString& prefix) {}

void epub_content_handler::skippedEntity(const Poco::XML::XMLString& name) {
	if (name == "rsquo") line += "’";
	else if (name == "lsquo") line += "‘";
	else if (name == "ldquo") line += "“";
	else if (name == "ldquo") line += "”";
	else if (name == "mdash") line += "—";
	else if (name == "ndash") line += "–";
	else if (name == "nbsp") line += " ";
}

void epub_content_handler::add_line(std::string line) {
	size_t index = 0;
	while (true) {
		index = line.find("\n");
		if (index == std::string::npos) break;
		line.replace(index, 1, " ");
		index++;
	}
	if (max_line_length > 0) {
		while (line.length() > max_line_length) {
			section.lines->push_back(line.substr(0, max_line_length));
			line = line.substr(max_line_length);
		}
	}
	section.lines->push_back(line);
}

void epub_content_handler::set_line_length(int n) {max_line_length = n;}

void epub_content_handler::ltrim(std::string& s) {
	if (s.empty()) return;
	size_t i;
	for (i = 0; i < s.length(); i++) {
		std::string c(s.substr(i, 1));
		if (c != "\n" && c != "\t" && c != " ") break;
	}
	s.erase(0, i);
}

epub::epub() :archive{0} {}

bool epub::load(const std::string& fname) {
	fp.open(fname, std::ios::binary);
	return this->load();
}

bool epub::load() {
	if (fp.fail()) return false;
	archive = new Poco::Zip::ZipArchive(fp);
	Poco::Zip::ZipArchive::FileHeaders::const_iterator header = archive->findHeader("META-INF/container.xml");
	if (header == archive->headerEnd()) return false;
	Poco::Zip::ZipInputStream zis(fp, header->second, true);
	Poco::XML::InputSource src(zis);
	Poco::XML::DOMParser parser;
	Poco::AutoPtr<Poco::XML::Document> doc = parser.parse(&src);
	Poco::XML::NamespaceSupport nsmap;
	nsmap.declarePrefix("container", "urn:oasis:names:tc:opendocument:xmlns:container");
	Poco::XML::Node *node = doc->getNodeByPathNS("container:container/container:rootfiles/container:rootfile", nsmap);
	if (node == nullptr) return false;
	std::string name = static_cast<Poco::XML::Element*>(node)->getAttribute("full-path");
	// Load the OPF file
	opf_path = Poco::Path(name, Poco::Path::PATH_UNIX).makeParent();
	parse_opf(name);
	return true;
}

void epub::parse_opf(std::string filename) {
	Poco::Zip::ZipArchive::FileHeaders::const_iterator header = archive->findHeader(filename);
	if (header == archive->headerEnd()) throw parse_error{"No OPF file found"};
	Poco::Zip::ZipInputStream zis(fp, header->second, true);
	Poco::XML::InputSource src(zis);
	Poco::XML::DOMParser parser;
	Poco::AutoPtr<Poco::XML::Document> doc = parser.parse(&src);
	Poco::XML::NamespaceSupport nsmap;
	nsmap.declarePrefix("opf", "http://www.idpf.org/2007/opf");
	Poco::XML::Node* manifest = doc->getNodeByPathNS("opf:package/opf:manifest", nsmap);
	if (!manifest) throw parse_error{"No manifest"};
	Poco::AutoPtr<Poco::XML::NodeList> children = manifest->childNodes();
	unsigned int len = children->length();
	for (unsigned int i = 0; i < len; i++) {
		Poco::XML::Node* node = children->item(i);
		if (node->nodeType() != Poco::XML::Node::ELEMENT_NODE) continue;
		Poco::XML::Element* e = static_cast<Poco::XML::Element*>(node);
		std::string href = e->getAttribute("href");
		Poco::Path filePath(opf_path);
		filePath.append(href);
		std::string id = e->getAttribute("id");
		manifest_items.insert(std::make_pair(id, filePath.toString(Poco::Path::PATH_UNIX)));
	}
	Poco::XML::Node* spine = doc->getNodeByPathNS("opf:package/opf:spine", nsmap);
	if (!spine) throw parse_error{"No spine"};
	children = spine->childNodes();
	len = children->length();
	for (unsigned int i = 0; i < len; i++) {
		Poco::XML::Node* node = children->item(i);
		if (node->nodeType() != Poco::XML::Node::ELEMENT_NODE) continue;
		Poco::XML::Element* element = static_cast<Poco::XML::Element*>(node);
		std::string idref = element->getAttribute("idref");
		spine_items.push_back(idref);
	}
}

epub::~epub() {
	if (archive) delete archive;
}

int epub::get_num_sections() const {
	return spine_items.size();
}

epub_section* epub::parse_section(unsigned int n, std::vector<std::string>* lines, unsigned int line_length) {
	std::string id = spine_items[n];
	std::map<std::string, std::string>::iterator it = manifest_items.find(id);
	if (it == manifest_items.end()) throw parse_error{("Unknown id: " + id).c_str()};
	std::string href = it->second;
	Poco::Zip::ZipArchive::FileHeaders::const_iterator header = archive->findHeader(href);
	if (header == archive->headerEnd()) throw parse_error{("File not found: " + href).c_str()};
	Poco::Zip::ZipInputStream zis(fp, header->second, true);
	Poco::XML::InputSource src(zis);
	Poco::XML::SAXParser parser = Poco::XML::SAXParser();
	epub_section* section;
	section = new epub_section(lines);
	epub_content_handler* handler = new epub_content_handler(*section);
	handler->set_line_length(line_length);
	parser.setContentHandler(handler);
	parser.parse(&src);
	delete handler;
	return section;
}

std::string epub::get_section_text(epub_section& section) {
	std::string data;
	for (std::vector<std::string>::iterator it = section.lines->begin(); it != section.lines->end(); it++) data += *it + "\n";
	return data;
}

epub_section::epub_section(std::vector<std::string>* v) :lines{v} {}
