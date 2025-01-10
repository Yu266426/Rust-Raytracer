use crate::{image::color::Color, perlin::Perlin, vec3::Vec3};

use super::Texture;

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, pos: &Vec3) -> Color {
        // Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * pos)))

        // Use turb(ulence) to offset what would be a sin wave in the z direction
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + (self.scale * pos.z + 10.0 * self.noise.turb(&pos, 7)).sin())
    }
}
