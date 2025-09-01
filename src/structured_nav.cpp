/* structured_nav.cpp - structured navigation code, e.g. navigation by heading in HTML documents or page in PDF's.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "structured_nav.hpp"
#include "constants.hpp"
#include "document.hpp"
#include "document_manager.hpp"
#include "utils.hpp"
#include <wx/menu.h>
#include <wx/string.h>
#include <wx/textctrl.h>

void structured_nav_manager::go_to_previous_heading(document_manager* doc_mgr) {
	navigate_to_heading(doc_mgr, false);
}

void structured_nav_manager::go_to_next_heading(document_manager* doc_mgr) {
	navigate_to_heading(doc_mgr, true);
}

void structured_nav_manager::go_to_previous_heading(document_manager* doc_mgr, int level) {
	navigate_to_heading(doc_mgr, false, level);
}

void structured_nav_manager::go_to_next_heading(document_manager* doc_mgr, int level) {
	navigate_to_heading(doc_mgr, true, level);
}

void structured_nav_manager::go_to_previous_page(document_manager* doc_mgr) {
	document* doc = doc_mgr->get_active_document();
	wxTextCtrl* text_ctrl = doc_mgr->get_active_text_ctrl();
	if (!doc || !text_ctrl) return;
	if (doc->buffer.count_markers_by_type(marker_type::page_break) == 0) {
		speak("No pages.");
		return;
	}
	size_t current_pos = text_ctrl->GetInsertionPoint();
	int prev_index = doc->previous_page_index(current_pos);
	if (prev_index == -1) {
		speak("No previous page.");
		return;
	}
	size_t offset = doc->offset_for_page(prev_index);
	text_ctrl->SetInsertionPoint(offset);
	long line;
	text_ctrl->PositionToXY(offset, 0, &line);
	wxString current_line = text_ctrl->GetLineText(line);
	speak(wxString::Format("Page %d: %s", prev_index + 1, current_line));
}

void structured_nav_manager::go_to_next_page(document_manager* doc_mgr) {
	document* doc = doc_mgr->get_active_document();
	wxTextCtrl* text_ctrl = doc_mgr->get_active_text_ctrl();
	if (!doc || !text_ctrl) return;
	if (doc->buffer.count_markers_by_type(marker_type::page_break) == 0) {
		speak("No pages.");
		return;
	}
	size_t current_pos = text_ctrl->GetInsertionPoint();
	int next_index = doc->next_page_index(current_pos);
	if (next_index == -1) {
		speak("No next page.");
		return;
	}
	size_t offset = doc->offset_for_page(next_index);
	text_ctrl->SetInsertionPoint(offset);
	long line;
	text_ctrl->PositionToXY(offset, 0, &line);
	wxString current_line = text_ctrl->GetLineText(line);
	speak(wxString::Format("Page %d: %s", next_index + 1, current_line));
}

void structured_nav_manager::create_heading_menu(wxMenu* menu) {
	menu->Append(ID_PREVIOUS_HEADING, "Previous heading\tShift+H");
	menu->Append(ID_NEXT_HEADING, "Next heading\tH");
	menu->AppendSeparator();
	for (int level = 1; level <= 6; ++level) {
		menu->Append(ID_PREVIOUS_HEADING_1 + (level - 1) * 2, wxString::Format("Previous heading level %d\tShift+%d", level, level));
		menu->Append(ID_NEXT_HEADING_1 + (level - 1) * 2, wxString::Format("Next heading level %d\t%d", level, level));
	}
}

void structured_nav_manager::navigate_to_heading(document_manager* doc_mgr, bool next, int specific_level) {
	document* doc = doc_mgr->get_active_document();
	wxTextCtrl* text_ctrl = doc_mgr->get_active_text_ctrl();
	if (!doc || !text_ctrl) return;
	if (doc->buffer.get_heading_markers().size() == 0) {
		speak("No headings.");
		return;
	}
	size_t current_pos = text_ctrl->GetInsertionPoint();
	int target_index = -1;
	target_index = next ? doc->next_heading_index(current_pos, specific_level) : doc->previous_heading_index(current_pos, specific_level);
	if (target_index == -1) {
		wxString msg = (specific_level == -1) ? wxString::Format("No %s heading", next ? "next" : "previous") : wxString::Format("No %s heading at level %d", next ? "next" : "previous", specific_level);
		speak(msg);
		return;
	}
	size_t offset = doc->offset_for_heading(target_index);
	text_ctrl->SetInsertionPoint(offset);
	const marker* heading_marker = doc->get_heading_marker(target_index);
	if (heading_marker) speak(wxString::Format("%s Heading level %d", heading_marker->text, heading_marker->level));
}
