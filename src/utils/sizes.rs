//! Sizes, ImageSizes (w, h)

use std::convert::From;



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sizes<T> { pub w: T, pub h: T }
pub fn sizes<T>(w: T, h: T) -> Sizes<T> { Sizes { w, h } }

pub type ImageSizes = Sizes<usize>;
pub fn image_sizes(w: usize, h: usize) -> ImageSizes { ImageSizes { w, h } }

impl<T: Copy> Sizes<T> {

    // pub fn to<R: std::convert::From<T>>(&self) -> Sizes<R> {
    //     Sizes { w: self.w.into(), h: self.h.into() }
    // }

    pub fn to_tuple(&self) -> (T, T) {
        (self.w, self.h)
    }

}

// TODO
// impl<T, R> From<Sizes<T>> for Sizes<R> {
//     fn from(sizes_old: Sizes<T>) -> Self {
//         // sizes(sizes_old.w as R, sizes_old.h as R)
//         Sizes {
//             w: sizes_old.w as R,
//             h: sizes_old.h as R,
//         }
//     }
// }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into() {
        let expected: Sizes<u32> = Sizes { w: 3840_u32, h: 2160_u32 };
        let actual  : Sizes<u32> = Sizes { w: 3840_usize, h: 2160_usize }.into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn to_tuple() {
        let expected: (u32, u32) = ( 3840_u32, 2160_u32 );
        let actual  : (u32, u32) = Sizes { w: 3840_u32, h: 2160_u32 }.to_tuple();
        assert_eq!(expected, actual);
    }

}
