use crate::{camera::Camera, film::{Color, Film}, shape::Shape, shape::sphere::Sphere};

pub fn render_scene(camera: &Camera, width: u32, height: u32, scene: &[Sphere]) -> Film{
    let mut film = Film::new(width, height);
    let background = Color::new(0.2, 0.2, 0.2, 1.0);

    for y in 0..height {
        for x in 0..width {
            // normalize (x, y)
            let s = x as f32 / width as f32;
            let t = y as f32 / height as f32;

            let ray = camera.get_ray(s,t);

            // sort sphere nearest t is color
            let mut color = background;
            let mut min_t = f32::MAX;
            for sphere in scene {
                if let Some(hit) = sphere.hit(&ray, 0.0, 20.0) {
                    if hit.get_t() < min_t {
                        color = sphere.get_color();
                        min_t = hit.get_t();
                    }
                }
            }

            film.set_pixel(x, y, color);
        }
    }

    return film;
}