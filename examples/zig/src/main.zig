const std = @import("std");

const space_physics_engine = @cImport({
    @cInclude("space_physics_engine.h");
});
const Point = space_physics_engine.Point;

pub fn main() !void {
    const a = Point{ .x = 84, .y = 45 };
    const b = Point{ .x = 0, .y = 39 };
    const m = space_physics_engine.mid_point(&a, &b);
    space_physics_engine.print_point(&a);
    space_physics_engine.print_point(&b);
    space_physics_engine.print_point(&m);
}

test "simple test" {
    const a = Point{ .x = 84, .y = 45 };
    const b = Point{ .x = 0, .y = 39 };
    const m = space_physics_engine.mid_point(&a, &b);
    try std.testing.expectEqual(Point{ .x = 42, .y = 42 }, m);
}
