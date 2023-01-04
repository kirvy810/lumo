use super::*;
use crate::ray::Ray;
use std::ops::Range;

pub struct HittableList(Vec<Box<dyn Hittable>>);

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_vec(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self(objects)
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.0.push(object);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        self.0
            .iter()
            .filter_map(|object| object.hit(r, t_range.clone()))
            .min_by(|x, y| x.t.total_cmp(&y.t))
    }
}

impl FromIterator<Box<dyn Hittable>> for HittableList {
    fn from_iter<T: IntoIterator<Item = Box<dyn Hittable>>>(iter: T) -> Self {
        Self::from_vec(Vec::from_iter(iter))
    }
}
