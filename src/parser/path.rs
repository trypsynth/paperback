use std::path::Path;

pub fn extract_title_from_path(path: &str) -> String {
	Path::new(path).file_stem().and_then(|s| s.to_str()).unwrap_or("Untitled").to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn extracts_title_from_path() {
		assert_eq!(extract_title_from_path("foo.txt"), "foo");
		assert_eq!(extract_title_from_path("/home/quin/books/worm.epub"), "worm");
		assert_eq!(extract_title_from_path("C:\\Users\\Quin\\Desktop\\file.log"), "file");
		assert_eq!(extract_title_from_path("/path/with/trailing/slash/"), "Untitled");
		assert_eq!(extract_title_from_path(""), "Untitled");
	}
}
