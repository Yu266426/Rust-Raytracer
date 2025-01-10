use std::{
    cell::RefCell,
    io::{self, Write},
};

use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{
    hittable::Hittable,
    image::{color::Color, Image},
    interval::Interval,
    ray::Ray,
    vec3::Vec3,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    image_height: usize,
    pub vfov: f64,
    pub center: Vec3,
    pub look_at: Vec3,
    pub v_up: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    top_left_pixel_pos: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pub samples_per_pixel: usize,
    pub pixel_samples_scale: f64,
    pub max_depth: usize,
    rng: RefCell<ThreadRng>,
}

impl Camera {
    pub fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            vfov: 90.0,
            center: Vec3::zero(),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            v_up: Vec3::up(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
            top_left_pixel_pos: Vec3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            samples_per_pixel: 10,
            pixel_samples_scale: 0.0,
            max_depth: 10,
            rng: RefCell::new(thread_rng()),
        }
    }

    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        vfov: f64,
        look_from: Vec3,
        look_at: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        let mut camera = Self::default();

        camera.aspect_ratio = aspect_ratio;
        camera.image_width = image_width;
        camera.vfov = vfov;
        camera.center = look_from;
        camera.look_at = look_at;
        camera.defocus_angle = defocus_angle;
        camera.focus_dist = focus_dist;
        camera.samples_per_pixel = samples_per_pixel;
        camera.max_depth = max_depth;

        camera.initialize();

        camera
    }

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as usize).max(1);

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // u, v, w unit basis vectors for camera coordinate frame
        let w = (self.center - self.look_at).normalize();
        let u = self.v_up.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Viewport goes in the negative z direction
        let viewport_top_left = self.center - self.focus_dist * w - 0.5 * (viewport_u + viewport_v);
        self.top_left_pixel_pos =
            viewport_top_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
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

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample_pos - ray_origin;
        let ray_time = self.rng.borrow_mut().gen();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = self.rng.borrow_mut();
        Vec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();

        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    fn ray_color(ray: &Ray, depth: usize, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::black();
        }

        if let Some(hit_record) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            } else {
                return Color::black();
            }
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
