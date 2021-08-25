//! SIVF Circle

use serde_derive::{Serialize, Deserialize};

use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::metric_units::MetricUnit;
use crate::sivf_misc::trait_render::TraitRender;
use crate::utils::vec2d::Vec2d;
use crate::utils::color::Color;
use crate::utils::sizes::ImageSizes;



#[derive(Clone, Debug, /*Serialize,*/ Deserialize)]
pub struct Circle {

    #[serde(alias="xy")]
    // TODO: force serde to create Vec2d from array [x, y]
    position: Vec2d<MetricUnit>,

    #[serde(alias="r")]
    radius: MetricUnit,

    // TODO: explain to serde that color can be read from str ff112233
    #[serde(alias="c")]
    color: Color,

    #[serde(default="deserialize::inverted", alias="c")]
    inverted: bool,
}

mod deserialize {

    pub(crate) fn inverted() -> bool {
        false
    }

}

impl TraitRender for Circle {
    fn render(&self, image_sizes: ImageSizes) -> Canvas {
        // let mut canvas: Canvas = Canvas::new(image_sizes);
        todo!()
    }
}
