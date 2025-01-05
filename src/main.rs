use std::rc::Rc;

use raytracer::{
    camera::Camera,
    hittable::{sphere::Sphere, HittableList},
    vec3::Vec3,
};

fn main() {
    // World
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new(16.0 / 9.0, 400, 100);

    camera.render(&world).save("test");
}
