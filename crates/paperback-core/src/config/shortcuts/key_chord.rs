//! [`KeyChord`]: a physical key combination, with parsing/formatting for the config file's
//! `"Ctrl+Shift+O"`-style strings and matching against raw platform key codes.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyChord {
	pub ctrl: bool,
	#[serde(default)]
	pub raw_ctrl: bool,
	pub alt: bool,
	pub shift: bool,
	pub key: String,
}

impl KeyChord {
	pub fn new(ctrl: bool, alt: bool, shift: bool, key: impl Into<String>) -> Self {
		let key_str = key.into();
		let normalized = Self::normalize_key_name(&key_str);
		Self { ctrl, raw_ctrl: false, alt, shift, key: normalized }
	}

	pub fn new_raw_ctrl(raw_ctrl: bool, alt: bool, shift: bool, key: impl Into<String>) -> Self {
		let key_str = key.into();
		let normalized = Self::normalize_key_name(&key_str);
		Self { ctrl: false, raw_ctrl, alt, shift, key: normalized }
	}

	pub fn normalize_key_name(key: &str) -> String {
		let trimmed = key.trim();
		if trimmed.eq_ignore_ascii_case("return") || trimmed.eq_ignore_ascii_case("enter") {
			"Enter".to_string()
		} else if trimmed.eq_ignore_ascii_case("space") {
			"Space".to_string()
		} else if trimmed.eq_ignore_ascii_case("tab") {
			"Tab".to_string()
		} else if trimmed.eq_ignore_ascii_case("backspace") || trimmed.eq_ignore_ascii_case("back") {
			"Backspace".to_string()
		} else if trimmed.eq_ignore_ascii_case("delete") || trimmed.eq_ignore_ascii_case("del") {
			"Delete".to_string()
		} else if trimmed.eq_ignore_ascii_case("escape") || trimmed.eq_ignore_ascii_case("esc") {
			"Escape".to_string()
		} else if trimmed.eq_ignore_ascii_case("home") {
			"Home".to_string()
		} else if trimmed.eq_ignore_ascii_case("end") {
			"End".to_string()
		} else if trimmed.eq_ignore_ascii_case("pageup")
			|| trimmed.eq_ignore_ascii_case("page up")
			|| trimmed.eq_ignore_ascii_case("pgup")
		{
			"PageUp".to_string()
		} else if trimmed.eq_ignore_ascii_case("pagedown")
			|| trimmed.eq_ignore_ascii_case("page down")
			|| trimmed.eq_ignore_ascii_case("pgdn")
		{
			"PageDown".to_string()
		} else if trimmed.eq_ignore_ascii_case("left") || trimmed.eq_ignore_ascii_case("left arrow") {
			"Left".to_string()
		} else if trimmed.eq_ignore_ascii_case("right") || trimmed.eq_ignore_ascii_case("right arrow") {
			"Right".to_string()
		} else if trimmed.eq_ignore_ascii_case("up") || trimmed.eq_ignore_ascii_case("up arrow") {
			"Up".to_string()
		} else if trimmed.eq_ignore_ascii_case("down") || trimmed.eq_ignore_ascii_case("down arrow") {
			"Down".to_string()
		} else if trimmed.len() >= 2
			&& trimmed.starts_with(['F', 'f'])
			&& trimmed[1..].chars().all(|c| c.is_ascii_digit())
		{
			format!("F{}", &trimmed[1..])
		} else if trimmed.len() == 1 {
			let ch = trimmed.chars().next().unwrap();
			if ch.is_ascii_alphabetic() { ch.to_ascii_uppercase().to_string() } else { trimmed.to_string() }
		} else {
			trimmed.to_string()
		}
	}

	pub fn to_shortcut_string(&self) -> String {
		let mut parts = Vec::new();
		if self.raw_ctrl {
			parts.push("RawCtrl");
		}
		if self.ctrl {
			parts.push("Ctrl");
		}
		if self.alt {
			parts.push("Alt");
		}
		if self.shift {
			parts.push("Shift");
		}
		parts.push(&self.key);
		parts.join("+")
	}

	pub fn parse(input: &str) -> Option<Self> {
		let trimmed = input.trim();
		if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("none") {
			return None;
		}
		let mut ctrl = false;
		let mut raw_ctrl = false;
		let mut alt = false;
		let mut shift = false;
		let mut remaining = trimmed;

		while let Some(plus_idx) = remaining.find('+') {
			let prefix = &remaining[..plus_idx];
			if prefix.eq_ignore_ascii_case("rawctrl") {
				raw_ctrl = true;
				remaining = &remaining[plus_idx + 1..];
			} else if prefix.eq_ignore_ascii_case("ctrl") || prefix.eq_ignore_ascii_case("control") {
				ctrl = true;
				remaining = &remaining[plus_idx + 1..];
			} else if prefix.eq_ignore_ascii_case("alt") {
				alt = true;
				remaining = &remaining[plus_idx + 1..];
			} else if prefix.eq_ignore_ascii_case("shift") {
				shift = true;
				remaining = &remaining[plus_idx + 1..];
			} else {
				break;
			}
		}
		let key = remaining.to_string();
		if key.is_empty() {
			return None;
		}
		let normalized = Self::normalize_key_name(&key);
		Some(Self { ctrl, raw_ctrl, alt, shift, key: normalized })
	}

	pub fn from_key_code(key_code: i32, ctrl: bool, alt: bool, shift: bool) -> Option<Self> {
		let key_name = match key_code {
			13 | 370 => "Enter".to_string(),
			9 => "Tab".to_string(),
			32 => "Space".to_string(),
			8 => "Backspace".to_string(),
			127 | 308 | 386 => "Delete".to_string(),
			27 => "Escape".to_string(),
			313 | 377 => "Home".to_string(),
			312 | 379 => "End".to_string(),
			366 | 376 => "PageUp".to_string(),
			367 | 381 => "PageDown".to_string(),
			314 | 378 => "Left".to_string(),
			316 | 380 => "Right".to_string(),
			315 | 382 => "Up".to_string(),
			317 | 383 => "Down".to_string(),
			340..=363 => format!("F{}", key_code - 340 + 1),
			65..=90 => (char::from_u32(key_code as u32)?).to_string(),
			97..=122 => (char::from_u32((key_code - 32) as u32)?).to_string(),
			48..=57 => (char::from_u32(key_code as u32)?).to_string(),
			324..=333 => (char::from_u32((key_code - 324 + 48) as u32)?).to_string(),
			44 | 188 => ",".to_string(),
			46 | 190 | 387 => ".".to_string(),
			47 | 191 | 388 => "/".to_string(),
			91 | 219 => "[".to_string(),
			93 | 221 => "]".to_string(),
			92 | 220 => "\\".to_string(),
			45 | 189 | 390 => "-".to_string(),
			61 | 187 => "=".to_string(),
			59 | 186 => ";".to_string(),
			39 | 222 => "'".to_string(),
			96 | 192 => "`".to_string(),
			_ => return None,
		};
		Some(Self { ctrl, raw_ctrl: false, alt, shift, key: key_name })
	}

	pub fn matches(&self, key_code: i32, ctrl: bool, alt: bool, shift: bool) -> bool {
		let self_ctrl = self.ctrl || self.raw_ctrl;
		if self_ctrl != ctrl || self.alt != alt || self.shift != shift {
			return false;
		}
		let key_str = self.key.as_str();
		if key_str.eq_ignore_ascii_case("Enter") {
			key_code == 13 || key_code == 370
		} else if key_str.eq_ignore_ascii_case("Tab") {
			key_code == 9
		} else if key_str.eq_ignore_ascii_case("Space") {
			key_code == 32
		} else if key_str.eq_ignore_ascii_case("Backspace") {
			key_code == 8
		} else if key_str.eq_ignore_ascii_case("Delete") {
			key_code == 127 || key_code == 308 || key_code == 386
		} else if key_str.eq_ignore_ascii_case("Escape") {
			key_code == 27
		} else if key_str.eq_ignore_ascii_case("Home") {
			key_code == 313 || key_code == 377
		} else if key_str.eq_ignore_ascii_case("End") {
			key_code == 312 || key_code == 379
		} else if key_str.eq_ignore_ascii_case("PageUp") {
			key_code == 366 || key_code == 376
		} else if key_str.eq_ignore_ascii_case("PageDown") {
			key_code == 367 || key_code == 381
		} else if key_str.eq_ignore_ascii_case("Left") {
			key_code == 314 || key_code == 378
		} else if key_str.eq_ignore_ascii_case("Right") {
			key_code == 316 || key_code == 380
		} else if key_str.eq_ignore_ascii_case("Up") {
			key_code == 315 || key_code == 382
		} else if key_str.eq_ignore_ascii_case("Down") {
			key_code == 317 || key_code == 383
		} else if key_str.starts_with(['F', 'f'])
			&& let Ok(num) = key_str[1..].parse::<i32>()
			&& (1..=24).contains(&num)
		{
			key_code == 340 + num - 1
		} else if key_str.len() == 1 {
			let ch = key_str.chars().next().unwrap();
			if ch.is_ascii_alphabetic() {
				let upper = ch.to_ascii_uppercase() as i32;
				key_code == upper || key_code == upper + 32
			} else if ch.is_ascii_digit() {
				key_code == ch as i32 || key_code == (ch as i32 - 48 + 324)
			} else {
				match ch {
					',' => key_code == 44 || key_code == 188,
					'.' => key_code == 46 || key_code == 190 || key_code == 387,
					'/' => key_code == 47 || key_code == 191 || key_code == 388,
					'\\' => key_code == 92 || key_code == 220,
					'[' => key_code == 91 || key_code == 219,
					']' => key_code == 93 || key_code == 221,
					'-' => key_code == 45 || key_code == 189 || key_code == 390,
					'=' => key_code == 61 || key_code == 187,
					';' => key_code == 59 || key_code == 186,
					'\'' => key_code == 39 || key_code == 222,
					'`' => key_code == 96 || key_code == 192,
					_ => key_code == ch as i32,
				}
			}
		} else {
			false
		}
	}
}
