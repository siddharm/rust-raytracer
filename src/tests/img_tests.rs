extern crate image;
extern crate rand;
use std::fs::File;
//use rand::prelude::*;
use image::*;
use rand::distributions::{Normal, Distribution};



fn main() {
    println!("hallo welt");
    
    let mut img = image::open("src/in.jpg").expect("opening image failed");
    //img.invert();
    let filtered = img;
    println!("got here");
    let mut out = File::create("out.png").unwrap();
    let (width, height) = filtered.dimensions();
    let mut rng = rand::thread_rng();
    let normal = Normal::new(15.0, 15.0);
    let mut noisy = filtered.brighten(-3);

    for x in 0..(width) {
        for y in 0..(height) {
            let offset = normal.sample(&mut rng) as u8;
            let px = noisy.get_pixel(x, y).map(|v| if v <= 255 - offset {v + offset} else {255});
            noisy.put_pixel(x, y, px);
        }
    }
    
    noisy.save("out.png").expect("saving image failed");
}