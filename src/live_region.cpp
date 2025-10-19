/* live_region.cpp - live region implementation for screen reader speech output.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include "live_region.hpp"
#include <oleacc.h>
#include <uiautomation.h>
#include <windows.h>

namespace {
static IAccPropServices* const acc_prop_services{nullptr};

bool init_live_region() {
	if (acc_prop_services != nullptr) {
		return true;
	}
	HRESULT hr = CoInitialize(nullptr);
	if (FAILED(hr) && hr != RPC_E_CHANGED_MODE) {
		return false;
	}
	hr = CoCreateInstance(CLSID_AccPropServices, nullptr, CLSCTX_INPROC, IID_IAccPropServices, reinterpret_cast<void**>(&acc_prop_services));
	return SUCCEEDED(hr);
}
} // namespace

bool set_live_region(wxWindow* window, live_region_mode mode) {
	if (window == nullptr) {
		return false;
	}
	if (!init_live_region()) {
		return false;
	}
	HWND hwnd = static_cast<HWND>(window->GetHandle());
	if (hwnd == nullptr) {
		return false;
	}
	VARIANT var;
	var.vt = VT_I4;
	var.lVal = static_cast<int>(mode);
	const HRESULT hr = acc_prop_services->SetHwndProp(hwnd, OBJID_CLIENT, CHILDID_SELF, LiveSetting_Property_GUID, var);
	return SUCCEEDED(hr);
}

bool notify_live_region_changed(wxWindow* window) {
	if (window == nullptr) {
		return false;
	}
	HWND const hwnd = static_cast<HWND>(window->GetHandle());
	if (hwnd == nullptr) {
		return false;
	}
	NotifyWinEvent(EVENT_OBJECT_LIVEREGIONCHANGED, hwnd, OBJID_CLIENT, CHILDID_SELF);
	return true;
}
#endif
