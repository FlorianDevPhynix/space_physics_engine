fn main() {
    let a = space_physics_engine::Point { x: 84., y: 45. };
    let b = space_physics_engine::Point { x: 0., y: 39. };

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", space_physics_engine::mid_point(&a, &b));
}
