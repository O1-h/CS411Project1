//! Project 1: The Rust Renderer -- scene types (given, complete).
//! JSON-deserializable via serde, matching scene_test.json's shape.

use crate::vec3::Vec3;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Material {
    pub color: [f64; 3],
    pub reflectivity: f64,
}

impl Material {
    pub fn color_vec(&self) -> Vec3 {
        Vec3::new(self.color[0], self.color[1], self.color[2])
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sphere {
    pub center: [f64; 3],
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn center_vec(&self) -> Vec3 {
        Vec3::new(self.center[0], self.center[1], self.center[2])
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Light {
    pub position: [f64; 3],
    pub intensity: f64,
}

impl Light {
    pub fn position_vec(&self) -> Vec3 {
        Vec3::new(self.position[0], self.position[1], self.position[2])
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CameraConfig {
    pub position: [f64; 3],
    pub look_at: [f64; 3],
    pub fov_degrees: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: CameraConfig,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn load_from_file(path: &str) -> Result<Scene, Box<dyn std::error::Error>> {
        let text = std::fs::read_to_string(path)?;
        let scene: Scene = serde_json::from_str(&text)?;
        Ok(scene)
    }
}
