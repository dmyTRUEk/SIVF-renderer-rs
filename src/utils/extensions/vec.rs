//! Vec<T> extensions

use std::slice::Iter;



pub trait ExtensionCollectToVec<I: Iterator> {
    fn collect_to_vec(self) -> Vec<<I as Iterator>::Item>;
}
impl<I: Iterator> ExtensionCollectToVec<I> for I {
    fn collect_to_vec(self) -> Vec<<I as Iterator>::Item> {
        self.collect::<Vec<_>>()
    }
}



pub trait ExtensionFlatten<T> {
    fn flatten(&self) -> Vec<T>;
}

impl<T: Clone> ExtensionFlatten<T> for Vec<Vec<T>> {
    fn flatten(&self) -> Vec<T> {
        // self.into_iter().flatten().collect()
        self.concat()
    }
}



pub trait ExtensionContains<T> {
    // TODO LATER: remove `_`
    fn contains_(&self, el: T) -> bool;
}
// impl ExtensionContains<&str> for Vec<&str> {
//     fn contains_(&self, el: &str) -> bool {
//         self.contains(&el)
//     }
// }
// impl ExtensionContains<String> for Vec<String> {
//     fn contains_(&self, el: String) -> bool {
//         self.contains(&el)
//     }
// }
impl ExtensionContains<&str> for Vec<String> {
    fn contains_(&self, el: &str) -> bool {
        self.contains(&el.to_string())
    }
}
// impl ExtensionContains<String> for Vec<&str> {
//     fn contains_(&self, el: String) -> bool {
//         self.contains(&&*el)
//     }
// }
// TODO: write for vec



pub trait ExtensionContainsStartsWith<T> {
    fn contains_starts_with(&self, el_start: T) -> bool;
}
// impl ExtensionContainsStartsWith<&str> for Vec<&str> {
//     fn contains_starts_with(&self, el_start: &str) -> bool {
//         self.iter().any(|el| el.starts_with(&el_start))
//     }
// }
// impl ExtensionContainsStartsWith<String> for Vec<String> {
//     fn contains_starts_with(&self, el_start: String) -> bool {
//         self.iter().any(|el| el.starts_with(&el_start))
//     }
// }
impl ExtensionContainsStartsWith<&str> for Vec<String> {
    // TODO: rewrite using predicate, like in str.find
    fn contains_starts_with(&self, el_start: &str) -> bool {
        self.iter().any(|el| el.starts_with(&el_start))
    }
}
// impl ExtensionContainsStartsWith<String> for Vec<&str> {
//     fn contains_starts_with(&self, el_start: String) -> bool {
//         self.iter().any(|el| el.starts_with(&el_start))
//     }
// }



pub trait ExtensionFindStartsWith<T, R> {
    fn find_starts_with(&self, els_start: T) -> Option<R>;
}
impl<const N: usize> ExtensionFindStartsWith<[&str; N], String> for Iter<'_, String> {
    fn find_starts_with<'a>(&'a self, els_start: [&str; N]) -> Option<String> {
        todo!()
        // let res: Option<&String> = self.find(|element| els_start.iter().any(|el| element.starts_with(el)));
        // match res {
        //     None => { None },
        //     Some(res) => {
        //         Some(res.to_string())
        //     }
        // }
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

