use log::{ info };
use std::io;
use std::io::Write;

const WIDTH:  i32 = 256;
const HEIGHT: i32 = 256;
const NORM:   f32 = 255.999;  

fn main() {
    println!("P3\n{WIDTH} {HEIGHT}\n255");

    for j in 0..WIDTH {
        info!("\rScanlines remaining: {}", HEIGHT - j);
        match io::stderr().flush() { _ => () };
        for i in 0..HEIGHT {
            let r = (i as f32) / ((WIDTH - 1) as f32);
            let g = (j as f32) / ((WIDTH - 1) as f32);
            let b = 0.0; 

            let ir: i32 = (NORM * r) as i32;
            let ig: i32 = (NORM * g) as i32;
            let ib: i32 = (NORM * b) as i32;

            println!("{ir} {ig} {ib}")
        }
    }

    info!("\rDone. \n")
}
