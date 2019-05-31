extern crate image;
extern crate rand;
extern crate cgmath;
use cgmath::*;


trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
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
}

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
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
    objects: Vec<Sphere>,

}

struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    color: Color,
}

fn create_scene() -> Scene {
    let scene = Scene {
        background_color: Color {
            r: 34,
            g: 46,              //blue-gray
            b: 56,
        },
        width: 800,
        height: 600,
        fov: 90.0,
        objects: vec!(
            Sphere {
                center: cgmath::Vector3::new(1.0, 0.0, -9.0),
                radius: 3.0,
                color: Color {
                    r: 95,
                    g: 65,      //orange
                    b: 25,
                }
            },
            
            Sphere {
                center: cgmath::Vector3::new(0.0, -2.0, -7.0),
                radius: 3.0,
                color: Color {
                    r: 84,
                    g: 54,      //wine
                    b: 46,
                }
            },
            ),

    };
    scene
}

fn render(scene: &Scene) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>{
    let mut imgbuf = image::ImageBuffer::new(scene.width, scene.height);

    
    for (x, y, px) in imgbuf.enumerate_pixels_mut(){
        //create prime ray
        //if the prime ray intersects with the objects in the scene
        //*px = image::Rgb([])
        let mut rgb = image::Rgb([scene.background_color.r, scene.background_color.g, scene.background_color.b]);
        let mut min_dist: f64 = std::f64::INFINITY;
        let pr: Ray = Ray::create_prime_ray(x, y, scene);
        
        for obj in scene.objects.iter() {
            let tmp = image::Rgb([obj.color.r, obj.color.g, obj.color.b]);

            match obj.intersect(&pr) {
                None => {},
                Some(a) => if a < min_dist {
                    min_dist = a;
                    rgb = tmp;
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