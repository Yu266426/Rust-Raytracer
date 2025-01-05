use std::{
    cell::RefCell,
    io::{self, Write},
};

use rand::{rngs::ThreadRng, Rng};

use crate::{
    hittable::Hittable,
    image::{color::Color, Image},
    interval::Interval,
    ray::Ray,
    vec3::Vec3,
};

pub struct Camera {
    _aspect_ratio: f64,
    image_width: usize,
    image_height: usize,
    center: Vec3,
    vfov: f64,
    top_left_pixel_pos: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: usize,
    pixel_samples_scale: f64,
    max_depth: usize,
    rng: RefCell<ThreadRng>,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        vfov: f64,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);
        let center = Vec3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focal_length;
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
            _aspect_ratio: aspect_ratio,
            image_width,
            image_height,
            center,
            vfov,
            top_left_pixel_pos,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            max_depth,
            rng: RefCell::new(rand::thread_rng()),
        }
    }

    pub fn render(&self, world: &impl Hittable) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);

        for row in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - row);
            io::stdout().flush().unwrap();

            for col in 0..self.image_width {
                let mut color = Color::black();

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(col, row);
                    color += Self::ray_color(&ray, self.max_depth, world);
                }

                image.set_pixel(color * self.pixel_samples_scale, row, col);
            }
        }
        println!("\rDone!                             ");

        image
    }

    fn get_ray(&self, col: usize, row: usize) -> Ray {
        // Construct a camera ray from origin and directed at randomly sampled point around pixel location i, j

        let offset = self.sample_square();
        let pixel_sample_pos = self.top_left_pixel_pos
            + (col as f64 + offset.x) * self.pixel_delta_u
            + (row as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = self.center;
        let ray_direction = pixel_sample_pos - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = self.rng.borrow_mut();
        Vec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }

    fn ray_color(ray: &Ray, depth: usize, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::black();
        }

        if let Some(hit_record) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
                return *attenuation * Self::ray_color(&scattered, depth - 1, world);
            } else {
                return Color::black();
            }
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
