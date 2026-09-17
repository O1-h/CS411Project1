//! Project 1: The Rust Renderer -- rendering core.

use crate::scene::{Scene, Sphere};
use crate::vec3::{Ray, Vec3};
use std::sync::Arc;
use std::thread;

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Ray-sphere intersection via the quadratic formula. Returns the
/// smallest positive `t` (distance along the ray) at which the ray
/// hits the sphere, or None if there's no hit in front of the ray
/// origin.
pub fn ray_sphere_intersect(ray: &Ray, sphere: &Sphere) -> Option<f64> {
    // TODO (Phase 2): standard ray-sphere intersection.
    // oc = ray.origin - sphere.center_vec()
    // a = ray.direction.dot(&ray.direction)  (== 1.0 since direction is normalized, but compute it anyway)
    // b = 2.0 * oc.dot(&ray.direction)
    // c = oc.dot(&oc) - sphere.radius * sphere.radius
    // discriminant = b*b - 4.0*a*c
    // if discriminant < 0.0, no hit -> None
    // otherwise compute both roots, return the smallest POSITIVE one (t > small epsilon),
    // or None if both roots are <= epsilon (sphere is entirely behind the ray).
    None
}

/// Find the closest sphere (if any) a ray hits, returning (sphere_index, t, hit_point, normal).
fn closest_hit(ray: &Ray, spheres: &[Sphere]) -> Option<(usize, f64, Vec3, Vec3)> {
    let mut closest: Option<(usize, f64)> = None;
    for (i, sphere) in spheres.iter().enumerate() {
        if let Some(t) = ray_sphere_intersect(ray, sphere) {
            if closest.is_none() || t < closest.unwrap().1 {
                closest = Some((i, t));
            }
        }
    }
    closest.map(|(i, t)| {
        let hit_point = ray.at(t);
        let normal = (hit_point - spheres[i].center_vec()).normalize();
        (i, t, hit_point, normal)
    })
}

/// Generate the camera ray for pixel (px, py) out of a `width` x
/// `height` image, given the camera's position/look_at/fov.
fn generate_camera_ray(scene: &Scene, px: u32, py: u32) -> Ray {
    // TODO (Phase 2): perspective projection.
    // - Compute the camera's forward/right/up basis vectors from
    //   position and look_at (forward = normalize(look_at - position);
    //   right = normalize(forward.cross(&Vec3::new(0.0,1.0,0.0)));
    //   up = right.cross(&forward)).
    // - Convert fov_degrees to radians, compute the viewport
    //   half-height = tan(fov/2), half-width = half-height * aspect_ratio.
    // - Map (px, py) to normalized device coordinates in [-1, 1] (with
    //   a 0.5-pixel offset so you sample pixel CENTERS, not corners),
    //   then to a direction: forward + right*ndc_x*half_width + up*ndc_y*half_height.
    // - Return Ray::new(camera.position, direction).
    let position = Vec3::new(scene.camera.position[0], scene.camera.position[1], scene.camera.position[2]);
    Ray::new(position, Vec3::new(0.0, 0.0, -1.0))
}

/// Single-bounce Lambertian shading with hard shadows. For each
/// light, cast a shadow ray from the hit point toward the light; if
/// occluded by any sphere, that light contributes zero.
fn shade(scene: &Scene, sphere_index: usize, hit_point: Vec3, normal: Vec3) -> Vec3 {
    // TODO (Phase 2):
    // let material = &scene.spheres[sphere_index].material;
    // let mut color = Vec3::zero();
    // for light in &scene.lights {
    //     let to_light = (light.position_vec() - hit_point).normalize();
    //     let shadow_ray = Ray::new(hit_point + normal * 1e-4, to_light); // offset to avoid self-intersection
    //     let in_shadow = closest_hit(&shadow_ray, &scene.spheres).is_some();
    //     if !in_shadow {
    //         let diffuse = normal.dot(&to_light).max(0.0) * light.intensity;
    //         color = color + material.color_vec() * diffuse;
    //     }
    // }
    // color.clamp01()
    let _ = (scene, sphere_index, hit_point, normal);
    Vec3::zero()
}

fn trace_ray(scene: &Scene, ray: &Ray) -> Vec3 {
    match closest_hit(ray, &scene.spheres) {
        Some((i, _t, hit_point, normal)) => shade(scene, i, hit_point, normal),
        None => Vec3::new(0.05, 0.05, 0.08), // background color
    }
}

fn to_pixel(color: Vec3) -> Pixel {
    let c = color.clamp01();
    Pixel {
        r: (c.x * 255.0) as u8,
        g: (c.y * 255.0) as u8,
        b: (c.z * 255.0) as u8,
    }
}

/// Sequential renderer: get this correct before parallelizing anything.
pub fn render_sequential(scene: &Scene) -> Vec<Pixel> {
    let mut pixels = Vec::with_capacity((scene.width * scene.height) as usize);
    for py in 0..scene.height {
        for px in 0..scene.width {
            let ray = generate_camera_ray(scene, px, py);
            pixels.push(to_pixel(trace_ray(scene, &ray)));
        }
    }
    pixels
}

/// Parallel renderer: divide the image into row-band tiles, one per
/// thread, each rendering into its own local buffer (no shared
/// framebuffer lock -- see the Pro-Tips in the assignment).
pub fn render_parallel(scene: &Arc<Scene>, num_threads: usize) -> Vec<Pixel> {
    // TODO (Phase 3): divide [0, scene.height) into `num_threads`
    // contiguous row bands. Use std::thread::scope so each worker
    // closure can borrow `&scene` directly (Arc<Scene> also works,
    // clone it per thread). Each thread renders its band into its OWN
    // Vec<Pixel>, returns it; after all threads join, concatenate the
    // bands back together in row order to reassemble the full image
    // (bands must stay in original top-to-bottom order).
    let _ = num_threads;
    render_sequential(scene)
}
