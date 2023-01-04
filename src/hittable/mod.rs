use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Vector3;
use std::ops::Range;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Vector3,
    pub n: Vector3,
    pub t: f64,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vector3, n: Vector3, t: f64, material: Arc<dyn Material>) -> Self {
        Self {
            p,
            n,
            t,
            material,
            front_face: false,
        }
    }

    pub fn set_face(mut self, r: &Ray) -> Self {
        self.front_face = r.direction.dot(&self.n) < 0.0;
        self.n = if self.front_face { self.n } else { -self.n };
        self
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_range: Range<f64>) -> Option<HitRecord>;
}

pub mod list;
pub mod sphere;
pub mod world;

pub use list::*;
pub use sphere::*;
pub use world::*;
