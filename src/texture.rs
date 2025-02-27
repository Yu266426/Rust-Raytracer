use std::rc::Rc;

use crate::{
    image::{color::Color, ExtImage},
    perlin::Perlin,
    vec3::Vec3,
};

pub enum TextureEnum {
    Checker {
        inv_scale: f64,
        even: Rc<TextureEnum>,
        odd: Rc<TextureEnum>,
    },
    Image {
        image: ExtImage,
    },
    Noise {
        noise: Perlin,
        scale: f64,
    },
    Color {
        albedo: Color,
    },
}

impl TextureEnum {
    pub fn checker(scale: f64, even: Rc<Self>, odd: Rc<Self>) -> Self {
        Self::Checker {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn checker_from_colors(scale: f64, color_1: Color, color_2: Color) -> Self {
        Self::checker(
            scale,
            Rc::new(Self::color(color_1)),
            Rc::new(Self::color(color_2)),
        )
    }

    pub fn image(file_name: &str) -> Self {
        Self::Image {
            image: ExtImage::load(file_name).unwrap(),
        }
    }

    pub fn noise(scale: f64) -> Self {
        Self::Noise {
            noise: Perlin::new(),
            scale,
        }
    }

    pub fn color(albedo: Color) -> Self {
        Self::Color { albedo }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::Color {
            albedo: Color::new(r, g, b),
        }
    }
}

impl TextureEnum {
    pub fn value(&self, uv: (f64, f64), pos: &Vec3) -> Color {
        match self {
            TextureEnum::Checker {
                inv_scale,
                even,
                odd,
            } => Self::checker_value(*inv_scale, even, odd, uv, pos),
            TextureEnum::Image { image } => Self::image_value(image, uv, pos),
            TextureEnum::Noise { noise, scale } => Self::noise_value(noise, *scale, uv, pos),
            TextureEnum::Color { albedo } => albedo.clone(),
        }
    }

    fn checker_value(
        inv_scale: f64,
        even: &Rc<TextureEnum>,
        odd: &Rc<TextureEnum>,
        uv: (f64, f64),
        pos: &Vec3,
    ) -> Color {
        let x = (inv_scale * pos.x).floor() as i32;
        let y = (inv_scale * pos.y).floor() as i32;
        let z = (inv_scale * pos.z).floor() as i32;

        if (x + y + z) % 2 == 0 {
            even.value(uv, pos)
        } else {
            odd.value(uv, pos)
        }
    }

    fn image_value(image: &ExtImage, uv: (f64, f64), _pos: &Vec3) -> Color {
        // Debug color (cyan) if no image
        if image.height() <= 0 {
            return Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
            };
        }

        let u = uv.0.clamp(0.0, 1.0);
        let v = 1.0 - uv.1.clamp(0.0, 1.0);

        let i = (u * image.width() as f64) as usize;
        let j = (v * image.height() as f64) as usize;
        let pixel = image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;

        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }

    fn noise_value(noise: &Perlin, scale: f64, _uv: (f64, f64), pos: &Vec3) -> Color {
        // Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * pos)))

        // Use turb(ulence) to offset what would be a sin wave in the z direction
        Color::new(0.5, 0.5, 0.5) * (1.0 + (scale * pos.z + 10.0 * noise.turb(&pos, 7)).sin())
    }
}
