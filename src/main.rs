extern crate image;
extern crate rand;
extern crate cgmath;
use cgmath::*;

/*-------------Enums--------------------*/
enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

enum Light {
    Directional(DirectionalLight),
    //Spherical(SphericalLight),
}

/*-------------Structs------------------*/
struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

struct DirectionalLight {
    direction: Vector3<f64>,
    color: Color,
    intensity: f64,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct Scene {
    background_color: Color,
    width: u32,
    height: u32,
    fov: f64,
    objects: Vec<Object>,
    light: Light
}

struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    color: Color,
    //albedo: f64,
}

struct Plane {
    origin: Vector3<f64>,
    normal: Vector3<f64>,
    color: Color,
    //albedo: f64,
}

/*-------------Traits-------------------*/

trait Intersectable {
    //fn distance(&self, ray: &Ray) -> f64;
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn get_pixel(&self, ray: &Ray) -> image::Rgb<u8>;
    fn surface_normal(&self, hit_pt: Vector3<f64>) -> Vector3<f64>;


}

/*-------------Implementations----------*/

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Object::Sphere(s) => s.intersect(ray),
            Object::Plane(p) => p.intersect(ray),
        }
    }

    fn get_pixel(&self, ray: &Ray) -> image::Rgb<u8> {
        match self {
            Object::Sphere(s) => s.get_pixel(ray),
            Object::Plane(p) => p.get_pixel(ray),
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
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal;
        let denom: f64 = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance)
            }
        }
        None
    }

    fn get_pixel(&self, ray: &Ray) -> image::Rgb<u8> {
        //let hit_point = ray.origin + (ray.direction * )

        image::Rgb([self.color.r, self.color.g, self.color.b])
    }

    fn surface_normal(&self, hit_pt: Vector3<f64>) -> Vector3<f64> {
        -self.normal
    } 

}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
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
            //more pythagorean theorem
            let adj2 = adj * adj;
            let dist = adj2 - (radius2 - d2);
            Some(dist)
        }   
    }

    fn get_pixel(&self, ray: &Ray) -> image::Rgb<u8> {
        image::Rgb([self.color.r, self.color.g, self.color.b])
    }

    fn surface_normal(&self, hit_pt: Vector3<f64>) -> Vector3<f64> {
        (hit_pt - self.center).normalize()
    }
}


impl Ray {
    fn create_prime_ray(x: u32, y: u32, scene: &Scene) -> Ray {
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


/*-------------Functions----------------*/


fn create_scene() -> Scene {
    let sphere1 = Sphere {
        center: cgmath::Vector3::new(-4.0, 2.0, -9.0),
        radius: 3.0,
        color: Color {
            r: 215,
            g: 138,      //pink
            b: 118,
        }
    };
    let sphere2 = Sphere {
        center: cgmath::Vector3::new(0.0, 2.0, -7.0),
        radius: 3.0,
        color: Color {
            r: 84,
            g: 54,      //wine
            b: 46,
        }
    };

    let plane1 = Plane {
        origin: cgmath::Vector3::new(0.0, -8.0, 0.0),
        normal: cgmath::Vector3::new(0.0, -7.0, 0.0),
        color: Color {
            r: 243,
            g: 202,      //yellow
            b: 64,
        },
    };

    let plane2 = Plane {
        origin: cgmath::Vector3::new(20.0, 0.0, 0.0),
        normal: cgmath::Vector3::new(10.0, 0.0, 0.0),
        color: Color {
            r: 78,
            g: 87,      //gray
            b: 94,
        },
    };

    let light1 = DirectionalLight {
        direction: cgmath::Vector3::new(10.0, -3.0, 5.0),
        color: Color {
            r: 153,
            g: 104,     //purple
            b: 136,
        },
        intensity: 200.0,
    };

    let scene = Scene {
        background_color: Color {
            r: 34,
            g: 46,      //navy
            b: 56,
        },
        width: 800,
        height: 600,
        fov: 90.0,
        objects: vec!(
            Object::Sphere(sphere1),
            Object::Sphere(sphere2),
            Object::Plane(plane1),
            Object::Plane(plane2),
        ),
        light: Light::Directional(light1),

    };
    scene
}

fn render(scene: &Scene) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>{
    let mut imgbuf = image::ImageBuffer::new(scene.width, scene.height);

    
    for (x, y, px) in imgbuf.enumerate_pixels_mut(){
        //create prime ray
        //if the prime ray intersects with the objects in the scene
        //*px = get pixel of object
        let mut rgb = image::Rgb([scene.background_color.r, scene.background_color.g, scene.background_color.b]);
        let mut min_dist: f64 = std::f64::INFINITY;
        let pr: Ray = Ray::create_prime_ray(x, y, scene);
        
        for obj in scene.objects.iter() {
            //let tmp = obj.get_pixel(&pr);

            match obj.intersect(&pr) {
                None => {},
                Some(a) => if a < min_dist {
                    min_dist = a;
                    rgb = obj.get_pixel(&pr);
                },
            }

            *px = rgb; 
        }
          
    }

    imgbuf
}

fn main() {

    println!("hallo");

    let scene = create_scene();

    let imgbuf = render(&scene);

    println!("finished rendering");

    imgbuf.save("output/thing.png").expect("saving image failed");

}