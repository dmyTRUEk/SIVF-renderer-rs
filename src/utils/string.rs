//! This file contains useful funcs for `&str`

#[cfg(test)]
#[path="string_test.rs"]
mod string_test;



pub trait TraitStrExtensionTrimEmptyLines {
    fn trim_empty_lines(&self) -> Self;
}
impl TraitStrExtensionTrimEmptyLines for String {
    fn trim_empty_lines(&self) -> Self {
        let mut res = String::from("");
        let mut found_non_empty_at_begin: bool = false;
        for line in self.lines() {
            if line.replace(" ", "") != "" {
                found_non_empty_at_begin = true;
            }
            if found_non_empty_at_begin {
                res += &(line.to_string() + "\n");
            }
        }
        let mut res2 = String::from("");
        let mut found_non_empty_at_end: bool = false;
        for line in res.lines().rev() {
            if line.replace(" ", "") != "" {
                found_non_empty_at_end = true;
            }
            if found_non_empty_at_end {
                res2 = (line.to_string() + "\n") + &res2;
            }
        }
        let mut chars = res2.chars();
        chars.next_back();
        chars.as_str().to_string()
    }
}



pub trait TraitStrExtensionTrimLinesByFirstLine {
    fn trim_lines_by_first_line(&self) -> Self;
}
impl TraitStrExtensionTrimLinesByFirstLine for String {
    fn trim_lines_by_first_line(&self) -> Self {
        let mut trim_len: usize = 0;
        for c in self.chars() {
            if c == ' ' {
                trim_len += 1
            }
            else {
                break;
            }
        }
        let mut res: String = "".to_string();
        for line in self.lines() {
            let new_line: String = line.get(trim_len..).unwrap_or("").to_string();
            res += &(new_line + "\n");
        }
        let mut chars = res.chars();
        chars.next_back();
        chars.as_str().to_string()
    }
}



pub trait TraitStrExtensionSplitAndKeep {
    fn split_and_keep(&self, func: impl Fn(char) -> bool) -> Vec<&str>;
}
impl TraitStrExtensionSplitAndKeep for String {
    fn split_and_keep(&self, func: impl Fn(char) -> bool) -> Vec<&str> {
        if self == "" { return vec![]; }
        let res: Vec<&str> = self.split_inclusive(func).collect::<Vec<&str>>();
        // println!("res = {:#?}", res);
        let res_len = res.len();
        let mut res2: Vec<&str> = vec![];
        for i in 0..res_len-1 {
            // println!("res[i] = {:#?}", res[i]);
            let (lhs, rhs) = res[i].split_at(res[i].len()-1);
            if lhs != "" { res2.push(lhs); }
            if rhs != "" { res2.push(rhs); }
            // println!("res2 = {:#?}", res2.clone());
        }
        res2.push(res[res_len-1]);
        res2
    }
}

