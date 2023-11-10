use core::ops::AddAssign;

use crate::math::prelude::*;

fn calc_mass(density: f32, volume: f32) -> f64 {
	density as f64 * volume as f64
}

fn calc_volume(radius: f32) -> f32 {
	4.0 / 3.0 * std::f32::consts::PI * radius.powi(3)
}

/// A planetary-mass object (PMO), planemo, or planetary body
#[derive(Debug)]
pub struct PlanetaryMassObject<'a> {
	/// This PMO will not be affected by any gravitational forces.
	/// Example: Stars, Black Holes, ... in single starsystem only worlds
	is_static: bool,

	pos: &'a mut DVec3,
	force: &'a mut DVec3,

	radius: f32,
	mass: f64
	//path: Vec<Vec3>
}

impl<'a> PlanetaryMassObject<'a> {
	pub fn new(radius: f32, density: f32) -> PlanetaryMassObject<'a> {
		PlanetaryMassObject{
			mass: calc_mass(density, calc_volume(radius)),
			radius,
			..Default::default()
		}
	}
}

impl<'a> Default for PlanetaryMassObject<'a> {
	fn default() -> Self {
		PlanetaryMassObject{
			is_static: false,
			pos: &mut DVec3::ZERO,
			force: &mut DVec3::ZERO,
			mass: 0.0,
			radius: 0.0
		}
	}
}

// pre-calculate path's of all the pmo's
// possibel future improvement: if the path is longer than max, switch mode
// calculate more points for those pmo's to extend path
// remove old points after a certain time threshold
// or: calculate path in both time directions, add and remove points for unfinished paths
// (path around current position)
pub fn calc_gravity_forces(pmos: Vec<PlanetaryMassObject>) {
	// iterate over all PMOs without repetition, order does not matter
	for entry in pmos.iter().enumerate() {
		for pmo in pmos.iter().enumerate().skip(entry.0 + 1) {
			println!("{} - {}", entry.0, pmo.0);

			/* let a_mass = entry.1.mass;
			let a_position = *entry.1.pos;

			let b_mass = pmo.1.mass;
			let b_position = *pmo.1.pos;

			// calculate forces that influence this object
			entry.1.force.add_assign(crate::gravity::calc_gravity_force(
				a_mass, a_position,
				b_mass, b_position
			)); */
		}
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn calc_gravity_forces() {
		use super::PlanetaryMassObject;
		let pmos = vec![
			PlanetaryMassObject::new(1000.0, 1.0),
			PlanetaryMassObject::new(1000.0, 1.0),
			PlanetaryMassObject::new(1000.0, 1.0),
		];
		super::calc_gravity_forces(pmos);
	}
}

/* use core::{iter::{Iterator, Enumerate}, borrow::BorrowMut};
/// A unique iterator that ignores the order.
struct UniqueIterator<'a, I, T>
	where I: Iterator<Item = (usize, &'a T)> + Clone
{
    current_item: Option<(usize, &'a T)>,
	main_iter: &'a mut I,
	second_iter: &'a mut I
}

// we want our count to start at one, so let's add a new() method to help.
// This isn't strictly necessary, but is convenient. Note that we start
// `count` at zero, we'll see why in `next()`'s implementation below.
impl<'a, I, T> UniqueIterator<'a, I, T>
	where I: Iterator<Item = (usize, &'a T)> + Clone
{
    fn new(iter: &'a mut I) -> UniqueIterator<'a, I, T> {
        UniqueIterator { current_item: iter.next(), main_iter: &mut iter, second_iter: &mut iter.clone() }
    }
}

impl<'a, I, T> Iterator for UniqueIterator<'a, I, T>
	where I: Iterator<Item = (usize, &'a T)> + Clone
{
    // we will be counting with usize
    type Item = (&'a T, &'a T);

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
		if let Some(current_item) = &self.current_item {
			let result = self.second_iter.next();
			if let Some(element) = result {
				return Some((&current_item.1, &element.1));
			} else {
				self.current_item = self.main_iter.next();
				self.second_iter = &mut self.main_iter.clone();

				self.next()
			}
		} else {
			None
		}
    }
} */