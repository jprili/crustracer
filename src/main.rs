use log::{ info };
use std::io;
use std::io::Write;

mod vec3;
mod colour;

use vec3::{ Vec3 }; 

const WIDTH:  i32 = 256;
const HEIGHT: i32 = 256;

fn main() {
    println!("P3\n{WIDTH} {HEIGHT}\n255");

    for j in 0..WIDTH {
        info!("\rScanlines remaining: {}", HEIGHT - j);
        match io::stderr().flush() { _ => () };
        for i in 0..HEIGHT {
            colour::write_colour(
                Vec3::new(
                    i as f64 / ((WIDTH  - 1) as f64),
                    j as f64 / ((HEIGHT - 1) as f64),
                    0.0
                )
            );
        }
    }

    info!("\rDone. \n")
}
