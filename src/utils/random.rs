//! Generates random numbers

use rand::{thread_rng, prelude::ThreadRng, Rng};



static mut RNG: Option<ThreadRng> = None;

/// This random is universal but maybe slow due to `.clone(…)`
pub fn random(min: f64, max: f64) -> f64 {
    unsafe {
        if RNG.is_none() {
            RNG = Some(thread_rng())
        }
        RNG.clone().unwrap().gen_range(min..max)
    }
}

