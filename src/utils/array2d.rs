//! Rectangular 2d array

use std::ops::{Index, IndexMut};

use crate::utils::sizes::{Sizes, Coordinates};



/// This structure mustnt be used outside implementation.
/// For creating `Array2d` use `Array2d::new()` instead.
#[derive(Clone, Debug, PartialEq)]
pub struct Array2d<T: Copy> {
    sizes: Sizes<usize>,
    // TODO: rewrite [elements] in flat structure and do measurements: flat vs nested(2d)
    elements: Vec<Vec<T>>,
}
// TODO: ? make iter:
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
            elements: vec![vec![fill; sizes.h]; sizes.w]
        }
    }

    pub fn from(given_array: Vec<Vec<T>>) -> Result<Self, &'static str> {
        let is_rectangle: bool = given_array.iter().all(|row| row.len() == given_array.first().unwrap().len() );
        return match is_rectangle {
            true => {
                let sizes: Sizes<usize> = Sizes::new(given_array.len(), given_array[0].len());
                Ok(Array2d{ sizes, elements: given_array })
            }
            false => {
                return Err("Given Vec<Vec<T>> is not rectangular");
            }
        };
    }

}

impl<T: Copy> Index<(usize, usize)> for Array2d<T> {
    type Output = T;
    fn index(&self, wh: (usize, usize)) -> &Self::Output {
        &self.elements[wh.0][wh.1]
    }
}

impl<T: Copy> Index<Coordinates<usize>> for Array2d<T> {
    type Output = T;
    fn index(&self, coordinates: Coordinates<usize>) -> &Self::Output {
        &self.elements[coordinates.w][coordinates.h]
    }
}

impl<T: Copy> IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, wh: (usize, usize)) -> &mut Self::Output {
        &mut self.elements[wh.0][wh.1]
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let expected: Array2d<i32> = Array2d { sizes: Sizes::new(2, 3), elements: vec![vec![0, 0, 0], vec![0, 0, 0]] };
        let actual  : Array2d<i32> = Array2d::new(Sizes::new(2, 3), 0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn ok() {
        let elements: Vec<Vec<char>> = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        let expected: Array2d<char> = Array2d { sizes: Sizes::new(2, 3), elements: elements.clone() };
        let actual  : Array2d<char> = Array2d::from(elements).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn err() {
        let elements: Vec<Vec<char>> = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f', 'g']];
        let expected: bool = true;
        let actual  : bool = Array2d::from(elements).is_err();
        assert_eq!(expected, actual);
    }

}
