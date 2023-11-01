use safer_ffi::prelude::repr_c;
use safer_ffi_deactivate::{ ffi_export, derive_ReprC };

/// A `struct` usable from both Rust and C
#[derive_ReprC(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/* Export a Rust function to the C world. */
/// Returns the middle point of `[a, b]`.
#[ffi_export]
pub fn mid_point(a: &Point, b: &Point) -> Point {
    Point {
        x: (a.x + b.x) / 2.,
        y: (a.y + b.y) / 2.,
    }
}

/// Pretty-prints a point using Rust's formatting logic.
#[ffi_export]
pub fn print_point(point: &Point) {
    println!("{:?}", point);
}

#[::safer_ffi_deactivate::derive_ReprC(opaque)]
#[derive(Debug)]
struct Simulation {
    sim_speed: f32,
}

impl Simulation {
    fn new(sim_speed: f32) -> Simulation {
        Simulation{ sim_speed }
    }

    fn simulate(self: &Self, delta: f32) {
        println!("simulation speed = {}; delta = {}", self.sim_speed, delta);
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[::safer_ffi_deactivate::ffi_export]
fn new_Simulation(sim_speed: f32) -> repr_c::Box<Simulation> {
    Box::new(Simulation::new(sim_speed)).into()
}

#[allow(dead_code)]
#[::safer_ffi_deactivate::ffi_export]
fn sim_simulate(sim: &'_ Simulation, delta: f32) {
    Simulation::simulate(sim, delta);
}

#[allow(dead_code)]
#[ffi_export]
fn sim_free(sim: repr_c::Box<Simulation>)
{
    drop(sim);
}

// The following function is only necessary for the header generation.
#[::safer_ffi::cfg_headers]// replaces cfg(feature = "headers")
#[test]
pub fn generate_headers() -> ::std::io::Result<()> {
    crate::generate("headers/")
}

#[::safer_ffi::cfg_headers]
pub fn generate( header_folder: impl AsRef<std::path::Path> ) -> std::io::Result<()> {
    let output_path = header_folder.as_ref().join("space_physics_engine.h");
    //print!("{}", output_path.display());

    if !header_folder.as_ref().is_dir() {
        std::fs::create_dir_all(header_folder)?;
    }

    safer_ffi::headers::builder()
        .to_file(output_path)?
        .generate()
}
