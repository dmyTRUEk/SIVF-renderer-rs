//! SIVF Metric Units



// TODO: convert from string expression: maybe use some lib
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MetricUnit {
    Pixels(f64),
    Percents(f64),
    // TODO LATER: add other metrics?
}

pub trait ExtensionToPixels {
    fn to_pixels(&self, size_along_axis: usize) -> f64;
}

impl ExtensionToPixels for MetricUnit {
    fn to_pixels(&self, size_along_axis: usize) -> f64 {
        match self {
            MetricUnit::Pixels(pixels) => { *pixels }
            MetricUnit::Percents(percents) => {
                (size_along_axis as f64) * (*percents) / 100.0_f64
            }
        }
    }
}

