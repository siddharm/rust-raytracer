extern crate image;
extern crate rand;
extern crate cgmath;
use cgmath::*;

use crate::rays::*;
use crate::scene::*;

/*-------------Enums--------------------*/

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

/*-------------Structs------------------*/

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub color: Color,
    pub albedo: f64,
}

pub struct Plane {
    pub origin: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub color: Color,
    pub albedo: f64,
}

/*-------------Traits-------------------*/

pub trait Intersectable {
    //returns tuple of distance to the intersection and the point of intersection
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)>;
    //returns an RGB struct of the desired pixel
    fn get_pixel(&mut self, hit_point: Vector3<f64>, light: &DirectionalLight) -> image::Rgb<u8>;
    //returns a vector representing the surface normal of the object
    fn surface_normal(&self, hit_point: Vector3<f64>) -> Vector3<f64>;
}

/*-------------Implementations----------*/

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        match self {
            Object::Sphere(s) => s.intersect(ray),
            Object::Plane(p) => p.intersect(ray),
        }
    }

    fn get_pixel(&mut self, hit_point: Vector3<f64>, light: &DirectionalLight) -> image::Rgb<u8> {

        match self {
            Object::Sphere(s) => s.get_pixel(hit_point, light),
            Object::Plane(p) => p.get_pixel(hit_point, light),
        }
        
    }

    fn surface_normal(&self, hit_pt: Vector3<f64>) -> Vector3<f64> {
        match self {
            Object::Sphere(s) => s.surface_normal(hit_pt),
            Object::Plane(p) => p.surface_normal(hit_pt),
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        let normal = self.normal;
        let denom: f64 = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                let hit_point = ray.origin + (ray.direction * distance);
                let tup = (distance, hit_point);
                return Some(tup)
            }
        }
        None
    }

    fn get_pixel(&mut self, hit_point: Vector3<f64>, light: &DirectionalLight) -> image::Rgb<u8> {
        let surface_normal = self.surface_normal(hit_point);
        let direction_to_light = -light.direction.normalize();
        let light_power = (direction_to_light.dot(surface_normal)).max(0.0) * light.intensity;
        let light_reflected = self.albedo / std::f64::consts::PI;


        let shadow_ray = Ray {
            origin: hit_point,
            direction: direction_to_light,
        };

        //fn takes shadow ray and scene
        //returns none or some
        //if some: do shadow

        //let illuminated = scene.trace(&shadow_ray);

        
        let color = (&(&(&self.color * &light.color) * light_power) * light_reflected).clamp();

        //let color = &self.color;
        
        image::Rgb([(255.0 * color.r) as u8, (255.0 * color.g) as u8, (255.0 * color.b) as u8])
        
    }

    fn surface_normal(&self, hit_pt: Vector3<f64>) -> Vector3<f64> {
        -self.normal
    } 

}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        //vector from ray origin to sphere center
        let ray_center = self.center - ray.origin;
        //vec from ray origin to point perpendicular  
        let adj = ray_center.dot(ray.direction);
        //does pythagorean theorem
        let d2 = ray_center.dot(ray_center) - (adj * adj);
        
        let radius2 = self.radius * self.radius;

        if d2 > radius2 {
            //println!("1");
            None
        } else {
            //more pythagorean theorem below
            let adj2 = adj * adj;
            let dist = adj2 - (radius2 - d2);
            let hit_point = ray.origin + (ray.direction * dist);
            let tup = (dist, hit_point);

            Some(tup)
        }   
    }

    fn get_pixel(&mut self, hit_point: Vector3<f64>, light: &DirectionalLight) -> image::Rgb<u8> {
        
        let surface_normal = self.surface_normal(hit_point);
        let direction_to_light = -light.direction;
        let light_power = (surface_normal.dot(direction_to_light)).max(0.0) * light.intensity;
        let light_reflected = self.albedo / std::f64::consts::PI;


        let shadow_ray = Ray {
            origin: hit_point,
            direction: direction_to_light,
        };

        //fn takes shadow ray and scene
        //returns none or some
        //if some: do shadow

        //let illuminated = scene.trace(&shadow_ray);

        
        let color = (&(&(&self.color * &light.color) * light_power) * light_reflected).clamp();

        //let color = &self.color;
        
        image::Rgb([(255.0 * color.r) as u8, (255.0 * color.g) as u8, (255.0 * color.b) as u8])
    }

    fn surface_normal(&self, hit_pt: Vector3<f64>) -> Vector3<f64> {
        (hit_pt - self.center).normalize()
    }
}