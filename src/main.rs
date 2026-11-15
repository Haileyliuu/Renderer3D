use crate::film::Color;

mod ray;
mod camera;
mod film;
mod shape;
mod render;
mod material;
mod obj_loader;
mod texture;
mod scenes;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    // height and width of png
    let aspect = 1.333;
    let width = 1024;
    let height = 768;

    //let scene = scenes::test_shapes::create()?;
    //let scene = scenes::table_vase::create()?;
    let scene = scenes::earth::create()?;

    let gray_background = Color::new(0.2, 0.2, 0.2, 1.0);
    let black_background = Color::new(0.0, 0.0, 0.0, 1.0);

    for (i, camera) in scene.cameras.iter().enumerate() {
        let film = render::render_scene(&camera, width, height, &scene.shapes, &scene.lights, black_background);
        let filename = format!("images/image{}.png", i+1);
        film.save(&filename);
    }

    Ok(())
}