//! SIVF Triangle

use crate::{
    sivf_misc::{
        canvas::Canvas,
        metric_units::{MetricUnit, ExtensionToPixels, Axis},
        render::{Render, RenderType},
    },
    utils::{
        color::{Color, TRANSPARENT},
        extensions::usize::ExtensionIndices,
        sizes::ImageSizes,
        vec2d::Vec2d,
    },
};



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
        color: Color,
        inverted: bool
    ) -> Self {
        Triangle { p1, p2, p3, color, inverted }
    }
}

impl Render for Triangle {
    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas {
        let mut canvas: Canvas = Canvas::new(image_sizes);
        let (wmax, hmax): (usize, usize) = (image_sizes.w, image_sizes.h);
        let shift: Vec2d<f64> =
            - Vec2d::new(wmax as f64, hmax as f64) / 2.0_f64
            + Vec2d::new(0.5_f64, 0.5_f64);
        let (p1, p2, p3): (Vec2d<f64>, Vec2d<f64>, Vec2d<f64>) = (
            Vec2d::new(self.p1.x.to_pixels(image_sizes, Axis::X), -self.p1.y.to_pixels(image_sizes, Axis::Y)),
            Vec2d::new(self.p2.x.to_pixels(image_sizes, Axis::X), -self.p2.y.to_pixels(image_sizes, Axis::Y)),
            Vec2d::new(self.p3.x.to_pixels(image_sizes, Axis::X), -self.p3.y.to_pixels(image_sizes, Axis::Y))
        );
        match render_type {
            RenderType::Cpu1 => {
                for h in hmax.indices() {
                    for w in wmax.indices() {
                        let pos: Vec2d<f64> = Vec2d::new(w as f64, h as f64);
                        let is_inside_figure: bool = (pos+shift).is_inside_triangle(p1, p2, p3);
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

