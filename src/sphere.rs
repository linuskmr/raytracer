use crate::{Ray, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
	pub center: Vec3,
	pub radius: f64,
}

impl Sphere {
	pub fn hits(&self, ray: &Ray) -> Option<f64> {
		let oc = ray.origin - self.center;
		let a = ray.direction.dot(ray.direction);
		let b = 2.0 * oc.dot(ray.direction);
		let c = oc.dot(oc) - self.radius * self.radius;
		let discriminant = b * b - 4.0 * a * c;

		if discriminant < 0.0 {
			// If discriminant is negative, there are no real roots, so return false as ray misses sphere
			return None;
		}

		// If discriminant is zero, there is one real root, so return true as ray hits sphere
		// If discriminant is positive, there are two real roots, so return true as ray hits sphere
		let t1 = (-b - discriminant.sqrt()) / (2.0 * a); // Way used by tutorial
		let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
		assert!(t1 < t2, "t1 (tutorial) should be less than t2");
		Some(t1)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hits() {
		let sphere = Sphere {
			center: Vec3(0.0, 0.0, 0.0),
			radius: 1.0,
		};
		let ray = Ray {
			origin: Vec3(0.0, 0.0, -5.0),
			direction: Vec3(0.0, 0.0, 1.0),
		};
		assert!(sphere.hits(&ray).is_some());
	}

	#[test]
	fn misses() {
		let sphere = Sphere {
			center: Vec3(0.0, 0.0, 0.0),
			radius: 1.0,
		};
		let ray = Ray {
			origin: Vec3(0.0, 0.0, -5.0),
			direction: Vec3(0.0, 1.0, 0.0),
		};
		assert!(sphere.hits(&ray).is_none());
	}
}