#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod bridge;
mod config;
pub mod document;
mod html_to_text;
pub mod parser;
mod pdfium;
mod reader_core;
pub mod session;
mod update;
mod utils;
mod xml_to_text;

pub use bridge::ffi;
