//! SIVF Circle

use crate::sivf_objects::_canvas::Canvas;
use crate::sivf_objects::_sivf_metric_units::SivfMetricUnit;
use crate::sivf_objects::_trait_render::TraitSivfRender;
use crate::utils::vec2d::Vec2d;
use crate::utils::color::Color;
use crate::utils::sizes::ImageSizes;



pub struct SivfCircle {
    position: Vec2d<SivfMetricUnit>,
    radius: SivfMetricUnit,
    color: Color,
}

impl TraitSivfRender for SivfCircle {
    fn render(&self, image_sizes: ImageSizes) -> Canvas {
        let mut canvas: Canvas = Canvas::new(image_sizes);
        todo!()
    }
}
