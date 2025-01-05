use std::{f64::consts::PI, rc::Rc};

use raytracer::{
    camera::Camera,
    hittable::{sphere::Sphere, HittableList},
    image::color::Color,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    vec3::Vec3,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const VFOV: f64 = 90.0;
const SAMPLES_PER_PIXEL: usize = 496;
const MAX_DEPTH: usize = 50;

#[allow(dead_code)]
fn sample_scene_1<'a>() -> (HittableList<'a>, Camera) {
    let mut world = HittableList::new();

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.50));
    let material_bubble: Rc<dyn Material> = Rc::new(Dielectric::new(1.00 / 1.50));
    let material_right: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        Rc::clone(&material_center),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.45,
        Rc::clone(&material_bubble),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_right),
    )));

    (
        world,
        Camera::new(
            ASPECT_RATIO,
            IMAGE_WIDTH,
            20.0,
            Vec3::new(-2.0, 2.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
            SAMPLES_PER_PIXEL,
            MAX_DEPTH,
        ),
    )
}

#[allow(dead_code)]
fn sample_scene_2<'a>() -> (HittableList<'a>, Camera) {
    let mut world = HittableList::new();

    let r = (PI / 4.0).cos();

    let material_left: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Rc::new(Sphere::new(
        Vec3::new(-r, 0.0, -1.0),
        r,
        Rc::clone(&material_left),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(r, 0.0, -1.0),
        r,
        Rc::clone(&material_right),
    )));

    (
        world,
        Camera::new(
            ASPECT_RATIO,
            IMAGE_WIDTH,
            VFOV,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            SAMPLES_PER_PIXEL,
            MAX_DEPTH,
        ),
    )
}

fn main() {
    let (world, camera) = sample_scene_1();

    camera.render(&world).save("test");
}
