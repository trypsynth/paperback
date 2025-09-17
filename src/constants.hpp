/* constants.hpp - contains app-wide constants.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include <wx/string.h>

inline const wxString APP_NAME = "Paperback";
inline const wxString APP_VERSION = "0.3";
inline const wxString APP_COPYRIGHT = "Copyright (C) 2025 Quin Gillespie. All rights reserved.";
inline const wxString APP_WEBSITE = "https://github.com/trypsynth/paperback";
inline constexpr int POSITION_SAVE_TIMER_INTERVAL = 5000;

enum {
	// File menu
	ID_EXPORT = wxID_HIGHEST + 1,
	ID_RECENT_DOCUMENTS_BASE,
	ID_RECENT_DOCUMENTS_END = ID_RECENT_DOCUMENTS_BASE + 10,
	// Go menu
	ID_FIND_NEXT,
	ID_FIND_PREVIOUS,
	ID_GO_TO,
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
	// Tools menu
	ID_WORD_COUNT,
	ID_DOC_INFO,
	ID_TABLE_OF_CONTENTS,
	ID_OPTIONS,
	// Help menu
	ID_HELP_INTERNAL,
	ID_DONATE,
};
