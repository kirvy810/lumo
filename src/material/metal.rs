use super::Material;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::random::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vector3::Vector3;

pub struct Metal {
    color: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Self { color, fuzz: 0.0 }
    }

    pub fn with_fuzz(self, fuzz: f64) -> Self {
        Self {
            color: self.color,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit: HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&r.direction.normalized(), &hit.n);
        let offset = self.fuzz * random_in_unit_sphere();
        let scattered = Ray::new(hit.p, reflected + offset);

        if 0.0 < scattered.direction.dot(&hit.n) {
            Some((self.color.clone(), scattered))
        } else {
            None
        }
    }
}

pub(super) fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    v - 2.0 * v.dot(n) * n
}
