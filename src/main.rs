mod ray;
mod camera;
mod film;
mod shape;
mod render;
mod material;

use glam::Vec3;
use camera::Camera;
use shape::sphere::Sphere;

use crate::{material::{Light, Material}, shape::{Shape, plane::Plane, polymesh::PolyMesh}};



fn main() {
    // height and width of png
    let aspect = 1.333;
    let width = 1024;
    let height = 768;

    // setup scene
    // materials
    let red = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.3, 0.3, 0.3), 100.0);
    let yellow = Material::new(Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.3, 0.3, 0.3), 100.0);
    let blue = Material::new(Vec3::new(0.2, 0.0, 0.9), Vec3::new(0.3, 0.3, 0.3), 100.0);
    let green = Material::new(Vec3::new(0.2, 0.8, 0.2), Vec3::new(0.3, 0.3, 0.3), 100.0);
    let light_gray = Material::new(Vec3::new(0.7, 0.7, 0.7), Vec3::new(0.1, 0.1, 0.1), 8.0);

    let shiny1 = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.5, 0.5, 0.5), 10.0);
    let shiny2 = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.5, 0.5, 0.5), 100.0);
    let shiny3 = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.5, 0.5, 0.5), 1000.0);
    let shiny4 = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.5, 0.5, 0.5), 10000.0);


    // shapes
    let sphere1 = Sphere::new(1.2, Vec3::new(-6.0, 1.0, 0.0), red);
    let sphere2 = Sphere::new(1.2, Vec3::new(-2.0, 1.0, 0.0), yellow);
    let sphere3 = Sphere::new(1.2, Vec3::new(2.0, 1.0, 0.0), blue);
    let sphere4 = Sphere::new(1.2, Vec3::new(6.0, 1.0, 0.0), green);
    

    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), light_gray);

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
    let pyramid = PolyMesh::new(pyramid_vertices, pyramid_triangles, Vec::new(), red);

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
        Vec3::new(1.0, 1.0, 1.0).normalize(), // 0
        Vec3::new(1.0, 1.0, 1.0).normalize(), // 1
        Vec3::new(-1.0, 1.0, -1.0).normalize(), // 2
        Vec3::new(1.0, 1.0, 1.0).normalize(), // 3
        Vec3::new(1.0, 1.0, -1.0).normalize(), // 4
    ];

    let smooth_mesh = PolyMesh::new(smooth_vertices, smooth_triangles, smooth_normals, yellow);
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(sphere1), Box::new(sphere2), Box::new(sphere3), Box::new(sphere4), Box::new(plane), Box::new(pyramid), Box::new(smooth_mesh)];
    
    // lights
    let white_light = Light::new(Vec3::new(-4.0, 10.0, 0.0), Vec3::new(0.8, 0.8, 0.8)); // white light
    let red_light = Light::new(Vec3::new(-4.0, 10.0, 0.0), Vec3::new(0.5, 0.2, 0.2)); // red light
    let green_light = Light::new(Vec3::new(4.0, 10.0, 0.0), Vec3::new(0.2, 0.5, 0.2)); // green light
    let blue_light = Light::new(Vec3::new(4.0, 5.0, 0.0), Vec3::new(0.1, 0.1, 0.5)); // blue light
    let lights = vec![white_light, blue_light];

    // angle 1
    let camera = Camera::new(Vec3::new(0.0, 8.0, 8.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera, width, height, &shapes, &lights);

    film.save("images/angle1.png");


    // angle 2
    let camera = Camera::new(Vec3::new(-4.0, 8.0, 8.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera, width, height, &shapes, &lights);

    film.save("images/angle2.png");


    // angle 3
    let camera = Camera::new(Vec3::new(0.0, 1.0, 10.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera, width, height, &shapes, &lights);

    film.save("images/angle3.png");


    // shinyness
    let shiny_sphere1 = Sphere::new(1.2, Vec3::new(-6.0, 1.0, 0.0), shiny1);
    let shiny_sphere2 = Sphere::new(1.2, Vec3::new(-2.0, 1.0, 0.0), shiny2);
    let shiny_sphere3 = Sphere::new(1.2, Vec3::new(2.0, 1.0, 0.0), shiny3);
    let shiny_sphere4 = Sphere::new(1.2, Vec3::new(6.0, 1.0, 0.0), shiny4);
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), light_gray);
    let shinyness: Vec<Box<dyn Shape>> = vec![Box::new(shiny_sphere1), Box::new(shiny_sphere2), Box::new(shiny_sphere3), Box::new(shiny_sphere4), Box::new(plane)];

    let camera = Camera::new(Vec3::new(0.0, 8.0, 8.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    aspect,
                                    60.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera, width, height, &shinyness, &lights);

    film.save("images/shinyness.png");


}

