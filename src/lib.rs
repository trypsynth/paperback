#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

pub mod config;
pub mod document;
mod html_to_text;
pub mod parser;
mod pdfium;
pub mod reader_core;
pub mod session;
pub mod ui_types;
pub mod update;
mod utils;
mod xml_to_text;
