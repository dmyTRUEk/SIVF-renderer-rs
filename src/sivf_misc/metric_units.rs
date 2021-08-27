//! SIVF Metric Units



// TODO: convert from string expression: maybe use some lib
#[derive(Clone, Debug, PartialEq)]
pub enum MetricUnit {
    Pixels(f64),
    Percents(f64),
    // TODO LATER: add other metrics?
}

pub trait ExtensionToPixels {
    // TODO: ? change [f64] to [usize]
    fn to_pixels(&self, size_along_axis: f64) -> f64;
}

impl ExtensionToPixels for MetricUnit {
    fn to_pixels(&self, size_along_axis: f64) -> f64 {
        match self {
            MetricUnit::Pixels(pixels) => { *pixels }
            MetricUnit::Percents(percents) => {
                size_along_axis * percents / 100.0
            }
        }
    }
}
