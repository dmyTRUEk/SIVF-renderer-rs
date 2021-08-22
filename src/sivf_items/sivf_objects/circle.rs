//! SIVF Circle

use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::metric_units::MetricUnit;
use crate::sivf_misc::trait_render::TraitRender;
use crate::utils::vec2d::Vec2d;
use crate::utils::color::Color;
use crate::utils::sizes::ImageSizes;



pub struct SivfCircle {
    position: Vec2d<MetricUnit>,
    radius: MetricUnit,
    color: Color,
    inverted: bool,
}

impl TraitRender for SivfCircle {
    fn render(&self, image_sizes: ImageSizes) -> Canvas {
        let mut canvas: Canvas = Canvas::new(image_sizes);
        todo!()
    }
}
