#pragma once
#include <wx/string.h>

inline const wxString APP_NAME = "Paperback";
inline const wxString APP_VERSION = "0.2";
inline const wxString APP_COPYRIGHT = "Copyright (C) 2025 Quin Gillespie. All rights reserved.";
inline const wxString APP_WEBSITE = "https://github.com/trypsynth/paperback";
inline constexpr int POSITION_SAVE_TIMER_INTERVAL = 5000;

enum {
	ID_EXPORT = wxID_HIGHEST + 1,
	ID_FIND_NEXT,
	ID_FIND_PREVIOUS,
	ID_GO_TO,
	ID_PREVIOUS_SECTION,
	ID_NEXT_SECTION,
	ID_PREVIOUS_PAGE,
	ID_NEXT_PAGE,
	ID_GO_TO_PAGE,
	ID_WORD_COUNT,
	ID_DOC_INFO,
	ID_TABLE_OF_CONTENTS,
};
