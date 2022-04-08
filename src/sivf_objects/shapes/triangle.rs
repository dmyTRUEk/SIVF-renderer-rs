//! SIVF Triangle

use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::metric_units::{MetricUnit, ExtensionToPixels};
use crate::sivf_misc::render::{Render, RenderType};
use crate::utils::vec2d::Vec2d;
use crate::utils::color::{Color, TRANSPARENT};
use crate::utils::sizes::ImageSizes;
use crate::utils::extensions::usize::ExtensionIndices;



#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    p1: Vec2d<MetricUnit>,
    p2: Vec2d<MetricUnit>,
    p3: Vec2d<MetricUnit>,
    color: Color,
    inverted: bool,
}

impl Triangle {

    pub const fn new(
        p1: Vec2d<MetricUnit>,
        p2: Vec2d<MetricUnit>,
        p3: Vec2d<MetricUnit>,
        radius: MetricUnit,
        color: Color,
        inverted: bool
    ) -> Self {
        Triangle { p1, p2, p3, color, inverted }
    }

}

impl Render for Triangle {

    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas {
        todo!();
        // let mut canvas: Canvas = Canvas::new(image_sizes);
        // let position: Vec2d<f64> = Vec2d::new(
        //     self.position.x.to_pixels(image_sizes.w),
        //     -self.position.y.to_pixels(image_sizes.h)   // minus here because math and array coords Y are inverted
        // );
        // let shift: Vec2d<f64> =
        //     - Vec2d::new(image_sizes.w as f64, image_sizes.h as f64) / 2.0_f64
        //     - position
        //     + Vec2d::new(0.5_f64, 0.5_f64);
        // // TODO LATER: think about this - - - - - - - - - - - - - - - >  !W!
        // let radius2: f64 = self.radius.to_pixels(image_sizes.w).powi(2);
        // match render_type {
        //     RenderType::Cpu1 => {
        //         for h in image_sizes.h.indices() {
        //             for w in image_sizes.w.indices() {
        //                 let (x, y): (f64, f64) = (w as f64, h as f64);
        //                 let is_inside_figure: bool = (Vec2d::new(x, y)+shift).is_inside_circle(radius2);
        //                 canvas.pixels[(w, h)] = if is_inside_figure ^ self.inverted { self.color } else { TRANSPARENT };
        //             }
        //         }
        //     }
        //     RenderType::Cpu(_n_cores) => todo!(),
        //     RenderType::CpuBest => todo!(),
        //     RenderType::Gpu => todo!(),
        // }
        // canvas
    }

}

