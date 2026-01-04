#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include "live_region.hpp"
#include <initguid.h>
#include <oleacc.h>
#include <uiautomation.h>
#include <windows.h>
#include <wx/window.h>

namespace {
IAccPropServices*& get_acc_prop_services() {
	static IAccPropServices* acc_prop_services{nullptr};
	return acc_prop_services;
}

bool init_live_region() {
	if (get_acc_prop_services() != nullptr) {
		return true;
	}
	HRESULT hr = CoInitialize(nullptr);
	if (FAILED(hr) && hr != RPC_E_CHANGED_MODE) {
		return false;
	}
	hr = CoCreateInstance(CLSID_AccPropServices, nullptr, CLSCTX_INPROC, IID_IAccPropServices, reinterpret_cast<void**>(&get_acc_prop_services()));
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
	VARIANT var{};
	var.vt = VT_I4;
	var.lVal = static_cast<int>(mode);
	const HRESULT hr = get_acc_prop_services()->SetHwndProp(hwnd, static_cast<DWORD>(OBJID_CLIENT), static_cast<DWORD>(CHILDID_SELF), LiveSetting_Property_GUID, var);
	return SUCCEEDED(hr);
}

bool notify_live_region_changed(wxWindow* window) {
	if (window == nullptr) {
		return false;
	}
	HWND hwnd = static_cast<HWND>(window->GetHandle());
	if (hwnd == nullptr) {
		return false;
	}
	NotifyWinEvent(EVENT_OBJECT_LIVEREGIONCHANGED, hwnd, OBJID_CLIENT, CHILDID_SELF);
	return true;
}
#endif
