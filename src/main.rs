//! This is main file

extern crate image;

mod help;
mod utils;

use std::env;

use image::{ImageBuffer, Rgba};

use crate::help::*;
use crate::utils::array2d::Array2d;
use crate::utils::color::{ColorModel, Color, TRANSPARENT};
use crate::utils::date_time::TraitDateTimeLocalToMyFormat;
use crate::utils::string::{TraitStrExtensionTrimEmptyLines, TraitStrExtensionTrimLinesByFirstLine};
// use crate::utils::vec2d::Vec2d;



// TODO: create struct/enum Sivf units (px, percents)
pub enum SivfMetricUnits {
    Pixels(i32),
    Percents(f64),
}

pub trait TraitSivfRender {
    fn render(&self) -> Canvas;
}

// #[derive(Clone, Copy, Debug, PartialEq)]
pub struct SivfStruct {
    pub image_sizes: (usize, usize),
    pub color_model: ColorModel,
    pub root_layer: SivfLayer,
}
impl TraitSivfRender for SivfStruct {
    fn render(&self) -> Canvas {
        // TODO
        let mut canvas = Canvas::new(self.image_sizes.0, self.image_sizes.1);
        for child in &self.root_layer.children {
            let canvas_child: Canvas = child.render();
        }
        canvas
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    // TODO: maybe use color from image library?
    pub pixels: Array2d<Color>
}
impl Canvas {
    pub fn new(w: usize, h: usize) -> Canvas {
        Canvas {
            pixels: Array2d::new(w, h, TRANSPARENT)
        }
    }

    pub fn combine(&mut self, canvas_other: Canvas) {
        // TODO
        *self = canvas_other
    }

    // TODO: what is second param in ImageBuffer generic
    pub fn to_image_buffer(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut image_buffer = ImageBuffer::new(self.pixels.width() as u32, self.pixels.height() as u32);
        for (w, h, pixel) in image_buffer.enumerate_pixels_mut() {
            let pixel_color: Color = self.pixels[(w as usize, h as usize)];
            *pixel = image::Rgba([pixel_color.r, pixel_color.g, pixel_color.b, pixel_color.a]);
        }
        image_buffer
    }
}



// #[derive(Clone, Copy, Debug, PartialEq)]
pub struct SivfLayer {
    pub children: Vec<Box<dyn TraitSivfRender>>
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
        let file_name_input = arg;

        let render_start_time: String = chrono::Local::now().to_my_format();

        // TODO
        let sivf_struct: SivfStruct = SivfStruct {
            image_sizes: (200, 100),
            color_model: ColorModel::ARGB,
            root_layer: SivfLayer {
                children: vec![]
            }
        };

        println!(r#"Starting render "{}"."#, file_name_input);

        let canvas = sivf_struct.render();

        println!("Render finished.");

        let image_sizes: (usize, usize) = sivf_struct.image_sizes;
        let file_name = format!("img_{}__{}x{}.png", render_start_time, image_sizes.0, image_sizes.1);
        // println!("file_name = {}", file_name);

        let image_buffer = canvas.to_image_buffer();
        image_buffer.save(file_name).unwrap();

        println!("Saving image finished.");

        println!("Program finished successfuly!");

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
