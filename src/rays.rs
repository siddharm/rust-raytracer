extern crate cgmath;
extern crate image;
extern crate rand;
use cgmath::*;

use crate::scene::*;
use crate::shapes::*;

/*-------------Structs------------------*/

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

/*-------------Traits-------------------*/

pub trait Traceable {
    fn trace<'a>(&'a self, scene: &'a Scene) -> Option<(&Object, Vector3<f64>)>;
}

/*-------------Implementations----------*/

impl Ray {
    pub fn create_prime_ray(x: u32, y: u32, scene: &Scene) -> Ray {
        //if the width is greater than the height, then the aspect ratio adjustment must be made
        assert!(scene.width > scene.height);
        let aspect_ratio = f64::from(scene.width) / f64::from(scene.height);
        let fov_adj = (scene.fov.to_radians() / 2.0).tan();

        let sensor_x =
            (((f64::from(x) + 0.5) / f64::from(scene.width)) * 2.0 - 1.0) * aspect_ratio * fov_adj;
        let sensor_y = (1.0 - ((f64::from(y) + 0.5) / f64::from(scene.height)) * 2.0) * fov_adj;

        Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(sensor_x, sensor_y, -1.0).normalize(),
        }
    }
}

impl Traceable for Ray {
    /*So that's it, huh? We're some kind of... Ray Tracer??*/

    fn trace<'a>(&'a self, scene: &'a Scene) -> Option<(&Object, Vector3<f64>)> {
        //fn trace(&self, scene: &Scene) -> Option<image::Rgb<u8>> {

        let mut ans = None;

        let mut min_dist: f64 = std::f64::INFINITY;
        //let mut min_hit_pt: Vector3<f64> = Vector3::new();

        for obj in scene.objects.iter() {
            match obj.intersect(&self) {
                None => {}
                Some((dist, hit_pt)) if dist < min_dist => {
                    min_dist = dist;
                    ans = Some((obj, hit_pt));
                    //ans = Some(obj.get_pixel(&hit_pt, &scene));
                }
                Some(_) => {}
            }
        }
        //print!("{}\n", min_dist);
        //if min_dist <= 9.0 {
        //  print!("same");
        //}
        ans
    }
}
