use std::{rc::Rc, time::Instant};

use nanorand::{tls_rng, Rng};
use raytracer::{
    camera::Camera,
    hittable::{bvh::BVHNode, quad::Quad, sphere::Sphere, HittableList},
    image::color::Color,
    material::Material,
    random::gen_range_f64,
    texture::TextureEnum,
    vec3::Vec3,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const VFOV: f64 = 20.0;
const SKY_COLOUR: Color = Color::new(0.7, 0.8, 1.0);
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;

#[allow(dead_code)]
fn weekend_1() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Material::lambertian_from_color(Color::new(0.5, 0.5, 0.5)));

    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, -1000.0, -0.0),
        1000.0,
        Rc::clone(&material_ground),
    )));

    let mut rng = tls_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.generate();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.generate::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.generate::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material;

                if choose_mat < 0.8 {
                    let albedo = (Vec3::random() * Vec3::random()).to_color();
                    sphere_material = Rc::new(Material::lambertian_from_color(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0).to_color();
                    let fuzz = gen_range_f64(0.0, 0.5);
                    sphere_material = Rc::new(Material::metal(albedo, fuzz));
                } else {
                    sphere_material = Rc::new(Material::dielectric(1.50));
                }

                world.add(Rc::new(Sphere::still(center, 0.2, sphere_material)));
            }
        }
    }

    let material_1 = Rc::new(Material::dielectric(1.50));
    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Rc::new(Material::lambertian_from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::still(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Rc::new(Material::metal(Color::new(0.7, 0.6, 0.5), 0.0));
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
            SKY_COLOUR,
            500,
            50,
        ),
    )
}

#[allow(dead_code)]
fn bouncing_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Material::lambertian_from_color(Color::new(0.5, 0.5, 0.5)));

    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, -1000.0, -0.0),
        1000.0,
        Rc::clone(&material_ground),
    )));

    let mut rng = tls_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.generate();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.generate::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.generate::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material;

                if choose_mat < 0.8 {
                    let albedo = (Vec3::random() * Vec3::random()).to_color();
                    sphere_material = Rc::new(Material::lambertian_from_color(albedo));

                    world.add(Rc::new(Sphere::moving(
                        center,
                        center + Vec3::new(0.0, gen_range_f64(0.0, 0.5), 0.0),
                        0.2,
                        sphere_material,
                    )));
                    continue;
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0).to_color();
                    let fuzz = gen_range_f64(0.0, 0.5);
                    sphere_material = Rc::new(Material::metal(albedo, fuzz));
                } else {
                    sphere_material = Rc::new(Material::dielectric(1.50));
                }

                world.add(Rc::new(Sphere::still(center, 0.2, sphere_material)));
            }
        }
    }

    let material_1 = Rc::new(Material::dielectric(1.50));
    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Rc::new(Material::lambertian_from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::still(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Rc::new(Material::metal(Color::new(0.7, 0.6, 0.5), 0.0));
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
            VFOV,
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.6,
            10.0,
            SKY_COLOUR,
            SAMPLES_PER_PIXEL,
            MAX_DEPTH,
        ),
    )
}

#[allow(dead_code)]
fn checkered_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let checker = Rc::new(TextureEnum::checker_from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let sphere_mat = Rc::new(Material::lambertian(Rc::clone(&checker)));

    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::clone(&sphere_mat),
    )));

    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::clone(&sphere_mat),
    )));

    (
        world,
        Camera::new(
            ASPECT_RATIO,
            IMAGE_WIDTH,
            VFOV,
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            1.0,
            SKY_COLOUR,
            SAMPLES_PER_PIXEL,
            MAX_DEPTH,
        ),
    )
}

#[allow(dead_code)]
fn earth() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let earth_texture = Rc::new(TextureEnum::image("earthmap.jpg"));
    let earth_surface = Rc::new(Material::lambertian(Rc::clone(&earth_texture)));

    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Rc::clone(&earth_surface),
    )));

    (
        world,
        Camera::new(
            ASPECT_RATIO,
            IMAGE_WIDTH,
            VFOV,
            Vec3::new(0.0, 0.0, 12.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            10.0,
            SKY_COLOUR,
            SAMPLES_PER_PIXEL,
            MAX_DEPTH,
        ),
    )
}

#[allow(dead_code)]
fn perlin_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let perlin_texture = Rc::new(TextureEnum::noise(4.0));
    let perlin_mat = Rc::new(Material::lambertian(Rc::clone(&perlin_texture)));

    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&perlin_mat),
    )));

    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::clone(&perlin_mat),
    )));

    (
        world,
        Camera::new(
            ASPECT_RATIO,
            IMAGE_WIDTH,
            VFOV,
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            10.0,
            SKY_COLOUR,
            SAMPLES_PER_PIXEL,
            MAX_DEPTH,
        ),
    )
}

#[allow(dead_code)]
fn quads() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let left_red = Rc::new(Material::lambertian_from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Rc::new(Material::lambertian_from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(Material::lambertian_from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Material::lambertian_from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Material::lambertian_from_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Rc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Rc::clone(&left_red),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Rc::clone(&back_green),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Rc::clone(&right_blue),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Rc::clone(&upper_orange),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Rc::clone(&lower_teal),
    )));

    (
        world,
        Camera::new(
            ASPECT_RATIO,
            IMAGE_WIDTH,
            80.0,
            Vec3::new(0.0, 0.0, 9.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            1.0,
            SKY_COLOUR,
            SAMPLES_PER_PIXEL,
            MAX_DEPTH,
        ),
    )
}

#[allow(dead_code)]
fn simple_light() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let perlin_texture = Rc::new(TextureEnum::noise(4.0));
    let perlin_mat = Rc::new(Material::lambertian(Rc::clone(&perlin_texture)));

    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&perlin_mat),
    )));
    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::clone(&perlin_mat),
    )));

    let diffuse_light = Rc::new(Material::diffuse_light_from_color(Color::new(
        4.0, 4.0, 4.0,
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Rc::clone(&diffuse_light),
    )));

    world.add(Rc::new(Sphere::still(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Rc::clone(&diffuse_light),
    )));

    (
        world,
        Camera::new(
            ASPECT_RATIO,
            IMAGE_WIDTH,
            VFOV,
            Vec3::new(26.0, 3.0, 6.0),
            Vec3::new(0.0, 2.0, 0.0),
            0.0,
            1.0,
            Color::new(0.0, 0.0, 0.0),
            SAMPLES_PER_PIXEL,
            MAX_DEPTH,
        ),
    )
}

#[allow(dead_code)]
fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let red = Rc::new(Material::lambertian_from_color(Color::new(
        0.65, 0.05, 0.05,
    )));
    let white = Rc::new(Material::lambertian_from_color(Color::new(
        0.73, 0.73, 0.73,
    )));
    let green = Rc::new(Material::lambertian_from_color(Color::new(
        0.12, 0.45, 0.15,
    )));
    let light = Rc::new(Material::diffuse_light_from_color(Color::new(
        15.0, 15.0, 15.0,
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Rc::clone(&green),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Rc::clone(&red),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        Rc::clone(&light),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Rc::clone(&white),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Rc::clone(&white),
    )));

    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Rc::clone(&white),
    )));

    (
        world,
        Camera::new(
            1.0,
            600,
            40.0,
            Vec3::new(278.0, 278.0, -800.0),
            Vec3::new(278.0, 278.0, 0.0),
            0.0,
            1.0,
            Color::new(0.0, 0.0, 0.0),
            200,
            50,
        ),
    )
}

fn main() {
    let (world, camera) = match 4 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => todo!(),
    };

    let now = Instant::now();
    camera.render(&world).save("test");
    let elapsed_time = now.elapsed();

    println!("Rendering took {:.2} seconds.", elapsed_time.as_secs_f64());
}
