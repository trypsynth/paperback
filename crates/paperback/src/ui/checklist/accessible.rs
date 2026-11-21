//! Microsoft Active Accessibility for the checklist, as wxkit's `PartsAccessible` answers it, since wxWidgets can't describe a window that draws itself on Windows: the window is a list, and each row a list item.

use wxdragon::{
	accessible::{AccObjectType, AccRole, AccState, AccStatus, AccessibleImpl},
	prelude::*,
};

use super::Checklist;

const EVENT_OBJECT_REORDER: u32 = 0x8004;
const EVENT_OBJECT_FOCUS: u32 = 0x8005;
const EVENT_OBJECT_SELECTION: u32 = 0x8006;
const EVENT_OBJECT_NAMECHANGE: u32 = 0x800C;

pub(super) struct ChecklistAccessible {
	list: Checklist,
}

impl ChecklistAccessible {
	pub(super) const fn new(list: Checklist) -> Self {
		Self { list }
	}

	/// The row a child id stands for; ids count from 1, since 0 is the list itself.
	fn row(&self, child_id: i32) -> Option<usize> {
		let index = usize::try_from(child_id).ok()?.checked_sub(1)?;
		(index < self.list.state.try_borrow().ok()?.rows.len()).then_some(index)
	}

	fn bounds(&self, index: usize) -> Option<Rect> {
		let top = self.list.state.try_borrow().ok()?.top;
		let height = self.list.row_height();
		let y = (i32::try_from(index).ok()? - i32::try_from(top).ok()?) * height;
		let corner = self.list.panel.client_to_screen(Point::new(0, y));
		Some(Rect::new(corner.x, corner.y, self.list.panel.get_client_size().width, height))
	}
}

impl AccessibleImpl for ChecklistAccessible {
	fn get_child_count(&self) -> (AccStatus, i32) {
		let count = self.list.state.try_borrow().map_or(0, |state| state.rows.len());
		(AccStatus::Ok, i32::try_from(count).unwrap_or(i32::MAX))
	}

	fn get_child(&self, child_id: i32) -> (AccStatus, Option<Accessible>) {
		// A row is a simple element, answered by this object, rather than an object of its own.
		if self.row(child_id).is_some() { (AccStatus::Ok, None) } else { (AccStatus::NotImplemented, None) }
	}

	fn get_role(&self, child_id: i32) -> (AccStatus, AccRole) {
		if child_id == 0 { (AccStatus::Ok, AccRole::List) } else { (AccStatus::Ok, AccRole::ListItem) }
	}

	fn get_name(&self, child_id: i32) -> (AccStatus, Option<String>) {
		if child_id == 0 {
			let label = self.list.label();
			return if label.is_empty() { (AccStatus::NotImplemented, None) } else { (AccStatus::Ok, Some(label)) };
		}
		match (self.row(child_id), self.list.state.try_borrow()) {
			(Some(index), Ok(state)) => (AccStatus::Ok, Some(state.name(index))),
			_ => (AccStatus::InvalidArg, None),
		}
	}

	fn get_state(&self, child_id: i32) -> (AccStatus, AccState) {
		let Some(index) = self.row(child_id) else { return (AccStatus::NotImplemented, AccState::empty()) };
		let Ok(state) = self.list.state.try_borrow() else { return (AccStatus::Fail, AccState::empty()) };
		let mut flags = AccState::FOCUSABLE | AccState::SELECTABLE;
		if index == state.current {
			flags |= AccState::SELECTED;
			if self.list.panel.has_focus() {
				flags |= AccState::FOCUSED;
			}
		}
		if index < state.top || index >= state.top + self.list.visible_rows() {
			flags |= AccState::OFFSCREEN;
		}
		(AccStatus::Ok, flags)
	}

	fn get_focus(&self) -> (AccStatus, i32, Option<Accessible>) {
		if !self.list.panel.has_focus() {
			return (AccStatus::False, 0, None);
		}
		let current = self.list.current().and_then(|index| i32::try_from(index + 1).ok()).unwrap_or(0);
		(AccStatus::Ok, current, None)
	}

	fn get_location(&self, child_id: i32) -> (AccStatus, Rect) {
		match self.row(child_id).and_then(|index| self.bounds(index)) {
			Some(bounds) => (AccStatus::Ok, bounds),
			None => (AccStatus::NotImplemented, Rect::default()),
		}
	}

	fn hit_test(&self, point: Point) -> (AccStatus, i32, Option<Accessible>) {
		let local = self.list.panel.screen_to_client(point);
		let size = self.list.panel.get_client_size();
		let Ok(state) = self.list.state.try_borrow() else { return (AccStatus::Fail, 0, None) };
		if local.x < 0 || local.y < 0 || local.x >= size.width || local.y >= size.height {
			return (AccStatus::False, 0, None);
		}
		let index = state.top + usize::try_from(local.y / self.list.row_height()).unwrap_or(0);
		if index >= state.rows.len() {
			return (AccStatus::Ok, 0, None);
		}
		(AccStatus::Ok, i32::try_from(index + 1).unwrap_or(0), None)
	}
}

fn send(list: &Checklist, event: u32, child_id: usize) {
	Accessible::notify_event(event, &list.panel, AccObjectType::Client, i32::try_from(child_id).unwrap_or(0));
}

/// Screen readers follow the focus, but can't see where it is inside a window that draws itself, so the current row is announced as focused.
pub(super) fn focused(list: &Checklist) {
	if let Some(index) = list.current() {
		send(list, EVENT_OBJECT_FOCUS, index + 1);
		send(list, EVENT_OBJECT_SELECTION, index + 1);
	}
}

pub(super) fn current_changed(list: &Checklist) {
	if list.panel.has_focus() {
		focused(list);
	}
}

pub(super) fn row_renamed(list: &Checklist, index: usize) {
	send(list, EVENT_OBJECT_NAMECHANGE, index + 1);
}

pub(super) fn rows_replaced(list: &Checklist) {
	send(list, EVENT_OBJECT_REORDER, 0);
	current_changed(list);
}
