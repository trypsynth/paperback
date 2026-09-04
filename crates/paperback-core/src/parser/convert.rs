//! Shared conversion engines used by multiple format parsers, as opposed to a parser for
//! any one format itself: [`html_to_text`] backs `chm`, `daisy`, `epub`, `html`, `markdown`
//! and `mobi`; [`xml_to_text`] backs `daisy`, `epub` and `fb2`; [`table_text`] backs both of
//! those plus `odt`, `pdf`, `powerpoint` and `word`.

pub(crate) mod block_elements;
mod format_spans;
mod line_builder;
mod list_style;

pub mod html_to_text;
pub mod table_text;
pub mod xml_to_text;
