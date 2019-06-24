extern crate image;
extern crate rand;
extern crate cgmath;
use cgmath::*;

use crate::shapes::*;
use crate::scene::*;

/*-------------Structs------------------*/

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

/*-------------Traits-------------------*/
/*
pub trait Traceable {
    fn trace(&self, objects: Vec<Object>) -> image::Rgb<u8>;


    //prime_ray.trace(self.objects)

}
*/

/*-------------Implementations----------*/

impl Ray {
    pub fn create_prime_ray(x: u32, y: u32, scene: &Scene) -> Ray {
        //if the width is greater than the height, then the aspect ratio adjustment must be made
        assert!(scene.width > scene.height);
        let aspect_ratio = scene.width as f64 / scene.height as f64;
        let fov_adj = (scene.fov.to_radians() / 2.0).tan();
       
        let sensor_x = (((x as f64 + 0.5)/scene.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_adj;
        let sensor_y = (1.0 - ((y as f64 + 0.5)/scene.height as f64) * 2.0) * fov_adj;

        Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(sensor_x, sensor_y, -1.0).normalize(),
        }
    }
}

//impl Traceable for Ray {
    /*So that's it, huh? We're some kind of... Ray Tracer??*/
    

//}
