//! Strings that printed for `-h` or `--help`

// use clap::App;



pub const HELP_STR: &str = r#"
    sivf-renderer-rs 0.1.0
    SIVF format renderer implementation on Rust.

    USAGE:
        sivf-renderer-rs [OPTIONS] [FILES]

    OPTIONS:
        -h, --help
            Show this help message.
        -l, --log=<0 or 1>
            Show logs if error occured.
        -p, --progress=<0 or 1>
            Show render progress.
        -r=<var>, --render=<var>
            Set renderer variant.
            Render variants:
            - cpu1    -> use 1 CPU core
            - cpu<N>  -> use N CPU cores
            - cpubest -> use best amount CPU cores
            - gpu     -> use GPU
        -n=<str>, --name=<str>
            Set name of the output file.
            Subsituted literals:
            - %f -> file input name
            - %s -> start render time
            - %e -> end render time
            - %w -> width of the image
            - %h -> height of the image

            Example: --name="img_%i_%s__%wx%h.png"
"#;



// pub fn create_cli_options() {
//     let matches = App::new("sivf-renderer-rs")
//         .version("0.1.0")
//         .author("Myshko Dm <dmytruek@gmail.com>")
//         .about("SIVF format renderer implementation on Rust.")
//         // .license("")
//         .arg(Arg::new("config")
//             .short('c')
//             .long("config")
//             .value_name("FILE")
//             .about("Sets a custom config file")
//             .takes_value(true),
//         )
//         .arg(Arg::new("output")
//             .about("Sets an optional output file")
//             .index(1),
//         )
//         .arg(Arg::new("debug")
//             .short('d')
//             .long("debug")
//             .multiple_occurrences(true)
//             .about("Turn debugging information on"),
//         )
//         .subcommand(App::new("test")
//             .about("does testing things")
//             .arg(Arg::new("list").short('l').about("lists test values")),
//         )
//         .get_matches();
// }

