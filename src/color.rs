use crate::vector3::Vector3;
use overload::overload;
use std::ops;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn gamma(self) -> Self {
        Self {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }

    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        (1.0 - t) * self + t * other
    }
}

impl From<Vector3> for Color {
    fn from(v: Vector3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

overload!((lhs: ?Color) + (rhs: ?Color) -> Color {
    Color::new(lhs.r + rhs.r, lhs.g + rhs.g, lhs.b + rhs.b)
});

overload!((lhs: ?Color) * (rhs: ?Color) -> Color {
    Color::new(lhs.r * rhs.r, lhs.g * rhs.g, lhs.b * rhs.b)
});

overload!((t: ?f64) * (c: ?Color) -> Color {
    Color::new(t * c.r, t * c.g, t * c.b)
});

overload!((c: ?Color) * (t: ?f64) -> Color {
    Color::new(t * c.r, t * c.g, t * c.b)
});

overload!((c: ?Color) / (t: ?f64) -> Color {
    Color::new(c.r / t, c.g / t, c.b / t)
});
