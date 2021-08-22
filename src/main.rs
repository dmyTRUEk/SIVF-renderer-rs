//! Main file

#![deny(

    // bad_style,
    // const_err,
    // dead_code,
    // improper_ctypes,
    // non_shorthand_field_patterns,
    // no_mangle_generic_items,
    // overflowing_literals,
    // path_statements ,
    // patterns_in_fns_without_body,
    // private_in_public,
    // unconditional_recursion,
    // unused,
    // unused_allocation,
    // unused_comparisons,
    // unused_parens,
    // while_true,

    // LINTS:
    // missing_debug_implementations,
    // missing_docs,
    // trivial_casts,
    // trivial_numeric_casts,
    // unused_extern_crates,
    // unused_import_braces,
    // unused_qualifications,
    // unused_results

)]

// TODO: this is not idiomatic?
extern crate image;
extern crate serde_json;
extern crate serde_yaml;
// extern crate derive_more;

mod help;
mod sivf_misc;
mod sivf_objects;
mod utils;

use std::env;
use std::fs::File;
use std::io::Read;

// use image::{ImageBuffer, Rgba};

use crate::help::HELP_STR;
use crate::utils::color::ColorModel;
use crate::utils::extensions::date_time::ExtensionDateTimeLocalToMyFormat;
use crate::utils::extensions::string::{ExtensionTrimEmptyLines, ExtensionTrimLinesByFirstLine, ExtensionRemoveCLikeComments};
use crate::utils::sizes::{image_sizes, ImageSizes};
use crate::sivf_misc::sivf_struct::SivfStruct;
use crate::sivf_misc::blend_types::BlendType;
use crate::sivf_objects::sivf_complex::layer::Layer;



// TODO: rewrite whole main using only functionals
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
        // TODO: then ask user for file to render
        println!("No files to render was provided.");
        println!("Exiting...");
        return;
    }

    // TODO: maybe use some cli lib for managing args
    // TODO: add cli options:
    //   - -h or --help -> help
    //   - --log -> show logs, if error
    //   - --progress -> show render progress
    // TODO:
    // if arg == "-h" {
    //     println!("{}", HELP_STR.to_string().trim_empty_lines().trim_lines_by_first_line());
    //     continue;
    // }

    for file_name_input in file_names {
        println!();

        print!("Reading file... ");
        // TODO: instead of [match] try to use [unwrap_or_else()]
        let sivf_file_as_string: String = match File::open(&file_name_input) {
            Ok(mut file) => {
                let mut file_content = String::new();
                file.read_to_string(&mut file_content).unwrap();
                file_content
            }
            Err(_) => {
                println!(r#"Can't open file "{}", skipping it"#, file_name_input);
                continue;
            }
        };
        // println!("file content = \n{}", file_content);
        println!("OK");

        print!("Removing comments... ");
        let sivf_file_as_string: String = match sivf_file_as_string.remove_comments() {
            Ok(v) => { v }
            Err(e) => {
                println!("Can't remove comments, skipping");
                continue;
            }
        };
        println!("OK");

        print!("Parsing file... ");
        let sivf_struct: SivfStruct = match serde_json::from_str(&sivf_file_as_string) {
            Ok(v) => { v }
            Err(e) => {
                println!(r#"Cant parse file: "{}""#, e);
                continue;
            }
        };
        // let sivf_struct: SivfStruct = SivfStruct {
        //     image_sizes: image_sizes(200, 100),
        //     color_model: ColorModel::ARGB,
        //     // root_layer: layer(BlendType::FullOverlap, vec![]),
        //     root_layer: Layer::new(BlendType::Overlap, vec![]),
        // };
        println!("OK");

        print!(r#"Rendering "{}"... "#, file_name_input);
        let render_time_start = chrono::Local::now();
        let canvas = sivf_struct.render();
        let render_time_end = chrono::Local::now();
        println!("Render finished in {}s.", (render_time_end-render_time_start).num_seconds());

        let image_sizes: ImageSizes = sivf_struct.image_sizes;
        let file_name = format!("img_{}__{}x{}.png", render_time_start.to_my_format(), image_sizes.w, image_sizes.h);
        // println!("file_name = {}", file_name);

        print!("Converting renderer array to image... ");
        let image_buffer = canvas.to_image_buffer();
        println!("OK");

        print!("Saving image... ");
        image_buffer.save(file_name).unwrap();
        println!("OK");

        println!("File render finished successfully.");
    }

    println!("\nProgram finished successfully!");
}
