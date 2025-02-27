use std::rc::Rc;

use nanorand::{tls_rng, Rng};

use crate::{hittable::HitRecord, image::color::Color, ray::Ray, texture::Texture, vec3::Vec3};

pub enum Material {
    Dielectric {
        albedo: Color,
        // Ratio of material's ior over ior of emclosing media
        refraction_index: f64,
    },
    DiffuseLight {
        texture: Rc<Texture>,
    },
    Lambertian {
        texture: Rc<Texture>,
    },
    Metal {
        albedo: Color,
        roughness: f64,
    },
    Isotropic {
        texture: Rc<Texture>,
    },
}

// Constructors
impl Material {
    pub fn dielectric(refraction_index: f64) -> Self {
        Self::Dielectric {
            albedo: Color::white(),
            refraction_index,
        }
    }

    pub fn diffuse_light(texture: Rc<Texture>) -> Self {
        Self::DiffuseLight { texture }
    }

    pub fn diffuse_light_from_color(albedo: Color) -> Self {
        Self::DiffuseLight {
            texture: Rc::new(Texture::color(albedo)),
        }
    }

    pub fn lambertian(texture: Rc<Texture>) -> Self {
        Self::Lambertian { texture }
    }

    pub fn lambertian_from_color(albedo: Color) -> Self {
        Self::Lambertian {
            texture: Rc::new(Texture::color(albedo)),
        }
    }

    pub fn metal(albedo: Color, roughness: f64) -> Self {
        Self::Metal { albedo, roughness }
    }

    pub fn isotropic(texture: Rc<Texture>) -> Self {
        Self::Isotropic { texture }
    }

    pub fn isotropic_from_color(albedo: Color) -> Self {
        Self::Isotropic {
            texture: Rc::new(Texture::color(albedo)),
        }
    }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Dielectric {
                albedo,
                refraction_index,
            } => Self::dielectric_scatter(albedo, *refraction_index, ray, hit_record),
            Material::DiffuseLight { .. } => None,
            Material::Lambertian { texture } => Self::lambertian_scatter(texture, ray, hit_record),
            Material::Metal { albedo, roughness } => {
                Self::metal_scatter(albedo, *roughness, ray, hit_record)
            }
            Material::Isotropic { texture } => Self::isotropic_scatter(texture, ray, hit_record),
        }
    }
    
    /// Calculates Schlick's approximation for reflectance
    fn dielectric_reflectance(cos: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }

    fn dielectric_scatter(
        albedo: &Color,
        refraction_index: f64,
        ray: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let refraction_index = if hit_record.front_face {
            1.0 / refraction_index
        } else {
            refraction_index
        };

        let unit_direction = ray.direction.normalize();
        let cos = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let cannot_refract = refraction_index * sin > 1.0;

        let direction: Vec3;
        if cannot_refract
            || Self::dielectric_reflectance(cos, refraction_index) > tls_rng().generate()
        {
            direction = unit_direction.reflected(&hit_record.normal);
        } else {
            direction = unit_direction
                .normalize()
                .refracted(&hit_record.normal, refraction_index);
        }

        Some((
            albedo.clone(),
            Ray::new(hit_record.pos, direction, ray.time),
        ))
    }

    fn lambertian_scatter(
        texture: &Rc<Texture>,
        ray: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some((
            texture.value(hit_record.uv, &hit_record.pos),
            Ray::new(hit_record.pos, scatter_direction, ray.time),
        ))
    }

    fn metal_scatter(
        albedo: &Color,
        roughness: f64,
        ray: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let mut reflected = ray.direction.reflected(&hit_record.normal);

        reflected = reflected.normalize() + (roughness * Vec3::random_unit());

        if reflected.dot(&hit_record.normal) > 0.0 {
            Some((
                albedo.clone(),
                Ray::new(hit_record.pos, reflected, ray.time),
            ))
        } else {
            None
        }
    }

    fn isotropic_scatter(
        texture: &Rc<Texture>,
        ray: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let scattered = Ray::new(hit_record.pos, Vec3::random_unit(), ray.time);
        let attenuation = texture.value(hit_record.uv, &hit_record.pos);

        Some((attenuation, scattered))
    }

    pub fn emitted(&self, uv: (f64, f64), pos: &Vec3) -> Color {
        match self {
            Material::DiffuseLight { texture } => texture.value(uv, pos),
            _ => Color::black(),
        }
    }
}
