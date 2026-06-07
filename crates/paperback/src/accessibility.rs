use wxdragon::prelude::WxWidget;

pub fn set_label(widget: &impl WxWidget, label: &str) {
	#[cfg(target_os = "macos")]
	widget.set_accessibility_label(label);
	#[cfg(not(target_os = "macos"))]
	let _ = (widget, label);
}
