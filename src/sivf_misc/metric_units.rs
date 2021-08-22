//! SIVF Metric Units

use serde_derive::{Serialize, Deserialize};



// TODO: convert from string expression: maybe use some lib
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MetricUnit {
    Pixels(i32),
    Percents(f64),
    // TODO LATER: add other metrics?
}
