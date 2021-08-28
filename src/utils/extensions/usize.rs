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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indices() {
        {   // usize
            let size: usize = 42;
            let expected: Range<usize> = 0..size;
            let actual  : Range<usize> = size.indices();
            assert_eq!(expected, actual)
        }
    }

}
