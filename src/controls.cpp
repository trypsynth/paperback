/* controls.cpp - custom UI control implementations.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "controls.hpp"

numeric_spin_ctrl::numeric_spin_ctrl(wxWindow* parent, wxWindowID id, long initial_value, long min_value, long max_value, long step) : wxTextCtrl(parent, id, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxTE_PROCESS_ENTER, wxTextValidator(wxFILTER_DIGITS)), min{min_value}, max{max_value}, step{step} {
	set_value(initial_value);
	Bind(wxEVT_KEY_DOWN, &numeric_spin_ctrl::on_key_down, this);
}

void numeric_spin_ctrl::set_value(long value) {
	if (value < min)
		value = min;
	else if (value > max)
		value = max;
	SetValue(wxString::Format("%ld", value));
	SetSelection(-1, -1);
}

long numeric_spin_ctrl::get_value() const {
	wxString input = GetValue().Trim(true).Trim(false);
	long value;
	if (input.ToLong(&value))
		return value;
	return min;
}

void numeric_spin_ctrl::set_range(long min_value, long max_value) {
	min = min_value;
	max = max_value;
}

void numeric_spin_ctrl::set_step(long step_value) {
	step = step_value;
}

void numeric_spin_ctrl::on_key_down(wxKeyEvent& event) {
	int key_code = event.GetKeyCode();
	if (key_code == WXK_UP)
		adjust_value(step);
	else if (key_code == WXK_DOWN)
		adjust_value(-step);
	else
		event.Skip();
}

void numeric_spin_ctrl::adjust_value(long delta) {
	wxString current_value = GetValue().Trim(true).Trim(false);
	long current;
	if (current_value.ToLong(&current)) {
		long new_value = current + delta;
		if (new_value < min)
			new_value = min;
		else if (new_value > max)
			new_value = max;
		SetValue(wxString::Format("%ld", new_value));
		SetSelection(-1, -1);
	}
}
