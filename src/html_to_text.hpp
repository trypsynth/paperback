#pragma once

#include <Poco/SAX/ContentHandler.h>
#include <Poco/SAX/Locator.h>
#include <string>
#include <vector>

class html_to_text : public Poco::XML::ContentHandler {
public:
	html_to_text();
	std::vector<std::string> lines() const {return lines_;}

protected:
	void setDocumentLocator(const Poco::XML::Locator* loc) override;
	void startDocument() override;
	void endDocument() override;
	void startElement(const Poco::XML::XMLString& uri, const Poco::XML::XMLString& localName, const Poco::XML::XMLString& qname, const Poco::XML::Attributes& attributes) override;
	void endElement(const Poco::XML::XMLString& uri, const Poco::XML::XMLString& localName, const Poco::XML::XMLString& qname) override;
	void characters(const Poco::XML::XMLChar ch[], int start, int length) override;
	void ignorableWhitespace(const Poco::XML::XMLChar ch[], int start, int length) override;
	void processingInstruction(const Poco::XML::XMLString& target, const Poco::XML::XMLString& data) override;
	void startPrefixMapping(const Poco::XML::XMLString& prefix, const Poco::XML::XMLString& uri) override;
	void endPrefixMapping(const Poco::XML::XMLString& prefix) override;
	void skippedEntity(const Poco::XML::XMLString& name) override;

private:
	const Poco::XML::Locator* locator;
	std::vector<std::string> lines_;
	std::string line;
	bool in_paragraph;
	bool in_body;
	bool ignore_whitespace;
	void add_line(const std::string& line);
};
