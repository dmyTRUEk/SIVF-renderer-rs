//! Rectangular 2d array

use std::ops::Index;



/// This structure mustnt be used outside implementation.
/// For creating `Array2d` use `Array2d::new()` instead.
#[derive(Clone, Debug, PartialEq)]
pub struct Array2d<T: Copy> {
    w: usize,
    h: usize,
    // TODO: rewrite in flat structure ?
    //   and do measurements: flat vs nested(2d)
    elements: Vec<Vec<T>>,
}

impl<T: Copy> Array2d<T> {
    pub fn width(&self)  -> usize { self.w }
    pub fn height(&self) -> usize { self.h }

    // pub fn index(&self, wh: (usize, usize)) -> T {
    //     self.elements[wh.0][wh.1]
    // }

    pub fn new(w: usize, h: usize, fill: T) -> Array2d<T> {
        // TODO
        Array2d {
            w, h, elements: vec![vec![fill; h]; w]
        }
    }

    pub fn from(given_array: Vec<Vec<T>>) -> Result<Array2d<T>, String> {
        // TODO:
        let is_rectangle: bool = given_array.iter().all(|row| row.len() == given_array.first().unwrap().len() );
        return match is_rectangle {
            true => {
                Ok(Array2d{ w: given_array.len(), h: given_array[0].len(), elements: given_array })
            }
            false => {
                return Err("Given Vec<Vec<T>> is not rectangular".to_string());
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



#[cfg(test)]
mod tests {
    use super::*;
    // TODO: maybe rename tests

    #[test]
    fn unit_test_array2d_new() {
        let expected: Array2d<i32> = Array2d { w: 2, h: 3, elements: vec![vec![0, 0, 0], vec![0, 0, 0]] };
        let actual  : Array2d<i32> = Array2d::new(2, 3, 0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn unit_test_array2d_from_ok() {
        let elements: Vec<Vec<char>> = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        let expected: Array2d<char> = Array2d { w: 2, h: 3, elements: elements.clone() };
        let actual  : Array2d<char> = Array2d::from(elements).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn unit_test_array2d_from_err() {
        let elements: Vec<Vec<char>> = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f', 'g']];
        let expected: bool = true;
        let actual  : bool = Array2d::from(elements).is_err();
        assert_eq!(expected, actual);
    }




}
