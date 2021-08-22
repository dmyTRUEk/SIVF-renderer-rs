//! SIVF Metric Units



#[derive(Clone, Debug)]
pub enum MetricUnit {
    Pixels(i32),
    Percents(f64),
    // TODO LATER: add other metrics?
}
