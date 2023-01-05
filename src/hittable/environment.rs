use super::{HitRecord, Hittable};
use crate::color::Color;
use crate::ray::Ray;
use std::ops::Range;

pub struct Environment<H: Hittable, B: Fn(&Ray) -> Color + Send + Sync> {
    world: H,
    bg: B,
}

impl<H, B> Environment<H, B>
where
    H: Hittable,
    B: Fn(&Ray) -> Color + Send + Sync,
{
    pub fn new(world: H, bg: B) -> Self {
        Self { world, bg }
    }

    pub fn background(&self, r: &Ray) -> Color {
        (self.bg)(r)
    }
}

impl<H, B> Hittable for Environment<H, B>
where
    H: Hittable,
    B: Fn(&Ray) -> Color + Send + Sync,
{
    fn hit(&self, r: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        self.world.hit(r, t_range)
    }
}
