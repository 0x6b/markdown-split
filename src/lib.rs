//! # markdown-split
//!
//! This crate provides a function to split a markdown text into sections based on headings. It is
//! useful for splitting a markdown text into smaller parts for further processing. The sections are
//! determined by the headings in the markdown text (h1-h6).
pub use split::split;
mod split;
