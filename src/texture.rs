use glam::{Vec2, Vec3};
use image::ImageReader;

#[derive(Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub texels: Vec<Vec3>,
}

impl Texture {
    pub fn load_texture(path: &str) -> Result<Texture, image::ImageError> {
        let image = ImageReader::open(path)?.decode()?.to_rgb8();
        let (width, height) = image.dimensions();

        let texels = image.pixels().map(|pixel| {
                Vec3::new(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                )
            }).collect();

        Ok(Texture {width: width as usize, height: height as usize, texels})
    }

    fn texel(&self, x: i32, y: i32) -> Vec3 {
        let x = x.rem_euclid(self.width as i32) as usize;
        let y = y.rem_euclid(self.height as i32) as usize;
        self.texels[y * self.width + x]
    }

    pub fn sample_nearest(&self, uv: Vec2) -> Vec3 {
        self.texel(((1.0 - uv.x) * self.width as f32).floor() as i32, ((1.0 - uv.y) * self.height as f32).floor() as i32)
    }

    pub fn sample_bilinear(&self, uv: Vec2) -> Vec3 {
        // from texel centers (x, y)
        let (x, y) = ((1.0 - uv.x) * self.width as f32 - 0.5, (1.0 - uv.y) * self.height as f32 - 0.5);
        let (s, t) = (x - x.floor(), y - y.floor());
        let (i, j) = (x.floor() as i32, y.floor() as i32);
        let top = self.texel(i, j).lerp(self.texel(i+1, j), s);
        let bot = self.texel(i, j + 1).lerp(self.texel(i+1, j+1), s);
        top.lerp(bot, t)
    }
}