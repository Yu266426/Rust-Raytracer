use std::io::{self, Write};

use crate::{
    hittable::Hittable,
    image::{color::Color, Image},
    interval::Interval,
    ray::Ray,
    vec3::Vec3,
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: usize,
    image_height: usize,
    center: Vec3,
    top_left_pixel_pos: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);
        let center = Vec3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = Vec3::zero();

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Viewport goes in the negative z direction
        let viewport_top_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - 0.5 * (viewport_u + viewport_v);
        let top_left_pixel_pos = viewport_top_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            top_left_pixel_pos,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &impl Hittable) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);

        for row in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - row);
            io::stdout().flush().unwrap();

            for col in 0..self.image_width {
                let pixel_center = self.top_left_pixel_pos
                    + (col as f64 * self.pixel_delta_u)
                    + (row as f64 * self.pixel_delta_v);

                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = Self::ray_color(&ray, world);

                image.set_pixel(color, row, col);
            }
        }

        image
    }

    fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
        if let Some(hit_record) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0)).to_color();
        }

        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
