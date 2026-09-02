use patois::nt;

/// Formats a duration given in whole seconds as a localized, comma-joined list of non-zero
/// segments (e.g. "1 hour, 5 minutes, 3 seconds"), omitting any segment that's zero unless the
/// whole duration is zero, in which case "0 seconds" is shown. Shared by the Word Count and
/// Document Info dialogs so a reading-time estimate and an audio duration read the same way.
pub(super) fn format_duration_seconds(total_seconds: u64) -> String {
	let hours = total_seconds / 3600;
	let minutes = (total_seconds % 3600) / 60;
	let seconds = total_seconds % 60;
	let mut parts: Vec<String> = Vec::new();
	if hours >= 1 {
		// TRANSLATORS: Duration segment for hours (e.g. "1 hour" / "5 hours"). The %d placeholder is replaced with the count.
		parts.push(nt("%d hour", "%d hours", hours).replacen("%d", &hours.to_string(), 1));
	}
	if minutes >= 1 {
		// TRANSLATORS: Duration segment for minutes (e.g. "1 minute" / "5 minutes"). The %d placeholder is replaced with the count.
		parts.push(nt("%d minute", "%d minutes", minutes).replacen("%d", &minutes.to_string(), 1));
	}
	if seconds >= 1 || total_seconds == 0 {
		// TRANSLATORS: Duration segment for seconds (e.g. "1 second" / "5 seconds"). The %d placeholder is replaced with the count.
		parts.push(nt("%d second", "%d seconds", seconds).replacen("%d", &seconds.to_string(), 1));
	}
	parts.join(", ")
}

/// [`format_duration_seconds`] for a duration given in milliseconds.
pub(super) fn format_duration_ms(total_ms: u64) -> String {
	format_duration_seconds(total_ms / 1000)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_locale;

	/// These assert English output from a function that translates, so they have to say which
	/// locale they mean and keep other tests from moving it underneath them.
	fn english() -> test_locale::LocaleGuard {
		test_locale::pinned_to("en")
	}

	#[test]
	fn format_duration_seconds_joins_nonzero_segments() {
		let _locale = english();
		assert_eq!(format_duration_seconds(3665), "1 hour, 1 minute, 5 seconds");
	}

	#[test]
	fn format_duration_seconds_omits_zero_segments() {
		let _locale = english();
		assert_eq!(format_duration_seconds(3600), "1 hour");
		assert_eq!(format_duration_seconds(60), "1 minute");
	}

	#[test]
	fn format_duration_seconds_of_zero_shows_zero_seconds() {
		let _locale = english();
		assert_eq!(format_duration_seconds(0), "0 seconds");
	}

	#[test]
	fn format_duration_seconds_pluralizes_each_segment_independently() {
		let _locale = english();
		assert_eq!(format_duration_seconds(2 * 3600 + 5 * 60 + 1), "2 hours, 5 minutes, 1 second");
	}

	#[test]
	fn format_duration_ms_floors_to_the_nearest_second() {
		let _locale = english();
		assert_eq!(format_duration_ms(2 * 60 * 60 * 1000 + 999), "2 hours");
	}
}
