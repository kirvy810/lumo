use super::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Vector3;
use std::ops::Range;
use std::sync::Arc;

pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let oc = &r.origin - &self.center;
        let a = r.direction.norm_squared();
        let b = oc.dot(&r.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let d = b * b - a * c;

        if d < 0.0 {
            return None;
        }

        [(-b - d.sqrt()) / a, (-b + d.sqrt()) / a]
            .into_iter()
            .find(|t| t_range.contains(t))
            .map(|t| {
                let p = r.at(t);
                let n = (&p - &self.center) / self.radius;
                HitRecord::new(p, n, t, self.material.clone()).set_face(r)
            })
    }
}
