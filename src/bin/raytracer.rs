//! Project 1: The Rust Renderer -- main binary.
//!
//! Run: cargo run --bin raytracer
//! Test: cargo test --bin raytracer

use raytracer::render::{render_parallel, render_sequential};
use raytracer::scene::Scene;
use std::sync::Arc;
use std::time::Instant;

fn write_png(pixels: &[raytracer::render::Pixel], width: u32, height: u32, path: &str) {
    let mut buf: Vec<u8> = Vec::with_capacity((width * height * 3) as usize);
    for p in pixels {
        buf.push(p.r);
        buf.push(p.g);
        buf.push(p.b);
    }
    image::save_buffer(path, &buf, width, height, image::ColorType::Rgb8)
        .expect("failed to write PNG");
}

fn main() {
    let scene = Arc::new(Scene::load_from_file("scene_test.json").expect("failed to load scene_test.json"));

    println!("Rendering sequential baseline...");
    let t0 = Instant::now();
    let seq_pixels = render_sequential(&scene);
    let seq_time = t0.elapsed().as_secs_f64();
    write_png(&seq_pixels, scene.width, scene.height, "render_sequential.png");
    println!("Sequential: {:.3}s\n", seq_time);

    println!("Threads |  Time (s) | Speedup");
    println!("--------+-----------+--------");
    println!("{:7} | {:9.3} | {:7.2}", 1, seq_time, 1.0);

    let mut last_pixels = seq_pixels;
    for &n in &[2usize, 4, 8] {
        let t1 = Instant::now();
        let pixels = render_parallel(&scene, n);
        let elapsed = t1.elapsed().as_secs_f64();
        println!("{:7} | {:9.3} | {:7.2}", n, elapsed, seq_time / elapsed);
        last_pixels = pixels;
    }

    write_png(&last_pixels, scene.width, scene.height, "render_parallel.png");
    println!("\nWrote render_sequential.png and render_parallel.png");
}

#[cfg(test)]
mod tests {
    use raytracer::render::ray_sphere_intersect;
    use raytracer::scene::{Material, Sphere};
    use raytracer::vec3::{Ray, Vec3};

    fn unit_sphere_at_origin() -> Sphere {
        Sphere {
            center: [0.0, 0.0, -5.0],
            radius: 1.0,
            material: Material { color: [1.0, 0.0, 0.0], reflectivity: 0.0 },
        }
    }

    #[test]
    fn ray_hits_sphere_dead_center() {
        let sphere = unit_sphere_at_origin();
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = ray_sphere_intersect(&ray, &sphere);
        assert!(hit.is_some());
        // Sphere center at z=-5, radius 1 -> nearest surface point at z=-4, distance 4 from origin.
        assert!((hit.unwrap() - 4.0).abs() < 1e-6);
    }

    #[test]
    fn ray_misses_sphere_entirely() {
        let sphere = unit_sphere_at_origin();
        // Aimed far off to the side -- should miss a radius-1 sphere.
        let ray = Ray::new(Vec3::new(0.0, 10.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(ray_sphere_intersect(&ray, &sphere).is_none());
    }

    #[test]
    fn ray_originating_inside_sphere_hits_far_wall() {
        let sphere = unit_sphere_at_origin();
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = ray_sphere_intersect(&ray, &sphere);
        assert!(hit.is_some());
        assert!((hit.unwrap() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn ray_pointing_away_from_sphere_does_not_hit_behind_it() {
        let sphere = unit_sphere_at_origin();
        // Sphere is at z=-5 (in front); ray points toward +z (away from it).
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(ray_sphere_intersect(&ray, &sphere).is_none());
    }

    // NOTE: the 1-thread vs. 8-thread pixel-identical test (Part B,
    // Requirement 4) needs a real scene_test.json loaded via
    // Scene::load_from_file, which isn't exercised in this unit-test
    // module to keep it independent of the working directory `cargo
    // test` is run from. Add it as an integration test in `tests/`
    // once render_parallel is implemented, comparing
    // render_sequential(&scene) against render_parallel(&scene, 8)
    // pixel-by-pixel.
}
