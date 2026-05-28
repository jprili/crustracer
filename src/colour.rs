use crate::constants::Vec3;
use crate::interval::Interval;

const INTENSITY: Interval = Interval { 
    min: 0.0, max: 0.999 
};

pub fn write_colour(colour: Vec3) {
    let r: f64 = *colour.x();
    let g: f64 = *colour.y();
    let b: f64 = *colour.z();

    let norm= 256.;
    let rb = (norm * INTENSITY.clamp(r)) as u8;
    let gb = (norm * INTENSITY.clamp(g)) as u8;
    let bb = (norm * INTENSITY.clamp(b)) as u8;

    println!("{rb} {gb} {bb}");
}
