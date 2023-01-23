//! SIVF Square

use crate::{
    sivf_misc::{
        canvas::Canvas,
        color::{Color, TRANSPARENT},
        metric_units::{MetricUnit, ExtensionToPixels, Axis},
        render::{Render, RenderType},
        sizes::ImageSizes,
    },
    utils::{
        extensions::usize::ExtensionIndices,
        vec2d::Vec2d,
    },
};



#[derive(Clone, Debug, PartialEq)]
pub struct Square {
    position: Vec2d<MetricUnit>,
    side: MetricUnit,
    color: Color,
    inverted: bool,
}

impl Square {
    pub const fn new(
        position: Vec2d<MetricUnit>,
        side: MetricUnit,
        color: Color,
        inverted: bool
    ) -> Self {
        Square { position, side, color, inverted }
    }
}

impl Render for Square {
    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas {
        let mut canvas: Canvas = Canvas::new(image_sizes);
        let (wmax, hmax): (usize, usize) = (image_sizes.w, image_sizes.h);
        let position: Vec2d<f64> = Vec2d::new(
            self.position.x.to_pixels(image_sizes, Axis::X),
            -self.position.y.to_pixels(image_sizes, Axis::Y)   // minus here because math and array coords Y are inverted
        );
        let shift: Vec2d<f64> =
            - Vec2d::new(wmax as f64, hmax as f64) / 2.0_f64
            - position
            + Vec2d::new(0.5_f64, 0.5_f64);
        // TODO LATER: think: use Axis:: X or Y?
        let side: f64 = self.side.to_pixels(image_sizes, Axis::Y);
        let vec_min: Vec2d<f64> = -Vec2d::new(side, side) / 2.0_f64;
        let vec_max: Vec2d<f64> =  Vec2d::new(side, side) / 2.0_f64;
        match render_type {
            RenderType::Cpu1 => {
                for h in hmax.indices() {
                    for w in wmax.indices() {
                        let pos: Vec2d<f64> = Vec2d::new(w as f64, h as f64);
                        let is_inside_figure: bool = (pos+shift).is_inside_rectangle(vec_min, vec_max);
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

