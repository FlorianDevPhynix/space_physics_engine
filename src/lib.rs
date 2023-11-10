use safer_ffi::prelude::repr_c;
use safer_ffi_deactivate::{ ffi_export, derive_ReprC };

pub mod gravity;
pub mod pmo;
pub mod dso;

pub mod math{
    pub use glam::*;

    pub mod prelude {
        pub use glam::{
            DQuat, dquat,
            DVec2, DVec3, DVec4, dvec2, dvec3, dvec4,
            DMat2, DMat3, DMat4, dmat2, dmat3, dmat4,
            DAffine2, DAffine3,
        };
    }
}

use math::prelude::*;

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

    fn get_transform(self: &Self) -> DVec3 {
        DVec3 { x: 0.0, y: 1.0, z: 1.0 }
    }
}

pub mod util {
    pub mod clamp {
        use crate::math::prelude::*;
        use crate::math::Vec3;

        pub fn calc_percentage(max_distance: f32, towards: DVec3, point: DVec3) -> f32 {
            let distance = towards.distance(point);
            let result = max_distance as f64 * 100.0 / distance;
            if result < f32::MAX as f64 {
                result as f32
            } else {
                println!("calc_percentage result is bigger than f32");
                f32::MAX
            }
        }

        pub fn calc_vec3_percentage(max_distance: f32, towards: Vec3, point: DVec3) -> f32 {
            calc_percentage(max_distance, towards.as_dvec3(), point)
        }

        pub fn bring_closer(percentage: f32, towards: DVec3, point: DVec3) -> Vec3 {
            point.lerp(towards, (100.0 - percentage as f64) / 100.0).as_vec3()
        }

        pub fn bring_vec3_closer(percentage: f32, towards: Vec3, point: DVec3) -> Vec3 {
            bring_closer(percentage, towards.as_dvec3(), point)
        }

        pub fn calc_scale(percentage: f32, scale: DVec3) -> Vec3 {
            (percentage as f64 * scale / 100.0).as_vec3()
        }
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

/* #[allow(dead_code)]
#[::safer_ffi_deactivate::ffi_export]
fn sim_get_transform(sim: &'_ Simulation) -> DVec3 {
    Simulation::get_transform(sim)
} */

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
