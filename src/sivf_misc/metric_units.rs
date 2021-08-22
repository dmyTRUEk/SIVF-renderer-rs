//! SIVF Metric Units

use serde_derive::{Serialize, Deserialize};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MetricUnit {
    Pixels(i32),
    Percents(f64),
    // TODO LATER: add other metrics?
}
