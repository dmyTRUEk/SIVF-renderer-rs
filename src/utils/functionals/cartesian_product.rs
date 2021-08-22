//! Cartesian product



// TODO: remove?

// pub fn partial_cartesian<T: Clone>(a: Vec<Vec<T>>, b: &[T]) -> Vec<Vec<T>> {
//     a.into_iter().flat_map(|xs| {
//         b.iter().cloned().map(|y| {
//             let mut vec = xs.clone();
//             vec.push(y);
//             vec
//         }).collect::<Vec<_>>()
//     }).collect()
// }
//
// pub fn cartesian_product_lists<T: Clone>(lists: &[&[T]]) -> Vec<Vec<T>> {
//     match lists.split_first() {
//         Some((first, rest)) => {
//             let init: Vec<Vec<T>> = first.iter().cloned().map(|n| vec![n]).collect();
//             rest.iter().cloned().fold(init, |vec, list| {
//                 partial_cartesian(vec, list)
//             })
//         }
//         None => { vec![] }
//     }
// }
//
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn lists() {
//         let expected: Vec<Vec<i32>> = vec![
//             vec![10, 1],
//             vec![10, 2],
//             vec![10, 3],
//             vec![20, 1],
//             vec![20, 2],
//             vec![20, 3],
//         ];
//         let actual: Vec<Vec<i32>> = cartesian_product_lists(&[&[10, 20], &[1, 2, 3]]);
//         assert_eq!(expected, actual);
//     }
// }
