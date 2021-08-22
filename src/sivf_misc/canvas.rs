//! SIVF Canvas

use image::{ImageBuffer, Rgba};

use crate::utils::array2d::Array2d;
use crate::utils::color::{Color, TRANSPARENT};
use crate::utils::sizes::{ImageSizes, Sizes, image_sizes};
use crate::sivf_misc::blend_types::BlendType;



#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    // TODO: maybe use color from image library?
    // TODO: do it private, and implement [get(w, h)]
    pub pixels: Array2d<Color>
}
impl Canvas {
    pub fn new(image_sizes: ImageSizes) -> Canvas {
        Canvas {
            pixels: Array2d::new(image_sizes, TRANSPARENT)
        }
    }

    pub fn sizes(&self) -> ImageSizes {
        image_sizes(self.pixels.width(), self.pixels.height())
    }

    pub fn blend_with(&mut self, canvas_other: Canvas, blend_type: BlendType) {
        // TODO
        *self = canvas_other
    }

    // TODO: what is second param in ImageBuffer generic
    pub fn to_image_buffer(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let image_sizes: Sizes<u32> = self.sizes().into_sizes();
        let mut image_buffer = ImageBuffer::new(image_sizes.w, image_sizes.h);
        for (w, h, pixel) in image_buffer.enumerate_pixels_mut() {
            let pixel_color: Color = self.pixels[(w as usize, h as usize)];
            *pixel = image::Rgba([pixel_color.r, pixel_color.g, pixel_color.b, pixel_color.a]);
        }
        image_buffer
    }
}
