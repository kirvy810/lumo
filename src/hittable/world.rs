use super::list::HittableList;
use super::{HitRecord, Hittable};
use crate::color::Color;
use crate::ray::Ray;
use std::ops::Range;

pub struct World<B: Fn(&Ray) -> Color + Send + Sync> {
    pub list: HittableList,
    bg: B,
}

impl<B> World<B>
where
    B: Fn(&Ray) -> Color + Send + Sync,
{
    pub fn new(list: HittableList, bg: B) -> Self {
        Self { list, bg }
    }

    pub fn background(&self, r: &Ray) -> Color {
        (self.bg)(r)
    }
}

impl<B> Hittable for World<B>
where
    B: Fn(&Ray) -> Color + Send + Sync,
{
    fn hit(&self, r: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        self.list.hit(r, t_range)
    }
}
