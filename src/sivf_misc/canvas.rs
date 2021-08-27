//! SIVF Canvas

use image::ImageBuffer;
use itertools::Itertools;

use crate::utils::array2d::Array2d;
use crate::utils::color::{Color, TRANSPARENT};
use crate::utils::sizes::{ImageSizes, Sizes};
use crate::sivf_misc::blend_types::BlendTypes;
use crate::utils::extensions::usize::ExtensionIndices;



#[derive(Clone, Debug)]
pub struct Canvas {
    // TODO: maybe use color from image library?
    // TODO: do it private, and implement [get(w, h)]
    pub pixels: Array2d<Color>
}

impl Canvas {

    pub fn new(image_sizes: ImageSizes) -> Self {
        Canvas { pixels: Array2d::new(image_sizes, TRANSPARENT) }
    }

    pub fn sizes(&self) -> ImageSizes {
        ImageSizes::new(self.pixels.width(), self.pixels.height())
    }

    pub fn blend_with(mut self, canvas_other: Canvas, blend_types: BlendTypes) -> Self {
        // TODO
        self = canvas_other;
        // let x = (0..3).map(|i| (i * 2)..(i * 2 + 2)).collect();
        // for (w, h) in vec![0..10, 100..110].multi_cartesian_product() {
        // }
        // for (w, h) in [ self.pixels.width().indices(), self.pixels.height().indices() ].multi_cartesian_product() {
        // }
        // for pixel use [blend_types::blend_colors]
        self
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
