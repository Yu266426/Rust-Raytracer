pub mod checker;
pub mod image_texture;
pub mod noise_texture;
pub mod solid_color;

use crate::{image::color::Color, vec3::Vec3};

pub trait Texture {
    fn value(&self, uv: (f64, f64), pos: &Vec3) -> Color;
}
