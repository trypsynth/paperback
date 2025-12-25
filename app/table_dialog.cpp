/* table_dialog.cpp - dialog for displaying tables.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
#include "table_dialog.hpp"
#include <wx/timer.h>
#ifdef __WXMSW__
#include <windows.h>
#endif

table_dialog::table_dialog(wxWindow* parent, const wxString& title, const wxString& html) :
	wxDialog(parent, wxID_ANY, title, wxDefaultPosition, wxDefaultSize, wxDEFAULT_DIALOG_STYLE | wxRESIZE_BORDER) {
	web_view = wxWebView::New(this, wxID_ANY);
	web_view->AddScriptMessageHandler("wx");
	Bind(wxEVT_WEBVIEW_LOADED, &table_dialog::on_webview_loaded, this, web_view->GetId());
	Bind(wxEVT_WEBVIEW_SCRIPT_MESSAGE_RECEIVED, &table_dialog::on_script_message, this, web_view->GetId());
	web_view->SetPage(html, "");
	auto* sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(web_view, 1, wxEXPAND | wxALL, 5);
	auto* button_sizer = CreateStdDialogButtonSizer(wxCLOSE);
	sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 5);
	SetSizerAndFit(sizer);
	Centre();
}

void table_dialog::simulate_click() {
#ifdef __WXMSW__
	wxPoint pos = web_view->GetScreenPosition();
	wxSize size = web_view->GetSize();
	int x = pos.x + size.x / 2;
	int y = pos.y + size.y / 2;
	int screenWidth = GetSystemMetrics(SM_CXSCREEN);
	int screenHeight = GetSystemMetrics(SM_CYSCREEN);
	int absX = (x * 65535) / screenWidth;
	int absY = (y * 65535) / screenHeight;
	INPUT inputMove = {0};
	inputMove.type = INPUT_MOUSE;
	inputMove.mi.dx = absX;
	inputMove.mi.dy = absY;
	inputMove.mi.dwFlags = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
	SendInput(1, &inputMove, sizeof(INPUT));
	INPUT inputDown = {0};
	inputDown.type = INPUT_MOUSE;
	inputDown.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;
	SendInput(1, &inputDown, sizeof(INPUT));
	INPUT inputUp = {0};
	inputUp.type = INPUT_MOUSE;
	inputUp.mi.dwFlags = MOUSEEVENTF_LEFTUP;
	SendInput(1, &inputUp, sizeof(INPUT));
#endif
}

void table_dialog::on_webview_loaded([[maybe_unused]] wxWebViewEvent& event) {
	wxTimer* timer = new wxTimer();
	timer->Bind(wxEVT_TIMER, [this, timer](wxTimerEvent&) {
		simulate_click();
		timer->Stop();
		delete timer;
	});
	timer->StartOnce(100);
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
