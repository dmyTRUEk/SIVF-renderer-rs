//! Separate iterable by rule
//!
//! ```
//! let (even, odd) = vec![1, 2, 3, 4, 5].separate(|v| v % 2 == 0).collect();
//! println!("{even}");   // 2, 4
//! println!("{odd}");    // 1, 3, 5
//! ```



pub trait ExtensionSeparate<T, F>
where
    T: Clone,
    F: Clone + Fn(T) -> bool,
{
    fn separate(&self, f: F) -> (Vec<T>, Vec<T>);
}

impl<T, F> ExtensionSeparate<T, F> for Vec<T>
where
    T: Clone,
    F: Clone + Fn(T) -> bool,
{
    fn separate(&self, f: F) -> (Vec<T>, Vec<T>) {
        self.iter().fold(
            (vec![], vec![]),
            |mut acc, el| {
                if f(el.clone()) { acc.0.push(el.clone()) } else { acc.1.push(el.clone()) }
                acc
            }
        )
    }
}



// TODO LATER:
// #[derive(Clone)]
// struct Separate<I, F> {
//     pub(crate) iter: I,
//     function: F,
//     inverse: bool,
// }
//
// impl<I, F> Separate<I, F>
// where
//     I: Clone + Iterator,
//     F: Clone + Fn(I::Item) -> bool,
// {
//     pub(crate) fn new(iter: I, function: F) -> (Separate<I, F>, Separate<I, F>) {(
//         Separate { iter: iter.clone(), function: function.clone(), inverse: false },
//         Separate { iter, function, inverse: true }
//     )}
// }
//
// impl<I, F> Iterator for Separate<I, F>
// where
//     I: Clone + Iterator,
//     F: Clone + (Fn(I::Item) -> bool) + std::ops::Fn(&<I as std::iter::Iterator>::Item) -> (),
// {
//     type Item = I::Item;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.find(&self.function)
//     }
// }
//
// pub trait ExtensionSeparate<I, F>
// where
//     I: Clone + Iterator,
//     F: Clone + Fn(I::Item) -> bool,
// {
//     fn separate(&self, function: F) -> (I, I);
// }
//
// impl<I, F> ExtensionSeparate<I, F> for std::vec::IntoIter<I::Item>
// where
//     I: Clone + Iterator,
//     F: Clone + Fn(I::Item) -> bool,
// {
//     fn separate(&self, function: F) -> (I, I) {
//         todo!()
//         // (
//         //     self.filter(function.clone()).iter,
//         //     self.filter(function).iter,
//         // )
//     }
// }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separate_vec() {
        let items: Vec<i32> = vec![1, 2, 3, 4, 5];
        let expected: (Vec<i32>, Vec<i32>) = (vec![2, 4], vec![1, 3, 5]);
        let actual  : (Vec<i32>, Vec<i32>) = items.separate(|v| v % 2 == 0);
        assert_eq!(expected, actual);
    }

    // #[test]
    // fn separate_next() {
    //     let separate = Separate {
    //         iter: vec![1, 2, 3, 4, 5],
    //         function: |v: i32| v % 2 == 0,
    //         inverse: false
    //     };
    // }
    //
    // #[test]
    // fn separate() {
    //     let items: Vec<i32> = vec![1, 2, 3, 4, 5];
    //     let expected: (Vec<i32>, Vec<i32>) = (vec![2, 4], vec![1, 3, 5]);
    //     let (iter_l, iter_r): (std::vec::IntoIter<i32>, std::vec::IntoIter<i32>) = items.into_iter().separate(|v| v % 2 == 0);
    //     let actual  : (Vec<i32>, Vec<i32>) = (iter_l.collect(), iter_r.collect());
    //     assert_eq!(expected, actual);
    //     assert!(false);
    // }

}
