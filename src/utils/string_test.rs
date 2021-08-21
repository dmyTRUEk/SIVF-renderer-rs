//! This file Tests `utils_str.rs` 

use super::*;



#[test]
fn unit_test_trim_empty_lines() {
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
fn unit_test_trim_lines_by_first_line() {
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
fn unit_test_split_and_keep() {
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

