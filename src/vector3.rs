use overload::overload;
use std::ops;

#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        self / self.norm()
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Self) -> Self {
        Self::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn is_nearly_zero(&self) -> bool {
        self.x.abs() < 1e-8 && self.y.abs() < 1e-8 && self.z.abs() < 1e-8
    }
}

overload!((u: ?Vector3) + (v: ?Vector3) -> Vector3 {
    Vector3::new(u.x + v.x, u.y + v.y, u.z + v.z)
});

overload!((u: ?Vector3) - (v: ?Vector3) -> Vector3 {
    Vector3::new(u.x - v.x, u.y - v.y, u.z - v.z)
});

overload!((u: ?Vector3) * (t: ?f64) -> Vector3 {
    Vector3::new(u.x * t, u.y * t, u.z * t)
});

overload!((t: ?f64) * (u: ?Vector3) -> Vector3 {
    Vector3::new(u.x * t, u.y * t, u.z * t)
});

overload!((u: ?Vector3) / (t: ?f64) -> Vector3 {
    Vector3::new(u.x / t, u.y / t, u.z / t)
});

overload!((u: &mut Vector3) += (v: ?Vector3) {
    u.x += v.x;
    u.y += v.y;
    u.z += v.z;
});

overload!(-(u: ?Vector3) -> Vector3 {
    Vector3::new(-u.x, -u.y, -u.z)
});

overload!((u: &mut Vector3) -= (v: ?Vector3) {
    u.x -= v.x;
    u.y -= v.y;
    u.z -= v.z;
});

overload!((u: &mut Vector3) *= (t: ?f64) {
    u.x *= t;
    u.y *= t;
    u.z *= t;
});

overload!((u: &mut Vector3) /= (t: ?f64) {
    u.x /= t;
    u.y /= t;
    u.z /= t;
});
