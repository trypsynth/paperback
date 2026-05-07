#[cfg(windows)]
pub fn ch_width(ch: char) -> usize {
	ch.len_utf16()
}
#[cfg(not(windows))]
pub fn ch_width(_ch: char) -> usize {
	1
}

#[cfg(windows)]
pub fn str_display_len(s: &str) -> usize {
	s.encode_utf16().count()
}
#[cfg(not(windows))]
pub fn str_display_len(s: &str) -> usize {
	s.chars().count()
}
