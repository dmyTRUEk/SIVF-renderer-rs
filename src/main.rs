//! Main file

// #![deny(
//
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
//
//     // LINTS:
//     missing_debug_implementations,
//     // missing_docs,
//     trivial_casts,
//     trivial_numeric_casts,
//     unused_extern_crates,
//     unused_import_braces,
//     unused_qualifications,
//     unused_results
//
// )]

extern crate image;
extern crate derive_more;

mod help;
mod sivf_misc;
mod sivf_objects;
mod utils;

use std::env;

// use image::{ImageBuffer, Rgba};
use std::fs::File;

use sivf_objects::sivf_complex::layer::Layer;
use sivf_misc::sivf_struct::SivfStruct;

use crate::help::HELP_STR;
use crate::utils::color::ColorModel;
use crate::utils::extensions::date_time::ExtensionDateTimeLocalToMyFormat;
use crate::utils::extensions::string::{ExtensionTrimEmptyLines, ExtensionTrimLinesByFirstLine};
use crate::utils::sizes::{image_sizes, ImageSizes};
use crate::sivf_misc::blend_types::BlendType;



// TODO: rewrite main using only functionals
fn main() {
    // get cli args
    let args_all: Vec<String> = env::args().collect();
    // println!("args_all = {:?}", args_all);

    // remove first cli arg (which is just path to this binary)
    let args: Vec<String> = (&args_all[1..]).to_vec();
    // println!("args = {:?}", args);

    // TODO: separate args?

    let file_names: Vec<String> = args
        .iter()
        .cloned()
        .filter(|arg| !arg.starts_with("-"))
        .collect();
    if file_names.is_empty() {
        println!("No files to render was provided.");
        println!("Exiting...");
        return;
    }

    // TODO: add cli options
    // TODO:
    // if arg == "-h" {
    //     println!("{}", HELP_STR.to_string().trim_empty_lines().trim_lines_by_first_line());
    //     continue;
    // }

    for file_name_input in file_names {

        print!("Reading file... ");
        let file = match File::open(&file_name_input) {
            Ok(f) => { f }
            Err(e) => {
                println!(r#"Can't open file "{}", skipping it"#, file_name_input);
                continue;
            }
        };
        println!("OK");

        // TODO
        let sivf_struct: SivfStruct = SivfStruct {
            image_sizes: image_sizes(200, 100),
            color_model: ColorModel::ARGB,
            // root_layer: layer(BlendType::FullOverlap, vec![]),
            root_layer: Layer::new(BlendType::Overlap, vec![]),
        };

        print!(r#"Rendering "{}"... "#, file_name_input);
        let render_time_start = chrono::Local::now();
        let canvas = sivf_struct.render();
        let render_time_end = chrono::Local::now();
        println!("Render finished in {}s.", (render_time_end-render_time_start).num_seconds());

        let image_sizes: ImageSizes = sivf_struct.image_sizes;
        let file_name = format!("img_{}__{}x{}.png", render_time_start.to_my_format(), image_sizes.w, image_sizes.h);
        // println!("file_name = {}", file_name);

        print!("Reading file... ");
        let image_buffer = canvas.to_image_buffer();
        println!("OK");

        print!("Reading file... ");
        image_buffer.save(file_name).unwrap();
        println!("OK");

        println!("File render finished successfully.");
    }

    println!("Program finished successfully!");
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
