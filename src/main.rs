//! Main file

// TODO LATER: turn on all
#![deny(

    bad_style,
    const_err,
    // dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    // unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,

    // LINTS:
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    // unused_results,

)]

use std::env;
use std::fs::File;
use std::io::Read;

// use image::{ImageBuffer, Rgba};
// use serde_yaml::Value;

mod help;
mod sivf_misc;
mod sivf_objects;
mod utils;

use crate::help::HELP_STR;
use crate::sivf_misc::sivf_struct::SivfStruct;
use crate::sivf_misc::blend_types::BlendType;
use crate::sivf_objects::sivf_complex::layer::Layer;
use crate::utils::color::ColorModel;
use crate::utils::extensions::date_time::ExtensionDateTimeLocalToMyFormat;
use crate::utils::extensions::string::{ExtensionTrimEmptyLines, ExtensionTrimLinesByFirstLine, ExtensionRemoveCLikeComments};
use crate::utils::sizes::ImageSizes;


// TODO: rewrite whole main using only functionals
fn main() {
    // get cli args
    let args_all: Vec<String> = env::args().collect();
    // println!("args_all = {:?}", args_all);

    // remove first cli arg (which is just path to this binary)
    let args: Vec<String> = (&args_all[1..]).to_vec();
    println!("args = {:?}", args);

    // TODO: separate args?

    let file_paths: Vec<String> = args
        .iter()
        .cloned()
        .filter(|arg| !arg.starts_with("-"))
        .collect();
    if file_paths.is_empty() {
        // TODO: then ask user for file to render
        println!("No files to render was provided.");
        println!("Exiting...");
        return;
    }

    // TODO: maybe use some cli lib for managing args
    // TODO: add cli options:
    //   -h --help -> help
    //   -l --log=0/1 -> show logs, if error
    //   -p --progress=0/1 -> show render progress
    //   -n="..." --name="%i_%s__%wx%h" -> name of the output file
    //     %f - file input name
    //     %s - start render time
    //     %e - end render time
    //     %w - width of the image
    //     %h - height of the image
    // TODO:
    // if arg == "-h" {
    //     println!("{}", HELP_STR.to_string().trim_empty_lines().trim_lines_by_first_line());
    //     continue;
    // }

    // TODO LATER: make it parallel, so many pictures at the same time can render
    for file_input_path in file_paths {
        println!();

        print!(r#"Reading file "{}"... "#, file_input_path);
        // TODO: instead of [match] try to use [unwrap_or_else()]
        let sivf_file_content: String = match File::open(&file_input_path) {
            Ok(mut file) => {
                let mut file_content = String::new();
                file.read_to_string(&mut file_content).unwrap();
                file_content
            }
            Err(_) => {
                println!(r#"Can't open file "{}", skipping it"#, file_input_path);
                continue;
            }
        };
        // println!("file content = \n{}", file_content);
        println!("OK");

        print!("Removing comments... ");
        let sivf_file_content: String = match sivf_file_content.remove_comments() {
            Ok(v) => { v }
            Err(e) => {
                println!("Can't remove comments, skipping");
                continue;
            }
        };
        println!("OK");

        println!("{}", &sivf_file_content);

        print!("Parsing file... ");
        let value: serde_yaml::Value = match serde_yaml::from_str(&sivf_file_content) {
            Ok(v) => { v }
            Err(e) => {
                println!(r#"Cant parse file: "{}""#, e);
                continue;
            }
        };
        let sivf_struct: SivfStruct = match SivfStruct::from(&value) {
            Ok(v) => { v }
            Err(e) => {
                println!(r#"Cant parse to SivfStruct: "{}""#, e);
                continue;
            }
        };
        println!("OK");

        println!("Parse result: {:#?}", sivf_struct);

        print!(r"Rendering ... ");
        let render_time_start = chrono::Local::now();
        let canvas = sivf_struct.render();
        let render_time_end = chrono::Local::now();
        print!("Render finished in {}s. ", (render_time_end-render_time_start).num_seconds());
        println!("OK");

        let image_sizes: ImageSizes = sivf_struct.image_sizes;
        // TODO LATER: separate this into function
        let file_output_name = format!(
            "img_{}__{}x{}.png",
            render_time_start.to_my_format(),
            image_sizes.w,
            image_sizes.h
        );
        // TODO: file_path_output
        let file_output_path: String = file_output_name;
        // println!("file_name = {}", file_name);

        print!("Converting rendered array to image... ");
        let image_buffer = canvas.to_image_buffer();
        println!("OK");

        println!(r#"Output file name: "{}""#, file_output_path);
        print!("Saving image... ");
        image_buffer.save(file_output_path).unwrap();
        println!("OK");

        println!("File render finished successfully.");
    }

    println!("\nProgram finished successfully!");
}
