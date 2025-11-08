/* menu_builder.hpp - Declarative menu builder utilities.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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
