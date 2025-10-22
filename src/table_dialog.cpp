/* table_dialog.cpp - dialog for displaying tables.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "table_dialog.hpp"

table_dialog::table_dialog(wxWindow* parent, const wxString& title, const wxString& html) :
	wxDialog(parent, wxID_ANY, title, wxDefaultPosition, wxDefaultSize, wxDEFAULT_DIALOG_STYLE | wxRESIZE_BORDER) {
	web_view = wxWebView::New(this, wxID_ANY);
	
	// Set up the message handler for JavaScript-to-C++ communication.
	web_view->AddScriptMessageHandler("wx");
	
	// Bind the event handlers BEFORE loading the page to avoid race conditions.
	Bind(wxEVT_WEBVIEW_LOADED, &table_dialog::on_webview_loaded, this, web_view->GetId());
	Bind(wxEVT_WEBVIEW_SCRIPT_MESSAGE_RECEIVED, &table_dialog::on_script_message, this, web_view->GetId());
	
	// Now, load the page.
	web_view->SetPage(html, "");
	
	// Set up the layout.
	auto* sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(web_view, 1, wxEXPAND | wxALL, 5);

	auto* button_sizer = CreateStdDialogButtonSizer(wxCLOSE);
	sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 5);

	SetSizerAndFit(sizer);
	Centre();
}

void table_dialog::on_webview_loaded(wxWebViewEvent& event) {
	// --- FOCUS LOGIC: Using your initial JavaScript suggestion ---
	// This finds the first element of any kind within the body and focuses it.
	web_view->RunScript(
		"var first = document.body.querySelector('*');"
		"if(first) first.focus();"
	);
	
	// --- ESCAPE KEY LOGIC (Unchanged, as it works) ---
	web_view->RunScript(
		"document.addEventListener('keydown', function(event) {"
		"    if (event.key === 'Escape' || event.keyCode === 27) {"
		"        window.wx.postMessage('close_dialog');"
		"    }"
		"});"
	);
}

void table_dialog::on_script_message(wxWebViewEvent& event) {
	if (event.GetString() == "close_dialog") {
		EndModal(wxID_CANCEL);
	}
}
