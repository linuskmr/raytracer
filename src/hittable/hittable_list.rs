//! Implements [`Hittable`] for a list of [`Hittable`]s.

use crate::hittable::{Hit, Hittable, Ray};

// Implements hittable for a vector of hittable objects (or some sort of reference).
impl<H: AsRef<dyn Hittable>> Hittable for &[H] {
	fn hits(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
		self.iter()
			// Check whether the ray hits the hittable object
			.filter_map(|hittable| hittable.as_ref().hits(ray, t_min, t_max))
			// Find the hit with the smallest t value
			.min_by(|hit1, hit2| hit1.t.total_cmp(&hit2.t))
	}
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;

	use crate::{hittable::Sphere, material, Vec3};

	use super::*;

	/// Mainly to make sure the typechecker is fine with &[AsRef<dyn Hittable>] as a Hittable
	#[test]
	fn hittable_list() {
		let hittables: Vec<Box<dyn Hittable>> = vec![
			Box::new(Sphere {
				center: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
				radius: 1.0,
				material: Rc::new(material::Lambertian::default()),
			}),
			Box::new(Sphere {
				center: Vec3 { x: 0.0, y: 0.0, z: 2.0 },
				radius: 1.0,
				material: Rc::new(material::Lambertian::default()),
			}),
		];
		let ray = Ray {
			origin: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
			direction: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
		};

		let hit = hittables.as_slice().hits(ray, 0.0, f64::INFINITY);
		assert_eq!(hit.map(|h| h.point), Some(Vec3 { x: 0.0, y: 0.0, z: -1.0 }));
	}
}