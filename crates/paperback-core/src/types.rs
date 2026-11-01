use crate::document::{DocLen, DocPos};

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
	Separator,
	Image,
	Figure,
	Formula,
}

#[derive(Debug, Clone)]
pub struct HeadingInfo {
	pub offset: DocPos,
	pub level: i32,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct FormatInfo {
	pub offset: DocPos,
	pub length: DocLen,
}

#[derive(Debug, Clone)]
pub struct LinkInfo {
	pub offset: DocPos,
	pub text: String,
	pub reference: String,
}

#[derive(Debug, Clone)]
pub struct ImageInfo {
	pub offset: DocPos,
	pub alt_text: String,
	/// Brackets and label included.
	pub length: DocLen,
	/// As the markup wrote it, unresolved; empty when there is none.
	pub source: String,
}

#[derive(Debug, Clone)]
pub struct ListInfo {
	pub offset: DocPos,
	pub item_count: i32,
	pub length: DocLen,
}

#[derive(Debug, Clone)]
pub struct ListItemInfo {
	pub offset: DocPos,
	pub level: i32,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct TableInfo {
	pub offset: DocPos,
	pub text: String,
	pub html_content: String,
	pub length: DocLen,
}

#[derive(Debug, Clone)]
pub struct FormulaInfo {
	pub offset: DocPos,
	pub text: String,
	pub mathml: String,
	/// Excludes the surrounding line breaks.
	pub length: DocLen,
}

#[derive(Debug, Clone)]
pub struct SeparatorInfo {
	pub offset: DocPos,
	pub length: DocLen,
}

#[derive(Debug, Clone)]
pub struct PageBreakInfo {
	pub offset: DocPos,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct NavRequest {
	pub position: DocPos,
	pub wrap: bool,
	pub direction: NavDirection,
	pub target: NavTarget,
	pub level_filter: i32,
}

#[derive(Debug, Clone)]
pub struct NavResult {
	pub found: bool,
	pub wrapped: bool,
	pub offset: DocPos,
	pub marker_level: i32,
	pub marker_text: String,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
	pub found: bool,
	pub wrapped: bool,
	/// The start of the document when nothing was found.
	pub position: DocPos,
	/// Where the match ends, which only for a plain query is its length past `position`.
	pub end: DocPos,
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
	pub start: DocPos,
	pub note: String,
	pub index: i32,
	pub wrapped: bool,
}

#[derive(Debug, Clone)]
pub struct BookmarkDisplayItem {
	pub start: DocPos,
	pub end: DocPos,
	pub note: String,
	pub is_whole_line: bool,
	pub audio_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct FilteredBookmarks {
	pub items: Vec<BookmarkDisplayItem>,
	pub closest_index: i32,
}

#[derive(Debug, Clone)]
pub struct BookmarkDisplayAtPosition {
	pub found: bool,
	pub note: String,
	pub snippet: String,
}

#[derive(Debug, Clone)]
pub struct LinkListItem {
	pub offset: DocPos,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct LinkList {
	pub items: Vec<LinkListItem>,
	pub closest_index: i32,
}

#[derive(Debug, Clone)]
pub struct ElementListItem {
	pub offset: DocPos,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct ElementList {
	pub items: Vec<ElementListItem>,
	pub closest_index: i32,
}

#[derive(Debug, Clone)]
pub struct HeadingTreeItem {
	pub offset: DocPos,
	pub text: String,
	pub parent_index: i32,
}

#[derive(Debug, Clone)]
pub struct HeadingTree {
	pub items: Vec<HeadingTreeItem>,
	pub closest_index: i32,
}

#[derive(Debug, Clone)]
pub struct BookmarkDisplayEntry {
	pub start: DocPos,
	pub end: DocPos,
	/// What tells two audio bookmarks in one chapter apart.
	pub audio_ms: Option<u64>,
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
