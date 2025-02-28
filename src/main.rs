use std::{sync::Arc, time::Instant};

use nanorand::{tls_rng, Rng};
use raytracer::{
    camera::Camera,
    hittable::{
        bvh::BVHNode,
        constant_medium::ConstantMedium,
        quad::{quad_box, Quad},
        sphere::Sphere,
        transform::{RotateY, Translate},
        Hittable, HittableList,
    },
    image::color::Color,
    material::Material,
    random::gen_range_f64,
    texture::Texture,
    vec3::Vec3,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const VFOV: f64 = 20.0;
const SKY_COLOUR: Color = Color::new(0.7, 0.8, 1.0);
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;

fn weekend_1() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Material::lambertian_from_color(Color::new(0.5, 0.5, 0.5)));

    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, -1000.0, -0.0),
        1000.0,
        Arc::clone(&material_ground),
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
                    sphere_material = Arc::new(Material::lambertian_from_color(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0).to_color();
                    let fuzz = gen_range_f64(0.0, 0.5);
                    sphere_material = Arc::new(Material::metal(albedo, fuzz));
                } else {
                    sphere_material = Arc::new(Material::dielectric(1.50));
                }

                world.add(Arc::new(Sphere::still(center, 0.2, sphere_material)));
            }
        }
    }

    let material_1 = Arc::new(Material::dielectric(1.50));
    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Arc::new(Material::lambertian_from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::still(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Arc::new(Material::metal(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::still(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let world = HittableList::from_object(Arc::new(BVHNode::from_hittable_list(world)));

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

fn bouncing_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Material::lambertian_from_color(Color::new(0.5, 0.5, 0.5)));

    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, -1000.0, -0.0),
        1000.0,
        Arc::clone(&material_ground),
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
                    sphere_material = Arc::new(Material::lambertian_from_color(albedo));

                    world.add(Arc::new(Sphere::moving(
                        center,
                        center + Vec3::new(0.0, gen_range_f64(0.0, 0.5), 0.0),
                        0.2,
                        sphere_material,
                    )));
                    continue;
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0).to_color();
                    let fuzz = gen_range_f64(0.0, 0.5);
                    sphere_material = Arc::new(Material::metal(albedo, fuzz));
                } else {
                    sphere_material = Arc::new(Material::dielectric(1.50));
                }

                world.add(Arc::new(Sphere::still(center, 0.2, sphere_material)));
            }
        }
    }

    let material_1 = Arc::new(Material::dielectric(1.50));
    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Arc::new(Material::lambertian_from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::still(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Arc::new(Material::metal(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::still(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let world = HittableList::from_object(Arc::new(BVHNode::from_hittable_list(world)));

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

fn checkered_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let checker = Arc::new(Texture::checker_from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let sphere_mat = Arc::new(Material::lambertian(Arc::clone(&checker)));

    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::clone(&sphere_mat),
    )));

    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::clone(&sphere_mat),
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

fn earth() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(Texture::image("earthmap.jpg"));
    let earth_surface = Arc::new(Material::lambertian(Arc::clone(&earth_texture)));

    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Arc::clone(&earth_surface),
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

fn perlin_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let perlin_texture = Arc::new(Texture::noise(4.0));
    let perlin_mat = Arc::new(Material::lambertian(Arc::clone(&perlin_texture)));

    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&perlin_mat),
    )));

    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&perlin_mat),
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

fn quads() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let left_red = Arc::new(Material::lambertian_from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Material::lambertian_from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Material::lambertian_from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Material::lambertian_from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Material::lambertian_from_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::clone(&left_red),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::clone(&back_green),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::clone(&right_blue),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Arc::clone(&upper_orange),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Arc::clone(&lower_teal),
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

fn simple_light() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let perlin_texture = Arc::new(Texture::noise(4.0));
    let perlin_mat = Arc::new(Material::lambertian(Arc::clone(&perlin_texture)));

    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&perlin_mat),
    )));
    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&perlin_mat),
    )));

    let diffuse_light = Arc::new(Material::diffuse_light_from_color(Color::new(
        4.0, 4.0, 4.0,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Arc::clone(&diffuse_light),
    )));

    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Arc::clone(&diffuse_light),
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

fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let red = Arc::new(Material::lambertian_from_color(Color::new(
        0.65, 0.05, 0.05,
    )));
    let white = Arc::new(Material::lambertian_from_color(Color::new(
        0.73, 0.73, 0.73,
    )));
    let green = Arc::new(Material::lambertian_from_color(Color::new(
        0.12, 0.45, 0.15,
    )));
    let light = Arc::new(Material::diffuse_light_from_color(Color::new(
        15.0, 15.0, 15.0,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::clone(&green),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::clone(&red),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        Arc::clone(&light),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::clone(&white),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Arc::clone(&white),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Arc::clone(&white),
    )));

    let box_1 = quad_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::new(Material::lambertian_from_color(Color::white())),
    );
    let box_1 = Arc::new(RotateY::new(box_1, 15.0));
    let box_1 = Arc::new(Translate::new(box_1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstantMedium::from_color(
        box_1,
        0.01,
        Color::black(),
    )));

    let box_2 = quad_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        Arc::new(Material::lambertian_from_color(Color::white())),
    );
    let box_2 = Arc::new(RotateY::new(box_2, -18.0));
    let box_2 = Arc::new(Translate::new(box_2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::from_color(
        box_2,
        0.01,
        Color::white(),
    )));

    let world = HittableList::from_object(Arc::new(BVHNode::from_hittable_list(world)));

    (
        world,
        Camera::new(
            1.0,
            400,
            40.0,
            Vec3::new(278.0, 278.0, -800.0),
            Vec3::new(278.0, 278.0, 0.0),
            0.0,
            1.0,
            Color::new(0.0, 0.0, 0.0),
            500,
            100,
        ),
    )
}

fn final_scene() -> (HittableList, Camera) {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Material::lambertian_from_color(Color::new(
        0.48, 0.83, 0.53,
    )));

    // Create ground boxes
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = gen_range_f64(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(quad_box(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1 - x0, y1 - y0, z1 - z0),
                Arc::clone(&ground),
            ));
        }
    }

    let mut world = HittableList::new();
    world.add(Arc::new(BVHNode::from_hittable_list(boxes1)));

    // Add light
    let light = Arc::new(Material::diffuse_light_from_color(Color::new(
        7.0, 7.0, 7.0,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    // Add moving sphere
    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Material::lambertian_from_color(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    // Add glass and metal spheres
    world.add(Arc::new(Sphere::still(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Material::dielectric(1.5)),
    )));
    world.add(Arc::new(Sphere::still(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Material::metal(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // Add boundary spheres with constant medium
    let boundary: Arc<dyn Hittable> = Arc::new(Sphere::still(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Material::dielectric(1.5)),
    ));
    world.add(Arc::clone(&boundary));
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::still(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Material::dielectric(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::white(),
    )));

    // Add earth and noise textured spheres
    let earth_texture = Arc::new(Texture::image("earthmap.jpg"));
    let earth_surface = Arc::new(Material::lambertian(earth_texture));
    world.add(Arc::new(Sphere::still(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        earth_surface,
    )));

    let perlin_texture = Arc::new(Texture::noise(0.2));
    let perlin_material = Arc::new(Material::lambertian(perlin_texture));
    world.add(Arc::new(Sphere::still(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        perlin_material,
    )));

    // Add box of random spheres
    let mut boxes2 = HittableList::new();
    let white = Arc::new(Material::lambertian_from_color(Color::new(
        0.73, 0.73, 0.73,
    )));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::still(
            Vec3::random_range(0.0, 165.0),
            10.0,
            Arc::clone(&white),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::from_hittable_list(boxes2)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    (
        world,
        Camera::new(
            1.0,
            800,
            40.0,
            Vec3::new(478.0, 278.0, -600.0),
            Vec3::new(278.0, 278.0, 0.0),
            0.0,
            1.0,
            Color::new(0.0, 0.0, 0.0),
            1000,
            50,
        ),
    )
}

fn main() {
    let (world, camera) = match 7 {
        0 => weekend_1(),
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => final_scene(),
        _ => todo!(),
    };

    let now = Instant::now();
    camera.render_single_threaded(&world).save("test");
    let elapsed_time = now.elapsed();

    println!("Rendering took {:.2} seconds.", elapsed_time.as_secs_f64());
}
