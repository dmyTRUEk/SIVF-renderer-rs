//! `&str` extensions



pub trait ExtensionCountChars {
    fn count_chars(&self, c: char) -> usize;
}
impl ExtensionCountChars for &str {
    fn count_chars(&self, ch: char) -> usize {
        self.chars().filter(|&c| c == ch).count()
    }
}



pub trait ExtensionsSplitOutsideBrackets {
    fn split_outside_brackets(&self, delimiter: char, left_bracket: char, right_bracket: char) -> Vec<String>;
}
impl ExtensionsSplitOutsideBrackets for &str {
    fn split_outside_brackets(&self, delimiter: char, left_bracket: char, right_bracket: char) -> Vec<String> {
        assert_ne!(left_bracket, right_bracket);
        let mut level: isize = 0;
        let mut split_indices: Vec<usize> = Vec::with_capacity(self.count_chars(delimiter));
        for (index, ch) in self.char_indices() {
            if ch == left_bracket {
                level += 1;
            }
            else if ch == right_bracket {
                level -= 1;
            }
            if level < 0 { panic!("Bad brackets sequence.") }
            if ch == delimiter && level == 0 {
                split_indices.push(index);
            }
        }
        split_indices.shrink_to_fit();
        if split_indices.is_empty() {
            return vec![self.to_string()];
        }
        let mut res: Vec<String> = Vec::with_capacity(split_indices.len());
        res.push(self[0..*split_indices.first().unwrap()].to_string());
        for i in 0..split_indices.len()-1 {
            res.push(self[split_indices[i]+1..split_indices[i+1]].to_string())
        }
        res.push(self[split_indices.last().unwrap()+1..self.len()].to_string());
        res
    }
}





#[cfg(test)]
mod tests {
    use crate::utils::extensions::str::ExtensionsSplitOutsideBrackets;

    #[test]
    fn split_outside_brackets() {
        let test_cases: Vec<_> = vec![
            (vec![""], ""),
            (vec!["a"], "a"),
            (vec!["abc"], "abc"),
            (vec!["a b c"], "a b c"),
            (vec!["a", "b", "c"], "a,b,c"),
            (vec!["(a,b,c)"], "(a,b,c)"),
            (vec!["a", "(b,b)", "c"], "a,(b,b),c"),
        ];
        for (ans, input) in test_cases {
            assert_eq!(ans, input.split_outside_brackets(',', '(', ')'));
        }
    }
}

