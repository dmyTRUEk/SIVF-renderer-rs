//! SIVF Square

use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::metric_units::{MetricUnit, ExtensionToPixels};
use crate::sivf_misc::trait_render::{TraitRender, RenderType};
use crate::utils::vec2d::Vec2d;
use crate::utils::color::{Color, TRANSPARENT};
use crate::utils::sizes::ImageSizes;
use crate::utils::extensions::usize::ExtensionIndices;


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

    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas {
        let mut canvas: Canvas = Canvas::new(image_sizes);
        let position: Vec2d<f64> = Vec2d::new(
            self.position.x.to_pixels(image_sizes.w),
            -self.position.y.to_pixels(image_sizes.h)   // minus here because math and array coords Y are inverted
        );
        let shift: Vec2d<f64> =
            - Vec2d::new(image_sizes.w as f64, image_sizes.h as f64) / 2.0_f64
            - position
            + Vec2d::new(0.5_f64, 0.5_f64);
        // TODO LATER: think about this - - - - - - - - - - - - - - - >  !W!
        let side: f64 = self.side.to_pixels(image_sizes.w);
        let vec_min: Vec2d<f64> = -Vec2d::new(side, side) / 2.0_f64;
        let vec_max: Vec2d<f64> =  Vec2d::new(side, side) / 2.0_f64;
        match render_type {
            RenderType::Cpu1 => {
                for h in image_sizes.h.indices() {
                    for w in image_sizes.w.indices() {
                        let (x, y): (f64, f64) = (w as f64, h as f64);
                        let is_inside_figure: bool = (Vec2d::new(x, y)+shift).is_inside_rectangle(vec_min, vec_max);
                        canvas.pixels[(w, h)] = if is_inside_figure ^ self.inverted { self.color } else { TRANSPARENT };
                    }
                }
            }
            _ => { todo!() }
        }
        canvas
    }
}