//! `String` extensions

// use regex::Regex;
use regex::RegexBuilder;



pub trait ExtensionTrimEmptyLines {
    fn trim_empty_lines(&self) -> Self;
}
impl ExtensionTrimEmptyLines for String {
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



pub trait ExtensionTrimLinesByFirstLine {
    fn trim_lines_by_first_line(&self) -> Self;
}
impl ExtensionTrimLinesByFirstLine for String {
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



pub trait ExtensionSplitAndKeep {
    fn split_and_keep(&self, func: impl Fn(char) -> bool) -> Vec<&str>;
}
impl ExtensionSplitAndKeep for String {
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



pub trait ExtensionRemoveCLikeComments {
    fn remove_comments(&self) -> Result<String, &str>;
    fn remove_comments_oneline(&self) -> String;
    fn remove_comments_multiline(&self) -> String;
}

// TODO LATER: rewrite it so it clear lines, instead of deleting.
//   this will be useful, if error on line N occurred
impl ExtensionRemoveCLikeComments for String {
    fn remove_comments(&self) -> Result<String, &str> {
        let str_removed_order_1: String = self.remove_comments_oneline().remove_comments_multiline();
        let str_removed_order_2: String = self.remove_comments_multiline().remove_comments_oneline();
        if str_removed_order_1 == str_removed_order_2 {
            Ok(str_removed_order_1)
        }
        else {
            Err("Cant remove comments, possibly due to nested comments")
        }
    }

    fn remove_comments_oneline(&self) -> String {
        // TODO: maybe [RegexBuilder] -> [Regex] and remove [.build], for shorter code
        let re = RegexBuilder::new(r" *//.*?(\n|\z)")
            .build()
            .unwrap();
        re.replace_all(self, "").to_string()
    }

    fn remove_comments_multiline(&self) -> String {
        // TODO: maybe [RegexBuilder] -> [Regex] and remove [.build], for shorter code
        let re = RegexBuilder::new(r" */\*(.|\n)*?\*/(\n|\z)")
            .build()
            .unwrap();
        re.replace_all(self, "").to_string()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_empty_lines() {
        assert_eq!(
            String::from(""),
            String::from("\n").trim_empty_lines()
        );
        assert_eq!(
            String::from(""),
            String::from("\n\n\n").trim_empty_lines()
        );

        assert_eq!(
            String::from("a"),
            String::from("\na").trim_empty_lines()
        );
        assert_eq!(
            String::from("a"),
            String::from("a\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a"),
            String::from("\na\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a"),
            String::from("\n\n\na").trim_empty_lines()
        );
        assert_eq!(
            String::from("a"),
            String::from("a\n\n\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a"),
            String::from("\n\n\na\n\n\n").trim_empty_lines()
        );

        assert_eq!(
            String::from("a\nb"),
            String::from("\na\nb").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\nb"),
            String::from("a\nb\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\nb"),
            String::from("\na\nb\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\nb"),
            String::from("\n\n\na\nb").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\nb"),
            String::from("a\nb\n\n\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\nb"),
            String::from("\n\n\na\nb\n\n\n").trim_empty_lines()
        );

        assert_eq!(
            String::from("a\nb"),
            String::from("\na\nb").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\n\nb"),
            String::from("a\n\nb\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\n\nb"),
            String::from("\na\n\nb\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\n\nb"),
            String::from("\n\n\na\n\nb").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\n\nb"),
            String::from("a\n\nb\n\n\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\n\nb"),
            String::from("\n\n\na\n\nb\n\n\n").trim_empty_lines()
        );


        assert_eq!(
            String::from("a\nb\nc"),
            String::from("\na\nb\nc").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\nb\nc"),
            String::from("a\nb\nc\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\nb\nc"),
            String::from("\na\nb\nc\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\nb\nc"),
            String::from("\n\n\na\nb\nc").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\nb\nc"),
            String::from("a\nb\nc\n\n\n").trim_empty_lines()
        );
        assert_eq!(
            String::from("a\nb\nc"),
            String::from("\n\n\na\nb\nc\n\n\n").trim_empty_lines()
        );

    }



    #[test]
    fn trim_lines_by_first_line() {
        assert_eq!(
            String::from("a\nb"),
            String::from("a\nb").trim_lines_by_first_line()
        );
        assert_eq!(
            String::from("a\nb"),
            String::from("   a\n   b").trim_lines_by_first_line()
        );
        assert_eq!(
            String::from("a\n b"),
            String::from("   a\n    b").trim_lines_by_first_line()
        );
    }



    #[test]
    fn split_and_keep() {
        assert_eq!(
            Vec::<&str>::new(),
            "".to_string().split_and_keep(|c| c == ' ')
        );
        assert_eq!(
            vec!["+"],
            "+".to_string().split_and_keep(|c| c == '+')
        );
        assert_eq!(
            vec!["+", "+"],
            "++".to_string().split_and_keep(|c| c == '+')
        );
        // assert_eq!(
        //     vec!["+", "2", "+"],
        //     "+2+".to_string().split_and_keep(|c| c == '+')
        // );
        assert_eq!(
            vec!["2", "+", "2"],
            "2+2".to_string().split_and_keep(|c| c == '+')
        );
        assert_eq!(
            vec!["2 ", "+", " 2"],
            "2 + 2".to_string().split_and_keep(|c| c == '+')
        );
        assert_eq!(
            vec!["1 ", "+", " 2  ", "+", "   3"],
            "1 + 2  +   3".to_string().split_and_keep(|c| c == '+')
        );
        assert_eq!(
            vec!["1 ", "+", " 2  ", "-", "   3"],
            "1 + 2  -   3".to_string().split_and_keep(|c| c == '+' || c == '-')
        );
    }



    #[test]
    fn remove_c_like_comments_oneline() {
        // at begin
        {

            {   // at begin 1
                let string: String = r#"
                    // comment
                    text
                    + some 5+r4n9e sumbols
                    yeah
                "#.to_string();
                let expected: String = r#"
                    text
                    + some 5+r4n9e sumbols
                    yeah
                "#.to_string();
                let actual: String = string.remove_comments_oneline();
                assert_eq!(expected, actual);
            }

            {   // at begin 2
                let string: String = r#"
                    // comment
                    // comment 2
                    text
                    + some 5+r4n9e sumbols
                    yeah
                "#.to_string();
                let expected: String = r#"
                    text
                    + some 5+r4n9e sumbols
                    yeah
                "#.to_string();
                let actual: String = string.remove_comments_oneline();
                assert_eq!(expected, actual);
            }

        }

        // at middle
        {

            {
                let string: String = r#"
                    text
                    // comment again
                    + some 5+r4n9e sumbols
                    // and once more time
                    // and some more
                    yeah
                "#.to_string();
                let expected: String = r#"
                    text
                    + some 5+r4n9e sumbols
                    yeah
                "#.to_string();
                let actual: String = string.remove_comments_oneline();
                assert_eq!(expected, actual);
            }

        }

        // at end
        {
            // TODO LATER: rewrite prettier

            {   // 1
                let string: String = r#"
                    text
                    + some 5+r4n9e sumbols
                    yeah
                    // comment"#.to_string();
                let expected: String = r#"
                    text
                    + some 5+r4n9e sumbols
                    yeah
"#.to_string();
                let actual: String = string.remove_comments_oneline();
                assert_eq!(expected, actual);
            }

            {   // 2
                let string: String = r#"
                    text
                    + some 5+r4n9e sumbols
                    yeah
                    // comment
                    // comment 2"#.to_string();
                let expected: String = r#"
                    text
                    + some 5+r4n9e sumbols
                    yeah
"#.to_string();
                let actual: String = string.remove_comments_oneline();
                assert_eq!(expected, actual);
            }

        }
    }

    #[test]
    fn remove_c_like_comments_multiline() {
        // at begin
        {

            {   // oneline
                let string: String = r#"
                /* comment */
                text
                // but this is not a multiline comment
                + some 5+r4n9e sumbols
                yeah
            "#.to_string();
                let expected: String = r#"
                text
                // but this is not a multiline comment
                + some 5+r4n9e sumbols
                yeah
            "#.to_string();
                let actual: String = string.remove_comments_multiline();
                assert_eq!(expected, actual);
            }

            {   // multiline
                let string: String = r#"
                /*
                    comment
                */
                text
                // but this is not a multiline comment
                + some 5+r4n9e sumbols
                yeah
            "#.to_string();
                let expected: String = r#"
                text
                // but this is not a multiline comment
                + some 5+r4n9e sumbols
                yeah
            "#.to_string();
                let actual: String = string.remove_comments_multiline();
                assert_eq!(expected, actual);
            }

        }

        // at middle
        {

            {   // oneline
                let string: String = r#"
                text
                /* comment */
                // but this is not a multiline comment
                + some 5+r4n9e sumbols
                yeah
            "#.to_string();
                let expected: String = r#"
                text
                // but this is not a multiline comment
                + some 5+r4n9e sumbols
                yeah
            "#.to_string();
                let actual: String = string.remove_comments_multiline();
                assert_eq!(expected, actual);
            }

            {   // multiline
                let string: String = r#"
                text
                /*
                    comment
                */
                // but this is not a multiline comment
                + some 5+r4n9e sumbols
                yeah
            "#.to_string();
                let expected: String = r#"
                text
                // but this is not a multiline comment
                + some 5+r4n9e sumbols
                yeah
            "#.to_string();
                let actual: String = string.remove_comments_multiline();
                assert_eq!(expected, actual);
            }

        }

        // at end
        {
            // TODO LATER: rewrite prettier

            {   // oneline
                let string: String = r#"
                    text
                    // but this is not a multiline comment
                    + some 5+r4n9e sumbols
                    yeah
                    /* comment */"#.to_string();
                let expected: String = r#"
                    text
                    // but this is not a multiline comment
                    + some 5+r4n9e sumbols
                    yeah
"#.to_string();
                let actual: String = string.remove_comments_multiline();
                assert_eq!(expected, actual);
            }

            {   // multiline
                let string: String = r#"
                    text
                    // but this is not a multiline comment
                    + some 5+r4n9e sumbols
                    yeah
                    /*
                        comment
                    */"#.to_string();
                let expected: String = r#"
                    text
                    // but this is not a multiline comment
                    + some 5+r4n9e sumbols
                    yeah
"#.to_string();
                let actual: String = string.remove_comments_multiline();
                assert_eq!(expected, actual);
            }

        }
    }

}
