use wxdragon::id::{ID_ABOUT, ID_EXIT};

// Re-export standard IDs
pub const EXIT: i32 = ID_EXIT;
pub const ABOUT: i32 = ID_ABOUT;

// Base for custom IDs
const BASE: i32 = 5000;

// File menu
pub const OPEN: i32 = BASE;
pub const CLOSE: i32 = BASE + 1;
pub const CLOSE_ALL: i32 = BASE + 2;
pub const SHOW_ALL_DOCUMENTS: i32 = BASE + 3;

// Recent documents (reserve 100 IDs)
pub const RECENT_DOCUMENT_BASE: i32 = BASE + 100;
pub const RECENT_DOCUMENT_MAX: i32 = BASE + 199;

// Go menu
pub const FIND: i32 = BASE + 200;
pub const FIND_NEXT: i32 = BASE + 201;
pub const FIND_PREVIOUS: i32 = BASE + 202;
pub const GO_TO_LINE: i32 = BASE + 210;
pub const GO_TO_PERCENT: i32 = BASE + 211;
pub const GO_TO_PAGE: i32 = BASE + 212;
pub const GO_BACK: i32 = BASE + 220;
pub const GO_FORWARD: i32 = BASE + 221;
pub const PREVIOUS_SECTION: i32 = BASE + 230;
pub const NEXT_SECTION: i32 = BASE + 231;
pub const PREVIOUS_HEADING: i32 = BASE + 240;
pub const NEXT_HEADING: i32 = BASE + 241;
pub const PREVIOUS_HEADING_1: i32 = BASE + 250;
pub const NEXT_HEADING_1: i32 = BASE + 251;
pub const PREVIOUS_HEADING_2: i32 = BASE + 252;
pub const NEXT_HEADING_2: i32 = BASE + 253;
pub const PREVIOUS_HEADING_3: i32 = BASE + 254;
pub const NEXT_HEADING_3: i32 = BASE + 255;
pub const PREVIOUS_HEADING_4: i32 = BASE + 256;
pub const NEXT_HEADING_4: i32 = BASE + 257;
pub const PREVIOUS_HEADING_5: i32 = BASE + 258;
pub const NEXT_HEADING_5: i32 = BASE + 259;
pub const PREVIOUS_HEADING_6: i32 = BASE + 260;
pub const NEXT_HEADING_6: i32 = BASE + 261;
pub const PREVIOUS_PAGE: i32 = BASE + 270;
pub const NEXT_PAGE: i32 = BASE + 271;
pub const PREVIOUS_BOOKMARK: i32 = BASE + 280;
pub const NEXT_BOOKMARK: i32 = BASE + 281;
pub const PREVIOUS_NOTE: i32 = BASE + 282;
pub const NEXT_NOTE: i32 = BASE + 283;
pub const JUMP_TO_ALL_BOOKMARKS: i32 = BASE + 284;
pub const JUMP_TO_BOOKMARKS_ONLY: i32 = BASE + 285;
pub const JUMP_TO_NOTES_ONLY: i32 = BASE + 286;
pub const VIEW_NOTE_TEXT: i32 = BASE + 287;
pub const PREVIOUS_LINK: i32 = BASE + 290;
pub const NEXT_LINK: i32 = BASE + 291;
pub const PREVIOUS_TABLE: i32 = BASE + 300;
pub const NEXT_TABLE: i32 = BASE + 301;
pub const PREVIOUS_LIST: i32 = BASE + 310;
pub const NEXT_LIST: i32 = BASE + 311;
pub const PREVIOUS_LIST_ITEM: i32 = BASE + 312;
pub const NEXT_LIST_ITEM: i32 = BASE + 313;

// Tools menu
pub const WORD_COUNT: i32 = BASE + 400;
pub const DOCUMENT_INFO: i32 = BASE + 401;
pub const TABLE_OF_CONTENTS: i32 = BASE + 402;
pub const ELEMENTS_LIST: i32 = BASE + 403;
pub const OPEN_CONTAINING_FOLDER: i32 = BASE + 404;
pub const OPEN_IN_WEB_VIEW: i32 = BASE + 405;
pub const IMPORT_DOCUMENT_DATA: i32 = BASE + 410;
pub const EXPORT_DOCUMENT_DATA: i32 = BASE + 411;
pub const EXPORT_TO_PLAIN_TEXT: i32 = BASE + 412;
pub const TOGGLE_BOOKMARK: i32 = BASE + 420;
pub const BOOKMARK_WITH_NOTE: i32 = BASE + 421;
pub const OPTIONS: i32 = BASE + 430;
pub const SLEEP_TIMER: i32 = BASE + 431;

// Help menu
pub const VIEW_HELP_BROWSER: i32 = BASE + 500;
pub const VIEW_HELP_PAPERBACK: i32 = BASE + 501;
pub const CHECK_FOR_UPDATES: i32 = BASE + 502;
pub const DONATE: i32 = BASE + 503;
