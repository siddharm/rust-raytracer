extern crate image;

use cgmath::*;
use image::*;
use image::Pixel::*;
use std::*;

struct Point {
    x: f64,
    y: f64,
    z: f64,
}


struct Color {
    red: f32,
    blue: f32,
    green: f32,
}

struct Sphere {
    center: Point,
    radius: f64,
    color: Color,

}

struct Scene {
    height: u32,
    width: u32,
    fov: f64,
    sphere: Sphere,
}

fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0,0,0,0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, to_rgba(&scene.sphere.color))
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image

}

fn test_can_render_scene(){
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                red: 0.2,
                green: 0.7,
                blue: 0.5
            },
        },
    };
    
    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width())
}

struct Ray {
    origin: Point,
    direction: Vector3<f64>,
}

trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        //hypotenuse
        let l: Vector3<f64> = self.center - ray.origin;
        let adj2 = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj2 * adj2);
        d2 < (self.radius * self.radius)
    }
}

impl Ray {
    fn create_prime(x: u32, y:u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adj = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = scene.width as f64 / scene.height as f64;
        let sensor_x = (((x as f64 + 0.5)/scene.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_adj;
        let sensor_y = (1.0 -((y as f64 + 0.5)/scene.height as f64) * 2.0) * fov_adj;
        Ray {
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }.normalize(),
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}


fn main() {
    println!("Hello, world!");
    test_can_render_scene();
    println!("got here");
}
