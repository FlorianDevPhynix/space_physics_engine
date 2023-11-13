pub mod clamp {
    use crate::math::prelude::*;
    use crate::math::Vec3;

    pub fn calc_percentage(max_distance: f32, towards: DVec3, point: DVec3) -> f32 {
        let distance = towards.distance(point);
        let result = max_distance as f64 * 100.0 / distance;
        if result < f32::MAX as f64 {
            result as f32
        } else {
            println!("calc_percentage result is bigger than f32");
            f32::MAX
        }
    }

    pub fn calc_vec3_percentage(max_distance: f32, towards: Vec3, point: DVec3) -> f32 {
        calc_percentage(max_distance, towards.as_dvec3(), point)
    }

    pub fn bring_closer(percentage: f32, towards: DVec3, point: DVec3) -> Vec3 {
        point.lerp(towards, (100.0 - percentage as f64) / 100.0).as_vec3()
    }

    pub fn bring_vec3_closer(percentage: f32, towards: Vec3, point: DVec3) -> Vec3 {
        bring_closer(percentage, towards.as_dvec3(), point)
    }

    pub fn calc_scale(percentage: f32, scale: DVec3) -> Vec3 {
        (percentage as f64 * scale / 100.0).as_vec3()
    }
}

fn factorial(n: usize) -> usize {
	let mut result = 1;
	for index in 1..n + 1 {
		result *= index;
	}
	result
}

#[allow(dead_code)]
fn factorial_long(n: usize) -> u64 {
	let mut result = 1_u64;
	for index in 1..n + 1 {
		result *= index as u64;
	}
	result
}

/// Iterates over all combinations of indeces ignoring the order.
#[derive(Debug, Default)]
pub struct UniqueIndexIterator {
    length: usize,

    left_index: usize,
    right_index: usize,
}

impl UniqueIndexIterator {
    pub fn new(length: usize) -> Self {
        Self { length, right_index: 1, ..Default::default() }
    }

	/// Calculate the length of the iterator.
	///
	/// Does not overflow as fast as [`complex_length`](fn@UniqueIndexIterator).
	pub fn length(&self) -> usize {
		(self.length.pow(2) - self.length) / 2
	}

	/// Calculate the length of the iterator using the Binomial Coefficient.
	///
	/// Overflows after inputs of 20. (>=21)
	/// [WolframAlpha (21-2)!][https://www.wolframalpha.com/input?i=factorial+%2821-2%29]
	pub fn complex_length(&self) -> Option<usize> {
		if self.length > 20 { return None; }
		let n = self.length;
		let k = 2;
		Some(factorial(n) / (factorial(k) * factorial(n - k)))
	}

	/// length of the iterator
	#[allow(dead_code)]
	fn complex_length_long(&self) -> u64 {
		let n = self.length;
		let k = 2;
		factorial_long(n) / (factorial_long(k) * factorial_long(n - k))
	}
}

impl Iterator for UniqueIndexIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
		if self.left_index >= self.length - 1 {
			return None;
		}

		let result = (self.left_index, self.right_index);
		if self.right_index >= self.length - 1 {
			// right index reached length, increment left index
			self.left_index += 1;
			self.right_index = self.left_index + 1;
		} else {
			self.right_index += 1;
		}
		Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::UniqueIndexIterator;

	#[test]
	fn faculty() {
		use super::factorial;
		assert_eq!(factorial(3), 3*2*1);
		assert_eq!(factorial(5), 5*4*3*2*1);
		assert_eq!(factorial(7), 7*6*5*4*3*2*1);
	}

	#[test]
	fn unique_index_iterator_length() {
		assert_eq!(UniqueIndexIterator::new(3).length(), 3);
		assert_eq!(UniqueIndexIterator::new(5).length(), 10);
		assert_eq!(UniqueIndexIterator::new(7).length(), 21);
		assert_eq!(UniqueIndexIterator::new(30).length(), 435);
	}

	#[test]
	fn unique_length_half_of_simple_calc() {
		for length in 2..30 {// max 20, overflows after that
			let iter = UniqueIndexIterator::new(length);

			let complex = iter.complex_length();
			if length < 21 {
				assert_eq!(
					complex.unwrap(),
					iter.length(),
					"length={}", length
				);
			} else {
				assert_eq!(
					complex,
					None
				);
			}
		}
	}
}