use lumo::camera::CameraBuilder;
use lumo::color::Color;
use lumo::hittable::World;
use lumo::hittable::list::HittableList;
use lumo::hittable::sphere::Sphere;
use lumo::material::{Dielectric, Diffuse, Material, Metal};
use lumo::ray::Ray;
use lumo::render::{Image, Renderer};
use lumo::vector3::Vector3;
use rand::Rng;
use std::sync::Arc;

fn random_balls() -> World<fn(&Ray) -> Color> {
    let glass = Arc::new(Dielectric::new(1.5));

    let mut balls = HittableList::from_vec(vec![
        Box::new(Sphere::new(
            Vector3::new(0.0, -1000.0, -1.0),
            1000.0,
            Arc::new(Diffuse::new(Color::new(1.0, 0.8, 0.95))),
        )),
        Box::new(Sphere::new(
            Vector3::new(-3.0, 1.0, 0.0),
            1.0,
            Arc::new(Diffuse::new(Color::new(0.0, 0.9, 0.5))),
        )),
        Box::new(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, glass.clone())),
        Box::new(Sphere::new(
            Vector3::new(3.0, 1.0, 0.0),
            1.0,
            Arc::new(Metal::new(Color::new(0.2, 0.5, 0.8)).with_fuzz(0.2)),
        )),
    ]);

    let mut rng = rand::thread_rng();
    for x in -10..=10 {
        for z in -10..=10 {
            let offset = Vector3::new(rng.gen_range(0.0..=0.9), 0.0, rng.gen_range(0.0..=0.9));
            let center = Vector3::new(x as f64, 0.2, z as f64) + offset;
            let material: Arc<dyn Material + Sync + Send> = match rng.gen_range(0..100) {
                0..=79 => Arc::new(Diffuse::new(Color::new(
                    rng.gen_range(0.7..=1.0),
                    rng.gen_range(0.7..=1.0),
                    rng.gen_range(0.7..=1.0),
                ))),
                80..=94 => Arc::new(
                    Metal::new(Color::new(
                        rng.gen_range(0.7..=1.0),
                        rng.gen_range(0.7..=1.0),
                        rng.gen_range(0.7..=1.0),
                    ))
                    .with_fuzz(rng.gen_range(0.0..=0.3)),
                ),
                _ => glass.clone(),
            };

            balls.add(Box::new(Sphere::new(center, 0.2, material)));
        }
    }

    World::new(balls, |r| {
        let t = 0.5 * (r.direction.normalized().y + 1.0);
        Color::WHITE.lerp(&Color::new(0.5, 0.7, 1.0), t)
    })
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1280;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples = 128;
    let depth = 8;

    let camera = CameraBuilder::new()
        .with_lookfrom(Vector3::new(11.0, 2.0, 6.0))
        .with_lookat(Vector3::new(0.0, 0.0, 0.0))
        .with_fov(std::f64::consts::FRAC_PI_8)
        .with_focus_dist(10.0)
        .with_aperture(0.1)
        .build();

    let world = random_balls();

    let renderer = Renderer::new(image_width, image_height, samples, depth, camera);
    let buffer = renderer.render(world);

    if let Err(e) = Image::new(image_width, image_height, buffer).save("rtiow.png") {
        eprintln!("{:?}", e);
    }
}
