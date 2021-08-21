//! Sizes, ImageSizes (w, h)



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sizes<T> { pub w: T, pub h: T }
pub fn sizes<T>(w: T, h: T) -> Sizes<T> { Sizes { w, h } }

pub type ImageSizes = Sizes<usize>;
pub fn image_sizes(w: usize, h: usize) -> Sizes<usize> { Sizes { w, h } }

// impl<T> Sizes<T> {
//
// }
