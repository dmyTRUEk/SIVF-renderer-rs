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

mod help;
mod sivf_misc;
mod sivf_objects;
mod utils;

use crate::help::HELP_STR;
use crate::utils::extensions::date_time::ExtensionDateTimeLocalToMyFormat;
use crate::utils::extensions::string::{ExtensionTrimEmptyLines, ExtensionTrimLinesByFirstLine, ExtensionRemoveCLikeComments};
use crate::utils::functionals::separate::ExtensionSeparate;
use crate::utils::sizes::ImageSizes;
use crate::utils::io::flush;
use crate::sivf_misc::sivf_struct::SivfStruct;
use crate::sivf_misc::trait_render::RenderType;
use crate::utils::extensions::vec::ExtensionContains;



fn main() {
    println!();

    // get cli args
    let args_all: Vec<String> = env::args().collect();
    // println!("args_all = {args_all:?}");

    // remove first cli arg (which is just path to this binary)
    let args: Vec<String> = (&args_all[1..]).to_vec();
    // println!("args = {args:?}");

    let (options, file_paths) = args.to_vec().separate(|v| v.starts_with('-'));
    // println!("options = {options:?}");
    // println!("file_paths = {file_paths:?}");

    if options.contains_("-h") || options.contains_("--help")  {
        println!("{help}", help=HELP_STR.to_string().trim_empty_lines().trim_lines_by_first_line());
        return;
    }

    // TODO LATER
    // let show_log: bool = options.contains_("-l") || options.contains_("--log=1");
    // let show_progress: bool = options.contains_("-p") || options.contains_("--progress=1");
    let render_type: RenderType = match options {
        option if option.contains_("-r=cpu1") || options.contains_("--render=cpu1") => { RenderType::Cpu1 }
        // TODO
        option if option.contains_("-r=cpu8") || options.contains_("--render=cpu8") => { RenderType::Cpu(8) }
        option if option.contains_("-r=cpubest") || options.contains_("--render=cpubest") => { RenderType::CpuBest }
        option if option.contains_("-r=gpu") || options.contains_("--render=gpu") => { RenderType::Gpu }
        _ => { RenderType::Cpu1 }
    };

    if file_paths.is_empty() {
        // TODO LATER: then ask user for file to render
        println!("No files to render was provided.");
        println!("Exiting...");
        return;
    }

    // TODO LATER: make it parallel, so many pictures at the same time can render
    //   or make render it self parallel, so image will be renderer faster
    for file_input_path in file_paths {
        print!(r#"Reading file "{file_input_path}"... "#);
        flush();
        // TODO?: instead of [match] try to use [unwrap_or_else()]
        let sivf_file_content: String = match File::open(&file_input_path) {
            Ok(mut file) => {
                let mut file_content = String::new();
                file.read_to_string(&mut file_content).unwrap();
                file_content
            }
            Err(_) => {
                println!(r#"ERROR: Can't open file "{file_input_path}", skipping it"#);
                continue;
            }
        };
        println!("OK");
        // println!("file content = \n{sivf_file_content}");

        print!("Removing comments... ");
        flush();
        // TODO?: instead of [match] try to use [unwrap_or_else()]
        let sivf_file_content: String = match sivf_file_content.remove_comments() {
            Ok(v) => { v }
            Err(_) => {
                println!("ERROR: Can't remove comments, skipping this file");
                continue;
            }
        };
        println!("OK");
        // println!("file content without comments = \n{sivf_file_content}");

        print!("Parsing file to YAML... ");
        flush();
        // TODO?: instead of [match] try to use [unwrap_or_else()]
        let value: serde_yaml::Value = match serde_yaml::from_str(&sivf_file_content) {
            Ok(v) => { v }
            Err(e) => {
                println!(r#"ERROR: Cant parse file: "{e}""#);
                continue;
            }
        };
        println!("OK");

        print!("Parsing YAML to SIVF struct... ");
        flush();
        let sivf_struct: SivfStruct = SivfStruct::from(&value);
        // println!("Parse result:\n{sivf_struct:#?}");
        println!("OK");

        print!("Rendering... ");
        flush();
        let render_time_start = chrono::Local::now();
        let canvas = sivf_struct.render(render_type);
        let render_time_end = chrono::Local::now();
        let render_time = render_time_end - render_time_start;
        print!("finished in {s}s {ms}ms. ", s=render_time.num_seconds(), ms=render_time.num_milliseconds() % 1000);
        // println!("Canvas result:\n{canvas:?}");
        println!("OK");

        print!("Converting rendered array to image... ");
        flush();
        let image_buffer = canvas.to_image_buffer();
        println!("OK");

        let image_sizes: ImageSizes = sivf_struct.image_sizes;
        // TODO LATER: separate this into function
        let file_output_name = format!(
            "img_{t}__{w}x{h}.png",
            t=render_time_start.to_my_format(),
            w=image_sizes.w,
            h=image_sizes.h
        );
        // TODO: file_path_output
        let file_output_path: String = file_output_name;
        // println!("file_name = {file_name}");
        print!(r#"Saving image as "{file_output_path}"... "#);
        flush();
        image_buffer.save(file_output_path).unwrap();
        println!("OK");

        println!("File render finished successfully.");
        println!();
    }

    println!("\nProgram finished successfully!");
}

