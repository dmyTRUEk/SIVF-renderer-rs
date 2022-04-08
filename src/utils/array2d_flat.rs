//! Rectangular 2d array (flat, much faster than Vec<Vec<T>>)

use std::ops::{Index, IndexMut};

use crate::utils::sizes::Sizes;
use crate::utils::extensions::vec::ExtensionFlatten;



#[derive(Clone, Debug, PartialEq)]
pub struct Array2d<T: Copy> {
    sizes: Sizes<usize>,
    // TODO: rewrite [elements] in flat structure and do measurements: flat vs nested(2d)
    elements: Vec<T>,
}
// TODO?: make iter:
//   ```
//   let array2d = Array2d::new(...);
//   for (w, h, item) in array2d.iter() {
//       ...
//   }
//   ```

impl<T: Copy> Array2d<T> {

    pub fn width(&self)  -> usize { self.sizes.w }
    pub fn height(&self) -> usize { self.sizes.h }

    pub fn new(sizes: Sizes<usize>, fill: T) -> Self {
        Array2d {
            sizes,
            elements: vec![fill; sizes.h*sizes.w]
        }
    }

    pub fn from(given_array: Vec<Vec<T>>) -> Self {
        let is_rectangle: bool = given_array.iter().all(|row| row.len() == given_array.first().unwrap().len() );
        return match is_rectangle {
            true => {
                let sizes: Sizes<usize> = Sizes::new(given_array[0].len(), given_array.len());
                Array2d { sizes, elements: given_array.flatten() }
            }
            false => {
                panic!("Given Vec<Vec<T>> is not rectangular");
            }
        };
    }

    fn wh_to_index(&self, w: usize, h: usize) -> usize {
        w + h * self.sizes.w
    }

}

impl<T: Copy> Index<(usize, usize)> for Array2d<T> {
    type Output = T;
    fn index(&self, wh: (usize, usize)) -> &Self::Output {
        let index: usize = self.wh_to_index(wh.0, wh.1);
        &self.elements[index]
    }
}

// impl<T: Copy> Index<Coordinates<usize>> for Array2dFlat<T> {
//     type Output = T;
//     fn index(&self, coordinates: Coordinates<usize>) -> &Self::Output {
//         &self.elements[coordinates.w][coordinates.h]
//     }
// }

impl<T: Copy> IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, wh: (usize, usize)) -> &mut Self::Output {
        let index: usize = self.wh_to_index(wh.0, wh.1);
        &mut self.elements[index]
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let expected: Array2d<i32> = Array2d { sizes: Sizes::new(3, 2), elements: vec![0, 0, 0, 0, 0, 0] };
        let actual  : Array2d<i32> = Array2d::new(Sizes::new(3, 2), 0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn ok() {
        let elements: Vec<Vec<char>> = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        let expected: Array2d<char> = Array2d { sizes: Sizes::new(3, 2), elements: elements.clone().flatten() };
        let actual  : Array2d<char> = Array2d::from(elements);
        assert_eq!(expected, actual);
    }

    // #[test]
    // fn err() {
    //     let elements: Vec<Vec<char>> = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f', 'g']];
    //     let expected: bool = true;
    //     let actual  : bool = Array2dFlat::from(elements).is_err();
    //     assert_eq!(expected, actual);
    // }

    #[test]
    fn get() {
        let elements: Vec<Vec<i32>> = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![9, 10, 11],
        ];
        let array2d_flat: Array2d<i32> = Array2d::from(elements);
        println!("{array2d_flat:#?}");

        assert_eq!(0, array2d_flat[(0, 0)]);
        assert_eq!(1, array2d_flat[(1, 0)]);
        assert_eq!(2, array2d_flat[(2, 0)]);

        assert_eq!(3, array2d_flat[(0, 1)]);
        assert_eq!(4, array2d_flat[(1, 1)]);
        assert_eq!(5, array2d_flat[(2, 1)]);

        assert_eq!(6, array2d_flat[(0, 2)]);
        assert_eq!(7, array2d_flat[(1, 2)]);
        assert_eq!(8, array2d_flat[(2, 2)]);

        assert_eq!(9 , array2d_flat[(0, 3)]);
        assert_eq!(10, array2d_flat[(1, 3)]);
        assert_eq!(11, array2d_flat[(2, 3)]);
    }

}

