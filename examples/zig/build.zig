const std = @import("std");

// Although this function looks imperative, note that its job is to
// declaratively construct a build graph that will be executed by an external
// runner.
pub fn build(b: *std.Build) void {
    var genpurpose = std.heap.GeneralPurposeAllocator(.{}){};
    var alloc = genpurpose.allocator();
    _ = alloc;

    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{
        //.default_target = .{
        //    .abi = .gnu,
        //},
    });

    // Standard optimization options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall. Here we do not
    // set a preferred release mode, allowing the user to decide how to optimize.
    const optimize = b.standardOptimizeOption(.{});

    const Mode = enum {
        Debug,
        Release,

        const Self = @This();
        fn into(self: Self) []const u8 {
            return switch (self) {
                .Debug => "debug",
                .Release => "release",
            };
        }
    };
    const mode = comptime Mode.Debug;

    // build library
    const c_lib_build = b.addSystemCommand(&[_][]const u8{ "cargo", "build", "-p", "space_physics_engine", "--features", "ffi_compile" });
    if (mode == .Release) {
        c_lib_build.addArg("--release");
    }
    c_lib_build.setName("cargo build");

    // build library c header file
    const c_lib_generate = b.addSystemCommand(&[_][]const u8{ "cargo", "run", "-p", "space_physics_engine", "--features", "headers", "--bin", "generate-headers" });
    //const c_lib_generate = b.addSystemCommand(&[_][]const u8{ "cargo", "test", "-p", "space_physics_engine", "--features", "headers", "--", "generate_headers" });
    c_lib_generate.cwd = "../../";
    c_lib_generate.setName("cargo run header-generate");
    c_lib_generate.step.dependOn(&c_lib_build.step);

    const exe = b.addExecutable(.{
        .name = "zig_example",
        // In this case the main source file is merely a path, however, in more
        // complicated build scripts, this could be a generated file.
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    //const lib = b.addObject(.{
    //    .name = "space_physics_engine",
    //    .target = target,
    //    .optimize = optimize,
    //    .root_source_file = .{ .path = "../../target/release/libspace_physics_engine.a" },
    //});
    //lib.step.dependOn(&c_lib_generate.step);

    exe.addLibraryPath(.{ .path = std.fmt.comptimePrint("../../target/{s}/", .{comptime mode.into()}) });
    exe.linkSystemLibrary("space_physics_engine");
    exe.linkLibC();
    exe.linkLibCpp();
    exe.addIncludePath(.{ .path = "../../target/headers/" });
    exe.step.dependOn(&c_lib_generate.step);

    // This declares intent for the executable to be installed into the
    // standard location when the user invokes the "install" step (the default
    // step when running `zig build`).
    b.installArtifact(exe);

    // This *creates* a Run step in the build graph, to be executed when another
    // step is evaluated that depends on it. The next line below will establish
    // such a dependency.
    const run_cmd = b.addRunArtifact(exe);

    // By making the run step depend on the install step, it will be run from the
    // installation directory rather than directly from within the cache directory.
    // This is not necessary, however, if the application depends on other installed
    // files, this ensures they will be present and in the expected location.
    run_cmd.step.dependOn(b.getInstallStep());

    // This allows the user to pass arguments to the application in the build
    // command itself, like this: `zig build run -- arg1 arg2 etc`
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    // This creates a build step. It will be visible in the `zig build --help` menu,
    // and can be selected like this: `zig build run`
    // This will evaluate the `run` step rather than the default, which is "install".
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    // Creates a step for unit testing. This only builds the test executable
    // but does not run it.
    const unit_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    unit_tests.addLibraryPath(.{ .path = std.fmt.comptimePrint("../../target/{s}/", .{comptime mode.into()}) });
    unit_tests.linkSystemLibrary("space_physics_engine");
    unit_tests.linkLibC();
    unit_tests.linkLibCpp();
    unit_tests.addIncludePath(.{ .path = "../../src/headers/" });
    unit_tests.step.dependOn(&c_lib_generate.step);

    const run_unit_tests = b.addRunArtifact(unit_tests);

    // Similar to creating the run step earlier, this exposes a `test` step to
    // the `zig build --help` menu, providing a way for the user to request
    // running the unit tests.
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);
}
