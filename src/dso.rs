use crate::math::prelude::*;

#[derive(Debug)]
pub struct DynamicSpaceObject {
	is_static: bool
}

impl Default for DynamicSpaceObject {
	fn default() -> Self {
        Self {
			is_static: false
		}
    }
}

// PMO path cache: some collection (with insert) of a struct that holds the PMOs calculated position,
// the timestep and time (integer based on multiple of physics timestep);
// when re-calculating compare the time and -step at the current position, with the next value
// in the cache, if timestep is different and current time is before cache point,
// calculate and insert new value, if timestep is different and current time is after cache point,
// skip over value, this way we don't delete values that could become relevant later;

// only problem still remains (for both PMOs and DSOs): when re-meeting with the start position,
// what if the calculated position based on the timestep doesn't match the start position;
// this could result in all sorts of problems, invalidate the whole PMO cache