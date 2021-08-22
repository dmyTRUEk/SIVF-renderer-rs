//! Main file

// #![deny(
//     bad_style,
//     const_err,
//     dead_code,
//     improper_ctypes,
//     non_shorthand_field_patterns,
//     no_mangle_generic_items,
//     overflowing_literals,
//     path_statements ,
//     patterns_in_fns_without_body,
//     private_in_public,
//     unconditional_recursion,
//     unused,
//     unused_allocation,
//     unused_comparisons,
//     unused_parens,
//     while_true,
//     // LINTS:
//     missing_debug_implementations,
//     // missing_docs,
//     trivial_casts,
//     trivial_numeric_casts,
//     unused_extern_crates,
//     unused_import_braces,
//     unused_qualifications,
//     unused_results
// )]

extern crate image;
extern crate derive_more;

use std::env;

use sivf_items::sivf_complex::layer::Layer;
use sivf_misc::sivf_struct::SivfStruct;

use crate::help::HELP_STR;
use crate::utils::color::ColorModel;
use crate::utils::extensions::date_time::ExtensionDateTimeLocalToMyFormat;
use crate::utils::extensions::string::{ExtensionTrimEmptyLines, ExtensionTrimLinesByFirstLine};
use crate::utils::sizes::{image_sizes, ImageSizes};

mod help;
mod sivf_misc;
mod sivf_items;
mod utils;

// use image::{ImageBuffer, Rgba};

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
        let file_name_input = arg;

        let render_start_time: String = chrono::Local::now().to_my_format();

        // TODO
        let sivf_struct: SivfStruct = SivfStruct {
            image_sizes: image_sizes(200, 100),
            color_model: ColorModel::ARGB,
            root_layer: Layer {
                children: vec![]
            }
        };

        println!(r#"Starting render "{}"."#, file_name_input);

        let canvas = sivf_struct.render();

        println!("Render finished.");

        let image_sizes: ImageSizes = sivf_struct.image_sizes;
        let file_name = format!("img_{}__{}x{}.png", render_start_time, image_sizes.w, image_sizes.h);
        // println!("file_name = {}", file_name);

        let image_buffer = canvas.to_image_buffer();
        image_buffer.save(file_name).unwrap();

        println!("Saving image finished.");

        println!("Program finished successfuly!");

    }
}



// TODO: remove this tests (tests mustnt be in [main])
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_2_plus_2_eq_4() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn unit_test_3_plus_3_eq_6() {
        assert_eq!(3 + 3, 6);
    }

}
