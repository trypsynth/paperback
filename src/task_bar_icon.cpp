/* task_bar_icon.cpp - wxTaskBarIcon implementation for Paperback.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "task_bar_icon.hpp"
#include "constants.hpp"
#include "main_window.hpp"
#include <memory>
#include <wx/defs.h>
#include <wx/event.h>
#include <wx/menu.h>
#include <wx/taskbar.h>
#include <wx/translation.h>

task_bar_icon::task_bar_icon(main_window* frame) : frame_{frame} {
	Bind(wxEVT_MENU, &task_bar_icon::on_restore_from_tray, this, ID_RESTORE);
	Bind(wxEVT_MENU, &task_bar_icon::on_exit_from_tray, this, wxID_EXIT);
	Bind(wxEVT_TASKBAR_LEFT_DCLICK, &task_bar_icon::on_tray_icon_activated, this);
	Bind(wxEVT_TASKBAR_LEFT_UP, &task_bar_icon::on_tray_icon_activated, this);
}

wxMenu* task_bar_icon::CreatePopupMenu() {
	std::unique_ptr<wxMenu> menu = std::make_unique<wxMenu>();
	menu->Append(ID_RESTORE, _("&Restore"));
	menu->AppendSeparator();
	menu->Append(wxID_EXIT, _("E&xit"));
	return menu.release();
}

void task_bar_icon::on_restore_from_tray(wxCommandEvent& /*unused*/) {
	frame_->Show(true);
	frame_->Raise();
	frame_->Iconize(false);
	frame_->CallAfter([frm = frame_] { frm->restore_focus_to_text(); });
}

void task_bar_icon::on_exit_from_tray(wxCommandEvent& /*unused*/) {
	frame_->Close(true);
}

void task_bar_icon::on_tray_icon_activated(wxTaskBarIconEvent& /*unused*/) {
	frame_->Show(true);
	frame_->Raise();
	frame_->Iconize(false);
	frame_->CallAfter([frm = frame_] { frm->restore_focus_to_text(); });
}
