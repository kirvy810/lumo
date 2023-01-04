use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material: Send + Sync {
    fn scatter(&self, r: &Ray, rec: HitRecord) -> Option<(Color, Ray)>;
}

mod dielectric;
mod diffuse;
mod metal;

pub use dielectric::*;
pub use diffuse::*;
pub use metal::*;
