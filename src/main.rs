extern crate image;
extern crate rand;
extern crate cgmath;
use cgmath::*;

mod lib;
use crate::lib::*;



/*-------------Functions----------------*/


fn create_scene() -> Scene {
    let sphere1 = Sphere {
        center: cgmath::Vector3::new(-4.0, 2.0, -9.0),
        radius: 3.0,
        color: Color {
            r: 0.84,
            g: 0.54,      //pink
            b: 0.46,
        },
        albedo: 0.5,
    };
    let sphere2 = Sphere {
        center: cgmath::Vector3::new(0.0, 2.0, -7.0),
        radius: 3.0,
        color: Color {
            r: 0.32,
            g: 0.21,      //wine
            b: 0.18,
        },
        albedo: 0.5,
    };

    let plane1 = Plane {
        origin: cgmath::Vector3::new(0.0, -8.0, 0.0),
        normal: cgmath::Vector3::new(0.0, -7.0, 0.0),
        color: Color {
            r: 0.95,
            g: 0.79,      //yellow
            b: 0.25,
        },
        albedo: 0.5,
    };

    let plane2 = Plane {
        origin: cgmath::Vector3::new(20.0, 0.0, 0.0),
        normal: cgmath::Vector3::new(10.0, 0.0, 0.0),
        color: Color {
            r: 0.30,
            g: 0.34,      //gray
            b: 0.36,
        },
        albedo: 0.5,
    };

    let light1 = DirectionalLight {
        direction: cgmath::Vector3::new(10.0, -3.0, 5.0),
        //direction: cgmath::Vector3::new(-10.0, 0.0, 0.0),
        color: Color {
            r: 0.60,
            g: 0.40,     //purple
            b: 0.53,
        },
        intensity: 200.0,
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