use std::rc::Rc;

use crate::{
    image::color::Color,
    texture::{solid_color::SolidColor, Texture},
    vec3::Vec3,
};

use super::Material;

pub struct DiffuseLight {
    texture: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            texture: Rc::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, uv: (f64, f64), pos: &Vec3) -> Color {
        self.texture.value(uv, pos)
    }
}
