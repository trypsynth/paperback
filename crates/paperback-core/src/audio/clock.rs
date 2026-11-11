//! Times in a recording as people type and read them, like `1:02:03`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeEntryError {
	NotATime,
	OutOfRange,
}

/// A leading `+` or `-` moves from `current_ms`, as Go to Line's `+n` and `-n` do.
pub fn resolve_entry_ms(text: &str, current_ms: u64, total_ms: u64) -> Result<u64, TimeEntryError> {
	let text = text.trim();
	let (sign, rest) = match text.as_bytes().first() {
		Some(&sign @ (b'+' | b'-')) => (Some(sign), &text[1..]),
		_ => (None, text),
	};
	let amount_ms = parse_ms(rest).ok_or(TimeEntryError::NotATime)?;
	let target_ms = match sign {
		Some(b'+') => current_ms.checked_add(amount_ms),
		Some(_) => current_ms.checked_sub(amount_ms),
		None => Some(amount_ms),
	};
	target_ms.filter(|target_ms| *target_ms <= total_ms).ok_or(TimeEntryError::OutOfRange)
}

/// `H:MM:SS`, `M:SS` or plain seconds, the seconds taking up to three decimals; parts after the first stay under 60, so `1:75` is a typo rather than 2:15.
#[must_use]
pub fn parse_ms(text: &str) -> Option<u64> {
	let parts: Vec<&str> = text.trim().split(':').collect();
	if parts.len() > 3 {
		return None;
	}
	let (seconds, larger) = parts.split_last()?;
	let seconds_ms = parse_seconds_ms(seconds)?;
	if !larger.is_empty() && seconds_ms >= 60_000 {
		return None;
	}
	let minutes = larger.last().map_or(Some(0), |minutes| parse_digits(minutes))?;
	if larger.len() == 2 && minutes >= 60 {
		return None;
	}
	let hours = if larger.len() == 2 { parse_digits(larger[0])? } else { 0 };
	hours.checked_mul(3_600_000)?.checked_add(minutes.checked_mul(60_000)?)?.checked_add(seconds_ms)
}

/// Truncated to the second, so a book never shows as further along than it is.
#[must_use]
pub fn format_ms(ms: u64) -> String {
	let seconds = ms / 1000;
	let (hours, minutes, seconds) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
	if hours > 0 { format!("{hours}:{minutes:02}:{seconds:02}") } else { format!("{minutes}:{seconds:02}") }
}

fn parse_seconds_ms(text: &str) -> Option<u64> {
	let Some((whole, fraction)) = text.split_once('.') else {
		return parse_digits(text)?.checked_mul(1000);
	};
	let whole_ms = parse_digits(whole)?.checked_mul(1000)?;
	if fraction.is_empty() || !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
		return None;
	}
	let millis: String = fraction.chars().chain("00".chars()).take(3).collect();
	whole_ms.checked_add(millis.parse().ok()?)
}

fn parse_digits(text: &str) -> Option<u64> {
	if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
		return None;
	}
	text.parse().ok()
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::{TimeEntryError, format_ms, parse_ms, resolve_entry_ms};

	#[rstest]
	#[case("0", 0)]
	#[case("90", 90_000)]
	#[case("1:30", 90_000)]
	#[case("90:00", 5_400_000)]
	#[case("1:02:03", 3_723_000)]
	#[case("01:02:03", 3_723_000)]
	#[case("1:30.5", 90_500)]
	#[case("1:30.25", 90_250)]
	#[case("1:30.125", 90_125)]
	#[case("1:30.1259", 90_125)]
	#[case(" 1:30 ", 90_000)]
	fn parses(#[case] text: &str, #[case] ms: u64) {
		assert_eq!(parse_ms(text), Some(ms));
	}

	#[rstest]
	#[case("")]
	#[case(":")]
	#[case("1:")]
	#[case(":30")]
	#[case("1:75")]
	#[case("1:60:00")]
	#[case("1:2:3:4")]
	#[case("1.5:00")]
	#[case("1:30.")]
	#[case("1:30.x")]
	#[case("-5")]
	#[case("1 30")]
	#[case("abc")]
	#[case("99999999999999999999")]
	fn rejects(#[case] text: &str) {
		assert_eq!(parse_ms(text), None);
	}

	#[rstest]
	#[case(0, "0:00")]
	#[case(59_999, "0:59")]
	#[case(90_500, "1:30")]
	#[case(3_599_000, "59:59")]
	#[case(3_723_000, "1:02:03")]
	#[case(36_000_000, "10:00:00")]
	fn formats(#[case] ms: u64, #[case] text: &str) {
		assert_eq!(format_ms(ms), text);
		assert_eq!(parse_ms(text), Some(ms / 1000 * 1000));
	}

	#[test]
	fn entries_move_from_the_current_time_with_a_sign() {
		assert_eq!(resolve_entry_ms("1:00", 30_000, 600_000), Ok(60_000));
		assert_eq!(resolve_entry_ms("+1:00", 30_000, 600_000), Ok(90_000));
		assert_eq!(resolve_entry_ms("-20", 30_000, 600_000), Ok(10_000));
		assert_eq!(resolve_entry_ms("10:00", 30_000, 600_000), Ok(600_000));
	}

	#[test]
	fn entries_past_either_end_are_out_of_range() {
		assert_eq!(resolve_entry_ms("10:01", 0, 600_000), Err(TimeEntryError::OutOfRange));
		assert_eq!(resolve_entry_ms("+10:00", 30_000, 600_000), Err(TimeEntryError::OutOfRange));
		assert_eq!(resolve_entry_ms("-31", 30_000, 600_000), Err(TimeEntryError::OutOfRange));
	}

	#[test]
	fn entries_that_are_not_times_say_so() {
		assert_eq!(resolve_entry_ms("", 0, 600_000), Err(TimeEntryError::NotATime));
		assert_eq!(resolve_entry_ms("+", 0, 600_000), Err(TimeEntryError::NotATime));
		assert_eq!(resolve_entry_ms("soon", 0, 600_000), Err(TimeEntryError::NotATime));
	}
}
