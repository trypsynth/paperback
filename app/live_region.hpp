#pragma once
#include <wx/window.h>

enum class live_region_mode {
	off = 0,
	polite = 1,
	assertive = 2
};

#ifdef _WIN32
bool set_live_region(wxWindow* window, live_region_mode mode = live_region_mode::polite);
bool notify_live_region_changed(wxWindow* window);
#else
inline bool set_live_region(wxWindow*, live_region_mode = live_region_mode::polite) {
	return false;
}

inline bool notify_live_region_changed(wxWindow*) {
	return false;
}
#endif
