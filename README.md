# Project 1: The Rust Renderer

Public template: https://github.com/cwillpinto/cs411-project01-rust-renderer

Full assignment: `Project_1_The_Rust_Renderer.md`.

The starter contains the project structure and incomplete rendering functions.
Complete the TODOs in the order described by the assignment, verify correctness
before benchmarking, and keep instructor solution material outside this folder.

## Files
- `src/vec3.rs` -- `Vec3`/`Ray` math (given, complete)
- `src/scene.rs` -- JSON-deserializable scene types (given, complete)
- `src/render.rs` -- complete `ray_sphere_intersect`, `generate_camera_ray`, `shade`, and `render_parallel`
- `src/bin/raytracer.rs` -- main binary + `cargo test --bin raytracer` unit tests for `ray_sphere_intersect`
- `scene_test.json` -- given test scene (4 spheres including a ground plane, 2 lights)

## Run
```bash
cargo test --bin raytracer     # ray-sphere intersection unit tests
cargo run --bin raytracer      # renders + benchmarks
```

## Verification

Run `cargo test --bin raytracer` first. The supplied unit tests cover four
ray-sphere intersection cases. Your submission must also provide evidence that
the sequential and parallel renders are pixel-identical before you interpret
any timing result.

## Submit
Due **Monday, September 21, 2026 at 11:59 PM**.

1. `DESIGN.md`
2. Full source, `speedup_plot.png`-equivalent benchmark table/plot
3. `cargo test --bin raytracer` passing
4. Rendered PNG output

These items collectively satisfy the embedded Parallel Phase Gate; it is not a
separately scored submission.
