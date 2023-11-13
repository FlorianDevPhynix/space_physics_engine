use safer_ffi::prelude::repr_c;
use safer_ffi_deactivate::{ ffi_export, derive_ReprC };

pub mod util;
pub mod units;
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

#[derive_ReprC(opaque)]
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

#[allow(dead_code)]
#[allow(non_snake_case)]
#[ffi_export]
fn new_Simulation(sim_speed: f32) -> repr_c::Box<Simulation> {
    Box::new(Simulation::new(sim_speed)).into()
}

#[allow(dead_code)]
#[ffi_export]
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
