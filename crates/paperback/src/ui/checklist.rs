//! A list that draws itself, each row with a check box, after wxkit's checklist sample: screen readers see it as a list, and hear each row as it becomes the current one.

use std::{cell::RefCell, rc::Rc};

use wxdragon::{dc::AutoBufferedPaintDC, prelude::*};

#[cfg(target_os = "windows")]
mod accessible;

/// wxWidgets' `wxWANTS_CHARS`, which wxDragon's panel styles leave out; without it a dialog takes the arrow keys and Tab for itself.
const WANTS_CHARS: i64 = 0x0004_0000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecklistRow {
	/// One per column, read together as the row's name.
	pub cells: Vec<String>,
	pub checked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecklistChange {
	Current(usize),
	Toggled(usize),
	Moved { from: usize, to: usize },
}

struct State {
	rows: Vec<ChecklistRow>,
	current: usize,
	top: usize,
	reorderable: bool,
	checked_word: String,
	unchecked_word: String,
	on_change: Option<Rc<dyn Fn(ChecklistChange)>>,
}

impl State {
	#[cfg(target_os = "windows")]
	fn name(&self, index: usize) -> String {
		let Some(row) = self.rows.get(index) else { return String::new() };
		let state = if row.checked { &self.checked_word } else { &self.unchecked_word };
		row.cells
			.iter()
			.filter(|cell| !cell.is_empty())
			.chain(std::iter::once(state))
			.cloned()
			.collect::<Vec<_>>()
			.join(", ")
	}
}

#[derive(Clone)]
pub struct Checklist {
	panel: Panel,
	state: Rc<RefCell<State>>,
}

impl Checklist {
	/// Create it right after its label, which names it.
	pub fn new(parent: &dyn WxWidget, checked_word: &str, unchecked_word: &str) -> Self {
		let panel = Panel::builder(parent).build();
		panel.set_style_raw(panel.get_style_raw() | WANTS_CHARS);
		panel.set_background_style(BackgroundStyle::Paint);
		let state = Rc::new(RefCell::new(State {
			rows: Vec::new(),
			current: 0,
			top: 0,
			reorderable: false,
			checked_word: checked_word.to_owned(),
			unchecked_word: unchecked_word.to_owned(),
			on_change: None,
		}));
		let list = Self { panel, state };
		#[cfg(target_os = "macos")]
		panel.set_accessibility_label(&list.label());
		#[cfg(target_os = "windows")]
		panel.set_accessible(Accessible::new(&panel, accessible::ChecklistAccessible::new(list.clone())));
		list.set_visible_rows(8);
		list.bind();
		list
	}

	#[must_use]
	pub const fn panel(&self) -> Panel {
		self.panel
	}

	/// Ctrl+Up and Ctrl+Down move the current row.
	pub fn set_reorderable(&self, reorderable: bool) {
		self.state.borrow_mut().reorderable = reorderable;
	}

	pub fn set_visible_rows(&self, rows: i32) {
		self.panel.set_min_size(Size::new(self.panel.get_char_width() * 40, self.row_height() * rows));
	}

	pub fn on_change(&self, callback: impl Fn(ChecklistChange) + 'static) {
		self.state.borrow_mut().on_change = Some(Rc::new(callback));
	}

	#[must_use]
	pub fn current(&self) -> Option<usize> {
		let state = self.state.borrow();
		(!state.rows.is_empty()).then_some(state.current)
	}

	/// Replaces the rows and makes `current` the current one, without telling `on_change`.
	pub fn set_rows(&self, rows: Vec<ChecklistRow>, current: usize) {
		{
			let mut state = self.state.borrow_mut();
			state.current = current.min(rows.len().saturating_sub(1));
			state.rows = rows;
		}
		self.scroll_to_current();
		self.panel.refresh(false, None);
		#[cfg(target_os = "windows")]
		accessible::rows_replaced(self);
	}

	#[cfg(any(target_os = "windows", target_os = "macos"))]
	fn label(&self) -> String {
		self.panel
			.get_prev_sibling()
			.and_then(|label| label.get_label())
			.map(|label| label.replace('&', "").trim_end_matches(':').trim().to_owned())
			.unwrap_or_default()
	}

	fn row_height(&self) -> i32 {
		self.panel.get_char_height() + 8
	}

	fn visible_rows(&self) -> usize {
		usize::try_from(self.panel.get_client_size().height / self.row_height()).unwrap_or(0).max(1)
	}

	fn scroll_to_current(&self) {
		let visible = self.visible_rows();
		let mut state = self.state.borrow_mut();
		if state.current < state.top {
			state.top = state.current;
		} else if state.current >= state.top + visible {
			state.top = state.current + 1 - visible;
		}
		state.top = state.top.min(state.rows.len().saturating_sub(visible));
	}

	fn notify(&self, change: ChecklistChange) {
		let callback = self.state.borrow().on_change.clone();
		if let Some(callback) = callback {
			callback(change);
		}
	}

	fn move_to(&self, index: usize) {
		{
			let mut state = self.state.borrow_mut();
			if state.rows.is_empty() {
				return;
			}
			let index = index.min(state.rows.len() - 1);
			if index == state.current {
				return;
			}
			state.current = index;
		}
		self.scroll_to_current();
		self.panel.refresh(false, None);
		#[cfg(target_os = "windows")]
		accessible::current_changed(self);
		self.notify(ChecklistChange::Current(self.state.borrow().current));
	}

	fn toggle(&self) {
		let index = {
			let mut state = self.state.borrow_mut();
			let current = state.current;
			let Some(row) = state.rows.get_mut(current) else { return };
			row.checked = !row.checked;
			current
		};
		self.panel.refresh(false, None);
		#[cfg(target_os = "windows")]
		accessible::row_renamed(self, index);
		self.notify(ChecklistChange::Toggled(index));
	}

	fn shift(&self, down: bool) {
		let (from, to) = {
			let mut state = self.state.borrow_mut();
			let from = state.current;
			let to = if down { from + 1 } else { from.wrapping_sub(1) };
			if !state.reorderable || to >= state.rows.len() {
				return;
			}
			state.rows.swap(from, to);
			state.current = to;
			(from, to)
		};
		self.scroll_to_current();
		self.panel.refresh(false, None);
		#[cfg(target_os = "windows")]
		accessible::rows_replaced(self);
		self.notify(ChecklistChange::Moved { from, to });
	}

	fn on_key(&self, event: &wxdragon::event::KeyboardEvent) -> bool {
		let Some(key) = event.get_key_code() else { return false };
		let (current, count) = {
			let state = self.state.borrow();
			(state.current, state.rows.len())
		};
		let page = self.visible_rows();
		match key {
			WXK_UP if event.control_down() => self.shift(false),
			WXK_DOWN if event.control_down() => self.shift(true),
			WXK_UP | WXK_NUMPAD_UP => self.move_to(current.saturating_sub(1)),
			WXK_DOWN | WXK_NUMPAD_DOWN => self.move_to(current + 1),
			WXK_HOME | WXK_NUMPAD_HOME => self.move_to(0),
			WXK_END | WXK_NUMPAD_END => self.move_to(count.saturating_sub(1)),
			WXK_PAGEUP | WXK_NUMPAD_PAGEUP => self.move_to(current.saturating_sub(page)),
			WXK_PAGEDOWN | WXK_NUMPAD_PAGEDOWN => self.move_to(current + page),
			WXK_SPACE if !event.control_down() && !event.alt_down() => self.toggle(),
			// The control wants every key, so it passes Tab on itself, as a standard control does.
			WXK_TAB if !event.control_down() => {
				self.panel.navigate(!event.shift_down());
			}
			_ => return false,
		}
		true
	}

	fn bind(&self) {
		let list = self.clone();
		self.panel.on_paint(move |_| list.draw());
		let list = self.clone();
		self.panel.on_key_down(move |event| {
			if let WindowEventData::Keyboard(key) = &event {
				let handled = list.on_key(key);
				key.event.skip(!handled);
			}
		});
		let list = self.clone();
		self.panel.on_mouse_left_down(move |event| {
			list.panel.set_focus();
			if let WindowEventData::MouseButton(mouse) = &event
				&& let Some(point) = mouse.get_position()
			{
				let top = list.state.borrow().top;
				list.move_to(top + usize::try_from(point.y / list.row_height()).unwrap_or(0));
			}
			event.skip(true);
		});
		let list = self.clone();
		self.panel.on_mouse_left_double_click(move |_| list.toggle());
		let list = self.clone();
		self.panel.on_mouse_wheel(move |event| {
			if let WindowEventData::MouseButton(mouse) = &event {
				let mut state = list.state.borrow_mut();
				let visible = list.visible_rows();
				let last_top = state.rows.len().saturating_sub(visible);
				state.top = if mouse.event.get_wheel_rotation() > 0 {
					state.top.saturating_sub(3)
				} else {
					(state.top + 3).min(last_top)
				};
				drop(state);
				list.panel.refresh(false, None);
			}
		});
		let list = self.clone();
		self.panel.on_set_focus(move |event| {
			list.panel.refresh(false, None);
			#[cfg(target_os = "windows")]
			accessible::focused(&list);
			event.skip(true);
		});
		let list = self.clone();
		self.panel.on_kill_focus(move |event| {
			list.panel.refresh(false, None);
			event.skip(true);
		});
		let list = self.clone();
		self.panel.on_size(move |event| {
			list.scroll_to_current();
			list.panel.refresh(false, None);
			event.skip(true);
		});
	}

	fn draw(&self) {
		let dc = AutoBufferedPaintDC::new(&self.panel);
		let background = SystemSettings::get_colour(SystemColour::ListBox);
		let text = SystemSettings::get_colour(SystemColour::ListBoxText);
		let highlight = SystemSettings::get_colour(SystemColour::Highlight);
		let highlight_text = SystemSettings::get_colour(SystemColour::HighlightText);
		let size = self.panel.get_client_size();
		dc.set_pen(background, 1, PenStyle::Solid);
		dc.set_brush(background, BrushStyle::Solid);
		dc.draw_rectangle(0, 0, size.width, size.height);
		if let Some(font) = self.panel.get_font() {
			dc.set_font(&font);
		}
		let height = self.row_height();
		let char_height = self.panel.get_char_height();
		let box_size = char_height - 2;
		let focused = self.panel.has_focus();
		let state = self.state.borrow();
		let columns = state.rows.iter().map(|row| row.cells.len()).max().unwrap_or(1).max(1);
		let text_left = 12 + box_size;
		let column_width = (size.width - text_left) / i32::try_from(columns).unwrap_or(1);
		let shown = state.top..state.rows.len().min(state.top + self.visible_rows() + 1);
		for (index, row) in shown.clone().zip(&state.rows[shown]) {
			let top = i32::try_from(index - state.top).unwrap_or(0) * height;
			let current = index == state.current && focused;
			let fill = if current { highlight } else { background };
			let ink = if current { highlight_text } else { text };
			dc.set_pen(fill, 1, PenStyle::Solid);
			dc.set_brush(fill, BrushStyle::Solid);
			dc.draw_rectangle(0, top, size.width, height);
			dc.set_pen(ink, 1, PenStyle::Solid);
			dc.set_brush(if row.checked { ink } else { fill }, BrushStyle::Solid);
			dc.draw_rectangle(6, top + (height - box_size) / 2, box_size, box_size);
			dc.set_text_foreground(ink);
			for (column, cell) in row.cells.iter().enumerate() {
				let left = text_left + i32::try_from(column).unwrap_or(0) * column_width;
				dc.set_clipping_region(left, top, column_width - 6, height);
				dc.draw_text(cell, left, top + 4);
				dc.destroy_clipping_region();
			}
		}
	}
}
