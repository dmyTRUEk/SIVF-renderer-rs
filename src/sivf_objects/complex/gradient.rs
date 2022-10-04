//! Gradient

use crate::{
    sivf_misc::{
        canvas::Canvas,
        metric_units::{MetricUnit, ExtensionToPixels},
        render::{Render, RenderType},
    },
    utils::{
        color::Color,
        extensions::usize::ExtensionIndices,
        sizes::ImageSizes,
        vec2d::Vec2d,
    },
};



#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GradientPoint<T: Copy> {
    pos: Vec2d<T>,
    sigma: T,
    color: Color,
}

impl<T: Copy> GradientPoint<T> {
    pub fn new(
        pos: Vec2d<T>,
        sigma: T,
        color: Color,
    ) -> Self {
        GradientPoint { pos, sigma, color }
    }
}



#[derive(Clone, Debug, PartialEq)]
pub struct Gradient {
    points: Vec<GradientPoint<MetricUnit>>,
    color: Color,
    is_fading: bool,
}

impl Gradient {
    pub const fn new(
        points: Vec<GradientPoint<MetricUnit>>,
        color: Color,
        is_fading: bool,
    ) -> Self {
        Gradient { points, color, is_fading }
    }
}

impl Render for Gradient {
    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas {
        let mut canvas: Canvas = Canvas::new(image_sizes);
        let (wmax, hmax): (usize, usize) = (image_sizes.w, image_sizes.h);
        let shift: Vec2d<f64> =
            - Vec2d::new(wmax as f64, hmax as f64) / 2.0_f64
            + Vec2d::new(0.5_f64, 0.5_f64);

        let points: Vec<GradientPoint<f64>> = self.points.iter()
            .map(|p: &GradientPoint<MetricUnit>|
                GradientPoint::<f64>::new(
                    Vec2d::<f64>::new(p.pos.x.to_pixels(wmax), -p.pos.y.to_pixels(hmax)),
                    p.sigma.to_pixels(hmax),
                    p.color
                )
            )
            .collect();

        fn gauss(dist2: f64, sigma: f64) -> f64 {
            (-dist2 / (2.0 * sigma.powi(2))).exp()
        }

        match render_type {
            RenderType::Cpu1 => {
                for h in hmax.indices() {
                    for w in wmax.indices() {
                        let pos: Vec2d<f64> = Vec2d::new(w as f64, h as f64);

                        let gausses: Vec<f64> = points.iter()
                            .map(|p| gauss((-p.pos+pos+shift).len2(), p.sigma))
                            .collect();
                        // assert!(gausses.iter().all(|&x| 0.0 <= x && x <= 1.0));
                        let gausses_sum: f64 = gausses.iter().sum();

                        // assert_eq!(points.len(), gausses.len()); // this obvious

                        // include fading:
                        let color: Color = if self.is_fading {
                            let color_without_fading: Color = Color::new(
                                points.iter().zip(&gausses).map(|(p, g)| g * (p.color.a as f64)).sum::<f64>().min(255.0) as u8,
                                points.iter().zip(&gausses).map(|(p, g)| g * (p.color.r as f64)).sum::<f64>().min(255.0) as u8,
                                points.iter().zip(&gausses).map(|(p, g)| g * (p.color.g as f64)).sum::<f64>().min(255.0) as u8,
                                points.iter().zip(&gausses).map(|(p, g)| g * (p.color.b as f64)).sum::<f64>().min(255.0) as u8,
                            );
                            let k: f64 = 1.0 - gausses_sum;
                            let fading_color: Color = Color::new(
                                (k * (self.color.a as f64)) as u8,
                                (k * (self.color.r as f64)) as u8,
                                (k * (self.color.g as f64)) as u8,
                                (k * (self.color.b as f64)) as u8,
                            );
                            Color::new(
                                color_without_fading.a + fading_color.a,
                                color_without_fading.r + fading_color.r,
                                color_without_fading.g + fading_color.g,
                                color_without_fading.b + fading_color.b,
                            )
                        }
                        else {
                            Color::new(
                                points.iter().zip(&gausses).map(|(p, g)| (g/gausses_sum) * (p.color.a as f64)).sum::<f64>().min(255.0) as u8,
                                points.iter().zip(&gausses).map(|(p, g)| (g/gausses_sum) * (p.color.r as f64)).sum::<f64>().min(255.0) as u8,
                                points.iter().zip(&gausses).map(|(p, g)| (g/gausses_sum) * (p.color.g as f64)).sum::<f64>().min(255.0) as u8,
                                points.iter().zip(&gausses).map(|(p, g)| (g/gausses_sum) * (p.color.b as f64)).sum::<f64>().min(255.0) as u8,
                            )
                        };

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
