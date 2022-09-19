use std::fmt;

use crate::Vec3;

pub struct Ray {
	pub origin: Vec3,
	pub direction: Vec3,
}

impl Ray {
	pub fn at(&self, t: f64) -> Vec3 {
		self.origin + (self.direction * t)
	}
}

impl fmt::Debug for Ray {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Ray({:?} -> {:?})", self.origin, self.direction)
	}
}