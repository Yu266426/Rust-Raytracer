use std::rc::Rc;

use raytracer::{
    camera::Camera,
    hittable::{sphere::Sphere, HittableList},
    image::color::Color,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    vec3::Vec3,
};

fn main() {
    // World
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

    // Camera
    let camera = Camera::new(16.0 / 9.0, 400, 90.0, 100, 50);

    camera.render(&world).save("test");
}
