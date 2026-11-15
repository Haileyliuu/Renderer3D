use glam::Vec3;
use rand::Rng;

use crate::{camera::Camera, material::{Light, Material}, scenes::scene::Scene, shape::{Shape, sphere::Sphere}, texture::Texture};

pub fn create() -> Result<Scene, Box<dyn std::error::Error>> {
    // height and width of png
    let aspect = 1.333;
    
    let white_light = Light::new(Vec3::new(5.0, 10.0, 5.0), Vec3::new(1.0, 1.0, 1.0)); 
    let lights = vec![white_light];
    
    let earth_texture = Texture::load_texture("textures/earth-map.jpg").expect("Could not load earth texture");;
    let earth_material = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.3, 0.3, 0.3), 100.0, Some(earth_texture));
    let earth = Sphere::new(2.5, Vec3::new(0.0, 2.5, 0.0), earth_material);
    let mut shapes: Vec<Box<dyn Shape>> = vec![Box::new(earth)];
    
    let star_material = Material::new(Vec3::new(10.0, 10.0, 10.0), Vec3::ZERO, 100.0, None);
    add_stars(&mut shapes, star_material);

    // angle 1
    let camera1 = Camera::new(Vec3::new(0.0, 8.0, 8.0), 
                                    Vec3::new(0.0, 2.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    // angle 2
    let camera2 = Camera::new(Vec3::new(8.0, 5.0, 6.0), 
                                    Vec3::new(0.0, 2.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    // angle 3
    let camera3 = Camera::new(Vec3::new(0.0, 5.0, -10.0), 
                                    Vec3::new(0.0, 2.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let cameras = vec![camera1, camera2, camera3];
    Ok(Scene {cameras, shapes, lights})
}

fn add_stars(shapes: &mut Vec<Box<dyn Shape>>, star_material: Material) {
    let mut rng = rand::thread_rng();

    for _ in 0..500 {
        let position = Vec3::new(
            rng.gen_range(-20.0..20.0),
            rng.gen_range(-20.0..20.0),
            rng.gen_range(-20.0..20.0),
        );

        let star = Sphere::new(0.05, position, star_material.clone());

        shapes.push(Box::new(star));
    }
}