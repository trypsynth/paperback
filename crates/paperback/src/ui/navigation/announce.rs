use std::{cell::RefCell, rc::Rc};

use patois::t;
use wxdragon::prelude::*;

use super::MarkerNavTarget;

pub(super) enum NavFoundFormat {
	TextOnly,
	TextWithLevel,
	PageFormat,
	LinkFormat,
	ImageFormat,
}

pub(super) struct NavAnnouncements {
	pub(super) not_supported: String,
	pub(super) not_found_next: String,
	pub(super) not_found_prev: String,
	pub(super) format: NavFoundFormat,
}

pub(super) fn nav_announcements(target: MarkerNavTarget, level_filter: i32) -> NavAnnouncements {
	match target {
		MarkerNavTarget::Section => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no sections to navigate
			not_supported: t("No sections."),
			// TRANSLATORS: Announced when there is no next section from the current position
			not_found_next: t("No next section."),
			// TRANSLATORS: Announced when there is no previous section from the current position
			not_found_prev: t("No previous section."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Heading(level) => {
			if level_filter > 0 {
				// TRANSLATORS: Announced when the document has no headings at the given level; %d is the heading level number
				let no_headings = t("No headings at level %d.");
				// TRANSLATORS: Announced when there is no next heading at the given level; %d is the heading level number
				let no_next = t("No next heading at level %d.");
				// TRANSLATORS: Announced when there is no previous heading at the given level; %d is the heading level number
				let no_prev = t("No previous heading at level %d.");
				NavAnnouncements {
					not_supported: no_headings.replacen("%d", &level.to_string(), 1),
					not_found_next: no_next.replacen("%d", &level.to_string(), 1),
					not_found_prev: no_prev.replacen("%d", &level.to_string(), 1),
					format: NavFoundFormat::TextWithLevel,
				}
			} else {
				NavAnnouncements {
					// TRANSLATORS: Announced when the document has no headings at all (no level filter applied)
					not_supported: t("No headings."),
					// TRANSLATORS: Announced when there is no next heading (no level filter applied)
					not_found_next: t("No next heading."),
					// TRANSLATORS: Announced when there is no previous heading (no level filter applied)
					not_found_prev: t("No previous heading."),
					format: NavFoundFormat::TextWithLevel,
				}
			}
		}
		MarkerNavTarget::Page => NavAnnouncements {
			// TRANSLATORS: Announced when "Go to Page" is used on a document that has no page numbers
			not_supported: t("No pages."),
			// TRANSLATORS: Announced when there is no next page from the current position
			not_found_next: t("No next page."),
			// TRANSLATORS: Announced when there is no previous page from the current position
			not_found_prev: t("No previous page."),
			format: NavFoundFormat::PageFormat,
		},
		MarkerNavTarget::Link => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no links to navigate
			not_supported: t("No links."),
			// TRANSLATORS: Announced when there is no next link from the current position
			not_found_next: t("No next link."),
			// TRANSLATORS: Announced when there is no previous link from the current position
			not_found_prev: t("No previous link."),
			format: NavFoundFormat::LinkFormat,
		},
		MarkerNavTarget::List => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no lists to navigate
			not_supported: t("No lists."),
			// TRANSLATORS: Announced when there is no next list from the current position
			not_found_next: t("No next list."),
			// TRANSLATORS: Announced when there is no previous list from the current position
			not_found_prev: t("No previous list."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::ListItem => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no list items to navigate
			not_supported: t("No list items."),
			// TRANSLATORS: Announced when there is no next list item from the current position
			not_found_next: t("No next list item."),
			// TRANSLATORS: Announced when there is no previous list item from the current position
			not_found_prev: t("No previous list item."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Table => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no tables to navigate
			not_supported: t("No tables."),
			// TRANSLATORS: Announced when there is no next table from the current position
			not_found_next: t("No next table."),
			// TRANSLATORS: Announced when there is no previous table from the current position
			not_found_prev: t("No previous table."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Formula => NavAnnouncements {
			// TRANSLATORS: Announcement when formula navigation is unavailable because the document has no formulas
			not_supported: t("No formulas."),
			// TRANSLATORS: Announcement when there is no next formula to navigate to
			not_found_next: t("No next formula."),
			// TRANSLATORS: Announcement when there is no previous formula to navigate to
			not_found_prev: t("No previous formula."),
			// The marker text is the formula's AsciiMath rendering; announce it as-is.
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Separator => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no separators to navigate
			not_supported: t("No separators."),
			// TRANSLATORS: Announced when there is no next separator from the current position
			not_found_next: t("No next separator."),
			// TRANSLATORS: Announced when there is no previous separator from the current position
			not_found_prev: t("No previous separator."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Image => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no images to navigate
			not_supported: t("No images."),
			// TRANSLATORS: Announced when there is no next image from the current position
			not_found_next: t("No next image."),
			// TRANSLATORS: Announced when there is no previous image from the current position
			not_found_prev: t("No previous image."),
			format: NavFoundFormat::ImageFormat,
		},
		MarkerNavTarget::Figure => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no figures to navigate
			not_supported: t("No figures."),
			// TRANSLATORS: Announced when there is no next figure from the current position
			not_found_next: t("No next figure."),
			// TRANSLATORS: Announced when there is no previous figure from the current position
			not_found_prev: t("No previous figure."),
			format: NavFoundFormat::ImageFormat,
		},
	}
}

pub(super) fn format_nav_found_message(
	ann: &NavAnnouncements,
	context_text: &str,
	context_index: i32,
	wrapped: bool,
	next: bool,
) -> String {
	let wrap_prefix =
		// TRANSLATORS: Prefix announced when navigation wraps around past the end/start of the document; the trailing space is significant
		if wrapped { if next { t("Wrapping to start. ") } else { t("Wrapping to end. ") } } else { String::new() };
	match ann.format {
		NavFoundFormat::TextOnly => format!("{wrap_prefix}{context_text}"),
		NavFoundFormat::TextWithLevel => {
			// TRANSLATORS: Announcement when landing on a heading; %s is the heading text, %d is the heading level number
			let template = t("%s Heading level %d");
			let message = template.replacen("%s", context_text, 1).replacen("%d", &context_index.to_string(), 1);
			format!("{wrap_prefix}{message}")
		}
		NavFoundFormat::PageFormat => {
			let message = page_announcement(context_index + 1, context_text);
			format!("{wrap_prefix}{message}")
		}
		NavFoundFormat::LinkFormat => {
			// TRANSLATORS: Suffix appended after a link's text when announcing navigation to a link; the leading space is significant
			let message = format!("{context_text}{}", t(" link"));
			format!("{wrap_prefix}{message}")
		}
		NavFoundFormat::ImageFormat => {
			let message = context_text.to_string();
			format!("{wrap_prefix}{message}")
		}
	}
}

/// The announcement for landing on page `page`, whose first line of real content is
/// `content` (empty for a blank or image-only page). Shared by page navigation and the
/// Go to page dialog so both read the same.
pub fn page_announcement(page: i32, content: &str) -> String {
	let page_text = page.to_string();
	if content.is_empty() {
		// TRANSLATORS: Announcement when landing on a page with no extractable text; %d is the page number
		t("Page %d").replacen("%d", &page_text, 1)
	} else {
		// TRANSLATORS: Announcement when landing on a page; %d is the page number, %s is the page text
		t("Page %d: %s").replacen("%d", &page_text, 1).replacen("%s", content, 1)
	}
}

/// How long after a jump the interrupting announcement is raised.
///
/// The interrupt must land after NVDA has started the focus-return chain (if it fires
/// before, the chain reads over the announcement instead), but as close to the chain's
/// start as possible, so as little of it escapes before the cut. NVDA's own find dialog
/// uses 100ms; Paperback's chain starts quickly, and 30ms was chosen by binary search on
/// the Find dialog (60ms left a barely-audible sliver of the chain, 10ms occasionally let
/// the full chain through after the announcement, 0ms always did).
const FOCUS_CHAIN_INTERRUPT_DELAY_MS: i32 = 30;

/// How long after a command raised by a menu item the interrupting announcement is raised.
///
/// Closing a popup menu sends focus back to the frame and the screen reader answers by reading
/// the frame's whole child chain -- window title, tab strip, then the book -- which reads over
/// anything said before that chain starts. That chain starts well after the one
/// [`FOCUS_CHAIN_INTERRUPT_DELAY_MS`] was tuned against: at 30ms the message was spoken first
/// and then cut off part-way through.
const MENU_FOCUS_CHAIN_INTERRUPT_DELAY_MS: i32 = 100;

/// Announces `message` shortly after whatever the reader just did, so it cuts off the
/// focus-chain announcement the screen reader starts when focus returns to the book.
///
/// Every announcement goes through this. A message raised the moment a command runs is spoken
/// before the screen reader has noticed the focus change that closing a menu or a dialog causes,
/// and the chain that follows reads straight over it: that is why choosing Play from the menu
/// read the book's title instead of "This document has no audio".
pub fn announce(live_region_label: StaticText, message: impl AsRef<str>) {
	announce_after(live_region_label, message, FOCUS_CHAIN_INTERRUPT_DELAY_MS);
}

/// Announces `message` for a command, waiting only when a menu item was used to raise it.
///
/// Closing a popup menu sends focus back to the frame and the screen reader answers by reading
/// the frame's whole child chain -- window title, tab strip, then the book -- which reads over
/// anything said before that chain starts. A shortcut never takes focus away from the book,
/// nothing reads over the message, and waiting would be latency paid for nothing.
///
/// Which of the two ran cannot be told from the command itself, because a menu click and a
/// shortcut produce the same event; callers pass that in as `from_keyboard`. See
/// [`crate::ui::commands::Ctx::from_keyboard`].
pub fn announce_for_command(live_region_label: StaticText, from_keyboard: bool, message: impl AsRef<str>) {
	if from_keyboard {
		live_region::announce(live_region_label, message.as_ref());
	} else {
		announce_after(live_region_label, message, MENU_FOCUS_CHAIN_INTERRUPT_DELAY_MS);
	}
}

/// Announces `message` for a command whose own effect makes the screen reader speak, so a shortcut
/// waits too.
///
/// [`announce_for_command`] skips the wait for a shortcut because a shortcut takes no focus away from
/// the book and so leaves nothing saying anything to read over. A command that changes the control's
/// selection breaks that: the change is itself reported by the screen reader, so a message raised the
/// instant the command runs is spoken *first* and leaves the reader's own "selected" hanging on the
/// end of it. Waiting swaps those round, so the report is cut off at its start instead of trailing
/// after the message.
///
/// The wait is shorter than a menu's because what it is cutting starts sooner: a selection change is
/// reported immediately, where a closing menu's focus chain takes the extra time to begin.
pub fn announce_for_selection_command(live_region_label: StaticText, from_keyboard: bool, message: impl AsRef<str>) {
	let delay_ms = if from_keyboard { FOCUS_CHAIN_INTERRUPT_DELAY_MS } else { MENU_FOCUS_CHAIN_INTERRUPT_DELAY_MS };
	announce_after(live_region_label, message, delay_ms);
}

/// The timer hangs off the live region itself rather than the frame, so a handler that never
/// sees the window can still announce. The one-shot `wxTimer` is kept alive through its single
/// tick by the `Rc`/`RefCell` it hands its own callback: the tick clears the cell, which drops
/// the timer and destroys the native timer. If the timer cannot be armed, fall back to
/// announcing immediately rather than silently dropping the message.
fn announce_after(live_region_label: StaticText, message: impl AsRef<str>, delay_ms: i32) {
	let message = message.as_ref();
	let timer_holder: Rc<RefCell<Option<Timer<StaticText>>>> = Rc::new(RefCell::new(None));
	let holder = Rc::clone(&timer_holder);
	let timer = Timer::new(&live_region_label);
	let announce = message.to_string();
	timer.on_tick(move |_event| {
		live_region::announce(live_region_label, &announce);
		*holder.borrow_mut() = None;
	});
	if !timer.start(delay_ms, true) {
		live_region::announce(live_region_label, message);
		return;
	}
	*timer_holder.borrow_mut() = Some(timer);
}
