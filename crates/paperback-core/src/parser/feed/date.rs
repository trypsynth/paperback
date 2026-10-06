//! Reading the dates a feed gives and writing them out for the reader.

use chrono::{DateTime, Datelike, NaiveDate};

use crate::t;

/// The date `raw` names, written as a day, a month name and a year in the feed's own time zone, or
/// `raw` itself, trimmed, when it is not a date this can read.
pub(super) fn feed_date_text(raw: &str) -> String {
	let trimmed = raw.trim();
	parse_date(trimmed).map_or_else(|| trimmed.to_string(), format_date)
}

/// Reads RSS's RFC 822 dates, then Atom's RFC 3339 ones, then a bare `YYYY-MM-DD`.
fn parse_date(text: &str) -> Option<NaiveDate> {
	DateTime::parse_from_rfc2822(text)
		.or_else(|_| DateTime::parse_from_rfc3339(text))
		.map(|date| date.date_naive())
		.ok()
		.or_else(|| NaiveDate::parse_from_str(text, "%Y-%m-%d").ok())
}

fn format_date(date: NaiveDate) -> String {
	// TRANSLATORS: How the publication date of a feed item is written, such as "6 October 2026"; {day} is the day of the month, {month} the month's name and {year} the year
	t("{day} {month} {year}")
		.replace("{day}", &date.day().to_string())
		.replace("{month}", &month_name(date.month()))
		.replace("{year}", &date.year().to_string())
}

fn month_name(month: u32) -> String {
	match month {
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 January 2026"
		1 => t("January"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 February 2026"
		2 => t("February"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 March 2026"
		3 => t("March"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 April 2026"
		4 => t("April"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 May 2026"
		5 => t("May"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 June 2026"
		6 => t("June"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 July 2026"
		7 => t("July"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 August 2026"
		8 => t("August"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 September 2026"
		9 => t("September"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 October 2026"
		10 => t("October"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 November 2026"
		11 => t("November"),
		// TRANSLATORS: Month name in the publication date of a feed item, such as "6 December 2026"
		_ => t("December"),
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::rfc822_gmt("Tue, 06 Oct 2026 09:00:00 GMT", "6 October 2026")]
	#[case::rfc822_keeps_the_feeds_own_day("Tue, 06 Oct 2026 23:30:00 -0500", "6 October 2026")]
	#[case::rfc822_named_zone_without_seconds("Tue, 06 Oct 2026 09:00 EST", "6 October 2026")]
	#[case::rfc822_without_weekday("6 Oct 2026 09:00:00 +0200", "6 October 2026")]
	#[case::rfc3339_keeps_the_feeds_own_day("2026-10-06T23:30:00-05:00", "6 October 2026")]
	#[case::plain_date("2026-01-31", "31 January 2026")]
	#[case::surrounding_whitespace("  2026-05-01\n", "1 May 2026")]
	#[case::december("Thu, 31 Dec 2026 12:00:00 +0000", "31 December 2026")]
	#[case::unreadable_text_is_kept("sometime last week", "sometime last week")]
	#[case::unreadable_text_is_trimmed("  soon  ", "soon")]
	fn writes_a_feed_date_for_the_reader(#[case] raw: &str, #[case] expected: &str) {
		assert_eq!(feed_date_text(raw), expected);
	}
}
