use std::rc::Rc;

use crate::{image::color::Color, vec3::Vec3};

use super::{solid_color::SolidColor, Texture};

pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, color_1: Color, color_2: Color) -> Self {
        Self::new(
            scale,
            Rc::new(SolidColor::new(color_1)),
            Rc::new(SolidColor::new(color_2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: (f64, f64), pos: &Vec3) -> Color {
        let x = (self.inv_scale * pos.x).floor() as i32;
        let y = (self.inv_scale * pos.y).floor() as i32;
        let z = (self.inv_scale * pos.z).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(uv, pos)
        } else {
            self.odd.value(uv, pos)
        }
    }
}
