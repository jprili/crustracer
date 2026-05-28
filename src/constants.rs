pub use std::f64::{ consts };

pub use rand::RngExt;

pub use crate::vec3::Vec3;
pub use crate::ray::Ray;
pub use crate::interval::Interval;

pub const PI:  f64 = consts::PI;
pub const INF: f64 = f64::INFINITY;

#[inline]
pub fn deg_to_rad(deg: f64) -> f64 {
    (deg * PI) / 180.0
}

/* 
Generates a random f64 in the range
[0.0, 1.0]
*/
#[inline]
pub fn rand_unit() -> f64 {
    rand::rng().random_range(0.0..=1.)
}

/* 
Generates a random f64 in the range
[lo, hi]
*/
#[inline]
pub fn rand_range(lo: f64, hi: f64) -> f64 {
    rand::rng().random_range(lo..=hi)
}