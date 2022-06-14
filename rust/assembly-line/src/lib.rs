// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PER_HOUR: u8 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let cars = speed as u32 * CARS_PER_HOUR as u32;

    match speed {
        0..=4 => cars as f64,
        5..=8 => cars as f64 * 0.9, 
        9..=10 => cars as f64 * 0.77,
        _ => panic!("invalid speed"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate_per_hour = production_rate_per_hour(speed);
    (rate_per_hour / 60.0) as u32
}
