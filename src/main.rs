use std::{rc::Rc, time::Instant};

use rand::{thread_rng, Rng};
use raytracer::{
    camera::Camera,
    hittable::{bvh::BVHNode, sphere::Sphere, HittableList},
    image::color::Color,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    vec3::Vec3,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const VFOV: f64 = 90.0;
const SAMPLES_PER_PIXEL: usize = 64;
const MAX_DEPTH: usize = 50;

#[allow(dead_code)]
fn sample_scene_1<'a>() -> (HittableList<'a>, Camera) {
    let mut world = HittableList::new();

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::still(
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

                    world.add(Rc::new(Sphere::moving(
                        center,
                        center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0),
                        0.2,
                        sphere_material,
                    )));
                    continue;
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0).to_color();
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Rc::new(Dielectric::new(1.50));
                }

                world.add(Rc::new(Sphere::still(center, 0.2, sphere_material)));
            }
        }
    }

    let material_1: Rc<dyn Material> = Rc::new(Dielectric::new(1.50));
    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::still(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::still(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let world = HittableList::from_object(Rc::new(BVHNode::from_hittable_list(world)));

    (
        world,
        Camera::new(
            ASPECT_RATIO,
            IMAGE_WIDTH,
            20.0,
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.6,
            10.0,
            SAMPLES_PER_PIXEL,
            50,
        ),
    )
}

#[allow(dead_code)]
fn weekend_1<'a>() -> (HittableList<'a>, Camera) {
    let mut world = HittableList::new();

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::still(
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

                world.add(Rc::new(Sphere::still(center, 0.2, sphere_material)));
            }
        }
    }

    let material_1: Rc<dyn Material> = Rc::new(Dielectric::new(1.50));
    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::still(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::still(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let world = HittableList::from_object(Rc::new(BVHNode::from_hittable_list(world)));

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
    let (world, camera) = weekend_1();

    let now = Instant::now();
    camera.render(&world).save("test");
    let elapsed_time = now.elapsed();

    println!("Rendering took {:.2} seconds.", elapsed_time.as_secs_f64());
}
