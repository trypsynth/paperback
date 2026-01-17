#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDirection {
	Next,
	Previous,
	None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavTarget {
	Section,
	Page,
	Heading,
	List,
	ListItem,
	Link,
	Table,
	Unknown,
}

#[derive(Debug, Clone)]
pub struct NavRequest {
	pub position: i64,
	pub wrap: bool,
	pub direction: NavDirection,
	pub target: NavTarget,
	pub level_filter: i32,
}

#[derive(Debug, Clone)]
pub struct NavResult {
	pub found: bool,
	pub wrapped: bool,
	pub offset: usize,
	pub marker_level: i32,
	pub marker_text: String,
}

#[derive(Debug, Clone)]
pub struct FfiSearchResult {
	pub found: bool,
	pub wrapped: bool,
	pub position: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookmarkFilterType {
	All,
	BookmarksOnly,
	NotesOnly,
}

#[derive(Debug, Clone)]
pub struct BookmarkNavResult {
	pub found: bool,
	pub start: i64,
	pub end: i64,
	pub note: String,
	pub index: i32,
	pub wrapped: bool,
}

#[derive(Debug, Clone)]
pub struct FfiBookmarkInfo {
	pub found: bool,
	pub note: String,
}

#[derive(Debug, Clone)]
pub struct FfiBookmarkDisplayItem {
	pub start: i64,
	pub end: i64,
	pub note: String,
	pub is_whole_line: bool,
	pub index: usize,
}

#[derive(Debug, Clone)]
pub struct FfiFilteredBookmarks {
	pub items: Vec<FfiBookmarkDisplayItem>,
	pub closest_index: i32,
}

#[derive(Debug, Clone)]
pub struct FfiBookmarkNavDisplay {
	pub found: bool,
	pub wrapped: bool,
	pub start: i64,
	pub end: i64,
	pub note: String,
	pub snippet: String,
	pub index: i32,
}

#[derive(Debug, Clone)]
pub struct FfiBookmarkDisplayAtPosition {
	pub found: bool,
	pub note: String,
	pub snippet: String,
}

#[derive(Debug, Clone)]
pub struct FfiLinkListItem {
	pub offset: usize,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct FfiLinkList {
	pub items: Vec<FfiLinkListItem>,
	pub closest_index: i32,
}

#[derive(Debug, Clone)]
pub struct FfiBookmarkDisplayEntry {
	pub start: i64,
	pub end: i64,
	pub note: String,
	pub snippet: String,
	pub is_whole_line: bool,
	pub index: usize,
}

#[derive(Debug, Clone)]
pub struct FfiFilteredBookmarkDisplay {
	pub items: Vec<FfiBookmarkDisplayEntry>,
	pub closest_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentListStatus {
	Missing,
	Open,
	Closed,
}

#[derive(Debug, Clone)]
pub struct FfiDocumentListItem {
	pub path: String,
	pub filename: String,
	pub status: DocumentListStatus,
}
