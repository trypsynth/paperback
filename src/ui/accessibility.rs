#[cfg(target_os = "linux")]
use std::{process::Command, sync::OnceLock, thread};

use wxdragon::prelude::StaticText;

fn sanitize_message(message: &str) -> String {
	let mut cleaned = String::new();
	for ch in message.chars() {
		if matches!(ch, '\n' | '\r' | '\t') {
			cleaned.push(' ');
		} else if !ch.is_control() {
			cleaned.push(ch);
		}
	}
	let collapsed = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
	collapsed.chars().take(512).collect()
}

#[cfg(target_os = "linux")]
fn has_spd_say() -> bool {
	static HAS_SPD_SAY: OnceLock<bool> = OnceLock::new();
	*HAS_SPD_SAY.get_or_init(|| Command::new("spd-say").arg("--version").output().is_ok())
}

#[cfg(target_os = "linux")]
fn has_gdbus() -> bool {
	static HAS_GDBUS: OnceLock<bool> = OnceLock::new();
	*HAS_GDBUS.get_or_init(|| Command::new("gdbus").arg("--version").output().is_ok())
}

#[cfg(target_os = "linux")]
fn present_with_orca(message: &str) -> bool {
	if !has_gdbus() {
		return false;
	}
	Command::new("gdbus")
		.arg("call")
		.arg("--session")
		.arg("--dest")
		.arg("org.gnome.Orca.Service")
		.arg("--object-path")
		.arg("/org/gnome/Orca/Service")
		.arg("--timeout")
		.arg("1")
		.arg("--method")
		.arg("org.gnome.Orca.Service.PresentMessage")
		.arg(message)
		.output()
		.map(|output| output.status.success())
		.unwrap_or(false)
}

#[cfg(target_os = "linux")]
fn speak_with_spd_say(message: &str) {
	if !has_spd_say() {
		return;
	}
	let _ = Command::new("spd-say").arg("-N").arg("paperback").arg("-P").arg("notification").arg(message).status();
}

pub fn announce(label: StaticText, message: &str) {
	live_region::announce(label, message);
	#[cfg(target_os = "linux")]
	{
		let spoken = sanitize_message(message);
		if spoken.is_empty() {
			return;
		}
		thread::spawn(move || {
			if !present_with_orca(&spoken) {
				speak_with_spd_say(&spoken);
			}
		});
	}
}
