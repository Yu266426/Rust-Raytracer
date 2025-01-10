use crate::{
    image::{color::Color, ExtImage},
    vec3::Vec3,
};

use super::Texture;

pub struct ImageTexture {
    image: ExtImage,
}

impl ImageTexture {
    pub fn new(file_name: &str) -> Self {
        Self {
            image: ExtImage::load(file_name).unwrap(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _pos: &Vec3) -> Color {
        // Debug color (cyan) if no image
        if self.image.height() <= 0 {
            return Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
            };
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;

        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
