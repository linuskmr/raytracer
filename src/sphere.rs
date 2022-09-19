use crate::{Ray, Vec3};

pub struct Sphere {
	pub center: Vec3,
	pub radius: f64,
}

impl Sphere {
	pub fn hits(&self, ray: &Ray) -> bool {
		let oc = ray.origin - self.center;
		let a = ray.direction.dot(ray.direction);
		let b = 2.0 * oc.dot(ray.direction);
		let c = oc.dot(oc) - self.radius * self.radius;
		let discriminant = b * b - 4.0 * a * c;
		// If discriminant is negative, there are no real roots, so return false as ray misses sphere
		// If discriminant is zero, there is one real root, so return true as ray hits sphere
		// If discriminant is positive, there are two real roots, so return true as ray hits sphere
		discriminant > 0.0
	}
}