use super::Material;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::random::random_unit_vector;
use crate::ray::Ray;

pub struct Diffuse {
    color: Color,
}

impl Diffuse {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _: &Ray, hit: HitRecord) -> Option<(Color, Ray)> {
        let direction = &hit.n + random_unit_vector();
        let direction = if direction.is_nearly_zero() {
            hit.n
        } else {
            direction
        };

        Some((self.color.clone(), Ray::new(hit.p, direction)))
    }
}
