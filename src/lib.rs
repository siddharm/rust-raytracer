extern crate image;
extern crate rand;
extern crate cgmath;
use cgmath::*;


/*-------------Enums--------------------*/
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}
/*
enum Light {
    Directional(DirectionalLight),
    //Spherical(SphericalLight),
}
*/
/*-------------Structs------------------*/
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

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
    //returns tple of distance to the intersection and the point of intersection
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)>;
    //returns an RGB struct of the desired pixel
    fn get_pixel(&mut self, hit_point: Vector3<f64>, light: &DirectionalLight) -> image::Rgb<u8>;
    //returns a vector representing the surface normal of the object
    fn surface_normal(&self, hit_point: Vector3<f64>) -> Vector3<f64>;
}

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


impl Renderable for Scene {
    fn render(&mut self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);

        for (x, y, px) in imgbuf.enumerate_pixels_mut() {
            //create prime ray
            //if the prime ray intersects with the objects in the scene
            //*px = get pixel of object
            let mut rgb = image::Rgb([(255.0 * self.background_color.r) as u8, (255.0 * self.background_color.g) as u8, (255.0 * self.background_color.b) as u8]);
            let mut min_dist: f64 = std::f64::INFINITY;
            let pr: Ray = Ray::create_prime_ray(x, y, self);
        
            for obj in &mut self.objects {
                //let tmp = obj.get_pixel(&pr);

                match obj.intersect(&pr) {
                    None => {},
                    Some((dist, hit_pt)) => if dist < min_dist {
                        min_dist = dist;
                        rgb = obj.get_pixel(hit_pt, &self.light);
                    },
                }

                *px = rgb; 
            }
          
        }

        imgbuf
    }
}


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
        let direction_to_light = -light.direction;
        let light_power = (surface_normal.dot(direction_to_light)).max(0.0) * light.intensity;
        let light_reflected = self.albedo / std::f64::consts::PI;
        
        let color = &(&(&self.color * &light.color) * light_power) * light_reflected;

        //let color = &self.color * &light.color * light_power * light_reflected;
        
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
        
        let color = &(&(&self.color * &light.color) * light_power) * light_reflected;

        //let color = &self.color * &light.color * light_power * light_reflected;

        //let color = &self.color;
        
        image::Rgb([(255.0 * color.r) as u8, (255.0 * color.g) as u8, (255.0 * color.b) as u8])
    }

    fn surface_normal(&self, hit_pt: Vector3<f64>) -> Vector3<f64> {
        (hit_pt - self.center).normalize()
    }
}

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
