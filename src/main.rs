extern crate image;
extern crate rand;
extern crate cgmath;

mod rays;
mod scene;
mod shapes;


use crate::shapes::*;
use crate::scene::*;

fn create_scene() -> Scene {
    //behind sphere
    let sphere1 = Sphere {
        center: cgmath::Vector3::new(-4.0, 3.0, -9.0),
        //center: cgmath::Vector3::new(0.0, 2.0, -4.0),
        radius: 3.0,
        /*
        color: Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        },
        */
        
        color: Color {
            r: 0.84,
            g: 0.54,      //pink
            b: 0.46,
        },
        
        albedo: 0.4,
    };
    
    //in front sphere
    let sphere2 = Sphere {
        //center: cgmath::Vector3::new(0.0, 2.0, -7.0),
        center: cgmath::Vector3::new(0.0, 2.0, -7.0),
        radius: 3.0,
        /*
        color: Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        },
        */
        /*
        color: Color {
            r: 0.32,
            g: 0.21,      //wine
            b: 0.18,
        },
        */

        color: Color {
            r: 0.07,
            g: 0.313,      //wine
            b: 0.35,
        },

        albedo: 0.4,
    };

    //bottom
    let plane1 = Plane {
        origin: cgmath::Vector3::new(0.0, -8.0, 0.0),
        normal: cgmath::Vector3::new(0.0, -7.0, 0.0),
        color: Color {
            /*
            r: 0.95,
            g: 0.79,      //yellow
            b: 0.25,
            */
            r: 0.80,
            g: 0.84,      //gray
            b: 0.86,
        },
        albedo: 0.05,
    };

    //right plane
    let plane2 = Plane {
        origin: cgmath::Vector3::new(18.0, 0.0, 0.0),
        normal: cgmath::Vector3::new(6.0, 0.0, 0.0),
        color: Color {
            /*
            r: 0.0,
            g: 0.0,
            b: 1.0,
            */
            
            r: 0.29,
            g: 0.3921,      //green
            b: 0.1921,
            
            
            /*
            r: 0.30,
            g: 0.34,      //gray
            b: 0.36,
            */
        },
        albedo: 0.1,
    };

    let light1 = DirectionalLight {
        //direction: cgmath::Vector3::new(2.0, -10.0, 4.0),
        direction: cgmath::Vector3::new(4.0, -9.0, 0.0),
        //direction: cgmath::Vector3::new(2.0, 1.0, -1.0),

        color: Color {
            
            r: 1.0,
            g: 1.0,
            b: 1.0,
            
            /*
            r: 0.60,
            g: 0.40,     //purple
            b: 0.53,
            */
        },
        intensity: 7.0,
    };

    let scene = Scene {
        background_color: Color {
            r: 0.13,    
            g: 0.18,      //navy
            b: 0.216,
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
        //lights: vec!(Light::Directional(light1),),
        light: light1,
    };
    scene
}


fn main() {

    println!("hallo");

    let mut scene = create_scene();

    let imgbuf = scene.render();

    println!("finished rendering");

    imgbuf.save("output/thing.png").expect("saving image failed");

}