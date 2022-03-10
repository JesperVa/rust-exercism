// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (speed as f64 *221f64)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    match speed {
        0 => 0,
        1..=4 => prod_per(speed, 1f64),
        5..=8 => prod_per(speed, 0.9),
        9..=10 => prod_per(speed, 0.77),
        _ => unimplemented!()
    }
}

fn prod_per(speed: u8, percentage: f64) -> u32 {
    (production_rate_per_hour(speed)/percentage) as u32
}
