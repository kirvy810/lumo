use crate::vector3::Vector3;
use rand::*;

fn random_real() -> f64 {
    thread_rng().gen_range(-1.0..=1.0)
}

pub fn random_in_unit_sphere() -> Vector3 {
    loop {
        let v = Vector3::new(random_real(), random_real(), random_real());

        if v.norm_squared() < 1.0 {
            return v;
        }
    }
}

pub fn random_unit_vector() -> Vector3 {
    random_in_unit_sphere().normalized()
}

pub fn random_in_hemisphere(n: &Vector3) -> Vector3 {
    let v = random_in_unit_sphere();

    if v.dot(n) > 0.0 {
        v
    } else {
        -v
    }
}

pub fn random_in_unit_disk() -> Vector3 {
    loop {
        let v = Vector3::new(random_real(), random_real(), 0.0);

        if v.norm_squared() < 1.0 {
            return v;
        }
    }
}
