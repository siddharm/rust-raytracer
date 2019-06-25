extern crate image;
extern crate rand;
extern crate cgmath;
use cgmath::*;

use crate::rays::*;
use crate::shapes::*;

/*-------------Enums--------------------*/

/*
enum Light {
    Directional(DirectionalLight),
    //Spherical(SphericalLight),
}
*/

/*-------------Structs------------------*/

pub struct DirectionalLight {
    pub direction: Vector3<f64>,
    pub color: Color,
    pub intensity: f64,
}

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub struct Scene {
    pub background_color: Color,
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Object>,
    pub light: DirectionalLight,
}

/*-------------Traits-------------------*/

pub trait Renderable {
    fn render(&mut self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>;
}

/*-------------Implementations----------*/

impl std::ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, _rhs: f64) -> Color {

        Color {
            r: self.r * _rhs,
            g: self.g * _rhs,
            b: self.b * _rhs,
        }
    }
}

impl std::ops::Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, _rhs: &Color) -> Color {

        Color {
            r: self.r * _rhs.r,
            g: self.g * _rhs.g,
            b: self.b * _rhs.b,
        }
    }
}

impl Color {
    pub fn clamp(self) -> Color {
        Color {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
        }
    }
}

impl Renderable for Scene {
    fn render(&mut self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);

        for (x, y, px) in imgbuf.enumerate_pixels_mut() {
            //create prime ray
            //if the prime ray intersects with the objects in the scene
            //*px = get pixel of object
            let pr: Ray = Ray::create_prime_ray(x, y, self);
            let hit_obj = pr.trace(&self);

            match hit_obj {
                None => *px = image::Rgb([(255.0 * self.background_color.r) as u8,
                                        (255.0 * self.background_color.g) as u8,
                                        (255.0 * self.background_color.b) as u8]),
                Some(a) => *px = a,
            }
        }

        imgbuf
    }
}
