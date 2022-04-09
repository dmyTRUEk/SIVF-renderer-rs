//! Date and Time extensions

use std::time::Instant;
use chrono::{DateTime, Local};



pub trait ExtensionDateTimeLocalToMyFormat {
    fn to_my_format(&self) -> String;
}
impl ExtensionDateTimeLocalToMyFormat for DateTime<Local> {
    fn to_my_format(&self) -> String {
        let year  : u32 = self.format("%Y").to_string().parse().unwrap();
        let month : u32 = self.format("%m").to_string().parse().unwrap();
        let day   : u32 = self.format("%d").to_string().parse().unwrap();
        let hour  : u32 = self.format("%H").to_string().parse().unwrap();
        let minute: u32 = self.format("%M").to_string().parse().unwrap();
        let second: u32 = self.format("%S").to_string().parse().unwrap();
        let ms    : u32 = self.format("%3f").to_string().parse().unwrap();
        format!("{year:04}_{month:02}_{day:02}__{hour:02}_{minute:02}_{second:02}_{ms:03}")
        // self.format("%Y_%m_%d__%H_%M_%S_%3f").to_string()
    }
}



/// Measure seconds elapsed for given code
///
/// # Example:
///
/// ```
/// let time = measure_time(|| {
///     sleep(Duration::from_secs(1));
/// });
/// println!("time = {time:.3}");   // approximately `1.000`
/// ```
pub fn measure_time(mut f: impl FnMut()) -> f64 {
    let start = Instant::now();
    f();
    let end = Instant::now();
    (end - start).as_nanos() as f64 / 1_000_000_000_f64
}



// fn get_date_time_my_format_string() -> String {
//     let dt_now = Local::now();
//     // println!("Time now is {dt_now}");
//     let year  : u32 = dt_now.format("%Y").to_string().parse().unwrap();
//     let month : u32 = dt_now.format("%m").to_string().parse().unwrap();
//     let day   : u32 = dt_now.format("%d").to_string().parse().unwrap();
//     let hour  : u32 = dt_now.format("%H").to_string().parse().unwrap();
//     let minute: u32 = dt_now.format("%M").to_string().parse().unwrap();
//     let second: u32 = dt_now.format("%S").to_string().parse().unwrap();
//     format!("{:04}_{:02}_{:02}__{:02}_{:02}_{:02}", year, month, day, hour, minute, second)
// }

