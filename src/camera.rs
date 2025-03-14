use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use nanorand::{Rng, WyRand};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
    background: Color,
    pub samples_per_pixel: usize,
    pub pixel_samples_scale: f64,
    pub max_depth: usize,
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
            background: Color::black(),
            samples_per_pixel: 10,
            pixel_samples_scale: 0.0,
            max_depth: 10,
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
        background: Color,
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
        camera.background = background;
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

    pub fn render_single_threaded(&self, world: &impl Hittable) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);

        let mut rng = WyRand::new();

        for row in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - row);
            io::stdout().flush().unwrap();

            for col in 0..self.image_width {
                let mut color = Color::black();

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(col, row, &mut rng);
                    color += self.ray_color(ray, self.max_depth, world);
                }

                image.set_pixel(color * self.pixel_samples_scale, row, col);
            }
        }
        println!("\rDone!                             ");

        image
    }

    pub fn render<H: Hittable + Sync>(&self, world: &H) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);

        // Define square chunk dimensions
        let chunk_size = 64;

        // Calculate the number of chunks in each dimension
        let chunks_x = (self.image_width + chunk_size - 1) / chunk_size;
        let chunks_y = (self.image_height + chunk_size - 1) / chunk_size;
        let total_chunks = chunks_x * chunks_y;

        let chunks_remaining = Arc::new(AtomicUsize::new(total_chunks));

        // Initial progress report
        println!(
            "Starting render: {} chunks to process ({} x {})",
            total_chunks, chunks_x, chunks_y
        );

        // Create a collection of all chunk coordinates
        let chunk_coordinates: Vec<(usize, usize)> = (0..chunks_y)
            .flat_map(|chunk_y| (0..chunks_x).map(move |chunk_x| (chunk_x, chunk_y)))
            .collect();

        // Process chunks in parallel and collect the results
        let chunk_results: Vec<_> = chunk_coordinates
            .into_par_iter()
            .map(|(chunk_x, chunk_y)| {
                let mut rng = WyRand::new();
                let mut local_buffer = Vec::with_capacity(chunk_size * chunk_size);

                // Calculate pixel bounds for this chunk
                let start_x = chunk_x * chunk_size;
                let start_y = chunk_y * chunk_size;
                let end_x = std::cmp::min(start_x + chunk_size, self.image_width);
                let end_y = std::cmp::min(start_y + chunk_size, self.image_height);

                // Process all pixels in the chunk
                for row in start_y..end_y {
                    for col in start_x..end_x {
                        let mut pixel_color = Color::black();

                        for _ in 0..self.samples_per_pixel {
                            let ray = self.get_ray(col, row, &mut rng);
                            pixel_color += self.ray_color(ray, self.max_depth, world);
                        }

                        local_buffer.push((row, col, pixel_color * self.pixel_samples_scale));
                    }
                }

                // Update progress after each chunk is processed
                let remaining = chunks_remaining.fetch_sub(1, Ordering::Relaxed) - 1;
                print!("\rChunks remaining: {}/{} ", remaining, total_chunks);
                if let Err(e) = io::stdout().flush() {
                    eprintln!("Warning: Could not flush stdout: {}", e);
                }

                local_buffer
            })
            .collect();

        // Set the pixels of the image
        for chunk_result in chunk_results {
            for (row, col, color) in chunk_result {
                image.set_pixel(color, row, col);
            }
        }

        println!("\rDone!                                          ");
        image
    }

    fn get_ray(&self, col: usize, row: usize, rng: &mut WyRand) -> Ray {
        // Construct a camera ray from origin and directed at randomly sampled point around pixel location i, j

        let offset = self.sample_square(rng);
        let pixel_sample_pos = self.top_left_pixel_pos
            + (col as f64 + offset.x) * self.pixel_delta_u
            + (row as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample_pos - ray_origin;
        let ray_time = rng.generate();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square(&self, rng: &mut WyRand) -> Vec3 {
        Vec3::new(
            rng.generate::<f64>() - 0.5,
            rng.generate::<f64>() - 0.5,
            0.0,
        )
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();

        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    fn ray_color(&self, ray: Ray, depth: usize, world: &impl Hittable) -> Color {
        let mut ray = ray;
        let mut color = Color::black();
        let mut attenuation_accumulator = Color::new(1.0, 1.0, 1.0);

        for _ in 0..depth {
            if let Some(hit_record) = world.hit(&ray, Interval::new(0.001, f64::INFINITY)) {
                let emission = hit_record.material.emitted(hit_record.uv, &hit_record.pos);
                // Emission is affected by all the attenuation up to this point
                color = color + attenuation_accumulator * emission;

                if let Some((attenuation, scattered)) =
                    hit_record.material.scatter(&ray, &hit_record)
                {
                    // Keep track of attenuation up to this point
                    attenuation_accumulator = attenuation_accumulator * attenuation;

                    // Set new ray
                    ray = scattered;
                } else {
                    // Hit light (purely emissive material)
                    break;
                }
            } else {
                // Hit nothing (Aka. hit background)
                color = color + attenuation_accumulator * self.background;
                break;
            }
        }

        color
    }
}
