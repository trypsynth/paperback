#pragma once
#include <functional>
#include <utility>
#include <vector>
#include <wx/menu.h>
#include <wx/string.h>

enum class menu_item_kind {
	item,
	separator,
	submenu,
	populate,
	submenu_populate,
};

struct menu_item {
	menu_item_kind kind{menu_item_kind::item};
	int id{wxID_ANY};
	wxString label{};
	std::vector<menu_item> children{};				   // for submenu
	std::function<void(wxMenu* parent)> custom_append; // for populate/submenu_populate/custom

	static menu_item item(int item_id, const wxString& text) {
		menu_item d;
		d.kind = menu_item_kind::item;
		d.id = item_id;
		d.label = text;
		return d;
	}

	static menu_item sep() {
		menu_item d;
		d.kind = menu_item_kind::separator;
		return d;
	}

	static menu_item submenu(const wxString& text, std::vector<menu_item> items) {
		menu_item d;
		d.kind = menu_item_kind::submenu;
		d.label = text;
		d.children = std::move(items);
		return d;
	}

	static menu_item populate(std::function<void(wxMenu* parent)> fn) {
		menu_item d;
		d.kind = menu_item_kind::populate;
		d.custom_append = std::move(fn);
		return d;
	}

	static menu_item submenu_populate(const wxString& text, std::function<void(wxMenu* submenu)> fn) {
		menu_item d;
		d.kind = menu_item_kind::submenu_populate;
		d.label = text;
		d.custom_append = std::move(fn);
		return d;
	}
};

struct menu {
	wxString title;
	std::vector<menu_item> items;
};

inline void append_items(wxMenu* menu, const std::vector<menu_item>& items) {
	for (const auto& it : items) {
		switch (it.kind) {
			case menu_item_kind::item:
				menu->Append(it.id, it.label);
				break;
			case menu_item_kind::separator:
				menu->AppendSeparator();
				break;
			case menu_item_kind::submenu: {
				auto* sub = new wxMenu();
				append_items(sub, it.children);
				menu->AppendSubMenu(sub, it.label);
				break;
			}
			case menu_item_kind::populate:
				if (it.custom_append) {
					it.custom_append(menu);
				}
				break;
			case menu_item_kind::submenu_populate: {
				auto* sub = new wxMenu();
				if (it.custom_append) {
					it.custom_append(sub);
				}
				menu->AppendSubMenu(sub, it.label);
				break;
			}
		}
	}
}
