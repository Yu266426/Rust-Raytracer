use core::f64;
use std::{
    io::{self, Write},
    rc::Rc,
};

use raytracer::{
    hittable::{sphere::Sphere, Hittable, HittableList},
    image::{color::Color, Image},
    interval::Interval,
    ray::Ray,
    vec3::Vec3,
};

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    if let Some(hit_record) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
        return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0)).to_color();
    }

    let unit_direction = ray.direction.unit();
    let a = 0.5 * (unit_direction.y + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;
    let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);

    // World
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
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

    // Render
    let mut image = Image::new(image_width, image_height);

    for row in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - row);
        io::stdout().flush().unwrap();

        for col in 0..image_width {
            let pixel_center =
                top_left_pixel_pos + (col as f64 * pixel_delta_u) + (row as f64 * pixel_delta_v);

            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray, &world);

            image.set_pixel(color, row, col);
        }
    }
    println!("\rDone                       ");

    image.save("test");
}
