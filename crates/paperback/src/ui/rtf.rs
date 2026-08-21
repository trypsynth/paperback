//! Windows `RichEdit` fast path for applying bold/italic/underline formatting in
//! `document_manager`, in place of one `wxTextCtrl::SetStyle` call per span: [`write`]
//! builds a minimal RTF blob from plain text and formatting segments, and [`stream`] feeds
//! it into the native control via `EM_STREAMIN`.

pub mod stream;
pub mod write;
