//! SIVF Metric Units

use crate::sivf_misc::sizes::ImageSizes;



// TODO: convert from string expression: maybe use some lib
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MetricUnit {
    Pixels(f64),
    Percents(f64, Option<Axis>),
    // TODO LATER: add other metrics?
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis { X, Y }

pub trait ExtensionToPixels {
    fn to_pixels(&self, sizes: ImageSizes, current_axis: Axis) -> f64;
}

impl ExtensionToPixels for MetricUnit {
    fn to_pixels(&self, sizes: ImageSizes, current_axis: Axis) -> f64 {
        match self {
            MetricUnit::Pixels(pixels) => { *pixels }
            MetricUnit::Percents(percents, specific_axis) => {
                (match specific_axis {
                    None => {
                        match current_axis {
                            Axis::X => { sizes.w }
                            Axis::Y => { sizes.h }
                        }
                    }
                    Some(Axis::X) => {
                        sizes.w
                    }
                    Some(Axis::Y) => {
                        sizes.h
                    }
                } as f64) * (*percents) / 100.0_f64
            }
        }
    }
}

