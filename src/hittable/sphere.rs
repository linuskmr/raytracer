use crate::{hittable, Ray, Vec3};
use crate::hittable::{Hit, Hittable};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
	pub center: Vec3,
	pub radius: f64,
}

impl Hittable for Sphere {
	fn hits(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
		let oc = ray.origin - self.center;
		let a = ray.direction.squared_length();
		let half_b = oc.dot(ray.direction);
		let c = oc.squared_length() - self.radius * self.radius;
		let discriminant = half_b * half_b - a * c;

		if discriminant < 0.0 {
			// If discriminant is negative, there are no real roots, so return false as ray misses sphere
			return None;
		}

		// Find the nearest root that lies in the acceptable range
		let discriminant_root = discriminant.sqrt();
		let t1 = (-half_b - discriminant_root) / a;
		let t2 = (-half_b + discriminant_root) / a;

		let t = if (t_min..t_max).contains(&t1) {
			t1
		} else if (t_min..t_max).contains(&t2) {
			t2
		} else {
			return None;
		};

		let point = ray.at(t);
		let outward_normal = (point - self.center) / self.radius;
		let intersection_side = hittable::calc_intersection_side(ray, outward_normal);
		let normal = hittable::calc_normal(intersection_side, outward_normal);
		Some(Hit { point, normal, t, intersection_side })
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hits() {
		let sphere = Sphere {
			center: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
			radius: 1.0,
		};
		let ray = Ray {
			origin: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
			direction: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
		};
		assert!(sphere.hits(ray, 0.0, 1000.0).is_some());
	}

	#[test]
	fn misses() {
		let sphere = Sphere {
			center: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
			radius: 1.0,
		};
		let ray = Ray {
			origin: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
			direction: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
		};
		assert!(sphere.hits(ray, 0.0, 1000.0).is_none());
	}
}