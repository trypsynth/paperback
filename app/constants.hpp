/* constants.hpp - contains app-wide constants.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "version.h"
#include <wx/string.h>

inline const wxString APP_NAME = "Paperback";
inline const wxString APP_VERSION = wxString::FromUTF8(PAPERBACK_VERSION_STRING);
inline const wxString APP_COPYRIGHT = "Copyright (C) 2025 Quin Gillespie. All rights reserved.";
inline const wxString APP_WEBSITE = "https://github.com/trypsynth/paperback";
inline constexpr int POSITION_SAVE_THROTTLE_MS = 1000;
inline constexpr int STATUS_UPDATE_THROTTLE_MS = 100;
inline constexpr int MAX_FIND_HISTORY_SIZE = 10;
inline constexpr int MAX_HEADING_LEVELS = 6;
inline constexpr int DEFAULT_RECENT_DOCUMENTS_TO_SHOW = 25;
inline constexpr int MAX_RECENT_DOCUMENTS_TO_SHOW = 100;
inline const wxString IPC_SERVICE = "paperback_ipc_service";
inline const wxString IPC_TOPIC_OPEN_FILE = "open_file";
inline const wxString IPC_COMMAND_ACTIVATE = "ACTIVATE";
inline const wxString IPC_HOST_LOCALHOST = "localhost";
inline const wxString SINGLE_INSTANCE_NAME = "paperback_running";
inline constexpr int DIALOG_PADDING = 10;

enum config_version {
	CONFIG_VERSION_LEGACY = 0,
	CONFIG_VERSION_1 = 1,
	CONFIG_VERSION_2 = 2,
	CONFIG_VERSION_CURRENT = CONFIG_VERSION_2
};

// Main menu constants.
enum {
	// File menu
	ID_RECENT_DOCUMENTS_BASE,
	ID_RECENT_DOCUMENTS_END = ID_RECENT_DOCUMENTS_BASE + 100,
	ID_SHOW_ALL_DOCUMENTS,
	// Go menu
	ID_FIND_NEXT,
	ID_FIND_PREVIOUS,
	ID_GO_TO_LINE,
	ID_GO_TO_PERCENT,
	ID_GO_BACK,
	ID_GO_FORWARD,
	ID_GO_TO_PAGE,
	ID_PREVIOUS_SECTION,
	ID_NEXT_SECTION,
	ID_PREVIOUS_HEADING,
	ID_NEXT_HEADING,
	ID_PREVIOUS_HEADING_1,
	ID_NEXT_HEADING_1,
	ID_PREVIOUS_HEADING_2,
	ID_NEXT_HEADING_2,
	ID_PREVIOUS_HEADING_3,
	ID_NEXT_HEADING_3,
	ID_PREVIOUS_HEADING_4,
	ID_NEXT_HEADING_4,
	ID_PREVIOUS_HEADING_5,
	ID_NEXT_HEADING_5,
	ID_PREVIOUS_HEADING_6,
	ID_NEXT_HEADING_6,
	ID_PREVIOUS_PAGE,
	ID_NEXT_PAGE,
	ID_NEXT_BOOKMARK,
	ID_PREVIOUS_BOOKMARK,
	ID_NEXT_NOTE,
	ID_PREVIOUS_NOTE,
	ID_TOGGLE_BOOKMARK,
	ID_BOOKMARK_WITH_NOTE,
	ID_JUMP_TO_BOOKMARK,
	ID_JUMP_TO_BOOKMARKS_ONLY,
	ID_JUMP_TO_NOTES,
	ID_VIEW_NOTE_TEXT,
	ID_NEXT_LINK,
	ID_PREVIOUS_LINK,
	ID_ACTIVATE_LINK,
	ID_PREVIOUS_TABLE,
	ID_NEXT_TABLE,
	ID_PREVIOUS_LIST,
	ID_NEXT_LIST,
	ID_PREVIOUS_LIST_ITEM,
	ID_NEXT_LIST_ITEM,
	// Tools menu
	ID_WORD_COUNT,
	ID_DOC_INFO,
	ID_TABLE_OF_CONTENTS,
	ID_LIST_ELEMENTS,
	ID_OPEN_CONTAINING_FOLDER,
	ID_EXPORT_TO_TEXT,
	ID_IMPORT,
	ID_EXPORT_DOCUMENT_DATA,
	ID_OPTIONS,
	ID_SLEEP_TIMER,
	// Help menu
	ID_HELP_INTERNAL,
	ID_DONATE,
	ID_CHECK_FOR_UPDATES,
};

// System tray menu constants.
enum {
	ID_RESTORE = wxID_HIGHEST + 1,
};
