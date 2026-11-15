use glam::Vec3;

use crate::{camera::Camera, material::Light, obj_loader, scenes::scene::Scene, shape::Shape};

pub fn create() -> Result<Scene, Box<dyn std::error::Error>> {
    // height and width of png
    let aspect = 1.333;

    // shapes
    let blueberry = obj_loader::load_obj_texture("models/blueberry.obj", "textures/Blueberry_BaseColor.png").expect("Could not load OBJ file");
    let table_vase = obj_loader::load_obj("models/table_vase.obj").expect("Could not load OBJ file");
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(table_vase)];
    
    // lights
    let white_light = Light::new(Vec3::new(-2.0, 10.0, -6.0), Vec3::new(1.0, 1.0, 1.0)); // white light
    let blue_light = Light::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.1, 0.1, 0.5)); // blue light
    let pink_light = Light::new(Vec3::new(4.0, 2.0, 1.0), Vec3::new(0.5, 0.3, 0.4)); // pink light
    let lights = vec![white_light, pink_light, blue_light];
    
    // angle 1
    let camera1 = Camera::new(Vec3::new(0.09, 3.42, -5.42), 
                                    Vec3::new(0.0, 2.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    // angle 2
    let camera2 = Camera::new(Vec3::new(5.0, 5.8, 1.0), 
                                    Vec3::new(0.0, 2.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    // angle 3
    let camera3 = Camera::new(Vec3::new(-5.09, 6.74, 5.0), 
                                    Vec3::new(0.0, 2.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let cameras = vec![camera1, camera2, camera3];
    Ok(Scene {cameras, shapes, lights})
}