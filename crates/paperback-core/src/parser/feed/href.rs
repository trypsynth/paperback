//! Resolving a link in a feed item's body against the item's own web address.

use crate::parser::{has_url_scheme, util::path::resolve_relative_path};

/// `href` resolved against `base`, the web address of the page the item stands for. A link that is already absolute, a `#fragment`, or any link when `base` is not an absolute web address comes back as written.
pub(super) fn resolve_feed_href(base: Option<&str>, href: &str) -> String {
	if href.is_empty() || href.starts_with('#') || has_url_scheme(href) {
		return href.to_string();
	}
	let Some((scheme, authority, path)) = base.and_then(split_web_address) else {
		return href.to_string();
	};
	if href.starts_with("//") {
		return format!("{scheme}:{href}");
	}
	if href.starts_with('/') {
		return format!("{scheme}://{authority}{href}");
	}
	if href.starts_with('?') {
		return format!("{scheme}://{authority}{path}{href}");
	}
	let (href_path, suffix) = href.find(['?', '#']).map_or((href, ""), |at| href.split_at(at));
	let folder = path.rfind('/').map_or("", |at| &path[..at]);
	let mut resolved = resolve_relative_path(folder, href_path);
	if href_path.ends_with('/') && !resolved.is_empty() {
		resolved.push('/');
	}
	format!("{scheme}://{authority}/{resolved}{suffix}")
}

/// An `http` or `https` address split into its scheme, its host and its path, with any query or fragment dropped.
fn split_web_address(address: &str) -> Option<(&str, &str, &str)> {
	let (scheme, rest) = address.split_once("://")?;
	if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
		return None;
	}
	let rest = rest.split(['?', '#']).next().unwrap_or_default();
	let (authority, path) = rest.find('/').map_or((rest, ""), |at| rest.split_at(at));
	(!authority.is_empty()).then_some((scheme, authority, path))
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	const BASE: &str = "https://blog.example/posts/first";

	#[rstest]
	#[case::absolute_link_is_kept(Some(BASE), "https://other.example/x", "https://other.example/x")]
	#[case::mailto_is_kept(Some(BASE), "mailto:me@blog.example", "mailto:me@blog.example")]
	#[case::scheme_relative_takes_the_base_scheme(Some(BASE), "//cdn.example/a.png", "https://cdn.example/a.png")]
	#[case::root_relative_takes_the_base_host(Some(BASE), "/about", "https://blog.example/about")]
	#[case::relative_resolves_against_the_base_folder(Some(BASE), "second", "https://blog.example/posts/second")]
	#[case::parent_segment_climbs(Some(BASE), "../tags/rust", "https://blog.example/tags/rust")]
	#[case::base_ending_in_a_slash_is_a_folder(
		Some("https://blog.example/posts/"),
		"second",
		"https://blog.example/posts/second"
	)]
	#[case::base_without_a_path(Some("https://blog.example"), "about", "https://blog.example/about")]
	#[case::base_query_and_fragment_are_dropped(Some("https://a.example/b/c?x=1#y"), "d", "https://a.example/b/d")]
	#[case::href_query_and_fragment_are_kept(
		Some(BASE),
		"second?page=2#top",
		"https://blog.example/posts/second?page=2#top"
	)]
	#[case::href_trailing_slash_is_kept(Some(BASE), "../archive/", "https://blog.example/archive/")]
	#[case::query_alone_keeps_the_base_page(Some(BASE), "?page=2", "https://blog.example/posts/first?page=2")]
	#[case::fragment_stays_in_the_document(Some(BASE), "#fn1", "#fn1")]
	#[case::no_base_keeps_the_link(None, "second", "second")]
	#[case::a_base_that_is_not_a_web_address_keeps_the_link(Some("posts/first"), "second", "second")]
	fn resolves_a_link_against_the_item(#[case] base: Option<&str>, #[case] href: &str, #[case] expected: &str) {
		assert_eq!(resolve_feed_href(base, href), expected);
	}
}
