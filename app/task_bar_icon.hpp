#pragma once
#include <wx/taskbar.h>

class main_window;

class task_bar_icon : public wxTaskBarIcon {
public:
	explicit task_bar_icon(main_window* frame);

protected:
	wxMenu* CreatePopupMenu() override;

private:
	void on_restore_from_tray(wxCommandEvent&);
	void on_exit_from_tray(wxCommandEvent&);
	void on_tray_icon_activated(wxTaskBarIconEvent&);

	main_window* frame_;
};
