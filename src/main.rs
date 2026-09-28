mod ray;
mod camera;
mod film;
mod sphere;
mod render;

use glam::Vec3;
use camera::Camera;
use film::Color;
use sphere::Sphere;



fn main() {
    // setup scene
    let sphere1 = Sphere::new(1.0, Vec3::new(1.0, 1.0, 1.0), Color::new(1.0, 0.0, 0.0, 1.0));
    let sphere2 = Sphere::new(1.5, Vec3::new(3.0, 2.0, 4.0), Color::new(0.0, 1.0, 0.0, 1.0));
    let sphere3 = Sphere::new(1.2, Vec3::new(4.0, 0.0, 6.0), Color::new(0.0, 0.0, 1.0, 1.0));
    let spheres = vec![sphere1, sphere2, sphere3];
    
    // angle 1
    let camera = Camera::new(Vec3::new(-5.0, 0.0, 0.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    1.5,
                                    90.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera, 1500, 1000, &spheres);

    film.save("images/scene1.png");


    // angle 2
     let camera = Camera::new(Vec3::new(2.0, 0.0, -5.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    1.5,
                                    90.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera, 1500, 1000, &spheres);

    film.save("images/scene2.png");


    // angle 3
    let camera = Camera::new(Vec3::new(3.0, 0.0, 10.0), 
                                    Vec3::new(0.0, 0.0, 0.0),
                                    1.5,
                                    90.0,
                                    Vec3::new(0.0, 1.0, 0.0));

    let film = render::render_scene(&camera, 1500, 1000, &spheres);

    film.save("images/scene3.png");

}

