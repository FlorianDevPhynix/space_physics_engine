use crate::math::prelude::*;

/// Calculate gravity Force between two objects
///
/// https://www.wikiwand.com/en/Newton%27s_law_of_universal_gravitation#Vector_form
///
/// force vector direction: object a -> object b
pub fn calc_gravity_force( a_mass: f64, a_position: DVec3, b_mass: f64, b_position: DVec3 ) -> DVec3 {
    // r2 - r1 -> dyn - static
    let distance = a_position.distance( b_position );
    // unit vector from object 1 to object 2
    let unit_between = ( a_position - b_position ) / distance;

    static GRAVITY_CONSTANT: f64 = 0.000000000066743015;
    // Newton's law of universal gravitation - Vector form
    // gravitational constant * ( m1 * m2 ) / distance^2 * unit_between
    -GRAVITY_CONSTANT * ( b_mass * a_mass ) / distance.powi(2) * unit_between
}

/// Calculate the acceleration vector from a force for a object with it's mass.
pub fn calc_acceleration( force: DVec3, mass: f64 ) -> DVec3 {

    // F = m * a => a = f / m
    force / mass
}

/// Calculate the velocity vector of a object in the future.
///
/// NOTE: formula expects constant acceleration, gravity does not provide constant force/acceleration.
/// Could result in wrong or inconsistent calculation results.
pub fn calc_future_velocity( time_step: f64, velocity: DVec3, acceleration: DVec3 ) -> DVec3 {

    // delta v = a * delta t => v - v0 = a * ( t - t0 ) => v = v0 + a * ( t - t0 )
    velocity + acceleration * time_step
}

/// Calculate a future position.
///
/// NOTE: formula expects constant acceleration, gravity does not provide constant force/acceleration.
/// Could result in wrong or inconsistent calculation results if time_Step too big.
pub fn calc_future_position( time_step: f64, acceleration: DVec3, velocity: DVec3, position: DVec3 ) -> DVec3 {

    // s = a / 2 * t^2 + v0 * t + s0
    acceleration / 2.0 * time_step.powi(2) + velocity * time_step + position
    // alternative: future_position = acceleration * time_step + position
}