use core::ops::AddAssign;
use std::cell::RefCell;

use crate::units::*;
use crate::math::prelude::*;

pub fn calc_mass(density: f32, volume: f64) -> f64 {
	density as f64 * volume
}

pub fn calc_volume(radius: f64) -> f64 {
	4.0 / 3.0 * std::f64::consts::PI * radius.powi(3)
}

/// A planetary-mass object (PMO), planemo, or planetary body
#[derive(Debug)]
#[allow(dead_code)]
pub struct PlanetaryMassObject {
	/// This PMO will not be affected by any gravitational forces.
	/// Example: Stars, Black Holes, ... in single starsystem only worlds
	is_static: bool,

	pos: RefCell<DVec3>,
	force: RefCell<DVec3>,

	radius: SolarRadius,
	mass: f64
	//path: Vec<Vec3>
}

impl PlanetaryMassObject {
	pub fn new(pos: DVec3, radius: SolarRadius, density: f32) -> PlanetaryMassObject {
		PlanetaryMassObject{
			pos: RefCell::new(pos),
			mass: calc_mass(density, calc_volume(radius.in_meters())),
			radius,
			..Default::default()
		}
	}
}

impl Default for PlanetaryMassObject {
	fn default() -> Self {
		PlanetaryMassObject{
			is_static: false,
			pos: RefCell::new(DVec3::ZERO),
			force: RefCell::new(DVec3::ZERO),
			mass: Default::default(),
			radius: SolarRadius::default()
		}
	}
}

/// pre-calculate path's of all the pmo's
/// possibel future improvement: if the path is longer than max, switch mode
/// calculate more points for those pmo's to extend path
/// remove old points after a certain time threshold
/// or: calculate path in both time directions, add and remove points for unfinished paths
/// (path around current position)
pub fn calc_gravity_forces(pmos: &Vec<PlanetaryMassObject>) {
	/* // iterate over all PMOs without repetition, order does not matter
	for entry in pmos.iter().enumerate() {
		for pmo in pmos.iter().enumerate().skip(entry.0 + 1) {
			println!("{} - {}", entry.0, pmo.0);

			let a_mass = entry.1.mass;
			let a_position = entry.1.pos.get();

			let b_mass = pmo.1.mass;
			let b_position = pmo.1.pos.get();

			// calculate forces that influence this object
			let force = crate::gravity::calc_gravity_force(
				a_mass, a_position,
				b_mass, b_position
			);
			entry.1.force.set(entry.1.force.get() + force);
			pmo.1.force.set(pmo.1.force.get() - force);
		}
	} */
	for index in crate::util::UniqueIndexIterator::new(pmos.len()) {
		//println!("{} - {}", index.0, index.1);

		let a_mass = pmos[index.0].mass;
		let a_position = pmos[index.0].pos.to_owned().into_inner();

		let b_mass = pmos[index.1].mass;
		let b_position = pmos[index.1].pos.to_owned().into_inner();

		// calculate forces that influence this object
		let force = crate::gravity::calc_gravity_force(
			a_mass, a_position,
			b_mass, b_position
		);

		/* let a_accel = crate::gravity::calc_acceleration(force, a_mass);
		let b_accel =crate::gravity::calc_acceleration(force, b_mass);

		let a_velocity = crate::gravity::calc_future_velocity(time_step, a_velocity, a_accel);
		let b_velocity =crate::gravity::calc_future_velocity(time_step, b_velocity, b_accel); */

		pmos[index.0].force.borrow_mut().add_assign(force);
		pmos[index.1].force.borrow_mut().add_assign(-force);
	}
}

#[cfg(test)]
mod test {
	use crate::units::*;
    use crate::math::prelude::*;
	use super::PlanetaryMassObject;

	fn setup_pmos() -> Vec<PlanetaryMassObject> {
		vec![
			PlanetaryMassObject::new(dvec3(5000.0, 5000.0, 5000.0), SolarRadius(1000.0), 1.0),
			PlanetaryMassObject::new(DVec3::ZERO, SolarRadius(1000.0), 1.0),
			PlanetaryMassObject::new(dvec3(5000.0, 0.0, 0.0), SolarRadius(1000.0), 1.0),
		]
	}

	#[test]
	fn calc_gravity_forces() {
		let pmos = setup_pmos();
		super::calc_gravity_forces(&pmos);
	}
}