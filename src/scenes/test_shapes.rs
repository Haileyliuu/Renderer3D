use glam::Vec3;

use crate::{camera::Camera, film::Color, material::{Light, Material}, obj_loader, render, scenes::scene::Scene, shape::{Shape, plane::Plane, polymesh::PolyMesh, sphere::Sphere}, texture::Texture};

pub fn create() -> Result<Scene, Box<dyn std::error::Error>> {
    // height and width of png
    let aspect = 1.333;
    let width = 1024;
    let height = 768;

    // materials
    let red = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.3, 0.3, 0.3), 100.0, None);
    let yellow = Material::new(Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.3, 0.3, 0.3), 100.0, None);
    let blue = Material::new(Vec3::new(0.2, 0.0, 0.9), Vec3::new(0.3, 0.3, 0.3), 100.0, None);
    let green = Material::new(Vec3::new(0.2, 0.8, 0.2), Vec3::new(0.3, 0.3, 0.3), 100.0, None);
    let light_gray = Material::new(Vec3::new(0.7, 0.7, 0.7), Vec3::new(0.1, 0.1, 0.1), 8.0, None);

    let shiny1 = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.5, 0.5, 0.5), 10.0, None);
    let shiny2 = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.5, 0.5, 0.5), 100.0, None);
    let shiny3 = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.5, 0.5, 0.5), 1000.0, None);
    let shiny4 = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.5, 0.5, 0.5), 10000.0, None);


    // shapes
    let sphere1 = Sphere::new(1.2, Vec3::new(-6.0, 1.0, 0.0), red.clone());
    let sphere2 = Sphere::new(1.2, Vec3::new(-2.0, 1.0, 0.0), yellow.clone());
    let sphere3 = Sphere::new(1.2, Vec3::new(2.0, 1.0, 0.0), blue.clone());
    let sphere4 = Sphere::new(1.2, Vec3::new(6.0, 1.0, 0.0), green.clone());
    

    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), light_gray.clone());

    let pyramid_vertices = vec![
        Vec3::new(-1.0, 0.0,  1.0), 
        Vec3::new( 1.0, 0.0,  1.0), 
        Vec3::new( 1.0, 0.0, -1.0), 
        Vec3::new(-1.0, 0.0, -1.0),
        Vec3::new( 0.0, 1.5,  0.0),
    ];

    let pyramid_triangles = vec![
        [0, 3, 2], [0, 2, 1], // base
        [0, 1, 4], [1, 2, 4], [2, 3, 4], [3, 0, 4], // sides
    ];
    let pyramid = PolyMesh::new(pyramid_vertices, pyramid_triangles, Vec::new(), Vec::new(), Vec::new(), Vec::new(),red.clone());

    let smooth_vertices = vec![
        Vec3::new(-1.0, 1.0, 4.0),
        Vec3::new( 1.0, 0.0, 4.0),
        Vec3::new( -1.0, 0.0, 2.0),
        Vec3::new( 1.0, 1.0, 2.0),
        Vec3::new( 1.0, 1.0, 6.0),
    ];

    let smooth_triangles = vec![
        [0, 1, 3],
        [0, 3, 2],
        [0, 4, 1],
    ];

    let smooth_normals = vec![
        Vec3::new(1.0, 1.0, 1.0).normalize(), // 0, 1, 3
        Vec3::new(-1.0, 1.0, -1.0).normalize(), // 2
        Vec3::new(1.0, 1.0, -1.0).normalize(), // 4
    ];

    let smooth_normals_indices = vec![
        [0, 0, 0],
        [0, 0, 1],
        [0, 2, 0],
    ];

    let smooth_mesh = PolyMesh::new(smooth_vertices, smooth_triangles, smooth_normals, smooth_normals_indices, Vec::new(),Vec::new(),yellow.clone());

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(plane), Box::new(sphere1), Box::new(sphere2), Box::new(sphere3), Box::new(sphere4), Box::new(pyramid), Box::new(smooth_mesh)];
    
    // lights
    let white_light = Light::new(Vec3::new(-4.0, 10.0, 0.0), Vec3::new(0.8, 0.8, 0.8)); // white light
    let red_light = Light::new(Vec3::new(-4.0, 10.0, 0.0), Vec3::new(0.5, 0.2, 0.2)); // red light
    let green_light = Light::new(Vec3::new(4.0, 10.0, 0.0), Vec3::new(0.2, 0.5, 0.2)); // green light
    let blue_light = Light::new(Vec3::new(4.0, 5.0, 0.0), Vec3::new(0.1, 0.1, 0.5)); // blue light
    let lights = vec![white_light, blue_light];

    // angle 1
    let camera1 = Camera::new(Vec3::new(0.0, 8.0, 8.0), 
                                    Vec3::new(0.0, 2.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    // angle 2
    let camera2 = Camera::new(Vec3::new(-4.0, 8.0, 8.0), 
                                    Vec3::new(0.0, 2.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    // angle 3
    let camera3 = Camera::new(Vec3::new(0.0, 1.0, 10.0), 
                                    Vec3::new(0.0, 2.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));


    // shinyness
    let shiny_sphere1 = Sphere::new(1.2, Vec3::new(-6.0, 1.0, 0.0), shiny1);
    let shiny_sphere2 = Sphere::new(1.2, Vec3::new(-2.0, 1.0, 0.0), shiny2);
    let shiny_sphere3 = Sphere::new(1.2, Vec3::new(2.0, 1.0, 0.0), shiny3);
    let shiny_sphere4 = Sphere::new(1.2, Vec3::new(6.0, 1.0, 0.0), shiny4);
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), light_gray.clone());
    let shinyness: Vec<Box<dyn Shape>> = vec![Box::new(shiny_sphere1), Box::new(shiny_sphere2), Box::new(shiny_sphere3), Box::new(shiny_sphere4), Box::new(plane)];

    let camera4 = Camera::new(Vec3::new(0.0, 8.0, 8.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera4, width, height, &shinyness, &lights, Color::new(0.2, 0.2, 0.2, 1.0));

    film.save("images/shinyness.png");

    
    let cameras = vec![camera1, camera2, camera3];
    Ok(Scene {cameras, shapes, lights})
}