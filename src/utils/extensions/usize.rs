//! `usize` extensions

use std::ops::Range;



pub trait ExtensionIndices {
    fn indices(&self) -> Range<usize>;
}

impl ExtensionIndices for usize {
    fn indices(&self) -> Range<usize> {
        Range { start: 0, end: *self }
    }
}
