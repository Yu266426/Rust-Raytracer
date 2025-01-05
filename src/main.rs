use std::{f64::consts::PI, rc::Rc};

use rand::{thread_rng, Rng};
use raytracer::{
    camera::Camera,
    hittable::{sphere::Sphere, HittableList},
    image::color::Color,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    vec3::Vec3,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 1920;
const VFOV: f64 = 90.0;
const SAMPLES_PER_PIXEL: usize = 64;
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
            10.0,
            3.4,
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
            0.0,
            1.0,
            SAMPLES_PER_PIXEL,
            MAX_DEPTH,
        ),
    )
}

#[allow(dead_code)]
fn sample_scene_3<'a>() -> (HittableList<'a>, Camera) {
    let mut world = HittableList::new();

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, -0.0),
        1000.0,
        Rc::clone(&material_ground),
    )));

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = (Vec3::random() * Vec3::random()).to_color();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0).to_color();
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Rc::new(Dielectric::new(1.50));
                }

                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material_1: Rc<dyn Material> = Rc::new(Dielectric::new(1.50));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    (
        world,
        Camera::new(
            ASPECT_RATIO,
            1200,
            20.0,
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.6,
            10.0,
            500,
            50,
        ),
    )
}

fn main() {
    let (world, camera) = sample_scene_1();

    camera.render(&world).save("weekend_1");
}
