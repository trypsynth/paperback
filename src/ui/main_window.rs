//! Main window implementation for Paperback.

use std::{
	cell::Cell,
	env,
	path::Path,
	process,
	rc::Rc,
	sync::{
		Mutex,
		atomic::{AtomicI32, AtomicI64, Ordering},
	},
};

use wxdragon::{prelude::*, timer::Timer, translations::translate as t};

use super::{
	dialogs,
	document_manager::DocumentManager,
	find::{self, FindDialogState},
	help::{self, MAIN_WINDOW_PTR},
	menu, menu_ids,
	navigation::{self, MarkerNavTarget},
	status, tray,
};
use crate::{
	config::ConfigManager,
	live_region,
	parser::{build_file_filter_string, parser_supports_extension},
	translation_manager::TranslationManager,
	ui_types::BookmarkFilterType,
};

const KEY_DELETE: i32 = 127;
const KEY_NUMPAD_DELETE: i32 = 330;

pub static SLEEP_TIMER_START_MS: AtomicI64 = AtomicI64::new(0);
pub static SLEEP_TIMER_DURATION_MINUTES: AtomicI32 = AtomicI32::new(0);

pub struct MainWindow {
	frame: Frame,
	doc_manager: Rc<Mutex<DocumentManager>>,
	_config: Rc<Mutex<ConfigManager>>,
	_tray_state: Rc<Mutex<Option<tray::TrayState>>>,
	_live_region_label: StaticText,
	_find_dialog: Rc<Mutex<Option<FindDialogState>>>,
}

impl MainWindow {
	pub fn new(config: Rc<Mutex<ConfigManager>>) -> Self {
		let app_title = t("Paperback");
		let frame = Frame::builder().with_title(&app_title).with_size(Size::new(800, 600)).build();
		MAIN_WINDOW_PTR.store(frame.handle_ptr() as usize, Ordering::SeqCst);
		frame.create_status_bar(1, 0, -1, "statusbar");
		frame.set_status_text(&t("Ready"), 0);
		let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
		frame.set_menu_bar(menu_bar);
		let panel = Panel::builder(&frame).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		let live_region_label = StaticText::builder(&panel).with_label("").with_size(Size::new(0, 0)).build();
		live_region_label.show(false);
		let _ = live_region::set_live_region(&live_region_label);
		let notebook = Notebook::builder(&panel).with_style(NotebookStyle::Top).build();
		#[cfg(windows)]
		notebook.msw_disable_composited();
		sizer.add(&notebook, 1, SizerFlag::Expand | SizerFlag::All, 0);
		panel.set_sizer(sizer, true);
		let doc_manager =
			Rc::new(Mutex::new(DocumentManager::new(frame, notebook, Rc::clone(&config), live_region_label)));
		let find_dialog = Rc::new(Mutex::new(None));
		Self::bind_menu_events(
			&frame,
			Rc::clone(&doc_manager),
			Rc::clone(&config),
			Rc::clone(&find_dialog),
			live_region_label,
		);
		let dm = Rc::clone(&doc_manager);
		let frame_copy = frame;
		let notebook = *doc_manager.lock().unwrap().notebook();
		notebook.on_page_changed(move |_event| {
			let dm_ref = match dm.try_lock() {
				Ok(dm_ref) => dm_ref,
				Err(_) => return,
			};
			update_title_from_manager(&frame_copy, &dm_ref);
		});
		let dm = Rc::clone(&doc_manager);
		let frame_copy = frame;
		notebook.on_key_down(move |event| {
			if let wxdragon::event::WindowEventData::Keyboard(key_event) = &event {
				if let Some(key) = key_event.get_key_code() {
					if key == KEY_DELETE || key == KEY_NUMPAD_DELETE {
						let mut dm = dm.lock().unwrap();
						if let Some(index) = dm.active_tab_index() {
							dm.close_document(index);
						}
						update_title_from_manager(&frame_copy, &dm);
						if dm.tab_count() == 0 {
							dm.notebook().set_focus();
						} else {
							dm.restore_focus();
						}
						event.skip(false);
						return;
					}
				}
			}
			event.skip(true);
		});
		let tray_state = Rc::new(Mutex::new(None));
		tray::bind_tray_events(frame, &doc_manager, &config, &tray_state);
		{
			let dm_for_close = Rc::clone(&doc_manager);
			let config_for_close = Rc::clone(&config);
			frame.on_close(move |_event| {
				let dm = dm_for_close.lock().unwrap();
				if let Some(tab) = dm.active_tab() {
					let path = tab.file_path.to_string_lossy();
					let cfg = config_for_close.lock().unwrap();
					cfg.set_app_string("active_document", &path);
					cfg.flush();
				}
				dm.save_all_positions();
			});
		}
		Self::schedule_restore_documents(frame, Rc::clone(&doc_manager), Rc::clone(&config));
		Self {
			frame,
			doc_manager,
			_config: config,
			_tray_state: tray_state,
			_live_region_label: live_region_label,
			_find_dialog: find_dialog,
		}
	}

	pub fn show(&self) {
		self.frame.show(true);
		self.frame.centre();
	}

	pub fn check_for_updates(&self, silent: bool) {
		help::run_update_check(silent);
	}

	pub fn open_file(&self, path: &Path) -> bool {
		if !self.ensure_parser_ready(path) {
			return false;
		}
		let result = self.doc_manager.lock().unwrap().open_file(Rc::clone(&self.doc_manager), path);
		if result {
			self.update_title();
			self.update_recent_documents_menu();
			self.doc_manager.lock().unwrap().restore_focus();
		}
		result
	}

	fn update_title(&self) {
		let dm = match self.doc_manager.try_lock() {
			Ok(dm) => dm,
			Err(_) => return,
		};
		if dm.tab_count() == 0 {
			self.frame.set_title(&t("Paperback"));
			self.frame.set_status_text(&t("Ready"), 0);
			return;
		}
		if let Some(tab) = dm.active_tab() {
			let title = tab.session.title();
			let display_title = if title.is_empty() {
				tab.file_path.file_name().map_or_else(|| t("Untitled"), |s| s.to_string_lossy().to_string())
			} else {
				title
			};
			let template = t("Paperback - {}");
			self.frame.set_title(&template.replace("{}", &display_title));
			let chars_label = t("{} chars");
			self.frame.set_status_text(&chars_label.replace("{}", &tab.session.content().len().to_string()), 0);
		}
	}

	/// Get the frame
	pub const fn frame(&self) -> &Frame {
		&self.frame
	}

	fn ensure_parser_ready(&self, path: &Path) -> bool {
		ensure_parser_ready_for_path(&self.frame, path, &self._config)
	}

	fn update_recent_documents_menu(&self) {
		let menu_bar = menu::create_menu_bar(&self._config.lock().unwrap());
		self.frame.set_menu_bar(menu_bar);
	}

	fn schedule_restore_documents(
		frame: Frame,
		doc_manager: Rc<Mutex<DocumentManager>>,
		config: Rc<Mutex<ConfigManager>>,
	) {
		let restore = config.lock().unwrap().get_app_bool("restore_previous_documents", true);
		if !restore {
			return;
		}
		#[derive(Default)]
		struct RestoreState {
			restored: bool,
			closing: bool,
		}
		let state = Rc::new(Mutex::new(RestoreState::default()));
		let state_for_close = Rc::clone(&state);
		frame.on_close(move |_event| {
			state_for_close.lock().unwrap().closing = true;
		});
		let state_for_destroy = Rc::clone(&state);
		frame.on_destroy(move |_event| {
			state_for_destroy.lock().unwrap().closing = true;
		});
		let state_for_idle = Rc::clone(&state);
		frame.on_idle(move |_event| {
			let mut state = state_for_idle.lock().unwrap();
			if state.restored || state.closing {
				return;
			}
			state.restored = true;
			drop(state);
			let paths = config.lock().unwrap().get_opened_documents_existing();
			for path in paths {
				let path = Path::new(&path);
				if !ensure_parser_ready_for_path(&frame, path, &config) {
					continue;
				}
				let _ = doc_manager.lock().unwrap().open_file(Rc::clone(&doc_manager), path);
			}
			let dm_ref = doc_manager.lock().unwrap();
			update_title_from_manager(&frame, &dm_ref);
			let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
			frame.set_menu_bar(menu_bar);
			dm_ref.restore_focus();
		});
	}

	fn handle_open(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>, config: &Rc<Mutex<ConfigManager>>) {
		let wildcard = build_file_filter_string();
		let dialog_title = t("Open Document");
		let dialog = FileDialog::builder(frame)
			.with_message(&dialog_title)
			.with_wildcard(&wildcard)
			.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
			.build();
		if dialog.show_modal() == wxdragon::id::ID_OK {
			if let Some(path) = dialog.get_path() {
				let path = std::path::Path::new(&path);
				if !ensure_parser_ready_for_path(frame, path, config) {
					return;
				}
				if doc_manager.lock().unwrap().open_file(Rc::clone(doc_manager), path) {
					let dm_ref = match doc_manager.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					update_title_from_manager(frame, &dm_ref);
					dm_ref.restore_focus();
				}
			}
		}
	}

	#[allow(clippy::too_many_lines)]
	fn bind_menu_events(
		frame: &Frame,
		doc_manager: Rc<Mutex<DocumentManager>>,
		config: Rc<Mutex<ConfigManager>>,
		find_dialog: Rc<Mutex<Option<FindDialogState>>>,
		live_region_label: StaticText,
	) {
		let frame_copy = *frame;
		let dm = Rc::clone(&doc_manager);
		let config = Rc::clone(&config);
		let find_dialog = Rc::clone(&find_dialog);
		let sleep_timer = Rc::new(Timer::new(frame));
		let sleep_timer_running = Rc::new(Cell::new(false));
		let sleep_timer_start_time = Rc::new(Cell::new(0i64));
		let sleep_timer_duration_minutes = Rc::new(Cell::new(0i32));
		let sleep_timer_for_tick = Rc::clone(&sleep_timer);
		let sleep_timer_running_for_tick = Rc::clone(&sleep_timer_running);
		let frame_for_timer = *frame;
		let dm_for_timer = Rc::clone(&doc_manager);
		let config_for_timer = Rc::clone(&config);
		sleep_timer.on_tick(move |_| {
			sleep_timer_running_for_tick.set(false);
			sleep_timer_for_tick.stop();
			SLEEP_TIMER_START_MS.store(0, Ordering::SeqCst);
			SLEEP_TIMER_DURATION_MINUTES.store(0, Ordering::SeqCst);
			{
				let dm = dm_for_timer.lock().unwrap();
				let cfg = config_for_timer.lock().unwrap();
				for i in 0..dm.tab_count() {
					if let Some(tab) = dm.get_tab(i) {
						let current_pos = tab.text_ctrl.get_insertion_point();
						let path_str = tab.file_path.to_string_lossy();
						cfg.set_document_position(&path_str, current_pos);
					}
				}
				cfg.flush();
			}
			frame_for_timer.close(true);
		});
		let status_update_timer = Rc::new(Timer::new(frame));
		let sleep_timer_running_for_status = Rc::clone(&sleep_timer_running);
		let sleep_timer_start_for_status = Rc::clone(&sleep_timer_start_time);
		let sleep_timer_duration_for_status = Rc::clone(&sleep_timer_duration_minutes);
		let dm_for_status = Rc::clone(&doc_manager);
		let frame_for_status = *frame;
		status_update_timer.on_tick(move |_| {
			if !sleep_timer_running_for_status.get() {
				return;
			}
			let dm = match dm_for_status.try_lock() {
				Ok(dm) => dm,
				Err(_) => return,
			};
			status::update_status_bar_with_sleep_timer(
				&frame_for_status,
				&dm,
				sleep_timer_start_for_status.get(),
				sleep_timer_duration_for_status.get(),
			);
		});
		status_update_timer.start(1000, false);
		let sleep_timer_for_menu = Rc::clone(&sleep_timer);
		let sleep_timer_running_for_menu = Rc::clone(&sleep_timer_running);
		let sleep_timer_start_for_menu = Rc::clone(&sleep_timer_start_time);
		let sleep_timer_duration_for_menu = Rc::clone(&sleep_timer_duration_minutes);
		frame.on_menu(move |event| {
			let id = event.get_id();
			match id {
				menu_ids::OPEN => {
					Self::handle_open(&frame_copy, &dm, &config);
				}
				menu_ids::CLOSE => {
					let mut dm = dm.lock().unwrap();
					if let Some(index) = dm.active_tab_index() {
						dm.close_document(index);
					}
					update_title_from_manager(&frame_copy, &dm);
					if dm.tab_count() == 0 {
						dm.notebook().set_focus();
					} else {
						dm.restore_focus();
					}
				}
				menu_ids::CLOSE_ALL => {
					let mut dm = dm.lock().unwrap();
					dm.close_all_documents();
					update_title_from_manager(&frame_copy, &dm);
					dm.notebook().set_focus();
				}
				menu_ids::EXIT => {
					dm.lock().unwrap().save_all_positions();
					process::exit(0);
				}
				menu_ids::FIND => {
					find::show_find_dialog(&frame_copy, &dm, &config, &find_dialog, live_region_label);
				}
				menu_ids::FIND_NEXT => {
					find::handle_find_action(&frame_copy, &dm, &config, &find_dialog, live_region_label, true);
				}
				menu_ids::FIND_PREVIOUS => {
					find::handle_find_action(&frame_copy, &dm, &config, &find_dialog, live_region_label, false);
				}
				menu_ids::GO_TO_LINE => {
					let mut dm_guard = dm.lock().unwrap();
					let Some(tab) = dm_guard.active_tab_mut() else {
						return;
					};
					let current_pos = tab.text_ctrl.get_insertion_point();
					if let Some(line) = dialogs::show_go_to_line_dialog(&frame_copy, &tab.session, current_pos) {
						let target_pos = tab.session.position_from_line(line);
						tab.text_ctrl.set_focus();
						tab.text_ctrl.set_insertion_point(target_pos);
						tab.text_ctrl.show_position(target_pos);
						tab.session.check_and_record_history(target_pos);
						let (history, history_index) = tab.session.get_history();
						let path_str = tab.file_path.to_string_lossy();
						let cfg = config.lock().unwrap();
						cfg.set_navigation_history(&path_str, history, history_index);
					}
				}
				menu_ids::GO_TO_PAGE => {
					let mut dm_guard = dm.lock().unwrap();
					let Some(tab) = dm_guard.active_tab_mut() else {
						return;
					};
					if tab.session.page_count() == 0 {
						live_region::announce(live_region_label, &t("No pages."));
						return;
					}
					let current_pos = tab.text_ctrl.get_insertion_point();
					let current_page = tab.session.current_page(current_pos);
					if let Some(target_pos) = dialogs::show_go_to_page_dialog(&frame_copy, &tab.session, current_page) {
						tab.text_ctrl.set_focus();
						tab.text_ctrl.set_insertion_point(target_pos);
						tab.text_ctrl.show_position(target_pos);
						tab.session.check_and_record_history(target_pos);
						let (history, history_index) = tab.session.get_history();
						let path_str = tab.file_path.to_string_lossy();
						let cfg = config.lock().unwrap();
						cfg.set_navigation_history(&path_str, history, history_index);
					}
				}
				menu_ids::GO_TO_PERCENT => {
					let mut dm_guard = dm.lock().unwrap();
					let Some(tab) = dm_guard.active_tab_mut() else {
						return;
					};
					let current_pos = tab.text_ctrl.get_insertion_point();
					if let Some(target_pos) = dialogs::show_go_to_percent_dialog(&frame_copy, &tab.session, current_pos)
					{
						tab.text_ctrl.set_focus();
						tab.text_ctrl.set_insertion_point(target_pos);
						tab.text_ctrl.show_position(target_pos);
						tab.session.check_and_record_history(target_pos);
						let (history, history_index) = tab.session.get_history();
						let path_str = tab.file_path.to_string_lossy();
						let cfg = config.lock().unwrap();
						cfg.set_navigation_history(&path_str, history, history_index);
					}
				}
				menu_ids::GO_BACK => {
					navigation::handle_history_navigation(&dm, &config, live_region_label, false);
				}
				menu_ids::GO_FORWARD => {
					navigation::handle_history_navigation(&dm, &config, live_region_label, true);
				}
				menu_ids::PREVIOUS_SECTION => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Section,
						false,
					);
				}
				menu_ids::NEXT_SECTION => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Section,
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(0),
						false,
					);
				}
				menu_ids::NEXT_HEADING => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(0),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_1 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(1),
						false,
					);
				}
				menu_ids::NEXT_HEADING_1 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(1),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_2 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(2),
						false,
					);
				}
				menu_ids::NEXT_HEADING_2 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(2),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_3 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(3),
						false,
					);
				}
				menu_ids::NEXT_HEADING_3 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(3),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_4 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(4),
						false,
					);
				}
				menu_ids::NEXT_HEADING_4 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(4),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_5 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(5),
						false,
					);
				}
				menu_ids::NEXT_HEADING_5 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(5),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_6 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(6),
						false,
					);
				}
				menu_ids::NEXT_HEADING_6 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(6),
						true,
					);
				}
				menu_ids::PREVIOUS_PAGE => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Page, false);
				}
				menu_ids::NEXT_PAGE => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Page, true);
				}
				menu_ids::PREVIOUS_BOOKMARK => {
					navigation::handle_bookmark_navigation(&dm, &config, live_region_label, false, false);
				}
				menu_ids::NEXT_BOOKMARK => {
					navigation::handle_bookmark_navigation(&dm, &config, live_region_label, true, false);
				}
				menu_ids::PREVIOUS_NOTE => {
					navigation::handle_bookmark_navigation(&dm, &config, live_region_label, false, true);
				}
				menu_ids::NEXT_NOTE => {
					navigation::handle_bookmark_navigation(&dm, &config, live_region_label, true, true);
				}
				menu_ids::JUMP_TO_ALL_BOOKMARKS => {
					navigation::handle_bookmark_dialog(
						&frame_copy,
						&dm,
						&config,
						live_region_label,
						BookmarkFilterType::All,
					);
				}
				menu_ids::JUMP_TO_BOOKMARKS_ONLY => {
					navigation::handle_bookmark_dialog(
						&frame_copy,
						&dm,
						&config,
						live_region_label,
						BookmarkFilterType::BookmarksOnly,
					);
				}
				menu_ids::JUMP_TO_NOTES_ONLY => {
					navigation::handle_bookmark_dialog(
						&frame_copy,
						&dm,
						&config,
						live_region_label,
						BookmarkFilterType::NotesOnly,
					);
				}
				menu_ids::TOGGLE_BOOKMARK => {
					navigation::handle_toggle_bookmark(&dm, &config, live_region_label);
				}
				menu_ids::BOOKMARK_WITH_NOTE => {
					navigation::handle_bookmark_with_note(&frame_copy, &dm, &config, live_region_label);
				}
				menu_ids::VIEW_NOTE_TEXT => {
					navigation::handle_view_note_text(&frame_copy, &dm, &config);
				}
				menu_ids::PREVIOUS_LINK => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Link, false);
				}
				menu_ids::NEXT_LINK => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Link, true);
				}
				menu_ids::PREVIOUS_TABLE => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Table,
						false,
					);
				}
				menu_ids::NEXT_TABLE => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Table, true);
				}
				menu_ids::PREVIOUS_SEPARATOR => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Separator,
						false,
					);
				}
				menu_ids::NEXT_SEPARATOR => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Separator,
						true,
					);
				}
				menu_ids::PREVIOUS_LIST => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::List, false);
				}
				menu_ids::NEXT_LIST => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::List, true);
				}
				menu_ids::PREVIOUS_LIST_ITEM => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::ListItem,
						false,
					);
				}
				menu_ids::NEXT_LIST_ITEM => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::ListItem,
						true,
					);
				}
				menu_ids::EXPORT_TO_PLAIN_TEXT => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					let Some(tab) = dm_ref.active_tab() else {
						return;
					};
					let default_name =
						tab.file_path.file_stem().map_or_else(|| t("document"), |s| s.to_string_lossy().to_string());
					let default_file = format!("{default_name}.txt");
					let wildcard = t("Plain text files (*.txt)|*.txt|All files (*.*)|*.*");
					let dialog = FileDialog::builder(&frame_copy)
						.with_message(&t("Export document to plain text"))
						.with_default_file(&default_file)
						.with_wildcard(&wildcard)
						.with_style(FileDialogStyle::Save | FileDialogStyle::OverwritePrompt)
						.build();
					if dialog.show_modal() == wxdragon::id::ID_OK {
						if let Some(path) = dialog.get_path() {
							if tab.session.export_content(&path).is_err() {
								let dialog =
									MessageDialog::builder(&frame_copy, &t("Failed to export document."), &t("Error"))
										.with_style(
											MessageDialogStyle::OK
												| MessageDialogStyle::IconError | MessageDialogStyle::Centre,
										)
										.build();
								dialog.show_modal();
							}
						}
					}
				}
				menu_ids::EXPORT_DOCUMENT_DATA => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					let Some(tab) = dm_ref.active_tab() else {
						return;
					};
					let default_name =
						tab.file_path.file_stem().map_or_else(|| t("document"), |s| s.to_string_lossy().to_string());
					let default_file = format!("{default_name}.paperback");
					let wildcard = t("Paperback files (*.paperback)|*.paperback");
					let dialog = FileDialog::builder(&frame_copy)
						.with_message(&t("Export notes and bookmarks"))
						.with_default_file(&default_file)
						.with_wildcard(&wildcard)
						.with_style(FileDialogStyle::Save | FileDialogStyle::OverwritePrompt)
						.build();
					if dialog.show_modal() == wxdragon::id::ID_OK {
						if let Some(path) = dialog.get_path() {
							let path_str = tab.file_path.to_string_lossy();
							config.lock().unwrap().export_document_settings(&path_str, &path);
							let dialog = MessageDialog::builder(
								&frame_copy,
								&t("Notes and bookmarks exported successfully."),
								&t("Export Successful"),
							)
							.with_style(
								MessageDialogStyle::OK
									| MessageDialogStyle::IconInformation
									| MessageDialogStyle::Centre,
							)
							.build();
							dialog.show_modal();
						}
					}
				}
				menu_ids::IMPORT_DOCUMENT_DATA => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					let Some(tab) = dm_ref.active_tab() else {
						return;
					};
					let wildcard = t("Paperback files (*.paperback)|*.paperback");
					let dialog = FileDialog::builder(&frame_copy)
						.with_message(&t("Import notes and bookmarks"))
						.with_wildcard(&wildcard)
						.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
						.build();
					if dialog.show_modal() == wxdragon::id::ID_OK {
						if let Some(path) = dialog.get_path() {
							let path_str = tab.file_path.to_string_lossy();
							{
								let config = config.lock().unwrap();
								config.import_settings_from_file(&path_str, &path);
								let max_pos = tab.text_ctrl.get_last_position();
								let pos = config.get_validated_document_position(&path_str, max_pos);
								if pos >= 0 {
									tab.text_ctrl.set_insertion_point(pos);
									tab.text_ctrl.show_position(pos);
								}
							}
							let dialog = MessageDialog::builder(
								&frame_copy,
								&t("Notes and bookmarks imported successfully."),
								&t("Import Successful"),
							)
							.with_style(
								MessageDialogStyle::OK
									| MessageDialogStyle::IconInformation
									| MessageDialogStyle::Centre,
							)
							.build();
							dialog.show_modal();
						}
					}
				}
				menu_ids::WORD_COUNT => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					if let Some(tab) = dm_ref.active_tab() {
						let stats = tab.session.stats();
						let msg_template = t("The document contains {} words.");
						let msg = msg_template.replace("{}", &stats.word_count.to_string());
						let title = t("Word count");
						let dialog = MessageDialog::builder(&frame_copy, &msg, &title)
							.with_style(MessageDialogStyle::OK)
							.build();
						dialog.show_modal();
					}
				}
				menu_ids::DOCUMENT_INFO => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					if let Some(tab) = dm_ref.active_tab() {
						let stats = tab.session.stats();
						let title = tab.session.title();
						let author = tab.session.author();
						dialogs::show_document_info_dialog(&frame_copy, &tab.file_path, &title, &author, stats);
					}
				}
				menu_ids::TABLE_OF_CONTENTS => {
					let mut dm_guard = dm.lock().unwrap();
					if let Some(tab) = dm_guard.active_tab_mut() {
						let current_pos = tab.text_ctrl.get_insertion_point();
						let current_pos_usize = usize::try_from(current_pos).unwrap_or(0);
						let current_toc_offset = tab.session.handle().find_closest_toc_offset(current_pos_usize);
						if let Some(offset) = dialogs::show_toc_dialog(
							&frame_copy,
							&tab.session.handle().document().toc_items,
							i32::try_from(current_toc_offset).unwrap_or(i32::MAX),
						) {
							tab.text_ctrl.set_focus();
							tab.text_ctrl.set_insertion_point(i64::from(offset));
							tab.text_ctrl.show_position(i64::from(offset));
							tab.session.check_and_record_history(i64::from(offset));
							let (history, history_index) = tab.session.get_history();
							let path_str = tab.file_path.to_string_lossy();
							let cfg = config.lock().unwrap();
							cfg.set_navigation_history(&path_str, history, history_index);
						}
					}
				}
				menu_ids::ELEMENTS_LIST => {
					let mut dm_guard = dm.lock().unwrap();
					if let Some(tab) = dm_guard.active_tab_mut() {
						let current_pos = tab.text_ctrl.get_insertion_point();
						if let Some(offset) = dialogs::show_elements_dialog(&frame_copy, &tab.session, current_pos) {
							tab.text_ctrl.set_focus();
							tab.text_ctrl.set_insertion_point(offset);
							tab.text_ctrl.show_position(offset);
							tab.session.check_and_record_history(offset);
							let (history, history_index) = tab.session.get_history();
							let path_str = tab.file_path.to_string_lossy();
							let cfg = config.lock().unwrap();
							cfg.set_navigation_history(&path_str, history, history_index);
						}
					}
				}
				menu_ids::OPEN_IN_WEB_VIEW => {
					let dm_ref = match dm.try_lock() {
						Ok(dm_ref) => dm_ref,
						Err(_) => return,
					};
					let Some(tab) = dm_ref.active_tab() else {
						return;
					};
					let current_pos = tab.text_ctrl.get_insertion_point();
					let temp_dir = env::temp_dir().to_string_lossy().to_string();
					if let Some(target_path) = tab.session.webview_target_path(current_pos, &temp_dir) {
						let url = format!("file:///{}", target_path.replace('\\', "/"));
						dialogs::show_web_view_dialog(
							&frame_copy,
							&t("Web View"),
							&url,
							true,
							Some(Box::new(|url| {
								if url.to_lowercase().starts_with("http://")
									|| url.to_lowercase().starts_with("https://")
									|| url.to_lowercase().starts_with("mailto:")
								{
									wxdragon::utils::launch_default_browser(
										url,
										wxdragon::utils::BrowserLaunchFlags::Default,
									);
									false
								} else {
									true
								}
							})),
						);
					} else {
						let dialog = MessageDialog::builder(
							&frame_copy,
							&t("Could not determine content to display in Web View."),
							&t("Error"),
						)
						.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
						.build();
						dialog.show_modal();
					}
				}
				menu_ids::OPEN_CONTAINING_FOLDER => {
					help::handle_open_containing_folder(&frame_copy, &dm);
				}
				menu_ids::OPTIONS => {
					let current_language = TranslationManager::instance().lock().unwrap().current_language();
					let options = {
						let cfg = config.lock().unwrap();
						dialogs::show_options_dialog(&frame_copy, &cfg)
					};
					let Some(options) = options else {
						return;
					};
					let (old_word_wrap, old_compact_menu) = {
						let cfg = config.lock().unwrap();
						(cfg.get_app_bool("word_wrap", false), cfg.get_app_bool("compact_go_menu", true))
					};
					let cfg = config.lock().unwrap();
					cfg.set_app_bool("restore_previous_documents", options.restore_previous_documents);
					cfg.set_app_bool("word_wrap", options.word_wrap);
					cfg.set_app_bool("minimize_to_tray", options.minimize_to_tray);
					cfg.set_app_bool("start_maximized", options.start_maximized);
					cfg.set_app_bool("compact_go_menu", options.compact_go_menu);
					cfg.set_app_bool("navigation_wrap", options.navigation_wrap);
					cfg.set_app_bool("check_for_updates_on_startup", options.check_for_updates_on_startup);
					cfg.set_app_int("recent_documents_to_show", options.recent_documents_to_show);
					cfg.set_app_string("language", &options.language);
					cfg.flush();
					drop(cfg);
					if old_word_wrap != options.word_wrap {
						let mut dm_ref = dm.lock().unwrap();
						dm_ref.apply_word_wrap(options.word_wrap);
						dm_ref.restore_focus();
					}
					if current_language != options.language || old_compact_menu != options.compact_go_menu {
						if current_language != options.language {
							let _ = TranslationManager::instance().lock().unwrap().set_language(&options.language);
						}
						let dm_ref = dm.lock().unwrap();
						update_title_from_manager(&frame_copy, &dm_ref);
					}
					let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
					frame_copy.set_menu_bar(menu_bar);
				}
				menu_ids::SLEEP_TIMER => {
					if sleep_timer_running_for_menu.get() {
						sleep_timer_for_menu.stop();
						sleep_timer_running_for_menu.set(false);
						sleep_timer_start_for_menu.set(0);
						sleep_timer_duration_for_menu.set(0);
						SLEEP_TIMER_START_MS.store(0, Ordering::SeqCst);
						SLEEP_TIMER_DURATION_MINUTES.store(0, Ordering::SeqCst);
						let dm_ref = dm.lock().unwrap();
						update_title_from_manager(&frame_copy, &dm_ref);
						live_region::announce(live_region_label, &t("Sleep timer cancelled."));
						return;
					}
					let initial_duration = config.lock().unwrap().get_app_int("sleep_timer_duration", 30);
					if let Some(duration) = dialogs::show_sleep_timer_dialog(&frame_copy, initial_duration) {
						{
							let cfg = config.lock().unwrap();
							cfg.set_app_int("sleep_timer_duration", duration);
							cfg.flush();
						}
						let duration_ms = u64::try_from(duration).unwrap_or(0) * 60 * 1000;
						sleep_timer_for_menu.start(i32::try_from(duration_ms).unwrap_or(i32::MAX), true);
						sleep_timer_running_for_menu.set(true);
						let now = std::time::SystemTime::now()
							.duration_since(std::time::UNIX_EPOCH)
							.ok()
							.and_then(|d| i64::try_from(d.as_millis()).ok())
							.unwrap_or(0);
						sleep_timer_start_for_menu.set(now);
						sleep_timer_duration_for_menu.set(duration);
						SLEEP_TIMER_START_MS.store(now, Ordering::SeqCst);
						SLEEP_TIMER_DURATION_MINUTES.store(duration, Ordering::SeqCst);
						let msg = if duration == 1 {
							t("Sleep timer set for 1 minute.")
						} else {
							t("Sleep timer set for %d minutes.").replace("%d", &duration.to_string())
						};
						live_region::announce(live_region_label, &msg);
					}
				}
				menu_ids::ABOUT => {
					dialogs::show_about_dialog(&frame_copy);
				}
				menu_ids::VIEW_HELP_BROWSER => {
					help::handle_view_help_browser(&frame_copy);
				}
				menu_ids::VIEW_HELP_PAPERBACK => {
					help::handle_view_help_paperback(&frame_copy, &dm, &config);
				}
				menu_ids::CHECK_FOR_UPDATES => {
					help::run_update_check(false);
				}
				menu_ids::DONATE => {
					help::handle_donate(&frame_copy);
				}
				_ => {
					if (menu_ids::RECENT_DOCUMENT_BASE..=menu_ids::RECENT_DOCUMENT_MAX).contains(&id) {
						let doc_index = id - menu_ids::RECENT_DOCUMENT_BASE;
						let recent_docs = {
							let config_guard = config.lock().unwrap();
							menu::recent_documents_for_menu(&config_guard)
						};
						if let Ok(doc_index) = usize::try_from(doc_index) {
							if let Some(path) = recent_docs.get(doc_index) {
								let path = Path::new(path);
								if !ensure_parser_ready_for_path(&frame_copy, path, &config) {
									return;
								}
								if dm.lock().unwrap().open_file(Rc::clone(&dm), path) {
									let dm_ref = dm.lock().unwrap();
									update_title_from_manager(&frame_copy, &dm_ref);
									dm_ref.restore_focus();
									let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
									frame_copy.set_menu_bar(menu_bar);
								}
							}
						}
					} else if id == menu_ids::SHOW_ALL_DOCUMENTS {
						let has_documents = {
							let config_guard = config.lock().unwrap();
							!config_guard.get_all_documents().is_empty()
						};
						if !has_documents {
							live_region::announce(live_region_label, &t("No recent documents."));
							return;
						}
						let open_paths = dm.lock().unwrap().open_paths();
						let config_for_dialog = Rc::clone(&config);
						let selection = dialogs::show_all_documents_dialog(&frame_copy, config_for_dialog, open_paths);
						if let Some(path) = selection {
							let path = Path::new(&path);
							if !ensure_parser_ready_for_path(&frame_copy, path, &config) {
								return;
							}
							if dm.lock().unwrap().open_file(Rc::clone(&dm), path) {
								let dm_ref = dm.lock().unwrap();
								update_title_from_manager(&frame_copy, &dm_ref);
								dm_ref.restore_focus();
								let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
								frame_copy.set_menu_bar(menu_bar);
							}
						} else {
							let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
							frame_copy.set_menu_bar(menu_bar);
						}
					}
				}
			}
		});
	}
}

fn ensure_parser_ready_for_path(frame: &Frame, path: &Path, config: &Rc<Mutex<ConfigManager>>) -> bool {
	let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();
	if extension.is_empty() || parser_supports_extension(extension) {
		return true;
	}
	let cfg = config.lock().unwrap();
	ensure_parser_for_unknown_file(frame, path, &cfg)
}

fn ensure_parser_for_unknown_file(parent: &Frame, path: &Path, config: &ConfigManager) -> bool {
	let path_str = path.to_string_lossy();
	let saved_format = config.get_document_format(&path_str);
	if !saved_format.is_empty() && parser_supports_extension(&saved_format) {
		return true;
	}
	let Some(format) = dialogs::show_open_as_dialog(parent, path) else {
		return false;
	};
	if !parser_supports_extension(&format) {
		let message = t("Unsupported format selected.");
		let title = t("Error");
		let dialog = MessageDialog::builder(parent, &message, &title)
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
			.build();
		dialog.show_modal();
		return false;
	}
	config.set_document_format(&path_str, &format);
	true
}

fn update_title_from_manager(frame: &Frame, dm: &DocumentManager) {
	let sleep_start = SLEEP_TIMER_START_MS.load(Ordering::SeqCst);
	let sleep_duration = SLEEP_TIMER_DURATION_MINUTES.load(Ordering::SeqCst);
	if dm.tab_count() == 0 {
		frame.set_title(&t("Paperback"));
		let mut status_text = t("Ready");
		if sleep_start > 0 {
			let remaining = status::calculate_sleep_timer_remaining(sleep_start, sleep_duration);
			if remaining > 0 {
				status_text = status::format_sleep_timer_status(&status_text, remaining);
			}
		}
		frame.set_status_text(&status_text, 0);
		return;
	}
	if let Some(tab) = dm.active_tab() {
		let title = tab.session.title();
		let display_title = if title.is_empty() {
			tab.file_path.file_name().map_or_else(|| t("Untitled"), |s| s.to_string_lossy().to_string())
		} else {
			title
		};
		let template = t("Paperback - {}");
		frame.set_title(&template.replace("{}", &display_title));
		let position = tab.text_ctrl.get_insertion_point();
		let status_info = tab.session.get_status_info(position);
		let mut status_text = status::format_status_text(&status_info);
		if sleep_start > 0 {
			let remaining = status::calculate_sleep_timer_remaining(sleep_start, sleep_duration);
			if remaining > 0 {
				status_text = status::format_sleep_timer_status(&status_text, remaining);
			}
		}
		frame.set_status_text(&status_text, 0);
	}
}
