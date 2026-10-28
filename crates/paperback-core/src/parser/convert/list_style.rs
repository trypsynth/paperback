//! Converters keep these on a stack so a nested list's numbering doesn't leak into its parent's once it closes.

#[derive(Clone, Debug)]
pub(super) struct ListStyle {
	pub(super) ordered: bool,
	pub(super) item_number: i32,
	pub(super) list_type: String,
}

impl Default for ListStyle {
	fn default() -> Self {
		Self { ordered: false, item_number: 1, list_type: "1".to_string() }
	}
}

impl ListStyle {
	pub(super) fn new(is_ordered: bool, start_attr: Option<&str>, type_attr: Option<&str>) -> Self {
		let mut style = Self::default();
		if is_ordered {
			style.ordered = true;
			if let Some(start_num) = start_attr.and_then(|s| s.parse::<i32>().ok()) {
				style.item_number = start_num;
			}
			if let Some(type_val) = type_attr {
				style.list_type = type_val.to_lowercase();
			}
		}
		style
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn unordered_list_ignores_start_and_type() {
		let style = ListStyle::new(false, Some("5"), Some("a"));
		assert!(!style.ordered);
		assert_eq!(style.item_number, 1);
		assert_eq!(style.list_type, "1");
	}

	#[test]
	fn ordered_list_defaults_when_attrs_absent() {
		let style = ListStyle::new(true, None, None);
		assert!(style.ordered);
		assert_eq!(style.item_number, 1);
		assert_eq!(style.list_type, "1");
	}

	#[test]
	fn ordered_list_reads_start_and_type() {
		let style = ListStyle::new(true, Some("3"), Some("A"));
		assert!(style.ordered);
		assert_eq!(style.item_number, 3);
		assert_eq!(style.list_type, "a", "type attribute is lowercased");
	}

	#[test]
	fn ordered_list_ignores_unparsable_start() {
		let style = ListStyle::new(true, Some("not-a-number"), None);
		assert_eq!(style.item_number, 1);
	}
}
