/* structured_nav.hpp - structured navigation header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include <cstddef>
#include <string>
#include <vector>

struct simple_heading_info {
	size_t offset;
	int level;
	std::string text;
};

class document_manager;
class wxMenu;

class structured_nav_manager {
public:
	static void go_to_previous_heading(document_manager* doc_mgr);
	static void go_to_next_heading(document_manager* doc_mgr);
	static void go_to_previous_heading(document_manager* doc_mgr, int level);
	static void go_to_next_heading(document_manager* doc_mgr, int level);
	static void go_to_previous_page(document_manager* doc_mgr);
	static void go_to_next_page(document_manager* doc_mgr);
	static void create_heading_menu(wxMenu* menu);

private:
	static void navigate_to_heading(document_manager* doc_mgr, bool next, int specific_level = -1);
};
