mod app;
mod audio;
mod background;
mod bookmarks;
mod commands;
mod dialogs;
mod document_manager;
mod find;
mod help;
mod icon;
#[cfg(target_os = "macos")]
mod mac_shortcuts;
mod main_window;
mod menu;
mod menu_ids;
mod navigation;
#[cfg(target_os = "windows")]
mod paragraph_format;
mod readability;
mod reader_ctrl;
mod reader_input;
#[cfg(target_os = "windows")]
mod rtf_stream;
mod selection;
mod sleep_timer;
mod sounds;
mod status;
mod text_render;
#[cfg(target_os = "windows")]
mod tray;
mod window_geometry;

pub use app::{PaperbackApp, quit_for_update};
#[cfg(target_os = "linux")]
pub use dialogs::{AssociationChoice, ChoiceAction, show_linux_setup_dialog};
pub use main_window::MainWindow;
