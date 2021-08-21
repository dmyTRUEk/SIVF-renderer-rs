//! This is main file

mod help;
mod utils;

use std::env;

use crate::help::*;
use crate::utils::string::{TraitStrExtensionTrimEmptyLines, TraitStrExtensionTrimLinesByFirstLine};



pub struct Vec2d<T> { pub x: T, pub y: T }

pub enum ColorModel {
    ARGB,
    RGBA,
    // TODO: add `CMYA`, `ACMY`, `XYZ`?
}

pub struct SIVFStruct {
    pub image_sizes: Vec2d<usize>,
    pub color_model: ColorModel,
    pub root_layer: SIVFLayer,
}

pub struct SIVFLayer {

}



fn main() {
    // get cli args
    let args_all: Vec<String> = env::args().collect();
    println!("args_all = {:?}", args_all);

    // remove first cli arg (which is just path to this binary)
    let args: Vec<String> = (&args_all[1..]).to_vec();
    println!("args = {:?}", args);

    for arg in args {
        if arg == "-h" {
            println!("{}", HELP_STR.to_string().trim_empty_lines().trim_lines_by_first_line());
        }
        // TODO: add cli options
        let file_name = arg;
        let sivf_struct: SIVFStruct;
    }
}

#[test]
fn unit_test_2_plus_2_eq_4() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn unit_test_3_plus_3_eq_6() {
    assert_eq!(3 + 3, 6);
}
