/* controls.hpp - custom UI control declarations.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include <wx/wx.h>

class numeric_spin_ctrl : public wxTextCtrl {
public:
	numeric_spin_ctrl(wxWindow* parent, wxWindowID id = wxID_ANY, long initial_value = 0, long min_value = 0, long max_value = 100, long step = 1);
	void set_value(long value);
	[[nodiscard]] long get_value() const;
	void set_range(long min_value, long max_value);
	void set_step(long step_value);

private:
	long min;
	long max;
	long step;

	void on_key_down(wxKeyEvent& event);
	void adjust_value(long delta);
};
