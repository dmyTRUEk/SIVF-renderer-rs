//! `&str` extensions



pub trait ExtensionCountChars {
    fn count_chars(&self, c: char) -> usize;
}
impl ExtensionCountChars for &str {
    fn count_chars(&self, ch: char) -> usize {
        self.chars().filter(|&c| c == ch).count()
    }
}

