//! SIVF Circle

use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::metric_units::{MetricUnit, ExtensionToPixels};
use crate::sivf_misc::render::{Render, RenderType};
use crate::utils::vec2d::Vec2d;
use crate::utils::color::{Color, TRANSPARENT};
use crate::utils::sizes::ImageSizes;
use crate::utils::extensions::usize::ExtensionIndices;



#[derive(Clone, Debug, PartialEq)]
pub struct Circle {
    position: Vec2d<MetricUnit>,
    radius: MetricUnit,
    color: Color,
    inverted: bool,
}

impl Circle {
    pub const fn new(
        position: Vec2d<MetricUnit>,
        radius: MetricUnit,
        color: Color,
        inverted: bool
    ) -> Self {
        Circle { position, radius, color, inverted }
    }
}

impl Render for Circle {
    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas {
        let mut canvas: Canvas = Canvas::new(image_sizes);
        let (wmax, hmax): (usize, usize) = (image_sizes.w, image_sizes.h);
        let position: Vec2d<f64> = Vec2d::new(
            self.position.x.to_pixels(wmax),
            -self.position.y.to_pixels(hmax)   // minus here because math and array coords Y are inverted
        );
        let shift: Vec2d<f64> =
            - Vec2d::new(wmax as f64, hmax as f64) / 2.0_f64
            - position
            + Vec2d::new(0.5_f64, 0.5_f64);
        // TODO LATER: think: use w or h?
        let radius2: f64 = self.radius.to_pixels(hmax).powi(2);
        match render_type {
            RenderType::Cpu1 => {
                for h in hmax.indices() {
                    for w in wmax.indices() {
                        let pos: Vec2d<f64> = Vec2d::new(w as f64, h as f64);
                        let is_inside_figure: bool = (pos+shift).is_inside_circle(radius2);
                        let is_draw_required: bool = is_inside_figure ^ self.inverted;
                        let color: Color = if is_draw_required { self.color } else { TRANSPARENT };
                        canvas.pixels[(w, h)] = color;
                    }
                }
            }
            RenderType::Cpu(_n_cores) => todo!(),
            RenderType::CpuMax => todo!(),
            RenderType::Gpu => todo!(),
        }
        canvas
    }
}

