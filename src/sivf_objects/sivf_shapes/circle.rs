//! SIVF Circle

use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::metric_units::MetricUnit;
use crate::sivf_misc::trait_render::TraitRender;
use crate::utils::vec2d::Vec2d;
use crate::utils::color::Color;
use crate::utils::sizes::ImageSizes;



#[derive(Clone, Debug, PartialEq)]
pub struct Circle {
    // TODO: force serde to create Vec2d from array [x, y]
    position: Vec2d<MetricUnit>,
    radius: MetricUnit,
    // TODO: explain to serde that color can be read from str ff112233
    color: Color,
    inverted: bool,
}

impl Circle {

    pub fn new(position: Vec2d<MetricUnit>, radius: MetricUnit, color: Color, inverted: bool) -> Self {
        Circle { position, radius, color, inverted }
    }

}

impl TraitRender for Circle {

    fn render(&self, image_sizes: ImageSizes) -> Canvas {
        // let mut canvas: Canvas = Canvas::new(image_sizes);
        todo!()
    }
}
