//! This is main file

use std::env;



fn main() {
    let args_all: Vec<String> = env::args().collect();
    println!("args_all = {:?}", args_all);

    let args: Vec<String> = (&args_all[1..]).to_vec();
    println!("args = {:?}", args);
}
