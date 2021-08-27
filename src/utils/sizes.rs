//! Sizes, ImageSizes (w, h)

use std::convert::TryFrom;



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sizes<T> { pub w: T, pub h: T }

pub type ImageSizes = Sizes<usize>;

impl<T: Copy> Sizes<T> {

    #[inline]
    pub fn new(w: T, h: T) -> Self {
        Sizes { w, h }
    }

    pub fn to_sizes<R: TryFrom<T>>(&self) -> Sizes<R> {
        Sizes {
            w: R::try_from(self.w).ok().unwrap(),
            h: R::try_from(self.h).ok().unwrap(),
        }
    }

    pub fn to_tuple(&self) -> (T, T) {
        (self.w, self.h)
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_sizes_explicit_type() {
        let expected: Sizes<u32> = Sizes { w: 3840_u32, h: 2160_u32 };
        let actual  : Sizes<u32> = Sizes { w: 3840_usize, h: 2160_usize }.to_sizes::<u32>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn to_sizes_implicit_type() {
        let expected: Sizes<u32> = Sizes { w: 3840_u32, h: 2160_u32 };
        let actual  : Sizes<u32> = Sizes { w: 3840_usize, h: 2160_usize }.to_sizes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn to_tuple() {
        let expected: (u32, u32) = ( 3840_u32, 2160_u32 );
        let actual  : (u32, u32) = Sizes { w: 3840_u32, h: 2160_u32 }.to_tuple();
        assert_eq!(expected, actual);
    }

}
