mod ray;
mod camera;
mod film;
mod shape;
mod render;
mod material;

use glam::Vec3;
use camera::Camera;
use film::Color;
use shape::sphere::Sphere;

use crate::material::{Light, Material};



fn main() {
    // setup scene
    let material1 = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.1, 0.1, 0.1), 8.0, Vec3::ZERO);
    let material2 = Material::new(Vec3::new(0.8, 1.0, 0.0), Vec3::new(0.1, 0.1, 0.1), 8.0, Vec3::ZERO);
    let material3 = Material::new(Vec3::new(0.2, 0.0, 0.9), Vec3::new(0.1, 0.1, 0.1), 8.0, Vec3::ZERO);

    let sphere1 = Sphere::new(1.0, Vec3::new(1.0, 1.0, 1.0), material1);
    let sphere2 = Sphere::new(1.5, Vec3::new(3.0, 2.0, 4.0), material2);
    let sphere3 = Sphere::new(1.2, Vec3::new(4.0, 0.0, 6.0), material3);
    let spheres = vec![sphere1, sphere2, sphere3];
    
    // lights
    let light1 = Light::new(Vec3::new(0.0, 10.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    let lights = vec![light1];

    // angle 1
    let camera = Camera::new(Vec3::new(-4.0, 5.0, 3.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    1.5,
                                    90.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera, 1500, 1000, &spheres, &lights);

    film.save("images/angle1.png");


    // angle 2
    let camera = Camera::new(Vec3::new(2.0, 0.0, -5.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    1.5,
                                    90.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera, 1500, 1000, &spheres, &lights);

    film.save("images/angle2.png");


    // angle 3
    let camera = Camera::new(Vec3::new(3.0, 0.0, 10.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    1.5,
                                    90.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera, 1500, 1000, &spheres, &lights);

    film.save("images/angle3.png");

}

