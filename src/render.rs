use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, World};
use crate::ray::Ray;
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::path::Path;

pub struct Image {
    width: usize,
    height: usize,
    buffer: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize, buffer: Vec<Color>) -> Self {
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn save<P: AsRef<Path>>(self, path: P) -> image::ImageResult<()> {
        let mut img = RgbImage::new(self.width as u32, self.height as u32);

        for (pixel, color) in img.pixels_mut().zip(self.buffer) {
            let r = (255.0 * color.r.clamp(0.0, 1.0)) as u8;
            let g = (255.0 * color.g.clamp(0.0, 1.0)) as u8;
            let b = (255.0 * color.b.clamp(0.0, 1.0)) as u8;

            *pixel = Rgb([r, g, b]);
        }

        img.save(path)
    }
}

pub struct Renderer {
    image_width: usize,
    image_height: usize,
    samples: u32,
    depth: u32,
    camera: Camera,
}

impl Renderer {
    pub fn new(
        image_width: usize,
        image_height: usize,
        samples: u32,
        depth: u32,
        camera: Camera,
    ) -> Self {
        Self {
            image_width,
            image_height,
            samples,
            depth,
            camera,
        }
    }

    pub fn render<B>(&self, world: World<B>) -> Vec<Color>
    where
        B: Fn(&Ray) -> Color + Send + Sync,
    {
        let image_size = self.image_height * self.image_width;
        let pb = ProgressBar::new(image_size as u64);
        pb.set_style(
            ProgressStyle::with_template("{spinner} [{elapsed_precise}] [{wide_bar}] {pos}/{len}")
                .unwrap()
                .progress_chars("#>-"),
        );

        let colors = (0..image_size)
            .into_par_iter()
            .map(|x| (x % self.image_width, x / self.image_width))
            .map(|(i, j)| {
                let color = (0..self.samples)
                    .map(|_| {
                        let (u, v) = self.uv(i, self.image_height - j);
                        let r = self.camera.ray(u, v);
                        Self::ray_color(r, &world, self.depth)
                    })
                    .fold(Color::BLACK, |x, y| x + y);

                pb.inc(1);

                (color / self.samples as f64).gamma()
            })
            .collect();

        pb.finish();
        colors
    }

    fn uv(&self, i: usize, j: usize) -> (f64, f64) {
        let (s, t) = rand::random::<(f64, f64)>();
        let u = (i as f64 + s) / (self.image_width as f64 - 1.0);
        let v = (j as f64 + t) / (self.image_height as f64 - 1.0);

        (u, v)
    }

    fn ray_color<B>(r: Ray, world: &World<B>, depth: u32) -> Color
    where
        B: Fn(&Ray) -> Color + Send + Sync,
    {
        if depth == 0 {
            return Color::BLACK;
        }

        if let Some(hit) = world.hit(&r, 1e-6..f64::INFINITY) {
            let material = hit.material.clone();
            return material
                .scatter(&r, hit)
                .map_or(Color::BLACK, |(color, scattered)| {
                    color * Self::ray_color(scattered, world, depth - 1)
                });
        }

        world.background(&r)
    }
}
