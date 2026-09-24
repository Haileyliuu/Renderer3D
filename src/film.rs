use image::{ImageBuffer, Rgba};

pub struct Film {
    width: u32,         // horizontal resolution, pixels
    height: u32,        // vertical resolution, pixels
    pixels: Vec<Color>  // row major; index = y * width + x
}

impl Film {
    pub fn new(w: u32, h: u32) -> Self{
        // all pixels black
        let num_pixels = (w*h) as usize;
        let black: Color = Color::new(0.0, 0.0, 0.0, 1.0);
        let pixels: Vec<Color> = vec![black; num_pixels];
        Film{width: w, height: h, pixels}
    }
    
    // sets pixel at (x, y) to color
    pub fn set_pixel(&mut self, x: u32, y: u32, c: Color) {
        let index = (y * self.width + x) as usize;
        self.pixels[index] = c;
    }

    // gets the color of pixel at (x, y)
    pub fn get_pixel(&self, x:u32, y:u32) -> Color{
        let index = (y * self.width + x) as usize;
        self.pixels[index] // returns color
    }

    pub fn save(&self, path: &str){
        let mut rgba_pixels: Vec<u8> = Vec::new();
        for i in 0..self.pixels.len() {
            rgba_pixels.append(&mut self.pixels[i].rgba());
        }

        // ImageBuffer<P, Container> 
        // P = pixel type, Container stores pixels
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(self.width, self.height, rgba_pixels)
        .expect("Incorrect number of pixel values");
        
        image.save(path).expect("Could not save image"); //.expect extracts inner value from option or result type
                                                            // operation successful -> returns value, fail -> crashes program, prints error
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color{red, green, blue, alpha}
    } 

    fn rgba(&self) -> Vec<u8> {
        vec![(self.red*255.0) as u8, (self.green*255.0) as u8,
                (self.blue*255.0) as u8, (self.alpha*255.0) as u8]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn set_and_get_pixel() {
        let mut film = Film::new(2, 2);

        let color = Color::new(1.0, 0.0, 0.0, 1.0);
        film.set_pixel(1, 0, color);

        let result = film.get_pixel(1, 0);

        assert_eq!(result.red, 1.0);
        assert_eq!(result.green, 0.0);
        assert_eq!(result.blue, 0.0);
        assert_eq!(result.alpha, 1.0);
    }

    #[test]
    fn new_film_starts_black() {
        let film = Film::new(2, 2);

        let pixel = film.get_pixel(0, 0);

        assert_eq!(pixel.red, 0.0);
        assert_eq!(pixel.green, 0.0);
        assert_eq!(pixel.blue, 0.0);
        assert_eq!(pixel.alpha, 1.0);
    }

    #[test]
    fn saves_correct_image() {
        let mut film = Film::new(2, 2);

        film.set_pixel(0, 0, Color::new(1.0, 0.0, 0.0, 1.0)); // red
        film.set_pixel(1, 0, Color::new(0.0, 1.0, 0.0, 1.0)); // green
        film.set_pixel(0, 1, Color::new(0.0, 0.0, 1.0, 1.0)); // blue
        film.set_pixel(1, 1, Color::new(1.0, 1.0, 1.0, 1.0)); // white

        let path = "test.png";

        film.save(&path);

        let image = image::open(&path)
            .expect("Could not open saved image")
            .to_rgba8();

        assert_eq!(image.width(), 2);
        assert_eq!(image.height(), 2);

        assert_eq!(image.get_pixel(0, 0).0, [255, 0, 0, 255]);
        assert_eq!(image.get_pixel(1, 0).0, [0, 255, 0, 255]);
        assert_eq!(image.get_pixel(0, 1).0, [0, 0, 255, 255]);
        assert_eq!(image.get_pixel(1, 1).0, [255, 255, 255, 255]);

        // Clean up the temporary test file (comment this out to view image created)
        fs::remove_file(path).expect("Could not remove test image");
    }
}