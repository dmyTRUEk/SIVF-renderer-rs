//! Trait Render

use crate::sivf_misc::canvas::Canvas;
use crate::utils::sizes::ImageSizes;



pub trait TraitRender {
    fn render(&self, image_sizes: ImageSizes) -> Canvas;
}
