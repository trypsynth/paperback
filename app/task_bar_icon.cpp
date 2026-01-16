#include "task_bar_icon.hpp"
#include "constants.hpp"
#include "main_window.hpp"
#include "menu_builder.hpp"
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
	append_items(menu.get(), {
		menu_item::item(ID_RESTORE, _("&Restore")),
		menu_item::sep(),
		menu_item::item(wxID_EXIT, _("E&xit")),
	});
	return menu.release();
}

void task_bar_icon::on_restore_from_tray(wxCommandEvent& /*unused*/) {
	frame_->Iconize(false);
	frame_->Show(true);
	frame_->Raise();
	frame_->CallAfter([frm = frame_] {
		frm->restore_focus_to_text();
	});
}

void task_bar_icon::on_exit_from_tray(wxCommandEvent& /*unused*/) {
	frame_->Close(true);
}

void task_bar_icon::on_tray_icon_activated(wxTaskBarIconEvent& /*unused*/) {
	frame_->Iconize(false);
	frame_->Show(true);
	frame_->Raise();
	frame_->CallAfter([frm = frame_] {
		frm->restore_focus_to_text();
	});
}
