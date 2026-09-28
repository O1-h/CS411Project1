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
    let t_load = Instant::now();
    let scene = Arc::new(Scene::load_from_file("scene_test.json").expect("failed to load scene_test.json"));
    let load_secs = t_load.elapsed().as_secs_f64();

    println!("Rendering sequential baseline...");
    let t0 = Instant::now();
    let seq_pixels = render_sequential(&scene);
    let seq_time = t0.elapsed().as_secs_f64();
    let t_enc = Instant::now();
    write_png(&seq_pixels, scene.width, scene.height, "render_sequential.png");
    let encode_secs = t_enc.elapsed().as_secs_f64();
    println!("Sequential: {:.3}s\n", seq_time);
    // Amdahl's Law
    let f = (load_secs + encode_secs) / (load_secs + seq_time + encode_secs);
    println!("Load: {:.4}s, PNG encode: {:.4}s, sequential fraction f = {:.4}", load_secs, encode_secs, f);

    println!("Threads |  Time (s) | Speedup | Amdahl");
    println!("--------+-----------+---------+-------");
    println!("{:7} | {:9.3} | {:7.2} | {:6.2}", 1, seq_time, 1.0, 1.0);

    let mut last_pixels = seq_pixels;
    for &n in &[2usize, 4, 8] {
        let t1 = Instant::now();
        let pixels = render_parallel(&scene, n);
        let elapsed = t1.elapsed().as_secs_f64();
        let amdahl = 1.0 / (f + (1.0 - f) / n as f64);
        println!("{:7} | {:9.3} | {:7.2} | {:6.2}", n, elapsed, seq_time / elapsed, amdahl);
        last_pixels = pixels;
    }

    write_png(&last_pixels, scene.width, scene.height, "render_parallel.png");
    println!("\nWrote render_sequential.png and render_parallel.png");
}

#[cfg(test)]
mod tests {
    use raytracer::render::{ray_sphere_intersect, render_parallel, render_sequential};
    use raytracer::scene::{Material, Scene, Sphere};
    use raytracer::vec3::{Ray, Vec3};
    use std::sync::Arc;

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

    #[test]
    fn one_thread_and_eight_threads_are_pixel_identical() {
        let scene = Arc::new(Scene::load_from_file("scene_test.json").expect("failed to load scene_test.json"));

        let seq = render_sequential(&scene);
        let one = render_parallel(&scene, 1);
        let eight = render_parallel(&scene, 8);

        assert_eq!(one.len(), eight.len());
        assert_eq!(seq.len(), eight.len());
        for i in 0..one.len() {
            assert_eq!((one[i].r, one[i].g, one[i].b), (eight[i].r, eight[i].g, eight[i].b), "1 vs 8 threads differ at pixel {i}");
            assert_eq!((seq[i].r, seq[i].g, seq[i].b), (eight[i].r, eight[i].g, eight[i].b), "sequential vs 8 threads differ at pixel {i}");
        }
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
