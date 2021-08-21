//! Date and Time utils

extern crate chrono;

use chrono::{ DateTime, Local };



pub trait TraitDateTimeLocalToMyFormat {
    fn to_my_format(&self) -> String;
}
impl TraitDateTimeLocalToMyFormat for DateTime<Local> {
    fn to_my_format(&self) -> String {
        let year  : u32 = dt_now.format("%Y").to_string().parse().unwrap();
        let month : u32 = dt_now.format("%m").to_string().parse().unwrap();
        let day   : u32 = dt_now.format("%d").to_string().parse().unwrap();
        let hour  : u32 = dt_now.format("%H").to_string().parse().unwrap();
        let minute: u32 = dt_now.format("%M").to_string().parse().unwrap();
        let second: u32 = dt_now.format("%S").to_string().parse().unwrap();
        format!("{:04}_{:02}_{:02}__{:02}_{:02}_{:02}", year, month, day, hour, minute, second)
    }
}



// fn get_date_time_my_format_string() -> String {
//     let dt_now = Local::now();
//     // println!("Time now is {}", dt_now);
//     let year  : u32 = dt_now.format("%Y").to_string().parse().unwrap();
//     let month : u32 = dt_now.format("%m").to_string().parse().unwrap();
//     let day   : u32 = dt_now.format("%d").to_string().parse().unwrap();
//     let hour  : u32 = dt_now.format("%H").to_string().parse().unwrap();
//     let minute: u32 = dt_now.format("%M").to_string().parse().unwrap();
//     let second: u32 = dt_now.format("%S").to_string().parse().unwrap();
//     format!("{:04}_{:02}_{:02}__{:02}_{:02}_{:02}", year, month, day, hour, minute, second)
// }
