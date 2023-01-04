use super::{reflect, Material};
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector3::Vector3;

pub struct Dielectric {
    index: f64,
}

impl Dielectric {
    pub fn new(index: f64) -> Self {
        Self { index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit: HitRecord) -> Option<(Color, Ray)> {
        let direction = r.direction.normalized();
        let cos_theta = (-direction.dot(&hit.n)).min(1.0);
        let index = if hit.front_face {
            1.0 / self.index
        } else {
            self.index
        };

        let direction = if index * (1.0 - cos_theta * cos_theta).sqrt() > 1.0
            || schlick(cos_theta, index) > rand::random()
        {
            reflect(&direction, &hit.n)
        } else {
            refract(&direction, &hit.n, index)
        };

        Some((Color::WHITE, Ray::new(hit.p, direction)))
    }
}

fn refract(uv: &Vector3, n: &Vector3, index: f64) -> Vector3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let perp = index * (uv + cos_theta * n);
    let par = -(1.0 - perp.norm_squared()).abs().sqrt() * n;
    perp + par
}

fn schlick(cos_theta: f64, index: f64) -> f64 {
    let r0 = ((1.0 - index) / (1.0 + index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
