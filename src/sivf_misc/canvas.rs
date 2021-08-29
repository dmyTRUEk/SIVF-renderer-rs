//! SIVF Canvas

use image::ImageBuffer;
use itertools::Itertools;

use crate::sivf_misc::blend_types::{BlendTypes, blend_colors};
use crate::sivf_misc::trait_render::RenderType;
// use crate::utils::array2d::Array2d;
use crate::utils::array2d_flat::Array2d;
use crate::utils::color::{Color, TRANSPARENT};
use crate::utils::sizes::{ImageSizes, Sizes};
use crate::utils::extensions::usize::ExtensionIndices;



#[derive(Clone, Debug)]
pub struct Canvas {
    // TODO: maybe use color from image library?
    // TODO: do it private, and implement [get(w, h)]
    // pub pixels: Array2d<Color>
    pub pixels: Array2d<Color>
}

impl Canvas {

    pub fn new(image_sizes: ImageSizes) -> Self {
        Canvas {
            pixels: Array2d::new(image_sizes, TRANSPARENT)
        }
    }

    pub fn sizes(&self) -> ImageSizes {
        ImageSizes::new(self.pixels.width(), self.pixels.height())
    }

    pub fn blend_with(&mut self, canvas_other: &Canvas, blend_types: &BlendTypes, render_type: &RenderType) {
        // TODO: use [render_type]
        match render_type {
            RenderType::Cpu1 => {
                for h in self.pixels.height().indices() {
                    for w in self.pixels.width().indices() {
                        let color1: Color = self.pixels[(w, h)];
                        let color2: Color = canvas_other.pixels[(w, h)];
                        self.pixels[(w, h)] = blend_colors(&color1, &color2, blend_types);
                    }
                }
            }
            _ => { todo!() }
        }
    }

    // TODO: understand what is second param in ImageBuffer generic
    pub fn to_image_buffer(&self) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let image_sizes: Sizes<u32> = self.sizes().to_sizes();
        let mut image_buffer = ImageBuffer::new(image_sizes.w, image_sizes.h);
        for (w, h, pixel) in image_buffer.enumerate_pixels_mut() {
            let pixel_color: Color = self.pixels[(w as usize, h as usize)];
            let rgba_array: [u8; 4] = [pixel_color.r, pixel_color.g, pixel_color.b, pixel_color.a];
            *pixel = image::Rgba(rgba_array);
        }
        image_buffer
    }

}
