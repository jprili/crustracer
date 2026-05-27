pub use std::f64::{ consts };

pub use crate::vec3::Vec3;
pub use crate::ray::Ray;

pub const PI:  f64 = consts::PI;
pub const INF: f64 = f64::INFINITY;

pub fn deg_to_rad(deg: f64) -> f64 {
    (deg * PI) / 180.0
}