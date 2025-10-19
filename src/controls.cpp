/* controls.cpp - custom UI control implementations.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "controls.hpp"
#include "constants.hpp"
#ifdef __WXMSW__
#include <windows.h>
#endif

accessible_slider::accessible_slider(wxWindow* parent, wxWindowID id, int value, int minValue, int maxValue) : wxSlider(parent, id, value, minValue, maxValue) {
	Bind(wxEVT_CHAR, &accessible_slider::on_char, this);
}

void accessible_slider::SetValue(int value) {
	wxSlider::SetValue(value);
#ifdef __WXMSW__
	NotifyWinEvent(EVENT_OBJECT_VALUECHANGE, static_cast<HWND>(GetHandle()), OBJID_CLIENT, CHILDID_SELF);
#endif
}

void accessible_slider::on_char(wxKeyEvent& event) {
	const int key = event.GetKeyCode();
	const int current_value = GetValue();
	const int min_value = GetMin();
	const int max_value = GetMax();
	int new_value{0};
	bool handled = false;
	switch (key) {
		case WXK_UP:
		case WXK_RIGHT:
			new_value = std::min(current_value + GetLineSize(), max_value);
			handled = true;
			break;
		case WXK_DOWN:
		case WXK_LEFT:
			new_value = std::max(current_value - GetLineSize(), min_value);
			handled = true;
			break;
		case WXK_PAGEUP:
			new_value = std::min(current_value + GetPageSize(), max_value);
			handled = true;
			break;
		case WXK_PAGEDOWN:
			new_value = std::max(current_value - GetPageSize(), min_value);
			handled = true;
			break;
		case WXK_END:
			new_value = min_value;
			handled = true;
			break;
		case WXK_HOME:
			new_value = max_value;
			handled = true;
			break;
		default:
			event.Skip();
			return;
	}
	if (handled) {
		SetValue(new_value);
		// Generate a scroll event so any bound handlers are notified.
		wxCommandEvent scroll_event(wxEVT_SLIDER, GetId());
		scroll_event.SetEventObject(this);
		scroll_event.SetInt(new_value);
		GetEventHandler()->ProcessEvent(scroll_event);
	}
}
