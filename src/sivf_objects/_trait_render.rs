//! Trait Render

use crate::sivf_objects::_canvas::Canvas;
use crate::utils::sizes::ImageSizes;



pub trait TraitSivfRender {
    fn render(&self, image_sizes: ImageSizes) -> Canvas;
}
