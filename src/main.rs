//! This is main file

use std::env;



fn main() {
    // get cli args
    let args_all: Vec<String> = env::args().collect();
    println!("args_all = {:?}", args_all);

    // remove first cli arg (which is just path to this binary)
    let args: Vec<String> = (&args_all[1..]).to_vec();
    println!("args = {:?}", args);
}

#[test]
fn unit_test_2_plus_2_eq_4() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn unit_test_3_plus_3_eq_6() {
    assert_eq!(3 + 3, 6);
}
