use crate::constants::Vec3;

pub fn write_colour(colour: Vec3) {
    let norm: f64 = 255.999;

    let r: f64 = *colour.x();
    let g: f64 = *colour.y();
    let b: f64 = *colour.z();

    let rb = (norm * r) as u8;
    let gb = (norm * g) as u8;
    let bb = (norm * b) as u8;

    println!("{rb} {gb} {bb}");
}
