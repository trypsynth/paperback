#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDirection {
	Next,
	Previous,
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
pub struct SearchResult {
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
pub struct BookmarkInfo {
	pub found: bool,
	pub note: String,
}

#[derive(Debug, Clone)]
pub struct BookmarkDisplayItem {
	pub start: i64,
	pub end: i64,
	pub note: String,
	pub is_whole_line: bool,
	pub index: usize,
}

#[derive(Debug, Clone)]
pub struct FilteredBookmarks {
	pub items: Vec<BookmarkDisplayItem>,
	pub closest_index: i32,
}

#[derive(Debug, Clone)]
pub struct BookmarkNavDisplay {
	pub found: bool,
	pub wrapped: bool,
	pub start: i64,
	pub end: i64,
	pub note: String,
	pub snippet: String,
	pub index: i32,
}

#[derive(Debug, Clone)]
pub struct BookmarkDisplayAtPosition {
	pub found: bool,
	pub note: String,
	pub snippet: String,
}

#[derive(Debug, Clone)]
pub struct LinkListItem {
	pub offset: usize,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct LinkList {
	pub items: Vec<LinkListItem>,
	pub closest_index: i32,
}

#[derive(Debug, Clone)]
pub struct HeadingTreeItem {
	pub offset: usize,
	pub text: String,
	pub level: i32,
	pub parent_index: i32,
}

#[derive(Debug, Clone)]
pub struct HeadingTree {
	pub items: Vec<HeadingTreeItem>,
	pub closest_index: i32,
}

#[derive(Debug, Clone)]
pub struct BookmarkDisplayEntry {
	pub start: i64,
	pub end: i64,
	pub note: String,
	pub snippet: String,
	pub is_whole_line: bool,
	pub index: usize,
}

#[derive(Debug, Clone)]
pub struct FilteredBookmarkDisplay {
	pub items: Vec<BookmarkDisplayEntry>,
	pub closest_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentListStatus {
	Missing,
	Open,
	Closed,
}

#[derive(Debug, Clone)]
pub struct DocumentListItem {
	pub path: String,
	pub filename: String,
	pub status: DocumentListStatus,
}
