#include "controls.hpp"
#include <algorithm>
#ifdef __WXMSW__
#include <windows.h>
#endif
#include <wx/event.h>
#include <wx/slider.h>
#include <wx/window.h>

accessible_slider::accessible_slider(wxWindow* parent, wxWindowID id, int value, int min_value, int max_value) : wxSlider(parent, id, value, min_value, max_value) {
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
			new_value = max_value;
			handled = true;
			break;
		case WXK_HOME:
			new_value = min_value;
			handled = true;
			break;
		default:
			event.Skip();
			return;
	}
	if (handled) {
		SetValue(new_value);
		// Generate a scroll event so any bound handlers are notified.
		wxCommandEvent e(wxEVT_SLIDER, GetId());
		e.SetInt(new_value);
		ProcessWindowEvent(e);
	}
}
