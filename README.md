# space_physics_engine

## WIP

This project is in a very early stage, so expect breaking changes.

## Use

### From Rust

Run `cargo add space_physics_engine` in your rust project or
add `space_physics_engine = "0.1.0"` to your *Cargo.toml*.

### Any C ABI compatible Programming Language

Clone or download the repository and compile it as a ffi static library using
`cargo build --features ffi_compile --release` with the binary output in `target/release`.
The header file can be generated with `cargo run --features headers --bin generate-headers`.
Those files will be in the `target/headers/` folder.

## Examples

Currently there are Rust, C and Zig examples.

### Rust

`cargo build -p rust_example`

`cargo run -p rust_example`

### C

In the `examples/c` folder use `make` to build the rust library,
generate the header file and compile those into the c executable.

Use `make run` to run the result after build.

### Zig

In the `examples/zig` folder use `zig build` to build the rust library,
generate the header file and compile those into the zig executable.

`zig build run` to run the result after build.

`zig build test` to run all tests
