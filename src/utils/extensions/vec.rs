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



pub trait ExtensionContains<T> {
    // TODO LATER: remove `_`
    fn contains_(&self, el: T) -> bool;
}

impl ExtensionContains<&str> for Vec<&str> {
    fn contains_(&self, el: &str) -> bool {
        self.contains(&el)
    }
}

impl ExtensionContains<String> for Vec<String> {
    fn contains_(&self, el: String) -> bool {
        self.contains(&el)
    }
}

impl ExtensionContains<&str> for Vec<String> {
    fn contains_(&self, el: &str) -> bool {
        self.contains(&el.to_string())
    }
}

impl ExtensionContains<String> for Vec<&str> {
    fn contains_(&self, el: String) -> bool {
        self.contains(&&*el)
    }
}



pub trait ExtensionCollectToVec<I: Iterator> {
    fn collect_vec(self) -> Vec<<I as Iterator>::Item>;
}
impl<I: Iterator> ExtensionCollectToVec<I> for I {
    fn collect_vec(self) -> Vec<<I as Iterator>::Item> {
        self.collect::<Vec<_>>()
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

