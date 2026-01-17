#pragma once
#include <wx/slider.h>
#include <wx/window.h>

// This is  a wrapper around wxSlider that fixes the backwards arrow key behavior on Windows.
class accessible_slider : public wxSlider {
public:
	accessible_slider(wxWindow* parent, wxWindowID id, int value, int min_value, int max_value);
	void SetValue(int value) override;

private:
	void on_char(wxKeyEvent& event);
};
