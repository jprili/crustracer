pub use std::f64::{ consts, INFINITY };

pub use crate::vec3::Vec3;
pub use crate::ray::Ray;

pub const PI: f64 = consts::PI;

pub fn deg_to_rad(deg: f64) -> f64 {
    (deg * PI) / 180.0
}