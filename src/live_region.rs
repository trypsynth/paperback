use wxdragon::prelude::{StaticText, WxWidget};

#[cfg(target_os = "windows")]
mod windows_impl {
	use std::{cell::RefCell, mem::ManuallyDrop};

	use windows::Win32::{
		Foundation::{HWND, RPC_E_CHANGED_MODE},
		System::{
			Com::{CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx},
			Variant::{VARIANT, VARIANT_0, VARIANT_0_0, VARIANT_0_0_0, VT_I4},
		},
		UI::{
			Accessibility::{CLSID_AccPropServices, IAccPropServices, LiveSetting_Property_GUID, NotifyWinEvent},
			WindowsAndMessaging::{CHILDID_SELF, EVENT_OBJECT_LIVEREGIONCHANGED, OBJID_CLIENT},
		},
	};
	use wxdragon::prelude::WxWidget;

	thread_local! {
		static ACC_PROP_SERVICES: RefCell<Option<IAccPropServices>> = const { RefCell::new(None) };
	}

	const LIVE_REGION_POLITE: u32 = 1;

	/// Note: this function initializes COM in STA mode on first use.
	pub fn set_live_region(window: &impl WxWidget) -> bool {
		let Some(acc_prop) = acc_prop_services() else {
			return false;
		};
		let Some(hwnd) = hwnd_from_widget(window) else {
			return false;
		};
		let variant = VARIANT {
			Anonymous: VARIANT_0 {
				Anonymous: ManuallyDrop::new(VARIANT_0_0 {
					vt: VT_I4,
					wReserved1: 0,
					wReserved2: 0,
					wReserved3: 0,
					Anonymous: VARIANT_0_0_0 { lVal: LIVE_REGION_POLITE.cast_signed() },
				}),
			},
		};
		unsafe {
			acc_prop
				.SetHwndProp(hwnd, OBJID_CLIENT.0.cast_unsigned(), CHILDID_SELF, LiveSetting_Property_GUID, &variant)
				.is_ok()
		}
	}

	pub fn notify_live_region_changed(window: &impl WxWidget) -> bool {
		let Some(hwnd) = hwnd_from_widget(window) else {
			return false;
		};
		unsafe {
			NotifyWinEvent(EVENT_OBJECT_LIVEREGIONCHANGED, hwnd, OBJID_CLIENT.0, CHILDID_SELF.cast_signed());
		}
		true
	}

	fn acc_prop_services() -> Option<IAccPropServices> {
		ACC_PROP_SERVICES.with(|cell| {
			if cell.borrow().is_none()
				&& let Some(service) = init_acc_prop_services()
			{
				*cell.borrow_mut() = Some(service);
			}
			cell.borrow().clone()
		})
	}

	fn init_acc_prop_services() -> Option<IAccPropServices> {
		unsafe {
			let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
			if hr.is_err() && hr != RPC_E_CHANGED_MODE {
				return None;
			}
			CoCreateInstance(&CLSID_AccPropServices, None, CLSCTX_INPROC_SERVER).ok()
		}
	}

	fn hwnd_from_widget(widget: &impl WxWidget) -> Option<HWND> {
		let handle = widget.get_handle();
		if handle.is_null() {
			return None;
		}
		Some(HWND(handle))
	}
}

#[cfg(not(target_os = "windows"))]
mod windows_impl {
	use wxdragon::prelude::wxWidget;

	pub fn set_live_region(_window: &impl WxWidget) -> bool {
		false
	}

	pub fn notify_live_region_changed(_window: &impl WxWidget) -> bool {
		false
	}
}

pub fn set_live_region(window: &impl WxWidget) -> bool {
	windows_impl::set_live_region(window)
}

pub fn announce(label: StaticText, message: &str) {
	label.set_label(message);
	let _ = windows_impl::notify_live_region_changed(&label);
}
