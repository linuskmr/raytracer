use std::fmt;

use crate::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ray {
	pub origin: Vec3,
	pub direction: Vec3,
}

impl Ray {
	pub fn at(&self, t: f64) -> Vec3 {
		self.origin + (self.direction * t)
	}
}

impl fmt::Display for Ray {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Ray({:?} -> {:?})", self.origin, self.direction)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn at() {
		let ray = Ray {
			origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
			direction: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
		};
		assert_eq!(ray.at(0.0), Vec3 { x: 0.0, y: 0.0, z: 0.0 });
		assert_eq!(ray.at(1.0), Vec3 { x: 1.0, y: 0.0, z: 0.0 });
		assert_eq!(ray.at(2.0), Vec3 { x: 2.0, y: 0.0, z: 0.0 });
	}
}