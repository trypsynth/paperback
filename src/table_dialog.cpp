/* table_dialog.cpp - dialog for displaying tables.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "table_dialog.hpp"

table_dialog::table_dialog(wxWindow* parent, const wxString& title, const wxString& html) :
	wxDialog(parent, wxID_ANY, title, wxDefaultPosition, wxDefaultSize, wxDEFAULT_DIALOG_STYLE | wxRESIZE_BORDER) {
	web_view = wxWebView::New(this, wxID_ANY);
	web_view->SetPage(html, "");

	auto* sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(web_view, 1, wxEXPAND | wxALL, 5);

	SetSizerAndFit(sizer);
	Centre();

	Bind(wxEVT_CHAR_HOOK, &table_dialog::on_char_hook, this);
}

void table_dialog::on_char_hook(wxKeyEvent& event) {
	if (event.GetKeyCode() == WXK_ESCAPE) {
		EndModal(wxID_CANCEL);
	} else {
		event.Skip();
	}
}