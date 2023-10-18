use ::safer_ffi::prelude::*;

/// A `struct` usable from both Rust and C
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

/* Export a Rust function to the C world. */
/// Returns the middle point of `[a, b]`.
#[ffi_export]
fn mid_point(a: &Point, b: &Point) -> Point {
    Point {
        x: (a.x + b.x) / 2.,
        y: (a.y + b.y) / 2.,
    }
}

/// Pretty-prints a point using Rust's formatting logic.
#[ffi_export]
fn print_point(point: &Point) {
    println!("{:?}", point);
}

#[test]
fn second() {
    ;
}

// The following function is only necessary for the header generation.
#[::safer_ffi::cfg_headers]// replaces cfg(feature = "headers")
#[test]
pub fn generate_headers() -> ::std::io::Result<()> {
    crate::generate("headers/")
}

#[::safer_ffi::cfg_headers]
pub fn generate( header_folder: impl AsRef<std::path::Path> ) -> std::io::Result<()> {
    let path = header_folder.as_ref().join("space_physics_engine.h");
    //print!("{}", path.display());
    safer_ffi::headers::builder()
        .to_file(path)?
        .generate()
}
