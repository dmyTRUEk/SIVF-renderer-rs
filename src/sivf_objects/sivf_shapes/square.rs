//! SIVF Square

use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::metric_units::MetricUnit;
use crate::sivf_misc::trait_render::TraitRender;
use crate::utils::vec2d::Vec2d;
use crate::utils::color::Color;
use crate::utils::sizes::ImageSizes;



#[derive(Clone, Debug, PartialEq)]
pub struct Square {
    position: Vec2d<MetricUnit>,
    side: MetricUnit,
    color: Color,
    inverted: bool,
}

impl Square {

    pub fn new(position: Vec2d<MetricUnit>, side: MetricUnit, color: Color, inverted: bool) -> Self {
        Square { position, side, color, inverted }
    }

}

impl TraitRender for Square {

    fn render(&self, image_sizes: ImageSizes) -> Canvas {
        todo!()
    }
}
