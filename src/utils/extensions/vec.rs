//! Vec<T> extensions



pub trait ExtensionFlatten<T> {
    fn flatten(&self) -> Vec<T>;
}

impl<T: Clone> ExtensionFlatten<T> for Vec<Vec<T>> {
    fn flatten(&self) -> Vec<T> {
        // self.into_iter().flatten().collect()
        self.concat()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten() {
        let v: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6, 7]];
        let expected: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        let actual  : Vec<i32> = v.flatten();
        assert_eq!(expected, actual);
    }
}
