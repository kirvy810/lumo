use crate::random::random_in_unit_disk;
use crate::ray::Ray;
use crate::vector3::Vector3;

pub struct CameraBuilder {
    lookfrom: Vector3,
    lookat: Vector3,
    vup: Vector3,
    fov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            lookfrom: Vector3::ZERO,
            lookat: Vector3::new(0.0, 0.0, -1.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            fov: std::f64::consts::FRAC_PI_2,
            aspect_ratio: 16.0 / 9.0,
            aperture: 0.0,
            focus_dist: 1.0,
        }
    }

    pub fn build(self) -> Camera {
        Camera::new(
            self.lookfrom,
            self.lookat,
            self.vup,
            self.fov,
            self.aspect_ratio,
            self.aperture,
            self.focus_dist,
        )
    }

    pub fn with_lookfrom(mut self, lookfrom: Vector3) -> Self {
        self.lookfrom = lookfrom;
        self
    }

    pub fn with_lookat(mut self, lookat: Vector3) -> Self {
        self.lookat = lookat;
        self
    }

    pub fn with_vup(mut self, vup: Vector3) -> Self {
        self.vup = vup;
        self
    }

    pub fn with_fov(mut self, fov: f64) -> Self {
        self.fov = fov;
        self
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_aperture(mut self, aperture: f64) -> Self {
        self.aperture = aperture;
        self
    }

    pub fn with_focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }
}

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vector3,
        lookat: Vector3,
        vup: Vector3,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let viewport_height = 2.0 * (fov / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&lookfrom - &lookat).normalized();
        let u = vup.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * &u;
        let vertical = focus_dist * viewport_height * &v;
        let lower_left_corner = &origin - &horizontal / 2.0 - &vertical / 2.0 - focus_dist * &w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let scale = self.lens_radius * random_in_unit_disk();
        let offset = &self.u * scale.x + &self.v * scale.y;
        let origin = &self.origin - offset;
        let direction =
            &self.lower_left_corner + u * &self.horizontal + v * &self.vertical - &origin;

        Ray::new(origin, direction)
    }
}
